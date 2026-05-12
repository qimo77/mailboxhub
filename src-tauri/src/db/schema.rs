use std::fs;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tauri::{Manager, Runtime};

use crate::error::AppResult;

pub async fn init_database<R: Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> AppResult<(SqlitePool, String)> {
    let app_dir = app_handle.path().app_data_dir()?;
    fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("mailbox.sqlite");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    let options = db_url
        .parse::<SqliteConnectOptions>()?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePool::connect_with(options).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
          id TEXT PRIMARY KEY,
          email TEXT NOT NULL UNIQUE,
          password TEXT NOT NULL,
          client_id TEXT NOT NULL,
          refresh_token TEXT NOT NULL,
          access_token TEXT,
          access_token_expires_at TEXT,
          is_active INTEGER NOT NULL DEFAULT 1,
          auto_connect INTEGER NOT NULL DEFAULT 1,
          last_sync_uid INTEGER,
          last_sync_at TEXT,
          last_error TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS emails (
          id TEXT PRIMARY KEY,
          account_id TEXT NOT NULL,
          uid INTEGER NOT NULL,
          message_id TEXT,
          subject TEXT NOT NULL,
          sender_name TEXT,
          sender_email TEXT,
          received_at TEXT,
          body_text TEXT,
          body_html TEXT,
          codes_json TEXT NOT NULL DEFAULT '[]',
          is_read INTEGER NOT NULL DEFAULT 0,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL,
          FOREIGN KEY(account_id) REFERENCES accounts(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_emails_account_uid ON emails(account_id, uid);",
    )
    .execute(&pool)
    .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_emails_account_received ON emails(account_id, received_at DESC);")
        .execute(&pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_emails_unread ON emails(account_id, is_read);")
        .execute(&pool)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
          key TEXT PRIMARY KEY,
          value TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;

    seed_setting(&pool, "poll_interval_seconds", "15").await?;
    seed_setting(&pool, "theme", "light").await?;
    seed_setting(&pool, "notifications_enabled", "true").await?;
    seed_setting(&pool, "launch_auto_connect", "true").await?;

    Ok((pool, db_path.to_string_lossy().to_string()))
}

async fn seed_setting(pool: &SqlitePool, key: &str, value: &str) -> AppResult<()> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        r#"
        INSERT INTO settings (key, value, updated_at)
        VALUES (?, ?, ?)
        ON CONFLICT(key) DO NOTHING;
        "#,
    )
    .bind(key)
    .bind(value)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}
