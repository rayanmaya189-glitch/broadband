import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { motion } from 'framer-motion';

type Variant = 'primary' | 'secondary' | 'danger' | 'ghost' | 'accent' | 'warning' | 'success';
type Size = 'sm' | 'md';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
  size?: Size;
  loading?: boolean;
  children: ReactNode;
}

const VARIANTS: Record<Variant, string> = {
  primary:
    'bg-primary-600 hover:bg-primary-500 text-white border border-primary-500/40 shadow-lg shadow-primary-900/40',
  accent:
    'bg-accent-500/15 hover:bg-accent-500/25 text-accent-300 border border-accent-500/40 shadow-lg shadow-accent-950/40',
  secondary:
    'bg-dark-700/60 hover:bg-dark-700 text-dark-200 border border-white/10',
  danger:
    'bg-red-600/15 hover:bg-red-600/25 text-red-300 border border-red-500/40',
  warning:
    'bg-amber-500/10 hover:bg-amber-500/20 text-amber-300 border border-amber-500/40',
  success:
    'bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-300 border border-emerald-500/40',
  ghost:
    'bg-transparent hover:bg-white/5 text-dark-300 hover:text-white border border-transparent',
};

const SIZES: Record<Size, string> = {
  sm: 'px-3 py-1.5 text-xs gap-1.5 rounded-lg',
  md: 'px-4 py-2 text-sm gap-2 rounded-xl',
};

export function Button({
  variant = 'primary',
  size = 'md',
  loading = false,
  disabled,
  children,
  className = '',
  ...rest
}: ButtonProps) {
  return (
    <motion.button
      whileTap={disabled || loading ? undefined : { scale: 0.97 }}
      disabled={disabled || loading}
      className={`inline-flex items-center justify-center font-medium transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-400 disabled:opacity-50 disabled:pointer-events-none ${VARIANTS[variant]} ${SIZES[size]} ${className}`}
      {...(rest as object)}
    >
      {loading && (
        <span className="h-3.5 w-3.5 animate-spin rounded-full border-2 border-current border-t-transparent" />
      )}
      {children}
    </motion.button>
  );
}
