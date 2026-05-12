use futures::{stream, StreamExt};
use tauri::{Emitter, Runtime};
use tokio::time::{sleep, Duration};

use crate::{
    app_state::AppState,
    db::{accounts, emails, settings},
    mail::imap_client,
    models::events::NewMailEvent,
};

pub async fn run_mail_poller<R: Runtime>(app: tauri::AppHandle<R>, state: AppState) {
    loop {
        let app_settings = settings::get_settings(&state.db).await.unwrap_or_default();
        let interval = app_settings.poll_interval_seconds.max(5);

        if !app_settings.launch_auto_connect {
            sleep(Duration::from_secs(interval)).await;
            continue;
        }

        if let Ok(account_list) = accounts::list_active_auto_connect_accounts(&state.db).await {
            stream::iter(account_list)
                .for_each_concurrent(4, |account| {
                    let app = app.clone();
                    let db = state.db.clone();
                    async move {
                        let account_id = account.id.clone();
                        let previous_unread = emails::unread_state(&db, &account_id)
                            .await
                            .map(|state| state.unread_count)
                            .unwrap_or(0);

                        match imap_client::sync_account(&db, account.clone()).await {
                            Ok(result) if result.inserted > 0 => {
                                if let Ok(updated) = emails::unread_state(&db, &account_id).await {
                                    if updated.unread_count > previous_unread {
                                        let latest =
                                            emails::latest_unread_summary(&db, &account_id)
                                                .await
                                                .ok()
                                                .flatten();
                                        let _ = app.emit(
                                            "mail:new",
                                            NewMailEvent {
                                                account_id: account_id.clone(),
                                                email_id: latest
                                                    .as_ref()
                                                    .map(|email| email.id.clone())
                                                    .unwrap_or_default(),
                                                subject: latest
                                                    .as_ref()
                                                    .map(|email| email.subject.clone())
                                                    .unwrap_or_else(|| "New mail".to_string()),
                                                sender: latest
                                                    .as_ref()
                                                    .and_then(|email| {
                                                        email
                                                            .sender_name
                                                            .clone()
                                                            .or(email.sender_email.clone())
                                                    })
                                                    .unwrap_or_else(|| account.email.clone()),
                                                code: latest
                                                    .as_ref()
                                                    .and_then(|email| email.codes.first().cloned()),
                                                unread_count: updated.unread_count,
                                            },
                                        );
                                    }
                                }
                            }
                            Ok(_) => {}
                            Err(error) => {
                                let _ = accounts::update_sync_error(
                                    &db,
                                    &account_id,
                                    &error.to_string(),
                                )
                                .await;
                            }
                        }
                    }
                })
                .await;
        }

        sleep(Duration::from_secs(interval)).await;
    }
}
