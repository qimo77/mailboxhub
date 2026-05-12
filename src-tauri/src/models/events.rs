use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMailEvent {
    pub account_id: String,
    pub email_id: String,
    pub subject: String,
    pub sender: String,
    pub code: Option<String>,
    pub unread_count: i64,
}
