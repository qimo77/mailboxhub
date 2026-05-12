use crate::{
    app_state::AppState,
    db::{accounts, emails},
    error::AppResult,
    mail::{imap_client, import::parse_account_import},
    models::account::{AccountSummary, ImportAccountsResult, SyncResult},
};

#[tauri::command]
pub async fn import_accounts(
    state: tauri::State<'_, AppState>,
    input: String,
) -> AppResult<ImportAccountsResult> {
    let (parsed, invalid) = parse_account_import(&input);
    accounts::import_accounts_result(&state.db, parsed, invalid).await
}

#[tauri::command]
pub async fn list_accounts(state: tauri::State<'_, AppState>) -> AppResult<Vec<AccountSummary>> {
    accounts::list_account_summaries(&state.db).await
}

#[tauri::command]
pub async fn sync_account_now(
    state: tauri::State<'_, AppState>,
    account_id: String,
) -> AppResult<SyncResult> {
    let account = accounts::get_account(&state.db, &account_id).await?;
    match imap_client::sync_account(&state.db, account).await {
        Ok(result) => Ok(result),
        Err(error) => {
            let message = error.to_string();
            accounts::update_sync_error(&state.db, &account_id, &message).await?;
            Ok(SyncResult {
                account_id,
                fetched: 0,
                inserted: 0,
                skipped: 0,
                error: Some(message),
            })
        }
    }
}

#[tauri::command]
pub async fn mark_account_all_read(
    state: tauri::State<'_, AppState>,
    account_id: String,
) -> AppResult<crate::models::email::AccountUnreadState> {
    sqlx::query("UPDATE emails SET is_read = 1, updated_at = ? WHERE account_id = ?")
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(&account_id)
        .execute(&state.db)
        .await?;
    emails::unread_state(&state.db, &account_id).await
}
