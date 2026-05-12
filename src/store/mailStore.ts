import { create } from 'zustand';
import * as api from '../lib/tauri';
import type { AccountSummary, ImportAccountsResult, SyncResult } from '../types/account';
import type { EmailDetail, EmailSummary, NewMailEvent } from '../types/email';

interface MailState {
  accounts: AccountSummary[];
  emails: EmailSummary[];
  selectedAccountId: string | null;
  selectedEmailId: string | null;
  selectedEmail: EmailDetail | null;
  loadingAccounts: boolean;
  loadingEmails: boolean;
  loadingEmail: boolean;
  syncingAccountId: string | null;
  error: string | null;
  importResult: ImportAccountsResult | null;
  loadAccounts: () => Promise<void>;
  importAccounts: (input: string) => Promise<ImportAccountsResult>;
  selectAccount: (accountId: string) => Promise<void>;
  selectEmail: (emailId: string) => Promise<void>;
  syncSelectedAccount: () => Promise<SyncResult | null>;
  markSelectedAccountAllRead: () => Promise<void>;
  markEveryAccountAllRead: () => Promise<void>;
  applyNewMailEvent: (event: NewMailEvent) => Promise<void>;
  clearImportResult: () => void;
}

type MailStateSetter = (partial: Partial<MailState> | ((state: MailState) => Partial<MailState>)) => void;

export const useMailStore = create<MailState>((set, get) => ({
  accounts: [],
  emails: [],
  selectedAccountId: null,
  selectedEmailId: null,
  selectedEmail: null,
  loadingAccounts: false,
  loadingEmails: false,
  loadingEmail: false,
  syncingAccountId: null,
  error: null,
  importResult: null,
  async loadAccounts() {
    set({ loadingAccounts: true, error: null });
    try {
      const accounts = await api.listAccounts();
      const selectedAccountId = get().selectedAccountId ?? accounts[0]?.id ?? null;
      set({ accounts, selectedAccountId, loadingAccounts: false });
      if (selectedAccountId) {
        await get().selectAccount(selectedAccountId);
      }
    } catch (error) {
      set({ error: errorMessage(error), loadingAccounts: false });
    }
  },
  async importAccounts(input) {
    const result = await api.importAccounts(input);
    set({ importResult: result });
    await get().loadAccounts();
    return result;
  },
  async selectAccount(accountId) {
    set({ selectedAccountId: accountId, selectedEmailId: null, selectedEmail: null, loadingEmails: true, error: null });
    try {
      const emails = await api.listEmails(accountId);
      set({ emails, loadingEmails: false });
    } catch (error) {
      set({ error: errorMessage(error), loadingEmails: false });
    }
  },
  async selectEmail(emailId) {
    set({ selectedEmailId: emailId, loadingEmail: true, error: null });
    try {
      const email = await api.getEmail(emailId);
      set({ selectedEmail: email, loadingEmail: false });
      if (!email.is_read) {
        const unread = await api.markEmailRead(emailId);
        set((state) => ({
          accounts: state.accounts.map((account) =>
            account.id === unread.account_id ? { ...account, unread_count: unread.unread_count } : account,
          ),
          emails: state.emails.map((item) => (item.id === emailId ? { ...item, is_read: true } : item)),
          selectedEmail: { ...email, is_read: true },
        }));
      }
    } catch (error) {
      set({ error: errorMessage(error), loadingEmail: false });
    }
  },
  async syncSelectedAccount() {
    const accountId = get().selectedAccountId;
    if (!accountId) return null;
    set({ syncingAccountId: accountId, error: null });
    try {
      const result = await api.syncAccountNow(accountId);
      await get().loadAccounts();
      await get().selectAccount(accountId);
      set({ syncingAccountId: null });
      return result;
    } catch (error) {
      set({ error: errorMessage(error), syncingAccountId: null });
      return null;
    }
  },
  async markSelectedAccountAllRead() {
    const accountId = get().selectedAccountId;
    if (!accountId) return;
    try {
      await markAccountsRead([accountId], set);
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  async markEveryAccountAllRead() {
    const accountIds = get().accounts.map((account) => account.id);
    if (accountIds.length === 0) return;
    try {
      await markAccountsRead(accountIds, set);
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  async applyNewMailEvent(event) {
    set((state) => ({
      accounts: state.accounts.map((account) =>
        account.id === event.accountId ? { ...account, unread_count: event.unreadCount } : account,
      ),
    }));
    if (get().selectedAccountId === event.accountId) {
      await get().selectAccount(event.accountId);
    }
  },
  clearImportResult() {
    set({ importResult: null });
  },
}));

async function markAccountsRead(accountIds: string[], set: MailStateSetter) {
  const unreadStates = await Promise.all(accountIds.map((accountId) => api.markAccountAllRead(accountId)));
  const readAccountIds = new Set(unreadStates.map((unread) => unread.account_id));
  set((state) => ({
    accounts: state.accounts.map((account) =>
      readAccountIds.has(account.id) ? { ...account, unread_count: 0 } : account,
    ),
    emails: state.emails.map((email) => (readAccountIds.has(email.account_id) ? { ...email, is_read: true } : email)),
    selectedEmail:
      state.selectedEmail && readAccountIds.has(state.selectedEmail.account_id)
        ? { ...state.selectedEmail, is_read: true }
        : state.selectedEmail,
  }));
}

function errorMessage(error: unknown) {
  if (typeof error === 'string') return error;
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message: unknown }).message);
  }
  return '操作失败';
}
