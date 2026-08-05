import { useEffect, useState } from 'react';
import { ScrollText, Download } from 'lucide-react';
import { listAuditLogs } from '../../api/admin/modules';
import type { AuditLog } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function AuditPage() {
  const [data, setData] = useState<AuditLog[]>([]);
  const [loading, setLoading] = useState(true);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listAuditLogs());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load audit logs', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const exportCsv = () => {
    if (data.length === 0) return;
    const header = ['id', 'action', 'entity_type', 'entity_id', 'user', 'ip_address', 'created_at'];
    const rows = data.map((l) => [
      l.id,
      l.action,
      l.entity_type ?? '',
      l.entity_id ?? '',
      l.user_name ?? l.user_id ?? '',
      l.ip_address ?? '',
      l.created_at ?? '',
    ]);
    const csv = [header, ...rows]
      .map((r) => r.map((v) => `"${String(v).replace(/"/g, '""')}"`).join(','))
      .join('\n');
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `audit-log-${new Date().toISOString().slice(0, 10)}.csv`;
    a.click();
    URL.revokeObjectURL(url);
    toast('Audit log exported');
  };

  const columns: Column<AuditLog>[] = [
    { key: 'id', header: 'ID', render: (l) => <span className="font-mono text-xs text-dark-500">#{l.id}</span> },
    { key: 'action', header: 'Action', render: (l) => <span className="font-medium text-white">{l.action}</span> },
    { key: 'entity_type', header: 'Entity', render: (l) => (
      <span className="text-dark-300">{l.entity_type ?? 'â€”'}{l.entity_id ? ` #${l.entity_id}` : ''}</span>
    )},
    { key: 'user_name', header: 'User', render: (l) => <span className="text-dark-300">{l.user_name ?? `#${l.user_id ?? 'system'}`}</span> },
    { key: 'ip_address', header: 'IP', render: (l) => <span className="font-mono text-xs text-dark-500">{l.ip_address ?? 'â€”'}</span> },
    { key: 'created_at', header: 'Timestamp', render: (l) => <span className="text-xs text-dark-500">{formatDateTime(l.created_at)}</span> },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Audit log"
          subtitle="A record of actions taken across the platform."
          icon={<ScrollText className="h-5 w-5" />}
          actions={
            <Button variant="secondary" onClick={exportCsv} disabled={data.length === 0}>
              <Download className="h-4 w-4" /> Export CSV
            </Button>
          }
        />

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={data}
            rowKey={(l) => l.id}
            emptyTitle="No audit events"
            emptyDescription="Audited actions will appear here."
          />
        )}
      </div>
    </PageTransition>
  );
}
