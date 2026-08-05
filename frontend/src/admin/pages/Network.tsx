import { useEffect, useState } from 'react';
import { Network, Server, Plus } from 'lucide-react';
import { getTopology, listIpPools, listVlans, createIpPool } from '../../api/admin/modules';
import type { IpPool, Vlan } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { toast } from '../lib/toast';

export function NetworkPage() {
  const [vlans, setVlans] = useState<Vlan[]>([]);
  const [pools, setPools] = useState<IpPool[]>([]);
  const [topology, setTopology] = useState<unknown | null>(null);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<'vlans' | 'pools' | 'topology'>('vlans');
  const [poolOpen, setPoolOpen] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const [v, p, t] = await Promise.all([listVlans(), listIpPools(), getTopology().catch(() => null)]);
      setVlans(v);
      setPools(p);
      setTopology(t);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load network data', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const vlanColumns: Column<Vlan>[] = [
    { key: 'vlan_id', header: 'VLAN ID', render: (v) => <span className="font-mono text-accent-300">{v.vlan_id}</span> },
    { key: 'name', header: 'Name', render: (v) => <span className="font-medium text-white">{v.name}</span> },
    { key: 'subnet', header: 'Subnet', render: (v) => <span className="font-mono text-xs text-dark-300">{v.subnet ?? 'â€”'}</span> },
    { key: 'description', header: 'Description', render: (v) => <span className="text-dark-400">{v.description ?? 'â€”'}</span> },
  ];

  const poolColumns: Column<IpPool>[] = [
    { key: 'name', header: 'Pool', render: (p) => <span className="font-medium text-white">{p.name ?? `#${p.id}`}</span> },
    { key: 'subnet', header: 'Subnet', render: (p) => <span className="font-mono text-xs text-dark-300">{p.subnet ?? 'â€”'}</span> },
    { key: 'gateway', header: 'Gateway', render: (p) => <span className="font-mono text-xs text-dark-400">{p.gateway ?? 'â€”'}</span> },
    {
      key: 'usage',
      header: 'Utilization',
      render: (p) => {
        const total = Number(p.total_addresses) || 0;
        const used = Number(p.used_addresses) || 0;
        const pct = total > 0 ? Math.round((used / total) * 100) : 0;
        return (
          <div className="flex items-center gap-2">
            <div className="h-1.5 w-24 overflow-hidden rounded-full bg-dark-700">
              <div
                className={`h-full rounded-full ${pct > 80 ? 'bg-red-500' : pct > 50 ? 'bg-amber-400' : 'bg-emerald-400'}`}
                style={{ width: `${pct}%` }}
              />
            </div>
            <span className="text-xs tabular-nums text-dark-400">{used}/{total}</span>
          </div>
        );
      },
    },
    { key: 'status', header: 'Status', render: (p) => <StatusBadge status={p.status} /> },
  ];

  const tabs = [
    { key: 'vlans' as const, label: `VLANs (${vlans.length})` },
    { key: 'pools' as const, label: `IP pools (${pools.length})` },
    { key: 'topology' as const, label: 'Topology' },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Network"
          subtitle="VLANs, IP pools and network topology."
          icon={<Network className="h-5 w-5" />}
          actions={tab === 'pools' ? (
            <Button onClick={() => setPoolOpen(true)}><Plus className="h-4 w-4" /> New pool</Button>
          ) : undefined}
        />

        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {tabs.map((t) => (
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

        {loading ? (
          <Spinner />
        ) : tab === 'vlans' ? (
          <DataTable columns={vlanColumns} data={vlans} rowKey={(v) => v.id} emptyTitle="No VLANs" emptyDescription="VLANs will appear here." />
        ) : tab === 'pools' ? (
          <DataTable columns={poolColumns} data={pools} rowKey={(p) => p.id} emptyTitle="No IP pools" emptyDescription="IP pools will appear here." />
        ) : (
          <div className="glass-card rounded-2xl p-6">
            {topology ? (
              <pre className="overflow-x-auto text-xs text-dark-400">{JSON.stringify(topology, null, 2)}</pre>
            ) : (
              <div className="flex flex-col items-center gap-3 py-12 text-dark-400">
                <Server className="h-8 w-8" />
                <p className="text-sm">Topology data unavailable from this role or device.</p>
              </div>
            )}
          </div>
        )}

        <NewPoolModal open={poolOpen} onClose={() => setPoolOpen(false)} onCreated={() => { setPoolOpen(false); void load(); }} />
      </div>
    </PageTransition>
  );
}

function NewPoolModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [subnet, setSubnet] = useState('');
  const [gateway, setGateway] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!subnet.trim()) return;
    setBusy(true);
    try {
      await createIpPool({ name: name.trim() || undefined, subnet: subnet.trim(), gateway: gateway.trim() || undefined });
      toast('IP pool created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create pool', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New IP pool" subtitle="Create an address pool for the network." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create pool</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Name" value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Subnet" required placeholder="10.0.0.0/24" value={subnet} onChange={(e) => setSubnet(e.target.value)} />
        <TextField label="Gateway" placeholder="10.0.0.1" value={gateway} onChange={(e) => setGateway(e.target.value)} />
      </form>
    </Modal>
  );
}
