import { AppShell } from '../components/AppShell';

interface MailboxPageProps {
  onImport: () => void;
  onSettings: () => void;
}

export function MailboxPage({ onImport, onSettings }: MailboxPageProps) {
  return <AppShell onImport={onImport} onSettings={onSettings} />;
}
