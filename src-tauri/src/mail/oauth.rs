use chrono::{Duration, Utc};
use serde::Deserialize;

use crate::{
    db::accounts,
    error::{AppError, AppResult},
    models::account::StoredAccount,
};

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: Option<i64>,
    refresh_token: Option<String>,
}

pub async fn access_token(pool: &sqlx::SqlitePool, account: &StoredAccount) -> AppResult<String> {
    if let (Some(token), Some(expires_at)) =
        (&account.access_token, &account.access_token_expires_at)
    {
        if let Ok(expires_at) = chrono::DateTime::parse_from_rfc3339(expires_at) {
            if expires_at.with_timezone(&Utc) > Utc::now() + Duration::seconds(60) {
                return Ok(token.clone());
            }
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .post("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .form(&[
            ("client_id", account.client_id.as_str()),
            ("refresh_token", account.refresh_token.as_str()),
            ("grant_type", "refresh_token"),
            (
                "scope",
                "https://outlook.office.com/IMAP.AccessAsUser.All offline_access",
            ),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Mail(format!(
            "OAuth token refresh failed: {status} {body}"
        )));
    }

    let token = response.json::<TokenResponse>().await?;
    let expires_at =
        Utc::now() + Duration::seconds(token.expires_in.unwrap_or(3600).saturating_sub(60));

    accounts::save_access_token(
        pool,
        &account.id,
        &token.access_token,
        &expires_at.to_rfc3339(),
    )
    .await?;

    if let Some(refresh_token) = token.refresh_token {
        sqlx::query("UPDATE accounts SET refresh_token = ?, updated_at = ? WHERE id = ?")
            .bind(refresh_token)
            .bind(Utc::now().to_rfc3339())
            .bind(&account.id)
            .execute(pool)
            .await?;
    }

    Ok(token.access_token)
}
