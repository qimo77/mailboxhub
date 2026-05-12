import { badgeCount } from '../lib/format';

interface UnreadBadgeProps {
  count: number;
}

export function UnreadBadge({ count }: UnreadBadgeProps) {
  if (count <= 0) return null;
  return (
    <span className="min-w-6 rounded-full bg-red-500 px-2 py-0.5 text-center text-xs font-semibold text-white shadow-lg shadow-red-500/30">
      {badgeCount(count)}
    </span>
  );
}
