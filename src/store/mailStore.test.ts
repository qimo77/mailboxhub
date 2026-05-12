import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useMailStore } from './mailStore';
import * as api from '../lib/tauri';

vi.mock('../lib/tauri', () => ({
  markAccountAllRead: vi.fn(),
}));

describe('mailStore markSelectedAccountAllRead', () => {
  beforeEach(() => {
    vi.mocked(api.markAccountAllRead).mockReset();
    useMailStore.setState({
      accounts: [
        {
          id: 'account-1',
          email: 'user@outlook.com',
          display_name: 'user',
          unread_count: 2,
          is_active: true,
          auto_connect: true,
          last_error: null,
          last_sync_at: null,
        },
        {
          id: 'account-2',
          email: 'other@outlook.com',
          display_name: 'other',
          unread_count: 3,
          is_active: true,
          auto_connect: true,
          last_error: null,
          last_sync_at: null,
        },
      ],
      emails: [
        {
          id: 'email-1',
          account_id: 'account-1',
          uid: 1,
          subject: 'First',
          sender_name: null,
          sender_email: null,
          received_at: null,
          codes: [],
          is_read: false,
        },
      ],
      selectedAccountId: 'account-1',
      selectedEmailId: 'email-1',
      selectedEmail: {
        id: 'email-1',
        account_id: 'account-1',
        uid: 1,
        subject: 'First',
        sender_name: null,
        sender_email: null,
        received_at: null,
        codes: [],
        is_read: false,
        body_text: 'Body',
        body_html: null,
      },
      error: null,
    });
  });

  it('marks every visible email read and clears the selected account unread count', async () => {
    vi.mocked(api.markAccountAllRead).mockResolvedValue({ account_id: 'account-1', unread_count: 0 });

    await useMailStore.getState().markSelectedAccountAllRead();

    expect(api.markAccountAllRead).toHaveBeenCalledWith('account-1');
    expect(useMailStore.getState().accounts[0].unread_count).toBe(0);
    expect(useMailStore.getState().emails[0].is_read).toBe(true);
    expect(useMailStore.getState().selectedEmail?.is_read).toBe(true);
  });

  it('marks every mailbox read at once', async () => {
    vi.mocked(api.markAccountAllRead).mockImplementation(async (accountId: string) => ({ account_id: accountId, unread_count: 0 }));

    await useMailStore.getState().markEveryAccountAllRead();

    expect(api.markAccountAllRead).toHaveBeenCalledWith('account-1');
    expect(api.markAccountAllRead).toHaveBeenCalledWith('account-2');
    expect(useMailStore.getState().accounts.every((account) => account.unread_count === 0)).toBe(true);
    expect(useMailStore.getState().emails[0].is_read).toBe(true);
    expect(useMailStore.getState().selectedEmail?.is_read).toBe(true);
  });
});
