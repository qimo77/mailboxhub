import { EmailPreviewCard } from '../components/EmailPreviewCard';
import { useMailStore } from '../store/mailStore';

export function EmailDetailPage() {
  const selectedEmail = useMailStore((state) => state.selectedEmail);
  const loadingEmail = useMailStore((state) => state.loadingEmail);
  return <EmailPreviewCard email={selectedEmail} loading={loadingEmail} />;
}
