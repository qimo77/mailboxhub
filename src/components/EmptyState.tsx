import type { LucideIcon } from 'lucide-react';
import { Inbox } from 'lucide-react';

interface EmptyStateProps {
  icon?: LucideIcon;
  title: string;
  description: string;
}

export function EmptyState({ icon: Icon = Inbox, title, description }: EmptyStateProps) {
  return (
    <div className="flex h-full flex-col items-center justify-center px-8 text-center text-slate-500">
      <div className="mb-4 rounded-3xl border border-slate-200 bg-white/70 p-5 text-slate-600">
        <Icon className="h-8 w-8" />
      </div>
      <h3 className="text-lg font-semibold text-slate-950">{title}</h3>
      <p className="mt-2 max-w-sm text-sm leading-6">{description}</p>
    </div>
  );
}
