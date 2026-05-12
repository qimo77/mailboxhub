use chrono::Utc;
use sqlx::{Row, SqlitePool};

use crate::{
    error::AppResult,
    models::email::{AccountUnreadState, EmailDetail, EmailSummary, StoredEmail},
};

pub async fn insert_email_if_new(pool: &SqlitePool, email: &StoredEmail) -> AppResult<bool> {
    let now = Utc::now().to_rfc3339();
    let codes_json = serde_json::to_string(&email.codes)?;
    let result = sqlx::query(
        r#"
        INSERT OR IGNORE INTO emails (
            id, account_id, uid, message_id, subject, sender_name, sender_email,
            received_at, body_text, body_html, codes_json, is_read, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)
        "#,
    )
    .bind(&email.id)
    .bind(&email.account_id)
    .bind(email.uid)
    .bind(&email.message_id)
    .bind(&email.subject)
    .bind(&email.sender_name)
    .bind(&email.sender_email)
    .bind(&email.received_at)
    .bind(&email.body_text)
    .bind(&email.body_html)
    .bind(codes_json)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn list_email_summaries(
    pool: &SqlitePool,
    account_id: &str,
) -> AppResult<Vec<EmailSummary>> {
    let rows = sqlx::query(
        r#"
        SELECT id, account_id, uid, subject, sender_name, sender_email, received_at, codes_json, is_read
        FROM emails
        WHERE account_id = ?
        ORDER BY COALESCE(received_at, created_at) DESC, uid DESC
        LIMIT 500
        "#,
    )
    .bind(account_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(row_to_summary).collect())
}

pub async fn get_email_detail(pool: &SqlitePool, email_id: &str) -> AppResult<EmailDetail> {
    let row = sqlx::query(
        r#"
        SELECT id, account_id, uid, subject, sender_name, sender_email, received_at,
               body_text, body_html, codes_json, is_read
        FROM emails
        WHERE id = ?
        "#,
    )
    .bind(email_id)
    .fetch_one(pool)
    .await?;

    let codes_json: String = row.get("codes_json");
    Ok(EmailDetail {
        id: row.get("id"),
        account_id: row.get("account_id"),
        uid: row.get("uid"),
        subject: row.get("subject"),
        sender_name: row.get("sender_name"),
        sender_email: row.get("sender_email"),
        received_at: row.get("received_at"),
        body_text: row.get("body_text"),
        body_html: row.get("body_html"),
        codes: serde_json::from_str(&codes_json).unwrap_or_default(),
        is_read: row.get::<i64, _>("is_read") == 1,
    })
}

pub async fn mark_email_read(pool: &SqlitePool, email_id: &str) -> AppResult<AccountUnreadState> {
    let account_id: String = sqlx::query("SELECT account_id FROM emails WHERE id = ?")
        .bind(email_id)
        .fetch_one(pool)
        .await?
        .get("account_id");

    sqlx::query("UPDATE emails SET is_read = 1, updated_at = ? WHERE id = ?")
        .bind(Utc::now().to_rfc3339())
        .bind(email_id)
        .execute(pool)
        .await?;

    unread_state(pool, &account_id).await
}

pub async fn unread_state(pool: &SqlitePool, account_id: &str) -> AppResult<AccountUnreadState> {
    let unread_count: i64 = sqlx::query(
        "SELECT COUNT(*) AS unread_count FROM emails WHERE account_id = ? AND is_read = 0",
    )
    .bind(account_id)
    .fetch_one(pool)
    .await?
    .get("unread_count");

    Ok(AccountUnreadState {
        account_id: account_id.to_string(),
        unread_count,
    })
}

pub async fn latest_unread_summary(
    pool: &SqlitePool,
    account_id: &str,
) -> AppResult<Option<EmailSummary>> {
    let row = sqlx::query(
        r#"
        SELECT id, account_id, uid, subject, sender_name, sender_email, received_at, codes_json, is_read
        FROM emails
        WHERE account_id = ? AND is_read = 0
        ORDER BY COALESCE(received_at, created_at) DESC, uid DESC
        LIMIT 1
        "#,
    )
    .bind(account_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(row_to_summary))
}

fn row_to_summary(row: sqlx::sqlite::SqliteRow) -> EmailSummary {
    let codes_json: String = row.get("codes_json");
    EmailSummary {
        id: row.get("id"),
        account_id: row.get("account_id"),
        uid: row.get("uid"),
        subject: row.get("subject"),
        sender_name: row.get("sender_name"),
        sender_email: row.get("sender_email"),
        received_at: row.get("received_at"),
        codes: serde_json::from_str(&codes_json).unwrap_or_default(),
        is_read: row.get::<i64, _>("is_read") == 1,
    }
}
