export interface InvalidImportLine {
  line_number: number;
  reason: string;
  value: string;
}

export interface ImportAccountsResult {
  imported: number;
  updated: number;
  invalid: InvalidImportLine[];
}

export interface AccountSummary {
  id: string;
  email: string;
  display_name: string;
  unread_count: number;
  is_active: boolean;
  auto_connect: boolean;
  last_error: string | null;
  last_sync_at: string | null;
}

export interface SyncResult {
  account_id: string;
  fetched: number;
  inserted: number;
  skipped: number;
  error: string | null;
}
