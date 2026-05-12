import { motion } from 'framer-motion';
import { formatDate, senderName } from '../lib/format';
import type { EmailSummary } from '../types/email';
import { CodePill } from './CodePill';
import { EmptyState } from './EmptyState';

interface EmailListProps {
  emails: EmailSummary[];
  selectedEmailId: string | null;
  loading: boolean;
  onSelect: (emailId: string) => void;
}

export function EmailList({ emails, selectedEmailId, loading, onSelect }: EmailListProps) {
  if (loading) {
    return <EmptyState title="正在读取邮件" description="正在从本地 SQLite 缓存加载邮件列表。" />;
  }

  if (emails.length === 0) {
    return <EmptyState title="暂无邮件" description="点击刷新连接 Outlook IMAP，或等待后台 5 秒轮询同步。" />;
  }

  return (
    <div className="h-full overflow-y-auto p-3 scrollbar-soft">
      <div className="space-y-2">
        {emails.map((email) => {
          const selected = selectedEmailId === email.id;
          return (
            <motion.button
              layout
              key={email.id}
              type="button"
              onClick={() => onSelect(email.id)}
              className={`w-full rounded-2xl border p-4 text-left transition ${
                selected ? 'border-sky-200 bg-sky-50' : 'border-slate-200 bg-white/60 hover:bg-slate-50'
              }`}
            >
              <div className="flex items-center justify-between gap-3">
                <div className="flex min-w-0 items-center gap-2">
                  {!email.is_read && <span className="h-2 w-2 rounded-full bg-red-500" />}
                  <p className="truncate text-sm font-semibold text-slate-950">{senderName(email.sender_name, email.sender_email)}</p>
                </div>
                <time className="text-xs text-slate-500">{formatDate(email.received_at)}</time>
              </div>
              <p className={`mt-2 line-clamp-2 text-sm ${email.is_read ? 'text-slate-500' : 'font-semibold text-slate-900'}`}>{email.subject}</p>
              {email.codes.length > 0 && (
                <div className="mt-3 flex flex-wrap gap-2">
                  {email.codes.slice(0, 3).map((code) => <CodePill key={code} code={code} />)}
                </div>
              )}
            </motion.button>
          );
        })}
      </div>
    </div>
  );
}
