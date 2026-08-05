import { toLabel } from '../../lib/format';

type Tone = 'success' | 'warning' | 'danger' | 'info' | 'neutral';

const SUCCESS = ['active', 'online', 'up', 'paid', 'completed', 'success', 'approved', 'published', 'delivered', 'open', 'enabled', 'healthy', 'resolved', 'operational', 'allocated', 'available', 'connected', 'sent', 'confirmed', 'posted'];
const WARNING = ['pending', 'in_progress', 'scheduled', 'firing', 'unverified', 'draft', 'processing', 'verifying', 'review', 'inactive', 'expiring', 'unpaid', 'partial', 'retrying', 'queued', 'requested'];
const DANGER = ['suspended', 'down', 'terminated', 'expired', 'cancelled', 'canceled', 'overdue', 'void', 'failed', 'blocked', 'revoked', 'disabled', 'closed', 'lost', 'rejected', 'error', 'critical', 'disconnected', 'unavailable', 'no_connection'];
const INFO = ['info', 'notice', 'upgrading', 'downgrading', 'syncing'];

const TONES: Record<Tone, string> = {
  success: 'bg-emerald-500/10 text-emerald-300 border-emerald-500/30',
  warning: 'bg-amber-500/10 text-amber-300 border-amber-500/30',
  danger: 'bg-red-500/10 text-red-300 border-red-500/30',
  info: 'bg-sky-500/10 text-sky-300 border-sky-500/30',
  neutral: 'bg-white/[0.05] text-dark-300 border-white/10',
};

export function statusTone(status: string | null | undefined): Tone {
  const s = (status ?? '').toLowerCase();
  if (!s) return 'neutral';
  if (SUCCESS.some((k) => s === k)) return 'success';
  if (DANGER.some((k) => s === k)) return 'danger';
  if (WARNING.some((k) => s === k)) return 'warning';
  if (INFO.some((k) => s === k)) return 'info';
  return 'neutral';
}

export function StatusBadge({ status, pulse = false }: { status: string | null | undefined; pulse?: boolean }) {
  const tone = statusTone(status);
  return (
    <span
      className={`inline-flex items-center gap-1.5 whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs font-medium ${TONES[tone]}`}
    >
      {pulse && <span className="relative flex h-1.5 w-1.5">
        <span className={`absolute inline-flex h-full w-full animate-ping rounded-full opacity-60 ${tone === 'success' ? 'bg-emerald-400' : tone === 'danger' ? 'bg-red-400' : tone === 'warning' ? 'bg-amber-400' : 'bg-sky-400'}`} />
        <span className={`relative inline-flex h-1.5 w-1.5 rounded-full ${tone === 'success' ? 'bg-emerald-400' : tone === 'danger' ? 'bg-red-400' : tone === 'warning' ? 'bg-amber-400' : 'bg-sky-400'}`} />
      </span>}
      {toLabel(status) || '—'}
    </span>
  );
}
