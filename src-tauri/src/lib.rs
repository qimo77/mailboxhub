mod app_state;
mod commands;
mod db;
mod error;
mod mail;
mod models;

use app_state::AppState;
use db::schema::init_database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let (db, database_path) =
                tauri::async_runtime::block_on(init_database(app.handle()))
                    .map_err(|error| Box::<dyn std::error::Error>::from(error.to_string()))?;
            let state = AppState { db, database_path };
            app.manage(state.clone());

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                mail::poller::run_mail_poller(app_handle, state).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::accounts::import_accounts,
            commands::accounts::list_accounts,
            commands::accounts::sync_account_now,
            commands::accounts::mark_account_all_read,
            commands::emails::list_emails,
            commands::emails::get_email,
            commands::emails::mark_email_read,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::get_database_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
