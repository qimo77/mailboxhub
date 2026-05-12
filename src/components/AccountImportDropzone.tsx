import { useRef, useState } from 'react';
import { motion } from 'framer-motion';
import { AlertCircle, CheckCircle2, UploadCloud } from 'lucide-react';
import { useMailStore } from '../store/mailStore';

export function AccountImportDropzone() {
  const [input, setInput] = useState('');
  const [dragging, setDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const importAccounts = useMailStore((state) => state.importAccounts);
  const importResult = useMailStore((state) => state.importResult);
  const [submitting, setSubmitting] = useState(false);

  async function handleImport() {
    if (!input.trim()) return;
    setSubmitting(true);
    try {
      await importAccounts(input);
      setInput('');
    } finally {
      setSubmitting(false);
    }
  }

  async function readFiles(files: FileList | null) {
    if (!files?.length) return;
    const texts = await Promise.all(Array.from(files).map((file) => file.text()));
    setInput((current) => [current, ...texts].filter(Boolean).join('\n'));
  }

  return (
    <div className="space-y-5">
      <div
        onDragOver={(event) => {
          event.preventDefault();
          setDragging(true);
        }}
        onDragLeave={() => setDragging(false)}
        onDrop={(event) => {
          event.preventDefault();
          setDragging(false);
          readFiles(event.dataTransfer.files);
        }}
        className={`rounded-3xl border border-dashed p-4 transition ${
          dragging ? 'border-sky-300 bg-sky-50' : 'border-slate-200 bg-white/60'
        }`}
      >
        <textarea
          value={input}
          onChange={(event) => setInput(event.target.value)}
          placeholder="KatieFerguson7034@outlook.com----fjdr3810----client_id----refresh_token"
          className="h-52 w-full resize-none rounded-2xl border border-slate-200 bg-white/80 p-4 font-mono text-sm leading-6 text-slate-800 outline-none placeholder:text-slate-400 focus:border-sky-300"
        />
        <div className="mt-4 flex flex-wrap items-center justify-between gap-3 text-sm text-slate-500">
          <button
            type="button"
            onClick={() => fileInputRef.current?.click()}
            className="inline-flex items-center gap-2 rounded-full border border-slate-200 bg-white/70 px-4 py-2 text-slate-700 hover:bg-slate-50"
          >
            <UploadCloud className="h-4 w-4" />
            拖拽或选择 txt 文件
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".txt,text/plain"
            multiple
            className="hidden"
            onChange={(event) => {
              void readFiles(event.target.files);
              event.currentTarget.value = '';
            }}
          />
          <button
            type="button"
            onClick={handleImport}
            disabled={!input.trim() || submitting}
            className="rounded-full bg-white px-5 py-2 font-semibold text-slate-950 transition hover:bg-sky-100 disabled:cursor-not-allowed disabled:opacity-40"
          >
            {submitting ? '导入中…' : '导入邮箱'}
          </button>
        </div>
      </div>

      {importResult && (
        <motion.div
          initial={{ opacity: 0, y: 8 }}
          animate={{ opacity: 1, y: 0 }}
          className="rounded-2xl border border-slate-200 bg-white/70 p-4 text-sm"
        >
          <div className="flex items-center gap-2 text-emerald-200">
            <CheckCircle2 className="h-4 w-4" />
            已新增 {importResult.imported} 个，更新 {importResult.updated} 个
          </div>
          {importResult.invalid.length > 0 && (
            <div className="mt-3 space-y-2 text-amber-200">
              {importResult.invalid.slice(0, 5).map((line) => (
                <div key={`${line.line_number}-${line.value}`} className="flex gap-2">
                  <AlertCircle className="mt-0.5 h-4 w-4 flex-none" />
                  <span>第 {line.line_number} 行：{line.reason}</span>
                </div>
              ))}
            </div>
          )}
        </motion.div>
      )}
    </div>
  );
}
