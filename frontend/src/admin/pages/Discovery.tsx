import { useEffect, useState } from 'react';
import { Radar, Plus, CheckCircle2 } from 'lucide-react';
import { listDiscoveryScans, createDiscoveryScan, listDiscoveryResults, approveDiscoveryResult } from '../../api/admin/modules';
import type { DiscoveryScan, DiscoveryResult } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'scans' | 'results';

export function DiscoveryPage() {
  const [scans, setScans] = useState<DiscoveryScan[]>([]);
  const [results, setResults] = useState<DiscoveryResult[]>([]);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<Tab>('scans');
  const [createOpen, setCreateOpen] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const [s, r] = await Promise.all([listDiscoveryScans().catch(() => []), listDiscoveryResults().catch(() => [])]);
      setScans(s); setResults(r);
    } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
    finally { setLoading(false); }
  };
  useEffect(() => { void load(); }, []);

  const handleApprove = async (result: DiscoveryResult) => {
    try { await approveDiscoveryResult(result.id); toast('Device approved'); void load(); }
    catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
  };

  const scanColumns: Column<DiscoveryScan>[] = [
    { key: 'name', header: 'Scan', render: (s) => <span className="font-medium text-white">{s.name ?? `Scan #${s.id}`}</span> },
    { key: 'scan_type', header: 'Type', render: (s) => <StatusBadge status={s.scan_type} /> },
    { key: 'target_range', header: 'Target', render: (s) => <span className="font-mono text-xs text-dark-300">{s.target_range ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (s) => <StatusBadge status={s.status} pulse={s.status === 'running'} /> },
    { key: 'devices_found', header: 'Found', render: (s) => <span className="tabular-nums text-dark-200">{s.devices_found}</span> },
    { key: 'started_at', header: 'Started', render: (s) => <span className="text-xs text-dark-500">{formatDateTime(s.started_at)}</span> },
    { key: 'completed_at', header: 'Completed', render: (s) => <span className="text-xs text-dark-500">{formatDateTime(s.completed_at)}</span> },
  ];

  const resultColumns: Column<DiscoveryResult>[] = [
    { key: 'ip_address', header: 'IP', render: (r) => <span className="font-mono text-xs text-accent-300">{r.ip_address}</span> },
    { key: 'mac_address', header: 'MAC', render: (r) => <span className="font-mono text-[11px] text-dark-400">{r.mac_address ?? '—'}</span> },
    { key: 'hostname', header: 'Hostname', render: (r) => <span className="text-dark-200">{r.hostname ?? '—'}</span> },
    { key: 'device_type', header: 'Type', render: (r) => <StatusBadge status={r.device_type ?? 'unknown'} /> },
    { key: 'manufacturer', header: 'Vendor', render: (r) => <span className="text-dark-300">{r.manufacturer ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (r) => <StatusBadge status={r.status} pulse={r.status === 'discovered'} /> },
    { key: 'discovered_at', header: 'Discovered', render: (r) => <span className="text-xs text-dark-500">{formatDateTime(r.discovered_at)}</span> },
    { key: 'actions', header: '', align: 'right', render: (r) => r.status === 'discovered' ? (
      <div onClick={(e) => e.stopPropagation()}>
        <Button variant="success" size="sm" onClick={() => void handleApprove(r)}><CheckCircle2 className="h-3.5 w-3.5" /> Approve</Button>
      </div>
    ) : null },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader title="Network discovery" subtitle="Scan networks and approve discovered devices." icon={<Radar className="h-5 w-5" />}
          actions={tab === 'scans' ? <Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New scan</Button> : undefined}
        />
        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {([['scans', 'Scans'], ['results', 'Results']] as [Tab, string][]).map(([k, l]) => (
            <button key={k} onClick={() => setTab(k)} className={`rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors ${tab === k ? 'bg-primary-500/20 text-primary-200' : 'text-dark-400 hover:text-white'}`}>{l}</button>
          ))}
        </div>
        {loading ? <Spinner /> : tab === 'scans' ? (
          <DataTable columns={scanColumns} data={scans} rowKey={(s) => s.id} emptyTitle="No scans" emptyDescription="Start a network scan to discover devices." />
        ) : (
          <DataTable columns={resultColumns} data={results} rowKey={(r) => r.id} emptyTitle="No results" emptyDescription="Discovered devices will appear here." />
        )}
        <CreateScanModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />
      </div>
    </PageTransition>
  );
}

function CreateScanModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState(''); const [scanType, setScanType] = useState('network'); const [targetRange, setTargetRange] = useState(''); const [busy, setBusy] = useState(false);
  const submit = async () => { setBusy(true); try { await createDiscoveryScan({ name: name.trim() || undefined, scan_type: scanType, target_range: targetRange.trim() || undefined }); toast('Scan started'); onCreated(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setBusy(false); } };
  return (
    <Modal open={open} onClose={onClose} title="New network scan" width="md" footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Start scan</Button></>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Scan name" value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. Office subnet" />
        <SelectField label="Scan type" value={scanType} onChange={(e) => setScanType(e.target.value)}
          options={[{ value: 'network', label: 'Network sweep' }, { value: 'arp', label: 'ARP scan' }, { value: 'snmp', label: 'SNMP walk' }]} />
        <TextField label="Target range" value={targetRange} onChange={(e) => setTargetRange(e.target.value)} placeholder="10.0.0.0/24" />
      </form>
    </Modal>
  );
}
