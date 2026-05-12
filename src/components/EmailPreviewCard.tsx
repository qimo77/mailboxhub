import { Loader2 } from 'lucide-react';
import { formatDate, senderName } from '../lib/format';
import type { EmailDetail } from '../types/email';
import { CodePill } from './CodePill';
import { EmptyState } from './EmptyState';

interface EmailPreviewCardProps {
  email: EmailDetail | null;
  loading: boolean;
}

export function EmailPreviewCard({ email, loading }: EmailPreviewCardProps) {
  if (loading) {
    return (
      <div className="flex h-full items-center justify-center text-slate-400">
        <Loader2 className="mr-2 h-5 w-5 animate-spin" /> 正在打开邮件…
      </div>
    );
  }

  if (!email) {
    return <EmptyState title="选择一封邮件" description="验证码、发件人和正文会在这里以阅读模式展示。" />;
  }

  const body = email.body_text || stripHtml(email.body_html || '') || '这封邮件没有可显示的正文。';

  return (
    <article className="flex h-full flex-col overflow-hidden">
      <header className="border-b border-slate-200/80 p-6">
        <div className="mb-4 flex flex-wrap gap-2">
          {email.codes.map((code) => <CodePill key={code} code={code} />)}
        </div>
        <h1 className="text-2xl font-semibold tracking-tight text-slate-950">{email.subject}</h1>
        <div className="mt-4 flex items-center justify-between gap-4 text-sm text-slate-500">
          <div>
            <p className="font-medium text-slate-700">{senderName(email.sender_name, email.sender_email)}</p>
            <p>{email.sender_email}</p>
          </div>
          <time>{formatDate(email.received_at)}</time>
        </div>
      </header>
      <div className="flex-1 overflow-y-auto p-6 scrollbar-soft">
        <pre className="whitespace-pre-wrap break-words font-sans text-[15px] leading-7 text-slate-700">{body}</pre>
      </div>
    </article>
  );
}

function stripHtml(html: string) {
  return html.replace(/<style[\s\S]*?<\/style>/gi, '').replace(/<script[\s\S]*?<\/script>/gi, '').replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim();
}
