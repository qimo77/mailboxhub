use imap::types::Fetch;
use sqlx::SqlitePool;

use crate::{
    db::{accounts, emails},
    error::{AppError, AppResult},
    mail::{oauth, parser},
    models::account::{StoredAccount, SyncResult},
};

pub async fn sync_account(pool: &SqlitePool, account: StoredAccount) -> AppResult<SyncResult> {
    let token = oauth::access_token(pool, &account).await?;
    let pool = pool.clone();
    let account_for_blocking = account.clone();
    let fetched_messages =
        tokio::task::spawn_blocking(move || fetch_messages(account_for_blocking, token))
            .await
            .map_err(|error| AppError::Mail(format!("IMAP task failed: {error}")))??;

    let fetched = fetched_messages.len();
    let mut inserted = 0;
    let mut skipped = 0;
    let mut max_uid = account.last_sync_uid;

    for (uid, body) in fetched_messages {
        max_uid = Some(max_uid.unwrap_or(0).max(uid));
        let email = parser::parse_message(&account.id, uid, &body);
        if emails::insert_email_if_new(&pool, &email).await? {
            inserted += 1;
        } else {
            skipped += 1;
        }
    }

    accounts::update_sync_success(&pool, &account.id, max_uid).await?;

    Ok(SyncResult {
        account_id: account.id,
        fetched,
        inserted,
        skipped,
        error: None,
    })
}

fn fetch_messages(account: StoredAccount, access_token: String) -> AppResult<Vec<(i64, Vec<u8>)>> {
    let client = imap::ClientBuilder::new("outlook.office365.com", 993)
        .connect()
        .map_err(|error| AppError::Mail(format!("IMAP connect failed: {error}")))?;

    let authenticator = OAuth2Authenticator {
        user: account.email.clone(),
        access_token,
    };
    let mut session = client
        .authenticate("XOAUTH2", &authenticator)
        .map_err(|(error, _)| AppError::Mail(format!("IMAP XOAUTH2 failed: {error}")))?;

    session
        .select("INBOX")
        .map_err(|error| AppError::Mail(format!("IMAP select INBOX failed: {error}")))?;

    let sequence = if let Some(last_uid) = account.last_sync_uid {
        format!("{}:*", last_uid + 1)
    } else {
        let uids = session
            .uid_search("ALL")
            .map_err(|error| AppError::Mail(format!("IMAP UID search failed: {error}")))?;
        let mut sorted = uids.into_iter().collect::<Vec<_>>();
        sorted.sort_unstable();
        let latest = sorted.into_iter().rev().take(50).collect::<Vec<_>>();
        if latest.is_empty() {
            let _ = session.logout();
            return Ok(Vec::new());
        }
        latest
            .into_iter()
            .rev()
            .map(|uid| uid.to_string())
            .collect::<Vec<_>>()
            .join(",")
    };

    let fetches = session
        .uid_fetch(sequence, fetch_body_attributes())
        .map_err(|error| AppError::Mail(format!("IMAP fetch failed: {error}")))?;
    let mut messages = Vec::new();

    for fetch in fetches.iter() {
        if let Some((uid, body)) = fetch_to_body(fetch) {
            messages.push((uid, body));
        }
    }

    let _ = session.logout();
    Ok(messages)
}

struct OAuth2Authenticator {
    user: String,
    access_token: String,
}

impl imap::Authenticator for OAuth2Authenticator {
    type Response = String;

    fn process(&self, _data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

fn fetch_body_attributes() -> &'static str {
    "(UID RFC822)"
}

fn fetch_to_body(fetch: &Fetch) -> Option<(i64, Vec<u8>)> {
    let uid = i64::from(fetch.uid?);
    let body = fetch.body()?.to_vec();
    Some((uid, body))
}

#[cfg(test)]
mod tests {
    use super::fetch_body_attributes;

    #[test]
    fn fetch_body_attributes_parenthesize_multiple_items_for_outlook() {
        assert_eq!(fetch_body_attributes(), "(UID RFC822)");
    }
}
