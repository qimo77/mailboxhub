import { motion } from 'framer-motion';
import { AlertCircle, Copy, Mail } from 'lucide-react';
import { initials } from '../lib/format';
import type { AccountSummary } from '../types/account';
import { UnreadBadge } from './UnreadBadge';

interface AccountListProps {
  accounts: AccountSummary[];
  selectedAccountId: string | null;
  onSelect: (accountId: string) => void;
}

export function AccountList({ accounts, selectedAccountId, onSelect }: AccountListProps) {
  return (
    <div className="min-h-0 flex-1 space-y-2 overflow-y-auto pr-1 scrollbar-soft">
      {accounts.map((account) => {
        const selected = account.id === selectedAccountId;
        return (
          <motion.div
            layout
            key={account.id}
            className={`group flex w-full items-center gap-2 rounded-2xl px-3 py-3 text-left transition ${
              selected ? 'bg-slate-950 text-white shadow-lg' : 'text-slate-700 hover:bg-slate-100/80'
            }`}
          >
            <button type="button" onClick={() => onSelect(account.id)} className="flex min-w-0 flex-1 items-center gap-3 text-left">
              <div className={`relative flex h-11 w-11 items-center justify-center rounded-2xl font-semibold ${selected ? 'bg-white text-slate-950' : 'bg-slate-100 text-slate-700'}`}>
                {initials(account.email)}
                {account.unread_count > 0 && (
                  <span className="absolute -right-0.5 -top-0.5 h-3 w-3 rounded-full bg-red-500 ring-2 ring-white" />
                )}
              </div>
              <div className="min-w-0 flex-1">
                <div className="flex items-center gap-2">
                  <p className="truncate text-sm font-semibold">{account.display_name}</p>
                  {account.last_error ? <AlertCircle className="h-3.5 w-3.5 text-amber-400" /> : <Mail className="h-3.5 w-3.5 opacity-50" />}
                </div>
                <p className={`truncate text-xs ${selected ? 'text-slate-400' : 'text-slate-500'}`}>{account.email}</p>
              </div>
            </button>
            <button
              type="button"
              onClick={() => void navigator.clipboard.writeText(account.email)}
              className={`rounded-full p-1.5 transition ${selected ? 'text-slate-300 hover:bg-white/10 hover:text-white' : 'text-slate-400 hover:bg-slate-200/70 hover:text-slate-700'}`}
              title="复制邮箱"
              aria-label={`复制 ${account.email}`}
            >
              <Copy className="h-3.5 w-3.5" />
            </button>
            <UnreadBadge count={account.unread_count} />
          </motion.div>
        );
      })}
    </div>
  );
}
