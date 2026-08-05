import { useEffect, useState } from 'react';
import { Repeat, Plus } from 'lucide-react';
import { createSubscription, listSubscriptions, subscriptionAction } from '../../api/admin/modules';
import type { Subscription } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { formatDateTime, formatCurrency } from '../lib/format';
import { toast } from '../lib/toast';

const ACTIONS: { action: 'suspend' | 'reactivate' | 'cancel'; label: string; variant: 'warning' | 'success' | 'danger' }[] = [
  { action: 'suspend', label: 'Suspend', variant: 'warning' },
  { action: 'reactivate', label: 'Reactivate', variant: 'success' },
  { action: 'cancel', label: 'Cancel', variant: 'danger' },
];

export function SubscriptionsPage() {
  const [data, setData] = useState<Subscription[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [acting, setActing] = useState<{ id: number; label: string } | null>(null);
  const [actingBusy, setActingBusy] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listSubscriptions());
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
      <span className="tabular-nums text-dark-200">{s.monthly_fee != null ? formatCurrency(s.monthly_fee) : 'â€”'}</span>
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
  const [customerId, setCustomerId] = useState('');
  const [planId, setPlanId] = useState('');
  const [startDate, setStartDate] = useState('');
  const [busy, setBusy] = useState(false);

  const reset = () => {
    setCustomerId('');
    setPlanId('');
    setStartDate('');
  };

  const submit = async () => {
    if (!customerId || !planId) return;
    setBusy(true);
    try {
      await createSubscription({
        customer_id: Number(customerId),
        plan_id: Number(planId),
        start_date: startDate || undefined,
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
      onClose={() => {
        reset();
        onClose();
      }}
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
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void submit();
        }}
        className="space-y-4"
      >
        <TextField
          label="Customer ID"
          type="number"
          required
          value={customerId}
          onChange={(e) => setCustomerId(e.target.value)}
        />
        <TextField
          label="Plan ID"
          type="number"
          required
          value={planId}
          onChange={(e) => setPlanId(e.target.value)}
        />
        <TextField label="Start date" type="date" value={startDate} onChange={(e) => setStartDate(e.target.value)} />
      </form>
    </Modal>
  );
}
