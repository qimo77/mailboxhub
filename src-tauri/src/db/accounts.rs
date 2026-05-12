use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::account::{AccountImportInput, AccountSummary, ImportAccountsResult, StoredAccount},
};

pub async fn upsert_accounts(
    pool: &SqlitePool,
    accounts: Vec<AccountImportInput>,
) -> AppResult<(usize, usize)> {
    let mut imported = 0;
    let mut updated = 0;

    for account in accounts {
        let existing = sqlx::query("SELECT id FROM accounts WHERE email = ?")
            .bind(&account.email)
            .fetch_optional(pool)
            .await?;
        let now = Utc::now().to_rfc3339();

        if let Some(row) = existing {
            let id: String = row.get("id");
            sqlx::query(
                r#"
                UPDATE accounts
                SET password = ?, client_id = ?, refresh_token = ?, last_error = NULL, updated_at = ?
                WHERE id = ?
                "#,
            )
            .bind(account.password)
            .bind(account.client_id)
            .bind(account.refresh_token)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
            updated += 1;
        } else {
            sqlx::query(
                r#"
                INSERT INTO accounts (
                    id, email, password, client_id, refresh_token, is_active, auto_connect, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, 1, 1, ?, ?)
                "#,
            )
            .bind(Uuid::new_v4().to_string())
            .bind(account.email)
            .bind(account.password)
            .bind(account.client_id)
            .bind(account.refresh_token)
            .bind(&now)
            .bind(&now)
            .execute(pool)
            .await?;
            imported += 1;
        }
    }

    Ok((imported, updated))
}

pub async fn list_account_summaries(pool: &SqlitePool) -> AppResult<Vec<AccountSummary>> {
    let rows = sqlx::query(
        r#"
        SELECT
            a.id,
            a.email,
            a.is_active,
            a.auto_connect,
            a.last_error,
            a.last_sync_at,
            COALESCE(SUM(CASE WHEN e.is_read = 0 THEN 1 ELSE 0 END), 0) AS unread_count
        FROM accounts a
        LEFT JOIN emails e ON e.account_id = a.id
        GROUP BY a.id
        ORDER BY a.created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let email: String = row.get("email");
            let display_name = email.split('@').next().unwrap_or(&email).to_string();
            AccountSummary {
                id: row.get("id"),
                email,
                display_name,
                unread_count: row.get::<i64, _>("unread_count"),
                is_active: row.get::<i64, _>("is_active") == 1,
                auto_connect: row.get::<i64, _>("auto_connect") == 1,
                last_error: row.get("last_error"),
                last_sync_at: row.get("last_sync_at"),
            }
        })
        .collect())
}

pub async fn import_accounts_result(
    pool: &SqlitePool,
    accounts: Vec<AccountImportInput>,
    invalid: Vec<crate::models::account::InvalidImportLine>,
) -> AppResult<ImportAccountsResult> {
    let (imported, updated) = upsert_accounts(pool, accounts).await?;
    Ok(ImportAccountsResult {
        imported,
        updated,
        invalid,
    })
}

pub async fn get_account(pool: &SqlitePool, account_id: &str) -> AppResult<StoredAccount> {
    let row = sqlx::query(
        r#"
        SELECT id, email, client_id, refresh_token, access_token, access_token_expires_at, last_sync_uid
        FROM accounts
        WHERE id = ? AND is_active = 1
        "#,
    )
    .bind(account_id)
    .fetch_one(pool)
    .await?;

    Ok(StoredAccount {
        id: row.get("id"),
        email: row.get("email"),
        client_id: row.get("client_id"),
        refresh_token: row.get("refresh_token"),
        access_token: row.get("access_token"),
        access_token_expires_at: row.get("access_token_expires_at"),
        last_sync_uid: row.get("last_sync_uid"),
    })
}

pub async fn list_active_auto_connect_accounts(pool: &SqlitePool) -> AppResult<Vec<StoredAccount>> {
    let rows = sqlx::query(
        r#"
        SELECT id, email, client_id, refresh_token, access_token, access_token_expires_at, last_sync_uid
        FROM accounts
        WHERE is_active = 1 AND auto_connect = 1
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| StoredAccount {
            id: row.get("id"),
            email: row.get("email"),
            client_id: row.get("client_id"),
            refresh_token: row.get("refresh_token"),
            access_token: row.get("access_token"),
            access_token_expires_at: row.get("access_token_expires_at"),
            last_sync_uid: row.get("last_sync_uid"),
        })
        .collect())
}

pub async fn save_access_token(
    pool: &SqlitePool,
    account_id: &str,
    access_token: &str,
    expires_at: &str,
) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE accounts
        SET access_token = ?, access_token_expires_at = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(access_token)
    .bind(expires_at)
    .bind(Utc::now().to_rfc3339())
    .bind(account_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_sync_success(
    pool: &SqlitePool,
    account_id: &str,
    last_uid: Option<i64>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE accounts
        SET last_sync_uid = COALESCE(?, last_sync_uid), last_sync_at = ?, last_error = NULL, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(last_uid)
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .bind(account_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_sync_error(pool: &SqlitePool, account_id: &str, error: &str) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE accounts
        SET last_error = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(error)
    .bind(Utc::now().to_rfc3339())
    .bind(account_id)
    .execute(pool)
    .await?;
    Ok(())
}
