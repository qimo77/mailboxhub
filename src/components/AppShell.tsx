import { motion } from 'framer-motion';
import { Inbox } from 'lucide-react';
import { useMailStore } from '../store/mailStore';
import { AccountList } from './AccountList';
import { EmailList } from './EmailList';
import { EmailPreviewCard } from './EmailPreviewCard';
import { EmptyState } from './EmptyState';
import { GlassPanel } from './GlassPanel';
import { TitleBar } from './TitleBar';

interface AppShellProps {
  onImport: () => void;
  onSettings: () => void;
}

export function AppShell({ onImport, onSettings }: AppShellProps) {
  const accounts = useMailStore((state) => state.accounts);
  const emails = useMailStore((state) => state.emails);
  const selectedAccountId = useMailStore((state) => state.selectedAccountId);
  const selectedEmailId = useMailStore((state) => state.selectedEmailId);
  const selectedEmail = useMailStore((state) => state.selectedEmail);
  const loadingEmails = useMailStore((state) => state.loadingEmails);
  const loadingEmail = useMailStore((state) => state.loadingEmail);
  const syncingAccountId = useMailStore((state) => state.syncingAccountId);
  const selectAccount = useMailStore((state) => state.selectAccount);
  const selectEmail = useMailStore((state) => state.selectEmail);
  const syncSelectedAccount = useMailStore((state) => state.syncSelectedAccount);
  const markSelectedAccountAllRead = useMailStore((state) => state.markSelectedAccountAllRead);
  const markEveryAccountAllRead = useMailStore((state) => state.markEveryAccountAllRead);

  return (
    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="flex h-full flex-col p-4">
      <GlassPanel className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <TitleBar
          onImport={onImport}
          onRefresh={() => void syncSelectedAccount()}
          onMarkAllRead={() => void markSelectedAccountAllRead()}
          onMarkEveryAccountRead={() => void markEveryAccountAllRead()}
          onSettings={onSettings}
          syncing={Boolean(syncingAccountId)}
        />
        <div className="grid min-h-0 flex-1 grid-cols-[300px_390px_minmax(0,1fr)] gap-0">
          <aside className="flex min-h-0 flex-col border-r border-slate-200/80 p-4">
            <div className="mb-4 flex flex-none items-center justify-between">
              <div>
                <p className="text-xs uppercase tracking-[0.3em] text-slate-500">Accounts</p>
                <h2 className="text-lg font-semibold text-slate-950">Outlook 邮箱</h2>
              </div>
            </div>
            <AccountList accounts={accounts} selectedAccountId={selectedAccountId} onSelect={(id) => void selectAccount(id)} />
          </aside>
          <main className="min-h-0 border-r border-slate-200/80">
            {selectedAccountId ? (
              <EmailList emails={emails} selectedEmailId={selectedEmailId} loading={loadingEmails} onSelect={(id) => void selectEmail(id)} />
            ) : (
              <EmptyState icon={Inbox} title="选择邮箱" description="左侧选择一个 Outlook 邮箱查看邮件。" />
            )}
          </main>
          <section className="min-h-0">
            <EmailPreviewCard email={selectedEmail} loading={loadingEmail} />
          </section>
        </div>
      </GlassPanel>
    </motion.div>
  );
}
