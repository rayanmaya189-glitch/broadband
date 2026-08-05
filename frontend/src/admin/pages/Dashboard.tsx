import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import {
  AlertTriangle,
  BellRing,
  CreditCard,
  LifeBuoy,
  Radio,
  Repeat,
  Target,
  TrendingUp,
  Users,
} from 'lucide-react';
import { getDashboardSummary, type DashboardSummary } from '../../api/admin/modules';
import { listSubscriptions, listInvoices, listTickets, listDevices } from '../../api/admin/modules';
import type { Subscription, Invoice, Ticket, NetworkDevice } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { StatCard } from '../components/ui/StatCard';
import { DataTable, type Column } from '../components/ui/DataTable';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency, formatDateTime, timeAgo } from '../lib/format';

const EMPTY: DashboardSummary = {
  total_customers: 0,
  active_subscriptions: 0,
  monthly_revenue: 0,
  overdue_invoices: 0,
  open_tickets: 0,
  open_leads: 0,
  devices_online: 0,
  devices_total: 0,
  active_alerts: 0,
};

export function DashboardPage() {
  const [summary, setSummary] = useState<DashboardSummary>(EMPTY);
  const [tickets, setTickets] = useState<Ticket[]>([]);
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [subscriptions, setSubscriptions] = useState<Subscription[]>([]);
  const [devices, setDevices] = useState<NetworkDevice[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let active = true;
    void (async () => {
      try {
        const [sum, tks, inv, subs, devs] = await Promise.all([
          getDashboardSummary(),
          listTickets(),
          listInvoices(),
          listSubscriptions(),
          listDevices(),
        ]);
        if (!active) return;
        setSummary(sum);
        setTickets(tks);
        setInvoices(inv);
        setSubscriptions(subs);
        setDevices(devs);
      } catch {
        /* dashboard is best-effort */
      } finally {
        if (active) setLoading(false);
      }
    })();
    return () => {
      active = false;
    };
  }, []);

  const ticketColumns: Column<Ticket>[] = [
    {
      key: 'subject',
      header: 'Subject',
      render: (t) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-white">{t.subject}</p>
          <p className="text-xs text-dark-500">
            {t.ticket_number ?? `#${t.id}`} Â· {t.customer_name ?? 'â€”'}
          </p>
        </div>
      ),
    },
    {
      key: 'priority',
      header: 'Priority',
      render: (t) => <StatusBadge status={t.priority} />,
    },
    {
      key: 'status',
      header: 'Status',
      render: (t) => <StatusBadge status={t.status} />,
    },
    {
      key: 'created_at',
      header: 'Created',
      render: (t) => <span className="text-xs text-dark-500">{timeAgo(t.created_at)}</span>,
    },
  ];

  const invoiceColumns: Column<Invoice>[] = [
    {
      key: 'invoice_number',
      header: 'Invoice',
      render: (i) => <span className="font-medium text-white">{i.invoice_number ?? `#${i.id}`}</span>,
    },
    {
      key: 'customer_name',
      header: 'Customer',
      render: (i) => <span className="text-dark-300">{i.customer_name ?? 'â€”'}</span>,
    },
    {
      key: 'total_amount',
      header: 'Amount',
      align: 'right',
      render: (i) => <span className="tabular-nums text-dark-200">{formatCurrency(i.total_amount ?? i.amount)}</span>,
    },
    {
      key: 'status',
      header: 'Status',
      render: (i) => <StatusBadge status={i.status} />,
    },
  ];

  const subscriptionColumns: Column<Subscription>[] = [
    {
      key: 'customer_name',
      header: 'Customer',
      render: (s) => <span className="text-dark-200">{s.customer_name ?? `#${s.customer_id}`}</span>,
    },
    {
      key: 'plan_name',
      header: 'Plan',
      render: (s) => <span className="text-dark-300">{s.plan_name ?? `#${s.plan_id}`}</span>,
    },
    {
      key: 'status',
      header: 'Status',
      render: (s) => <StatusBadge status={s.status} />,
    },
    {
      key: 'expiry_date',
      header: 'Expires',
      render: (s) => <span className="text-xs text-dark-500">{formatDateTime(s.expiry_date)}</span>,
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-6">
        <PageHeader
          title="Dashboard"
          subtitle="Overview of your ISP operations at a glance."
          icon={<TrendingUp className="h-5 w-5" />}
        />

        {loading ? (
          <Spinner label="Loading overviewâ€¦" />
        ) : (
          <>
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
              <StatCard
                label="Total customers"
                value={summary.total_customers.toLocaleString('en-IN')}
                hint={`${summary.active_subscriptions} active subscriptions`}
                icon={<Users className="h-5 w-5" />}
                tone="primary"
                index={0}
              />
              <StatCard
                label="Active subscriptions"
                value={summary.active_subscriptions.toLocaleString('en-IN')}
                hint="Across all branches"
                icon={<Repeat className="h-5 w-5" />}
                tone="accent"
                index={1}
              />
              <StatCard
                label="Collected revenue"
                value={formatCurrency(summary.monthly_revenue)}
                hint={`${summary.overdue_invoices} overdue invoices`}
                icon={<CreditCard className="h-5 w-5" />}
                tone="success"
                index={2}
              />
              <StatCard
                label="Open tickets"
                value={summary.open_tickets.toLocaleString('en-IN')}
                hint={`${summary.open_leads} open leads`}
                icon={<LifeBuoy className="h-5 w-5" />}
                tone="warning"
                index={3}
              />
            </div>

            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
              <StatCard
                label="Devices online"
                value={`${summary.devices_online}/${summary.devices_total}`}
                hint="Network devices"
                icon={<Radio className="h-5 w-5" />}
                tone="success"
                index={0}
              />
              <StatCard
                label="Active alerts"
                value={summary.active_alerts.toLocaleString('en-IN')}
                hint="Firing monitoring alerts"
                icon={<BellRing className="h-5 w-5" />}
                tone={summary.active_alerts > 0 ? 'danger' : 'success'}
                index={1}
              />
              <StatCard
                label="Open leads"
                value={summary.open_leads.toLocaleString('en-IN')}
                hint="Sales pipeline"
                icon={<Target className="h-5 w-5" />}
                tone="accent"
                index={2}
              />
              <StatCard
                label="Overdue invoices"
                value={summary.overdue_invoices.toLocaleString('en-IN')}
                hint="Awaiting collection"
                icon={<AlertTriangle className="h-5 w-5" />}
                tone={summary.overdue_invoices > 0 ? 'warning' : 'success'}
                index={3}
              />
            </div>

            <div className="grid gap-6 xl:grid-cols-2">
              <section className="min-w-0">
                <div className="mb-3 flex items-center justify-between">
                  <h2 className="text-sm font-semibold text-white">Recent tickets</h2>
                  <Link to="/admin/tickets" className="text-xs text-accent-400 transition-colors hover:text-accent-300">
                    View all â†’
                  </Link>
                </div>
                <DataTable
                  columns={ticketColumns}
                  data={tickets.slice(0, 5)}
                  rowKey={(t) => t.id}
                  emptyTitle="No tickets yet"
                  emptyDescription="Support tickets will appear here."
                />
              </section>

              <section className="min-w-0">
                <div className="mb-3 flex items-center justify-between">
                  <h2 className="text-sm font-semibold text-white">Recent invoices</h2>
                  <Link to="/admin/billing" className="text-xs text-accent-400 transition-colors hover:text-accent-300">
                    View all â†’
                  </Link>
                </div>
                <DataTable
                  columns={invoiceColumns}
                  data={invoices.slice(0, 5)}
                  rowKey={(i) => i.id}
                  emptyTitle="No invoices yet"
                  emptyDescription="Invoices generated by billing will appear here."
                />
              </section>
            </div>

            <section className="min-w-0">
              <div className="mb-3 flex items-center justify-between">
                <h2 className="text-sm font-semibold text-white">Subscriptions</h2>
                <Link to="/admin/subscriptions" className="text-xs text-accent-400 transition-colors hover:text-accent-300">
                  View all â†’
                </Link>
              </div>
              <DataTable
                columns={subscriptionColumns}
                data={subscriptions.slice(0, 5)}
                rowKey={(s) => s.id}
                emptyTitle="No subscriptions yet"
                emptyDescription="Customer subscriptions will appear here."
              />
            </section>

            {devices.length > 0 && (
              <section>
                <div className="mb-3 flex items-center justify-between">
                  <h2 className="text-sm font-semibold text-white">Device status</h2>
                  <Link to="/admin/devices" className="text-xs text-accent-400 transition-colors hover:text-accent-300">
                    View all â†’
                  </Link>
                </div>
                <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4">
                  {devices.slice(0, 8).map((d) => (
                    <div key={d.id} className="glass-card rounded-xl px-4 py-3">
                      <div className="flex items-center justify-between gap-2">
                        <p className="truncate text-sm font-medium text-white">{d.name}</p>
                        <StatusBadge status={d.status} pulse={d.status === 'online'} />
                      </div>
                      <p className="mt-1 text-xs text-dark-500">{d.ip_address ?? d.device_type}</p>
                    </div>
                  ))}
                </div>
              </section>
            )}
          </>
        )}
      </div>
    </PageTransition>
  );
}
