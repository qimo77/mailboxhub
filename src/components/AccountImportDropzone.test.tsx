// @vitest-environment jsdom

import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { AccountImportDropzone } from './AccountImportDropzone';

const importAccounts = vi.fn();

vi.mock('../store/mailStore', () => ({
  useMailStore: (selector: (state: unknown) => unknown) =>
    selector({
      importAccounts,
      importResult: null,
    }),
}));

describe('AccountImportDropzone', () => {
  beforeEach(() => {
    importAccounts.mockReset();
  });

  it('loads account text from a dropped txt file', async () => {
    render(<AccountImportDropzone />);

    const file = new File(
      ['example@outlook.com----password----client_id----refresh_token'],
      'accounts.txt',
      { type: 'text/plain' },
    );

    fireEvent.drop(screen.getByRole('textbox'), {
      dataTransfer: {
        files: [file],
      },
    });

    await waitFor(() => {
      expect((screen.getByRole('textbox') as HTMLTextAreaElement).value).toBe(
        'example@outlook.com----password----client_id----refresh_token',
      );
    });
  });
});
