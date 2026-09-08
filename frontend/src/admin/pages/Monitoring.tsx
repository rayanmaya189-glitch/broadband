import { useEffect, useState } from 'react';
import { Activity, BellRing, CheckCircle2, Plus } from 'lucide-react';
import { alertAction, createAlert, listAlerts } from '../../api/admin/modules';
import type { MonitoringAlert } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField, TextArea } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { timeAgo } from '../lib/format';
import { toast } from '../lib/toast';

export function MonitoringPage() {
  const [data, setData] = useState<MonitoringAlert[]>([]);
  const [loading, setLoading] = useState(true);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [severityFilter, setSeverityFilter] = useState('');
  const [statusFilter, setStatusFilter] = useState('');

  const load = async () => {
    setLoading(true);
    try {
      setData(await listAlerts());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load alerts', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const runAction = async (alert: MonitoringAlert, action: 'acknowledge' | 'resolve') => {
    setActing({ id: alert.id, action });
    try {
      await alertAction(alert.id, action);
      toast(`Alert ${action}d`);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : `Failed to ${action} alert`, 'error');
    } finally {
      setActing(null);
    }
  };

  const filtered = data.filter((a) => {
    if (severityFilter && a.severity !== severityFilter) return false;
    if (statusFilter && a.status !== statusFilter) return false;
    return true;
  });

  const firing = data.filter((a) => a.status === 'firing').length;
  const acknowledged = data.filter((a) => a.status === 'acknowledged').length;

  const columns: Column<MonitoringAlert>[] = [
    { key: 'device_name', header: 'Device', render: (a) => (
      <div>
        <p className="font-medium text-white">{a.device_name ?? `#${a.device_id ?? '—'}`}</p>
        <p className="text-xs text-dark-500">{a.metric ?? ''}</p>
      </div>
    )},
    { key: 'severity', header: 'Severity', render: (a) => <StatusBadge status={a.severity} /> },
    { key: 'message', header: 'Message', render: (a) => <span className="max-w-md truncate text-dark-300">{a.message ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (a) => <StatusBadge status={a.status} pulse={a.status === 'firing'} /> },
    { key: 'triggered_at', header: 'Triggered', render: (a) => <span className="text-xs text-dark-500">{timeAgo(a.triggered_at)}</span> },
    {
      key: 'actions', header: 'Actions', align: 'right',
      render: (a) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          {a.status === 'firing' && (
            <Button variant="warning" size="sm" loading={acting?.id === a.id && acting.action === 'acknowledge'}
              onClick={() => void runAction(a, 'acknowledge')}>
              Acknowledge
            </Button>
          )}
          <Button variant="success" size="sm" loading={acting?.id === a.id && acting.action === 'resolve'}
            onClick={() => void runAction(a, 'resolve')}>
            Resolve
          </Button>
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Monitoring"
          subtitle="Alerts and health status across the network."
          icon={<Activity className="h-5 w-5" />}
          actions={
            <Button onClick={() => setCreateOpen(true)}>
              <Plus className="h-4 w-4" /> New alert
            </Button>
          }
        />

        <div className="grid gap-4 sm:grid-cols-3">
          <div className="glass-card flex items-center gap-4 rounded-2xl p-5">
            <div className="flex h-10 w-10 items-center justify-center rounded-xl border border-red-500/30 bg-red-500/10 text-red-400">
              <BellRing className="h-5 w-5" />
            </div>
            <div>
              <p className="text-2xl font-semibold tabular-nums text-white">{firing}</p>
              <p className="text-xs text-dark-400">Firing alerts</p>
            </div>
          </div>
          <div className="glass-card flex items-center gap-4 rounded-2xl p-5">
            <div className="flex h-10 w-10 items-center justify-center rounded-xl border border-amber-500/30 bg-amber-500/10 text-amber-400">
              <CheckCircle2 className="h-5 w-5" />
            </div>
            <div>
              <p className="text-2xl font-semibold tabular-nums text-white">{acknowledged}</p>
              <p className="text-xs text-dark-400">Acknowledged</p>
            </div>
          </div>
          <div className="glass-card flex items-center gap-4 rounded-2xl p-5">
            <div className="flex h-10 w-10 items-center justify-center rounded-xl border border-emerald-500/30 bg-emerald-500/10 text-emerald-400">
              <Activity className="h-5 w-5" />
            </div>
            <div>
              <p className="text-2xl font-semibold tabular-nums text-white">{data.length}</p>
              <p className="text-xs text-dark-400">Total alerts</p>
            </div>
          </div>
        </div>

        <div className="flex flex-wrap gap-2">
          <SelectField
            value={severityFilter}
            onChange={(e) => setSeverityFilter(e.target.value)}
            options={[
              { value: '', label: 'All severities' },
              { value: 'critical', label: 'Critical' },
              { value: 'high', label: 'High' },
              { value: 'medium', label: 'Medium' },
              { value: 'low', label: 'Low' },
            ]}
            className="w-40"
          />
          <SelectField
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value)}
            options={[
              { value: '', label: 'All statuses' },
              { value: 'firing', label: 'Firing' },
              { value: 'acknowledged', label: 'Acknowledged' },
              { value: 'resolved', label: 'Resolved' },
            ]}
            className="w-40"
          />
        </div>

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={filtered}
            rowKey={(a) => a.id}
            emptyTitle="No alerts"
            emptyDescription="Monitoring alerts will appear here when triggered."
          />
        )}

        <CreateAlertModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />
      </div>
    </PageTransition>
  );
}

function CreateAlertModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [deviceId, setDeviceId] = useState('');
  const [severity, setSeverity] = useState('medium');
  const [title, setTitle] = useState('');
  const [message, setMessage] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!deviceId || !title.trim() || !message.trim()) return;
    setBusy(true);
    try {
      await createAlert({
        device_id: Number(deviceId),
        severity,
        title: title.trim(),
        message: message.trim(),
      });
      toast('Alert created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create alert', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New alert" subtitle="Manually create a monitoring alert." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create alert</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Device ID" type="number" required value={deviceId} onChange={(e) => setDeviceId(e.target.value)} />
        <SelectField label="Severity" value={severity} onChange={(e) => setSeverity(e.target.value)}
          options={[
            { value: 'low', label: 'Low' },
            { value: 'medium', label: 'Medium' },
            { value: 'high', label: 'High' },
            { value: 'critical', label: 'Critical' },
          ]}
        />
        <TextField label="Title" required value={title} onChange={(e) => setTitle(e.target.value)} placeholder="e.g. High CPU load" />
        <TextArea label="Message" required value={message} onChange={(e) => setMessage(e.target.value)} placeholder="Describe the alert…" />
      </form>
    </Modal>
  );
}
