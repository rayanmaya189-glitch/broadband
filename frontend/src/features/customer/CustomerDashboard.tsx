import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { CreditCard, LifeBuoy, Wifi, TrendingUp, ArrowRight } from 'lucide-react';
import { useCustomerAuthStore } from '../../store/customerAuthStore';
import { getMySubscription, getMyInvoices, getBandwidthUsage } from '../../api/customer';
import { formatCurrency, formatDate } from '../../utils/format';

export default function CustomerDashboard() {
  const user = useCustomerAuthStore((s) => s.user);
  const [subscription, setSubscription] = useState<Record<string, unknown> | null>(null);
  const [invoices, setInvoices] = useState<Record<string, unknown>[]>([]);
  const [usage, setUsage] = useState<Record<string, unknown> | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let active = true;
    (async () => {
      try {
        const [sub, inv] = await Promise.all([getMySubscription(), getMyInvoices()]);
        if (!active) return;
        setSubscription(sub);
        setInvoices(inv.slice(0, 5));
        if (sub?.id) {
          const u = await getBandwidthUsage(sub.id as number).catch(() => null);
          if (active) setUsage(u);
        }
      } catch { /* dashboard is best-effort */ }
      finally { if (active) setLoading(false); }
    })();
    return () => { active = false; };
  }, []);

  const sub = subscription as Record<string, unknown> | null;
  const overdueCount = invoices.filter((i) => i.status === 'overdue').length;

  return (
    <div className="space-y-6">
      {/* Welcome */}
      <div>
        <h1 className="text-2xl font-bold text-white">Welcome back, {user?.name ?? 'there'} 👋</h1>
        <p className="mt-1 text-sm text-dark-400">Here&apos;s your account overview.</p>
      </div>

      {/* Stat cards */}
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {[
          { label: 'Plan', value: (sub?.plan_name as string) ?? 'No plan', icon: Wifi, color: 'text-primary-400' },
          { label: 'Status', value: (sub?.status as string) ?? 'inactive', icon: TrendingUp, color: sub?.status === 'active' ? 'text-emerald-400' : 'text-amber-400' },
          { label: 'Download', value: usage?.download_kbps ? `${((usage.download_kbps as number) / 1000).toFixed(0)} Mbps` : '—', icon: Wifi, color: 'text-accent-400' },
          { label: 'Overdue invoices', value: overdueCount.toString(), icon: CreditCard, color: overdueCount > 0 ? 'text-red-400' : 'text-dark-400' },
        ].map((card) => (
          <div key={card.label} className="rounded-xl border border-white/[0.06] bg-white/[0.02] p-4">
            <div className="flex items-center gap-3">
              <card.icon className={`h-5 w-5 ${card.color}`} />
              <span className="text-xs text-dark-500">{card.label}</span>
            </div>
            <p className="mt-2 text-lg font-semibold text-white capitalize">{card.value}</p>
          </div>
        ))}
      </div>

      {/* Subscription details */}
      {sub && (
        <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] p-5">
          <h2 className="mb-3 text-sm font-semibold text-white">Your subscription</h2>
          <div className="grid gap-3 text-sm sm:grid-cols-3">
            <div><p className="text-xs text-dark-500">Plan</p><p className="text-dark-200">{sub.plan_name as string ?? `#${sub.plan_id}`}</p></div>
            <div><p className="text-xs text-dark-500">Monthly fee</p><p className="text-dark-200">{formatCurrency(sub.monthly_fee as number)}</p></div>
            <div><p className="text-xs text-dark-500">Expiry</p><p className="text-dark-200">{formatDate(sub.expiry_date as string)}</p></div>
          </div>
        </div>
      )}

      {/* Quick actions */}
      <div className="grid gap-3 sm:grid-cols-3">
        {[
          { label: 'View invoices', href: '/portal/invoices', icon: CreditCard },
          { label: 'Submit a ticket', href: '/portal/tickets', icon: LifeBuoy },
          { label: 'Browse plans', href: '/plans', icon: TrendingUp },
        ].map((action) => (
          <Link key={action.href} to={action.href} className="flex items-center gap-3 rounded-xl border border-white/[0.06] bg-white/[0.02] p-4 transition-colors hover:bg-white/[0.04]">
            <action.icon className="h-5 w-5 text-primary-400" />
            <span className="text-sm font-medium text-dark-200">{action.label}</span>
            <ArrowRight className="ml-auto h-4 w-4 text-dark-500" />
          </Link>
        ))}
      </div>

      {/* Recent invoices */}
      {invoices.length > 0 && (
        <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] p-5">
          <div className="mb-3 flex items-center justify-between">
            <h2 className="text-sm font-semibold text-white">Recent invoices</h2>
            <Link to="/portal/invoices" className="text-xs text-accent-400 hover:text-accent-300">View all →</Link>
          </div>
          <div className="space-y-2">
            {invoices.map((inv) => (
              <div key={inv.id as number} className="flex items-center justify-between gap-3 rounded-lg border border-white/[0.04] px-4 py-2.5 text-sm">
                <div>
                  <p className="font-medium text-white">{(inv.invoice_number as string) ?? `#${inv.id}`}</p>
                  <p className="text-xs text-dark-500">Due: {formatDate(inv.due_date as string)}</p>
                </div>
                <div className="text-right">
                  <p className="tabular-nums text-dark-200">{formatCurrency((inv.total_amount as number) ?? (inv.amount as number))}</p>
                  <span className={`text-xs ${inv.status === 'paid' ? 'text-emerald-400' : inv.status === 'overdue' ? 'text-red-400' : 'text-dark-500'}`}>
                    {(inv.status as string)}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {loading && (
        <div className="py-8 text-center text-sm text-dark-500">Loading your account…</div>
      )}
    </div>
  );
}
