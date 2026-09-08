import { useEffect, useState } from 'react';
import { Repeat, Plus, History } from 'lucide-react';
import {
  createSubscription,
  listSubscriptions,
  subscriptionAction,
  getSubscriptionHistory,
  listCustomers,
  listPlans,
} from '../../api/admin/modules';
import type { Subscription, SubscriptionHistory, Plan } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { useAdminStore } from '../adminStore';
import { formatDateTime, formatCurrency, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

const ACTIONS: { action: 'suspend' | 'reactivate' | 'cancel'; label: string; variant: 'warning' | 'success' | 'danger' }[] = [
  { action: 'suspend', label: 'Suspend', variant: 'warning' },
  { action: 'reactivate', label: 'Reactivate', variant: 'success' },
  { action: 'cancel', label: 'Cancel', variant: 'danger' },
];

interface CustomerOption { id: number; name: string; phone?: string; customer_code?: string }

export function SubscriptionsPage() {
  const [data, setData] = useState<Subscription[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [acting, setActing] = useState<{ id: number; label: string } | null>(null);
  const [actingBusy, setActingBusy] = useState(false);
  const [historyTarget, setHistoryTarget] = useState<Subscription | null>(null);
  const [history, setHistory] = useState<SubscriptionHistory[]>([]);
  const [historyLoading, setHistoryLoading] = useState(false);
  const [upgradeTarget, setUpgradeTarget] = useState<Subscription | null>(null);
  const [plans, setPlans] = useState<Plan[]>([]);

  const load = async () => {
    setLoading(true);
    try {
      const [subs, plns] = await Promise.all([listSubscriptions(), listPlans()]);
      setData(subs);
      setPlans(plns);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load subscriptions', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const runAction = async (action: (typeof ACTIONS)[number]['action'], sub: Subscription) => {
    setActing({ id: sub.id, label: action });
    setActingBusy(true);
    try {
      await subscriptionAction(sub.id, action);
      toast(`Subscription ${action}d`);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : `Failed to ${action}`, 'error');
    } finally {
      setActingBusy(false);
      setActing(null);
    }
  };

  const openHistory = async (sub: Subscription) => {
    setHistoryTarget(sub);
    setHistoryLoading(true);
    setHistory([]);
    try {
      setHistory(await getSubscriptionHistory(sub.id));
    } catch {
      setHistory([]);
    } finally {
      setHistoryLoading(false);
    }
  };

  const columns: Column<Subscription>[] = [
    {
      key: 'customer_name',
      header: 'Customer',
      render: (s) => (
        <div>
          <p className="font-medium text-white">{s.customer_name ?? `Customer #${s.customer_id}`}</p>
          <p className="text-xs text-dark-500">ID {s.id}</p>
        </div>
      ),
    },
    { key: 'plan_name', header: 'Plan', render: (s) => <span className="text-dark-300">{s.plan_name ?? `#${s.plan_id}`}</span> },
    { key: 'monthly_fee', header: 'Fee', align: 'right', render: (s) => (
      <span className="tabular-nums text-dark-200">{s.monthly_fee != null ? formatCurrency(s.monthly_fee) : '—'}</span>
    )},
    { key: 'status', header: 'Status', render: (s) => <StatusBadge status={s.status} pulse={s.status === 'active'} /> },
    { key: 'start_date', header: 'Start', render: (s) => <span className="text-xs text-dark-500">{formatDateTime(s.start_date)}</span> },
    { key: 'expiry_date', header: 'Expires', render: (s) => <span className="text-xs text-dark-500">{formatDateTime(s.expiry_date)}</span> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (s) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => void openHistory(s)} title="History">
            <History className="h-4 w-4" />
          </Button>
          {s.status === 'active' && (
            <Button variant="accent" size="sm" onClick={() => setUpgradeTarget(s)} title="Upgrade/Downgrade">
              Upgrade
            </Button>
          )}
          {ACTIONS.map((a) => (
            <Button
              key={a.action}
              variant={a.variant}
              size="sm"
              loading={acting?.id === s.id && acting.label === a.action && actingBusy}
              disabled={acting !== null}
              onClick={() => void runAction(a.action, s)}
            >
              {a.label}
            </Button>
          ))}
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Subscriptions"
          subtitle="Active and historical customer subscriptions."
          icon={<Repeat className="h-5 w-5" />}
          actions={
            <Button onClick={() => setCreateOpen(true)}>
              <Plus className="h-4 w-4" /> New subscription
            </Button>
          }
        />

        <DataTable
          columns={columns}
          data={data}
          rowKey={(s) => s.id}
          loading={loading}
          emptyTitle="No subscriptions"
          emptyDescription="Create a subscription for a customer to get started."
        />

        <CreateSubscriptionModal
          open={createOpen}
          onClose={() => setCreateOpen(false)}
          onCreated={() => {
            setCreateOpen(false);
            void load();
          }}
        />

        <Modal
          open={historyTarget !== null}
          onClose={() => setHistoryTarget(null)}
          title="Subscription history"
          subtitle={historyTarget ? `${historyTarget.customer_name ?? `#${historyTarget.customer_id}`} — ${historyTarget.plan_name ?? ''}` : undefined}
          width="lg"
        >
          {historyLoading ? (
            <Spinner />
          ) : history.length === 0 ? (
            <p className="py-6 text-center text-sm text-dark-500">No history recorded for this subscription.</p>
          ) : (
            <div className="space-y-2 max-h-96 overflow-y-auto">
              {history.map((h) => (
                <div key={h.id} className="flex items-center justify-between gap-3 rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3">
                  <div>
                    <p className="text-sm font-medium text-white">{toLabel(h.action)}</p>
                    {h.old_value && <p className="text-xs text-dark-500">From: {h.old_value}</p>}
                    {h.new_value && <p className="text-xs text-dark-500">To: {h.new_value}</p>}
                    {h.performed_by_name && <p className="text-xs text-dark-500">By: {h.performed_by_name}</p>}
                  </div>
                  <span className="shrink-0 text-xs text-dark-500">{formatDateTime(h.created_at)}</span>
                </div>
              ))}
            </div>
          )}
        </Modal>

        <Modal
          open={upgradeTarget !== null}
          onClose={() => setUpgradeTarget(null)}
          title="Upgrade/Downgrade subscription"
          subtitle={upgradeTarget ? `${upgradeTarget.customer_name ?? `#${upgradeTarget.customer_id}`} — currently on ${upgradeTarget.plan_name ?? `#${upgradeTarget.plan_id}`}` : undefined}
          width="md"
        >
          {upgradeTarget && (
            <UpgradeDowngradeForm
              subscription={upgradeTarget}
              plans={plans}
              onDone={() => {
                setUpgradeTarget(null);
                void load();
              }}
            />
          )}
        </Modal>
      </div>
    </PageTransition>
  );
}

function CreateSubscriptionModal({
  open,
  onClose,
  onCreated,
}: {
  open: boolean;
  onClose: () => void;
  onCreated: () => void;
}) {
  const [customers, setCustomers] = useState<CustomerOption[]>([]);
  const [plans, setPlans] = useState<Plan[]>([]);
  const [customerId, setCustomerId] = useState('');
  const [planId, setPlanId] = useState('');
  const [billingPeriod, setBillingPeriod] = useState('1');
  const [busy, setBusy] = useState(false);
  const [loadingOpts, setLoadingOpts] = useState(true);

  useEffect(() => {
    if (!open) return;
    setLoadingOpts(true);
    void Promise.all([
      listCustomers({ page: 1, page_size: 200 }).then((r) =>
        setCustomers(r.customers.map((c) => ({ id: c.id, name: c.name, phone: c.phone, customer_code: c.customer_code })))
      ).catch(() => undefined),
      listPlans().then(setPlans).catch(() => undefined),
    ]).finally(() => setLoadingOpts(false));
  }, [open]);

  const reset = () => {
    setCustomerId('');
    setPlanId('');
    setBillingPeriod('1');
  };

  const submit = async () => {
    if (!customerId || !planId) return;
    setBusy(true);
    try {
      const branchId = useAdminStore.getState().branchId ?? undefined;
      await createSubscription({
        customer_id: Number(customerId),
        plan_id: Number(planId),
        billing_period_months: Number(billingPeriod),
        branch_id: branchId,
      });
      toast('Subscription created');
      reset();
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create subscription', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open={open}
      onClose={() => { reset(); onClose(); }}
      title="New subscription"
      subtitle="Attach a plan to a customer."
      width="md"
      footer={
        <>
          <Button variant="ghost" onClick={onClose}>Cancel</Button>
          <Button onClick={submit} loading={busy}>Create subscription</Button>
        </>
      }
    >
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        {loadingOpts ? (
          <Spinner label="Loading options…" />
        ) : (
          <>
            <SelectField
              label="Customer"
              required
              value={customerId}
              onChange={(e) => setCustomerId(e.target.value)}
              options={customers.map((c) => ({ value: String(c.id), label: `${c.name} (${c.customer_code ?? `#${c.id}`})` }))}
              placeholder="Select a customer"
            />
            <SelectField
              label="Plan"
              required
              value={planId}
              onChange={(e) => setPlanId(e.target.value)}
              options={plans.map((p) => ({ value: String(p.id), label: p.name }))}
              placeholder="Select a plan"
            />
          </>
        )}
        <SelectField label="Billing period" value={billingPeriod} onChange={(e) => setBillingPeriod(e.target.value)}
          options={[
            { value: '1', label: 'Monthly' },
            { value: '3', label: 'Quarterly (3 months)' },
            { value: '6', label: 'Half-yearly (6 months)' },
            { value: '12', label: 'Annual (12 months)' },
          ]}
        />
      </form>
    </Modal>
  );
}

function UpgradeDowngradeForm({
  subscription,
  plans,
  onDone,
}: {
  subscription: Subscription;
  plans: Plan[];
  onDone: () => void;
}) {
  const [newPlanId, setNewPlanId] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!newPlanId) return;
    const action = Number(newPlanId) > subscription.plan_id ? 'upgrade' : 'downgrade';
    setBusy(true);
    try {
      await subscriptionAction(subscription.id, action as 'upgrade' | 'downgrade', { plan_id: Number(newPlanId) });
      toast(`Subscription ${action}d`);
      onDone();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to change plan', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="rounded-xl border border-white/[0.06] bg-dark-950/40 p-4 text-sm text-dark-300">
        <p>Current plan: <span className="text-white">{subscription.plan_name ?? `#${subscription.plan_id}`}</span></p>
        <p>Monthly fee: <span className="text-white">{subscription.monthly_fee != null ? formatCurrency(subscription.monthly_fee) : '—'}</span></p>
      </div>
      <SelectField
        label="New plan"
        value={newPlanId}
        onChange={(e) => setNewPlanId(e.target.value)}
        options={plans.map((p) => ({ value: String(p.id), label: p.name }))}
        placeholder="Select new plan"
      />
      <div className="flex justify-end gap-2">
        <Button variant="ghost" onClick={onDone}>Cancel</Button>
        <Button onClick={submit} loading={busy} disabled={!newPlanId}>Change plan</Button>
      </div>
    </div>
  );
}
