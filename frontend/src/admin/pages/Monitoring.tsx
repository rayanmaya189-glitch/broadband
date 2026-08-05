import { useEffect, useState } from 'react';
import { Activity, BellRing, CheckCircle2 } from 'lucide-react';
import { alertAction, listAlerts } from '../../api/admin/modules';
import type { MonitoringAlert } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { timeAgo } from '../lib/format';
import { toast } from '../lib/toast';

export function MonitoringPage() {
  const [data, setData] = useState<MonitoringAlert[]>([]);
  const [loading, setLoading] = useState(true);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);

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

  const firing = data.filter((a) => a.status === 'firing').length;
  const acknowledged = data.filter((a) => a.status === 'acknowledged').length;

  const columns: Column<MonitoringAlert>[] = [
    { key: 'device_name', header: 'Device', render: (a) => (
      <div>
        <p className="font-medium text-white">{a.device_name ?? `#${a.device_id ?? 'â€”'}`}</p>
        <p className="text-xs text-dark-500">{a.metric ?? ''}</p>
      </div>
    )},
    { key: 'severity', header: 'Severity', render: (a) => <StatusBadge status={a.severity} /> },
    { key: 'message', header: 'Message', render: (a) => <span className="max-w-md truncate text-dark-300">{a.message ?? 'â€”'}</span> },
    { key: 'status', header: 'Status', render: (a) => <StatusBadge status={a.status} pulse={a.status === 'firing'} /> },
    { key: 'triggered_at', header: 'Triggered', render: (a) => <span className="text-xs text-dark-500">{timeAgo(a.triggered_at)}</span> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
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

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={data}
            rowKey={(a) => a.id}
            emptyTitle="No alerts"
            emptyDescription="Monitoring alerts will appear here when triggered."
          />
        )}
      </div>
    </PageTransition>
  );
}
