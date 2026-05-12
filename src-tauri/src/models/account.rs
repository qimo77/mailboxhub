use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountImportInput {
    pub email: String,
    pub password: String,
    pub client_id: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvalidImportLine {
    pub line_number: usize,
    pub reason: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportAccountsResult {
    pub imported: usize,
    pub updated: usize,
    pub invalid: Vec<InvalidImportLine>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountSummary {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub unread_count: i64,
    pub is_active: bool,
    pub auto_connect: bool,
    pub last_error: Option<String>,
    pub last_sync_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StoredAccount {
    pub id: String,
    pub email: String,
    pub client_id: String,
    pub refresh_token: String,
    pub access_token: Option<String>,
    pub access_token_expires_at: Option<String>,
    pub last_sync_uid: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyncResult {
    pub account_id: String,
    pub fetched: usize,
    pub inserted: usize,
    pub skipped: usize,
    pub error: Option<String>,
}
