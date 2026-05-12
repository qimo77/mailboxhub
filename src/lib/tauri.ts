import { invoke } from '@tauri-apps/api/core';
import type { AccountSummary, ImportAccountsResult, SyncResult } from '../types/account';
import type { AccountUnreadState, EmailDetail, EmailSummary } from '../types/email';
import type { AppSettings, AppSettingsPatch } from '../types/settings';

export async function importAccounts(input: string) {
  return invoke<ImportAccountsResult>('import_accounts', { input });
}

export async function listAccounts() {
  return invoke<AccountSummary[]>('list_accounts');
}

export async function syncAccountNow(accountId: string) {
  return invoke<SyncResult>('sync_account_now', { accountId });
}

export async function listEmails(accountId: string) {
  return invoke<EmailSummary[]>('list_emails', { accountId });
}

export async function getEmail(emailId: string) {
  return invoke<EmailDetail>('get_email', { emailId });
}

export async function markEmailRead(emailId: string) {
  return invoke<AccountUnreadState>('mark_email_read', { emailId });
}

export async function markAccountAllRead(accountId: string) {
  return invoke<AccountUnreadState>('mark_account_all_read', { accountId });
}

export async function getSettings() {
  return invoke<AppSettings>('get_settings');
}

export async function updateSettings(patch: AppSettingsPatch) {
  return invoke<AppSettings>('update_settings', { patch });
}

export async function getDatabasePath() {
  return invoke<string>('get_database_path');
}
