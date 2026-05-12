import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { AnimatePresence, motion } from 'framer-motion';
import { LoginPage } from './pages/LoginPage';
import { MailboxPage } from './pages/MailboxPage';
import { SettingsPage } from './pages/SettingsPage';
import { useMailStore } from './store/mailStore';
import { useSettingsStore } from './store/settingsStore';
import type { NewMailEvent } from './types/email';

type View = 'mailbox' | 'import' | 'settings';

function App() {
  const [view, setView] = useState<View>('mailbox');
  const accounts = useMailStore((state) => state.accounts);
  const loadAccounts = useMailStore((state) => state.loadAccounts);
  const applyNewMailEvent = useMailStore((state) => state.applyNewMailEvent);
  const settings = useSettingsStore((state) => state.settings);
  const loadSettings = useSettingsStore((state) => state.loadSettings);

  useEffect(() => {
    void loadSettings();
    void loadAccounts();
  }, [loadAccounts, loadSettings]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    void listen<NewMailEvent>('mail:new', async (event) => {
      await applyNewMailEvent(event.payload);
      if (settings.notifications_enabled) {
        let granted = await isPermissionGranted();
        if (!granted) {
          const permission = await requestPermission();
          granted = permission === 'granted';
        }
        if (granted) {
          sendNotification({
            title: event.payload.code ? 'OpenAI 验证码' : '新邮件',
            body: event.payload.code || `${event.payload.sender}\n${event.payload.subject}`,
          });
        }
      }
    }).then((dispose) => {
      unlisten = dispose;
    });
    return () => unlisten?.();
  }, [applyNewMailEvent, settings.notifications_enabled]);

  const effectiveView = accounts.length === 0 || view === 'import' ? 'import' : view;

  return (
    <div className="h-full w-full bg-transparent text-slate-950">
      <AnimatePresence mode="wait">
        {effectiveView === 'settings' ? (
          <motion.div key="settings" className="h-full" exit={{ opacity: 0 }}>
            <SettingsPage onBack={() => setView('mailbox')} />
          </motion.div>
        ) : effectiveView === 'import' ? (
          <motion.div key="import" className="h-full" exit={{ opacity: 0 }}>
            <LoginPage />
            {accounts.length > 0 && (
              <button
                onClick={() => setView('mailbox')}
                className="fixed bottom-6 left-1/2 -translate-x-1/2 rounded-full border border-slate-200 bg-white px-5 py-2 text-sm font-semibold text-slate-950 shadow-glass"
              >
                进入邮箱管理
              </button>
            )}
          </motion.div>
        ) : (
          <motion.div key="mailbox" className="h-full" exit={{ opacity: 0 }}>
            <MailboxPage onImport={() => setView('import')} onSettings={() => setView('settings')} />
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

export default App;
