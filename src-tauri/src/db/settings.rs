use chrono::Utc;
use sqlx::{Row, SqlitePool};

use crate::{
    error::AppResult,
    models::settings::{AppSettings, AppSettingsPatch},
};

pub async fn get_settings(pool: &SqlitePool) -> AppResult<AppSettings> {
    let rows = sqlx::query("SELECT key, value FROM settings")
        .fetch_all(pool)
        .await?;
    let mut settings = AppSettings::default();

    for row in rows {
        let key: String = row.get("key");
        let value: String = row.get("value");
        match key.as_str() {
            "poll_interval_seconds" => {
                settings.poll_interval_seconds = value.parse().unwrap_or(15).max(5);
            }
            "theme" => settings.theme = value,
            "notifications_enabled" => settings.notifications_enabled = value == "true",
            "launch_auto_connect" => settings.launch_auto_connect = value == "true",
            _ => {}
        }
    }

    Ok(settings)
}

pub async fn update_settings(pool: &SqlitePool, patch: AppSettingsPatch) -> AppResult<AppSettings> {
    if let Some(value) = patch.poll_interval_seconds {
        set_setting(pool, "poll_interval_seconds", &value.max(5).to_string()).await?;
    }
    if let Some(value) = patch.theme {
        set_setting(pool, "theme", &value).await?;
    }
    if let Some(value) = patch.notifications_enabled {
        set_setting(
            pool,
            "notifications_enabled",
            if value { "true" } else { "false" },
        )
        .await?;
    }
    if let Some(value) = patch.launch_auto_connect {
        set_setting(
            pool,
            "launch_auto_connect",
            if value { "true" } else { "false" },
        )
        .await?;
    }

    get_settings(pool).await
}

async fn set_setting(pool: &SqlitePool, key: &str, value: &str) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO settings (key, value, updated_at)
        VALUES (?, ?, ?)
        ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at
        "#,
    )
    .bind(key)
    .bind(value)
    .bind(Utc::now().to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}
