import type { PropsWithChildren } from 'react';

interface GlassPanelProps extends PropsWithChildren {
  className?: string;
}

export function GlassPanel({ children, className = '' }: GlassPanelProps) {
  return (
    <section className={`rounded-3xl border border-slate-200/80 bg-white/80 shadow-glass backdrop-blur-2xl ${className}`}>
      {children}
    </section>
  );
}
