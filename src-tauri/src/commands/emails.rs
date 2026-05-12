use crate::{
    app_state::AppState,
    db::emails,
    error::AppResult,
    models::email::{AccountUnreadState, EmailDetail, EmailSummary},
};

#[tauri::command]
pub async fn list_emails(
    state: tauri::State<'_, AppState>,
    account_id: String,
) -> AppResult<Vec<EmailSummary>> {
    emails::list_email_summaries(&state.db, &account_id).await
}

#[tauri::command]
pub async fn get_email(
    state: tauri::State<'_, AppState>,
    email_id: String,
) -> AppResult<EmailDetail> {
    emails::get_email_detail(&state.db, &email_id).await
}

#[tauri::command]
pub async fn mark_email_read(
    state: tauri::State<'_, AppState>,
    email_id: String,
) -> AppResult<AccountUnreadState> {
    emails::mark_email_read(&state.db, &email_id).await
}
