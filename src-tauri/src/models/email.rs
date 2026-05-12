use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSummary {
    pub id: String,
    pub account_id: String,
    pub uid: i64,
    pub subject: String,
    pub sender_name: Option<String>,
    pub sender_email: Option<String>,
    pub received_at: Option<String>,
    pub codes: Vec<String>,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDetail {
    pub id: String,
    pub account_id: String,
    pub uid: i64,
    pub subject: String,
    pub sender_name: Option<String>,
    pub sender_email: Option<String>,
    pub received_at: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub codes: Vec<String>,
    pub is_read: bool,
}

#[derive(Debug, Clone)]
pub struct StoredEmail {
    pub id: String,
    pub account_id: String,
    pub uid: i64,
    pub message_id: Option<String>,
    pub subject: String,
    pub sender_name: Option<String>,
    pub sender_email: Option<String>,
    pub received_at: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountUnreadState {
    pub account_id: String,
    pub unread_count: i64,
}
