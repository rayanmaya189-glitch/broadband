import { useEffect, useState } from 'react';
import { ScrollText, Download, Search } from 'lucide-react';
import { searchAuditLogs, exportAuditLogs, listAuditEvents } from '../../api/admin/modules';
import type { AuditLog, AuditEvent } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { SelectField, TextField } from '../components/ui/FormField';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function AuditPage() {
  const [data, setData] = useState<AuditLog[]>([]);
  const [events, setEvents] = useState<AuditEvent[]>([]);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<'logs' | 'events'>('logs');

  // Filters
  const [actionFilter, setActionFilter] = useState('');
  const [entityFilter, setEntityFilter] = useState('');
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');

  const load = async () => {
    setLoading(true);
    try {
      const params: Record<string, string | number | undefined> = {};
      if (actionFilter) params.action = actionFilter;
      if (entityFilter) params.entity_type = entityFilter;
      if (startDate) params.start_date = startDate;
      if (endDate) params.end_date = endDate;

      const hasFilters = Object.keys(params).length > 0;
      const logs = hasFilters
        ? await searchAuditLogs(params as { action?: string; entity_type?: string; start_date?: string; end_date?: string })
        : await searchAuditLogs({});
      setData(logs);

      const evts = await listAuditEvents().catch(() => []);
      setEvents(evts);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load audit data', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const handleSearch = () => {
    void load();
  };

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

  const logColumns: Column<AuditLog>[] = [
    { key: 'id', header: 'ID', render: (l) => <span className="font-mono text-xs text-dark-500">#{l.id}</span> },
    { key: 'action', header: 'Action', render: (l) => <span className="font-medium text-white">{l.action}</span> },
    { key: 'entity_type', header: 'Entity', render: (l) => (
      <span className="text-dark-300">{l.entity_type ?? '—'}{l.entity_id ? ` #${l.entity_id}` : ''}</span>
    )},
    { key: 'user_name', header: 'User', render: (l) => <span className="text-dark-300">{l.user_name ?? `#${l.user_id ?? 'system'}`}</span> },
    { key: 'ip_address', header: 'IP', render: (l) => <span className="font-mono text-xs text-dark-500">{l.ip_address ?? '—'}</span> },
    { key: 'created_at', header: 'Timestamp', render: (l) => <span className="text-xs text-dark-500">{formatDateTime(l.created_at)}</span> },
  ];

  const eventColumns: Column<AuditEvent>[] = [
    { key: 'id', header: 'ID', render: (e) => <span className="font-mono text-xs text-dark-500">#{e.id}</span> },
    { key: 'event_type', header: 'Event', render: (e) => <span className="font-medium text-white">{e.event_type}</span> },
    { key: 'entity_type', header: 'Entity', render: (e) => (
      <span className="text-dark-300">{e.entity_type ?? '—'}{e.entity_id ? ` #${e.entity_id}` : ''}</span>
    )},
    { key: 'created_at', header: 'Timestamp', render: (e) => <span className="text-xs text-dark-500">{formatDateTime(e.created_at)}</span> },
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

        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {[
            { key: 'logs' as const, label: `Audit logs (${data.length})` },
            { key: 'events' as const, label: `Event stream (${events.length})` },
          ].map((t) => (
            <button
              key={t.key}
              onClick={() => setTab(t.key)}
              className={`rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors ${
                tab === t.key ? 'bg-primary-500/20 text-primary-200' : 'text-dark-400 hover:text-white'
              }`}
            >
              {t.label}
            </button>
          ))}
        </div>

        {tab === 'logs' && (
          <div className="flex flex-wrap items-end gap-3">
            <div className="w-44"><TextField label="Action" value={actionFilter} onChange={(e) => setActionFilter(e.target.value)} placeholder="e.g. create, update" /></div>
            <div className="w-44"><TextField label="Entity type" value={entityFilter} onChange={(e) => setEntityFilter(e.target.value)} placeholder="e.g. customer" /></div>
            <TextField label="From" type="date" value={startDate} onChange={(e) => setStartDate(e.target.value)} />
            <TextField label="To" type="date" value={endDate} onChange={(e) => setEndDate(e.target.value)} />
            <Button onClick={handleSearch}>
              <Search className="h-4 w-4" /> Search
            </Button>
          </div>
        )}

        {loading ? (
          <Spinner />
        ) : tab === 'logs' ? (
          <DataTable
            columns={logColumns}
            data={data}
            rowKey={(l) => l.id}
            emptyTitle="No audit events"
            emptyDescription="Audited actions will appear here."
          />
        ) : (
          <DataTable
            columns={eventColumns}
            data={events}
            rowKey={(e) => e.id}
            emptyTitle="No events"
            emptyDescription="Event stream entries will appear here."
          />
        )}
      </div>
    </PageTransition>
  );
}
