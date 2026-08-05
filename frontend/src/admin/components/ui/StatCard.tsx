import type { ReactNode } from 'react';
import { motion } from 'framer-motion';

interface StatCardProps {
  label: string;
  value: ReactNode;
  hint?: ReactNode;
  icon: ReactNode;
  tone?: 'primary' | 'accent' | 'success' | 'warning' | 'danger';
  index?: number;
  footer?: ReactNode;
}

const TONES = {
  primary: 'text-primary-400 bg-primary-500/10 border-primary-500/30',
  accent: 'text-accent-400 bg-accent-500/10 border-accent-500/30',
  success: 'text-emerald-400 bg-emerald-500/10 border-emerald-500/30',
  warning: 'text-amber-400 bg-amber-500/10 border-amber-500/30',
  danger: 'text-red-400 bg-red-500/10 border-red-500/30',
} as const;

export function StatCard({ label, value, hint, icon, tone = 'accent', index = 0, footer }: StatCardProps) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 14 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.05, duration: 0.35, ease: 'easeOut' }}
      whileHover={{ y: -3 }}
      className="glass-card group rounded-2xl p-5"
    >
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <p className="text-xs font-medium uppercase tracking-wider text-dark-400">{label}</p>
          <p className="mt-2 text-2xl font-semibold tabular-nums text-white">{value}</p>
          {hint && <p className="mt-1 text-xs text-dark-400">{hint}</p>}
        </div>
        <div className={`flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border ${TONES[tone]}`}>
          {icon}
        </div>
      </div>
      {footer && <div className="mt-3 border-t border-white/5 pt-3">{footer}</div>}
    </motion.div>
  );
}
