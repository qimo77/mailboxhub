use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub poll_interval_seconds: u64,
    pub theme: String,
    pub notifications_enabled: bool,
    pub launch_auto_connect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsPatch {
    pub poll_interval_seconds: Option<u64>,
    pub theme: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub launch_auto_connect: Option<bool>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            poll_interval_seconds: 5,
            theme: "light".to_string(),
            notifications_enabled: true,
            launch_auto_connect: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppSettings;

    #[test]
    fn default_theme_is_light() {
        assert_eq!(AppSettings::default().theme, "light");
    }

    #[test]
    fn default_poll_interval_uses_minimum_supported_refresh() {
        assert_eq!(AppSettings::default().poll_interval_seconds, 5);
    }
}
