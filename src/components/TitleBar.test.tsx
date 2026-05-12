// @vitest-environment jsdom

import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { TitleBar } from './TitleBar';

describe('TitleBar', () => {
  afterEach(() => {
    cleanup();
  });

  it('does not render fake macOS window controls when native controls are enabled', () => {
    render(<TitleBar onImport={() => undefined} onRefresh={() => undefined} onMarkAllRead={() => undefined} onMarkEveryAccountRead={() => undefined} onSettings={() => undefined} syncing={false} />);

    expect(screen.queryByLabelText('window controls')).toBeNull();
  });

  it('runs one-click mark-all-read from the toolbar', () => {
    const onMarkAllRead = vi.fn();

    render(<TitleBar onImport={() => undefined} onRefresh={() => undefined} onMarkAllRead={onMarkAllRead} onMarkEveryAccountRead={() => undefined} onSettings={() => undefined} syncing={false} />);

    fireEvent.click(screen.getByRole('button', { name: '当前邮箱已读' }));

    expect(onMarkAllRead).toHaveBeenCalledOnce();
  });

  it('marks every mailbox read from the toolbar', () => {
    const onMarkEveryAccountRead = vi.fn();

    render(<TitleBar onImport={() => undefined} onRefresh={() => undefined} onMarkAllRead={() => undefined} onMarkEveryAccountRead={onMarkEveryAccountRead} onSettings={() => undefined} syncing={false} />);

    fireEvent.click(screen.getByRole('button', { name: '全部邮箱已读' }));

    expect(onMarkEveryAccountRead).toHaveBeenCalledOnce();
  });
});
