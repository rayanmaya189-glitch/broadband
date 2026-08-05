import { useEffect, useState } from 'react';
import { Wrench, Plus, CalendarClock } from 'lucide-react';
import { createInstallation, installationAction, listInstallations } from '../../api/admin/modules';
import type { Installation } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { formatDateTime, timeAgo } from '../lib/format';
import { toast } from '../lib/toast';

const ACTIONS: { action: 'start' | 'complete' | 'cancel'; label: string; variant: 'accent' | 'success' | 'danger' }[] = [
  { action: 'start', label: 'Start', variant: 'accent' },
  { action: 'complete', label: 'Complete', variant: 'success' },
  { action: 'cancel', label: 'Cancel', variant: 'danger' },
];

export function InstallationsPage() {
  const [data, setData] = useState<Installation[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listInstallations());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load installations', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const runAction = async (action: string, inst: Installation) => {
    setActing({ id: inst.id, action });
    try {
      await installationAction(inst.id, action as 'start' | 'complete' | 'cancel');
      toast(`Installation ${action}ed`);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : `Failed to ${action} installation`, 'error');
    } finally {
      setActing(null);
    }
  };

  const columns: Column<Installation>[] = [
    { key: 'customer_name', header: 'Customer', render: (i) => (
      <div>
        <p className="font-medium text-white">{i.customer_name ?? `Customer #${i.customer_id ?? 'â€”'}`}</p>
        <p className="max-w-xs truncate text-xs text-dark-500">{i.address ?? ''}</p>
      </div>
    )},
    { key: 'scheduled_at', header: 'Scheduled', render: (i) => (
      <span className="flex items-center gap-1.5 text-xs text-dark-300">
        <CalendarClock className="h-3.5 w-3.5 text-dark-500" /> {formatDateTime(i.scheduled_at)}
      </span>
    )},
    { key: 'technician_name', header: 'Technician', render: (i) => <span className="text-dark-400">{i.technician_name ?? `#${i.technician_id ?? 'â€”'}`}</span> },
    { key: 'status', header: 'Status', render: (i) => <StatusBadge status={i.status} /> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (i) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          {ACTIONS.filter((a) => !(i.status === 'completed' && a.action === 'complete')).map((a) => (
            <Button
              key={a.action}
              variant={a.variant}
              size="sm"
              loading={acting?.id === i.id && acting.action === a.action}
              disabled={acting !== null}
              onClick={() => void runAction(a.action, i)}
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
          title="Installations"
          subtitle="Scheduling and field work for new connections."
          icon={<Wrench className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New installation</Button>}
        />

        <DataTable
          columns={columns}
          data={data}
          rowKey={(i) => i.id}
          loading={loading}
          emptyTitle="No installations"
          emptyDescription="Scheduled installations will appear here."
        />

        <CreateInstallationModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />
      </div>
    </PageTransition>
  );
}

function CreateInstallationModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [customerId, setCustomerId] = useState('');
  const [address, setAddress] = useState('');
  const [scheduledAt, setScheduledAt] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!customerId) return;
    setBusy(true);
    try {
      await createInstallation({
        customer_id: Number(customerId),
        address: address.trim() || undefined,
        scheduled_at: scheduledAt || undefined,
      });
      toast('Installation created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create installation', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New installation" subtitle="Schedule an installation for a customer." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Customer ID" type="number" required value={customerId} onChange={(e) => setCustomerId(e.target.value)} />
        <TextField label="Address" value={address} onChange={(e) => setAddress(e.target.value)} />
        <TextField label="Scheduled time" type="datetime-local" value={scheduledAt} onChange={(e) => setScheduledAt(e.target.value)} />
      </form>
    </Modal>
  );
}
