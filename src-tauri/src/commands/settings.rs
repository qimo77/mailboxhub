use crate::{
    app_state::AppState,
    db::settings,
    error::AppResult,
    models::settings::{AppSettings, AppSettingsPatch},
};

#[tauri::command]
pub async fn get_settings(state: tauri::State<'_, AppState>) -> AppResult<AppSettings> {
    settings::get_settings(&state.db).await
}

#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, AppState>,
    patch: AppSettingsPatch,
) -> AppResult<AppSettings> {
    settings::update_settings(&state.db, patch).await
}

#[tauri::command]
pub async fn get_database_path(state: tauri::State<'_, AppState>) -> AppResult<String> {
    Ok(state.database_path.clone())
}
