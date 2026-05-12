import { motion } from 'framer-motion';
import { MailPlus, Sparkles } from 'lucide-react';
import { AccountImportDropzone } from '../components/AccountImportDropzone';
import { GlassPanel } from '../components/GlassPanel';

export function LoginPage() {
  return (
    <div className="flex h-full items-center justify-center p-8">
      <motion.div initial={{ opacity: 0, y: 20, scale: 0.98 }} animate={{ opacity: 1, y: 0, scale: 1 }} className="w-full max-w-3xl">
        <GlassPanel className="overflow-hidden p-8">
          <div className="mb-8 flex items-start justify-between gap-6">
            <div>
              <div className="mb-4 inline-flex items-center gap-2 rounded-full border border-sky-200 bg-sky-50 px-3 py-1 text-xs font-medium text-sky-700">
                <Sparkles className="h-3.5 w-3.5" /> mailboxhub
              </div>
              <h1 className="text-4xl font-semibold tracking-tight text-slate-950">批量导入 Outlook 邮箱</h1>
              <p className="mt-3 max-w-xl text-sm leading-6 text-slate-600">
                粘贴或拖入 txt 文件，系统会保存到本地 SQLite，并在后台通过 OAuth + IMAP 自动同步验证码邮件。
              </p>
            </div>
            <div className="rounded-3xl border border-slate-200 bg-white/70 p-4 text-slate-700">
              <MailPlus className="h-8 w-8" />
            </div>
          </div>
          <AccountImportDropzone />
        </GlassPanel>
      </motion.div>
    </div>
  );
}
