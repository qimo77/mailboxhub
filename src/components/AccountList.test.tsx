// @vitest-environment jsdom

import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { AccountList } from './AccountList';
import type { AccountSummary } from '../types/account';

vi.mock('framer-motion', () => ({
  motion: {
    button: 'button',
    div: 'div',
  },
}));

function account(index: number): AccountSummary {
  return {
    id: `account-${index}`,
    email: `user${index}@outlook.com`,
    display_name: `user${index}`,
    unread_count: 0,
    is_active: true,
    auto_connect: true,
    last_error: null,
    last_sync_at: null,
  };
}

describe('AccountList', () => {
  afterEach(() => {
    cleanup();
  });

  beforeEach(() => {
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });
  });

  it('renders every imported account', () => {
    const accounts = Array.from({ length: 10 }, (_, index) => account(index));

    render(<AccountList accounts={accounts} selectedAccountId={null} onSelect={() => undefined} />);

    expect(screen.getAllByRole('button')).toHaveLength(20);
    expect(screen.getByText('user9@outlook.com')).toBeTruthy();
  });

  it('copies an account email without selecting the account', () => {
    const onSelect = vi.fn();
    render(<AccountList accounts={[account(3)]} selectedAccountId={null} onSelect={onSelect} />);

    fireEvent.click(screen.getByRole('button', { name: '复制 user3@outlook.com' }));

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('user3@outlook.com');
    expect(onSelect).not.toHaveBeenCalled();
  });
});
