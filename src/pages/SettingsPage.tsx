import { motion } from 'framer-motion';
import { AlertTriangle, ArrowLeft, Bell, Database, Moon, Power, RefreshCw } from 'lucide-react';
import { GlassPanel } from '../components/GlassPanel';
import { useSettingsStore } from '../store/settingsStore';

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const settings = useSettingsStore((state) => state.settings);
  const databasePath = useSettingsStore((state) => state.databasePath);
  const updateSettings = useSettingsStore((state) => state.updateSettings);

  return (
    <div className="flex h-full items-center justify-center p-6">
      <motion.div initial={{ opacity: 0, y: 16 }} animate={{ opacity: 1, y: 0 }} className="w-full max-w-4xl">
        <GlassPanel className="p-6">
          <div className="mb-8 flex items-center justify-between">
            <div>
              <p className="text-xs uppercase tracking-[0.3em] text-slate-500">Settings</p>
              <h1 className="mt-2 text-3xl font-semibold text-slate-950">设置</h1>
            </div>
            <button onClick={onBack} className="rounded-full border border-slate-200 bg-white/70 px-4 py-2 text-sm text-slate-700 hover:bg-slate-50">
              <ArrowLeft className="mr-2 inline h-4 w-4" /> 返回邮箱
            </button>
          </div>

          <div className="grid gap-4 md:grid-cols-2">
            <SettingCard icon={RefreshCw} title="自动刷新时间" description="后台轮询 Outlook INBOX 的间隔。">
              <div className="flex items-center gap-3">
                <input
                  type="number"
                  min={5}
                  value={settings.poll_interval_seconds}
                  onChange={(event) => void updateSettings({ poll_interval_seconds: Number(event.target.value) })}
                  className="w-24 rounded-xl border border-slate-200 bg-white/80 px-3 py-2 text-slate-950 outline-none"
                />
                <span className="text-sm text-slate-400">秒</span>
              </div>
            </SettingCard>

            <SettingCard icon={Moon} title="浅色模式" description="默认使用明亮的 macOS 玻璃风格。">
              <Toggle enabled={settings.theme === 'light'} onClick={() => void updateSettings({ theme: settings.theme === 'light' ? 'dark' : 'light' })} />
            </SettingCard>

            <SettingCard icon={Bell} title="macOS 通知" description="新邮件和验证码到达时弹出系统通知。">
              <Toggle enabled={settings.notifications_enabled} onClick={() => void updateSettings({ notifications_enabled: !settings.notifications_enabled })} />
            </SettingCard>

            <SettingCard icon={Power} title="启动自动连接" description="应用启动后自动连接已导入邮箱。">
              <Toggle enabled={settings.launch_auto_connect} onClick={() => void updateSettings({ launch_auto_connect: !settings.launch_auto_connect })} />
            </SettingCard>
          </div>

          <div className="mt-4 rounded-2xl border border-slate-200 bg-white/70 p-5">
            <div className="flex items-start gap-3">
              <Database className="mt-0.5 h-5 w-5 text-sky-200" />
              <div className="min-w-0">
                <p className="font-semibold text-slate-950">SQLite 数据库位置</p>
                <p className="mt-1 break-all font-mono text-xs text-slate-400">{databasePath || '加载中…'}</p>
              </div>
            </div>
          </div>

          <div className="mt-4 rounded-2xl border border-amber-200 bg-amber-50 p-5 text-amber-800">
            <div className="flex gap-3">
              <AlertTriangle className="mt-0.5 h-5 w-5 flex-none" />
              <p className="text-sm leading-6">
                当前版本按需求将邮箱密码、client_id 和 refresh_token 保存到本地 SQLite。生产环境建议迁移到 macOS Keychain 存储敏感凭据。
              </p>
            </div>
          </div>
        </GlassPanel>
      </motion.div>
    </div>
  );
}

interface SettingCardProps {
  icon: React.ComponentType<{ className?: string }>;
  title: string;
  description: string;
  children: React.ReactNode;
}

function SettingCard({ icon: Icon, title, description, children }: SettingCardProps) {
  return (
    <div className="rounded-2xl border border-slate-200 bg-white/70 p-5">
      <div className="flex items-start justify-between gap-4">
        <div className="flex gap-3">
          <div className="rounded-2xl bg-slate-100 p-2 text-slate-600">
            <Icon className="h-5 w-5" />
          </div>
          <div>
            <p className="font-semibold text-slate-950">{title}</p>
            <p className="mt-1 text-sm leading-5 text-slate-400">{description}</p>
          </div>
        </div>
        {children}
      </div>
    </div>
  );
}

function Toggle({ enabled, onClick }: { enabled: boolean; onClick: () => void }) {
  return (
    <button onClick={onClick} className={`relative h-7 w-12 rounded-full transition ${enabled ? 'bg-sky-400' : 'bg-slate-200'}`}>
      <span className={`absolute top-1 h-5 w-5 rounded-full bg-white transition ${enabled ? 'left-6' : 'left-1'}`} />
    </button>
  );
}
