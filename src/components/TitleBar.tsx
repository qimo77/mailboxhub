import { Bell, CheckCheck, RefreshCw, Search, Settings, UploadCloud } from 'lucide-react';

interface TitleBarProps {
  onImport: () => void;
  onRefresh: () => void;
  onMarkAllRead: () => void;
  onMarkEveryAccountRead: () => void;
  onSettings: () => void;
  syncing: boolean;
}

export function TitleBar({ onImport, onRefresh, onMarkAllRead, onMarkEveryAccountRead, onSettings, syncing }: TitleBarProps) {
  return (
    <div className="drag-region flex h-16 items-center gap-4 border-b border-slate-200/80 px-5">
      <div className="no-drag flex h-10 max-w-xl flex-1 items-center gap-2 rounded-full border border-slate-200 bg-white/70 px-4 text-slate-500">
        <Search className="h-4 w-4" />
        <input className="w-full bg-transparent text-sm text-slate-800 outline-none placeholder:text-slate-400" placeholder="搜索邮箱、发件人、标题或验证码" />
      </div>
      <div className="no-drag ml-auto flex items-center gap-2">
        <button onClick={onImport} className="rounded-full border border-slate-200 bg-white/70 p-2 text-slate-600 hover:bg-slate-50" title="导入邮箱">
          <UploadCloud className="h-4 w-4" />
        </button>
        <button onClick={onRefresh} className="rounded-full border border-slate-200 bg-white/70 p-2 text-slate-600 hover:bg-slate-50" title="刷新">
          <RefreshCw className={`h-4 w-4 ${syncing ? 'animate-spin' : ''}`} />
        </button>
        <button onClick={onMarkAllRead} className="rounded-full border border-slate-200 bg-white/70 p-2 text-slate-600 hover:bg-slate-50" title="当前邮箱已读" aria-label="当前邮箱已读">
          <CheckCheck className="h-4 w-4" />
        </button>
        <button onClick={onMarkEveryAccountRead} className="rounded-full border border-slate-200 bg-white/70 px-3 py-2 text-xs font-semibold text-slate-600 hover:bg-slate-50" title="全部邮箱已读" aria-label="全部邮箱已读">
          全部已读
        </button>
        <div className="rounded-full border border-slate-200 bg-white/70 p-2 text-slate-600">
          <Bell className="h-4 w-4" />
        </div>
        <button onClick={onSettings} className="rounded-full border border-slate-200 bg-white/70 p-2 text-slate-600 hover:bg-slate-50" title="设置">
          <Settings className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
}
