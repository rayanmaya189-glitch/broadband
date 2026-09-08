import { useEffect, useState } from 'react';
import { Network, Server, Plus, Trash2 } from 'lucide-react';
import {
  getTopology,
  listIpPools,
  listVlans,
  createVlan,
  deleteVlan,
  createIpPool,
  listPppoeSessions,
  terminatePppoeSession,
  listMacBindings,
  createMacBinding,
} from '../../api/admin/modules';
import type { IpPool, Vlan, PppoeSession, MacBinding } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { useAdminStore } from '../adminStore';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'vlans' | 'pools' | 'topology' | 'pppoe' | 'mac';

export function NetworkPage() {
  const [vlans, setVlans] = useState<Vlan[]>([]);
  const [pools, setPools] = useState<IpPool[]>([]);
  const [topology, setTopology] = useState<unknown | null>(null);
  const [pppoe, setPppoe] = useState<PppoeSession[]>([]);
  const [macBindings, setMacBindings] = useState<MacBinding[]>([]);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<Tab>('vlans');
  const [poolOpen, setPoolOpen] = useState(false);
  const [vlanOpen, setVlanOpen] = useState(false);
  const [macOpen, setMacOpen] = useState(false);
  const [deleteVlanTarget, setDeleteVlanTarget] = useState<Vlan | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [v, p, t, pp, mb] = await Promise.all([
        listVlans(), listIpPools(), getTopology().catch(() => null),
        listPppoeSessions().catch(() => []), listMacBindings().catch(() => []),
      ]);
      setVlans(v);
      setPools(p);
      setTopology(t);
      setPppoe(pp);
      setMacBindings(mb);
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
    { key: 'subnet', header: 'Subnet', render: (v) => <span className="font-mono text-xs text-dark-300">{v.subnet ?? '—'}</span> },
    { key: 'description', header: 'Description', render: (v) => <span className="text-dark-400">{v.description ?? '—'}</span> },
    {
      key: 'actions', header: '', align: 'right',
      render: (v) => (
        <div onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => setDeleteVlanTarget(v)} title="Delete VLAN">
            <Trash2 className="h-3.5 w-3.5 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  const poolColumns: Column<IpPool>[] = [
    { key: 'name', header: 'Pool', render: (p) => <span className="font-medium text-white">{p.name ?? `#${p.id}`}</span> },
    { key: 'subnet', header: 'Subnet', render: (p) => <span className="font-mono text-xs text-dark-300">{p.subnet ?? '—'}</span> },
    { key: 'gateway', header: 'Gateway', render: (p) => <span className="font-mono text-xs text-dark-400">{p.gateway ?? '—'}</span> },
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
    { key: 'status', header: 'Status', render: (p) => <StatusBadge status={p.status ?? 'active'} /> },
  ];

  const pppoeColumns: Column<PppoeSession>[] = [
    { key: 'username', header: 'Username', render: (s) => <span className="font-medium text-white">{s.username ?? '—'}</span> },
    { key: 'customer_name', header: 'Customer', render: (s) => <span className="text-dark-300">{s.customer_name ?? '—'}</span> },
    { key: 'ip_address', header: 'IP', render: (s) => <span className="font-mono text-xs text-dark-300">{s.ip_address ?? '—'}</span> },
    { key: 'nas_ip', header: 'NAS', render: (s) => <span className="font-mono text-xs text-dark-400">{s.nas_ip ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (s) => <StatusBadge status={s.status} pulse={s.status === 'active'} /> },
    { key: 'started_at', header: 'Started', render: (s) => <span className="text-xs text-dark-500">{formatDateTime(s.started_at)}</span> },
    {
      key: 'actions', header: '', align: 'right',
      render: (s) => s.status === 'active' ? (
        <div onClick={(e) => e.stopPropagation()}>
          <Button variant="danger" size="sm" onClick={async () => {
            try { await terminatePppoeSession(s.id); toast('Session terminated'); void load(); }
            catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
          }}>Terminate</Button>
        </div>
      ) : null,
    },
  ];

  const macColumns: Column<MacBinding>[] = [
    { key: 'mac_address', header: 'MAC Address', render: (m) => <span className="font-mono text-xs text-accent-300">{m.mac_address}</span> },
    { key: 'ip_address', header: 'IP Address', render: (m) => <span className="font-mono text-xs text-dark-300">{m.ip_address ?? '—'}</span> },
    { key: 'customer_name', header: 'Customer', render: (m) => <span className="text-dark-300">{m.customer_name ?? '—'}</span> },
    { key: 'port', header: 'Port', render: (m) => <span className="text-dark-400">{m.port ?? '—'}</span> },
    { key: 'is_active', header: 'Status', render: (m) => <StatusBadge status={m.is_active !== false ? 'active' : 'inactive'} /> },
  ];

  const tabs: { key: Tab; label: string }[] = [
    { key: 'vlans', label: `VLANs (${vlans.length})` },
    { key: 'pools', label: `IP pools (${pools.length})` },
    { key: 'pppoe', label: `PPPoE (${pppoe.length})` },
    { key: 'mac', label: `MAC bindings (${macBindings.length})` },
    { key: 'topology', label: 'Topology' },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Network"
          subtitle="VLANs, IP pools, PPPoE sessions and network topology."
          icon={<Network className="h-5 w-5" />}
          actions={
            <>
              {tab === 'vlans' && <Button onClick={() => setVlanOpen(true)}><Plus className="h-4 w-4" /> New VLAN</Button>}
              {tab === 'pools' && <Button onClick={() => setPoolOpen(true)}><Plus className="h-4 w-4" /> New pool</Button>}
              {tab === 'mac' && <Button onClick={() => setMacOpen(true)}><Plus className="h-4 w-4" /> New binding</Button>}
            </>
          }
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
        ) : tab === 'pppoe' ? (
          <DataTable columns={pppoeColumns} data={pppoe} rowKey={(s) => s.id} emptyTitle="No PPPoE sessions" emptyDescription="Active PPPoE sessions will appear here." />
        ) : tab === 'mac' ? (
          <DataTable columns={macColumns} data={macBindings} rowKey={(m) => m.id} emptyTitle="No MAC bindings" emptyDescription="MAC address bindings will appear here." />
        ) : (
          <div className="glass-card rounded-2xl p-6">
            {topology ? (
              <pre className="overflow-x-auto text-xs text-dark-400">{JSON.stringify(topology, null, 2)}</pre>
            ) : (
              <div className="flex flex-col items-center gap-3 py-12 text-dark-400">
                <Server className="h-8 w-8" />
                <p className="text-sm">Topology data unavailable.</p>
              </div>
            )}
          </div>
        )}

        <NewPoolModal open={poolOpen} onClose={() => setPoolOpen(false)} onCreated={() => { setPoolOpen(false); void load(); }} />
        <NewVlanModal open={vlanOpen} onClose={() => setVlanOpen(false)} onCreated={() => { setVlanOpen(false); void load(); }} />
        <NewMacBindingModal open={macOpen} onClose={() => setMacOpen(false)} onCreated={() => { setMacOpen(false); void load(); }} />

        <ConfirmDialog
          open={deleteVlanTarget !== null}
          onClose={() => setDeleteVlanTarget(null)}
          onConfirm={async () => {
            if (!deleteVlanTarget) return;
            try { await deleteVlan(deleteVlanTarget.id); toast('VLAN deleted'); setDeleteVlanTarget(null); void load(); }
            catch (err) { toast(err instanceof Error ? err.message : 'Failed to delete VLAN', 'error'); }
          }}
          title="Delete VLAN?"
          description={`Delete VLAN "${deleteVlanTarget?.name}" (ID ${deleteVlanTarget?.vlan_id}). This may disconnect devices.`}
          confirmLabel="Delete VLAN"
          danger
        />
      </div>
    </PageTransition>
  );
}

function cidrHostCount(cidr: string): number | null {
  const m = cidr.trim().match(/^(?:\d{1,3}\.){3}\d{1,3}\/(\d{1,2})$/);
  if (!m) return null;
  const prefix = Number(m[1]);
  if (prefix < 1 || prefix > 30) return null;
  return Math.pow(2, 32 - prefix) - 2;
}

function NewPoolModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [cidr, setCidr] = useState('');
  const [gateway, setGateway] = useState('');
  const [poolType, setPoolType] = useState('dhcp');
  const [totalCount, setTotalCount] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!cidr.trim() || !totalCount) return;
    setBusy(true);
    try {
      await createIpPool({
        name: name.trim() || undefined,
        cidr: cidr.trim(),
        gateway: gateway.trim() || undefined,
        pool_type: poolType,
        total_count: Number(totalCount),
        branch_id: useAdminStore.getState().branchId ?? undefined,
      });
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
        <TextField label="CIDR" required placeholder="10.0.0.0/24" value={cidr}
          onChange={(e) => {
            setCidr(e.target.value);
            const hosts = cidrHostCount(e.target.value);
            if (hosts !== null) setTotalCount(String(hosts));
          }} />
        <TextField label="Gateway" placeholder="10.0.0.1" value={gateway} onChange={(e) => setGateway(e.target.value)} />
        <SelectField label="Pool type" value={poolType} onChange={(e) => setPoolType(e.target.value)}
          options={[
            { value: 'dhcp', label: 'DHCP (dynamic)' },
            { value: 'static', label: 'Static' },
            { value: 'pppoe', label: 'PPPoE' },
          ]}
        />
        <TextField label="Total addresses" type="number" required value={totalCount} onChange={(e) => setTotalCount(e.target.value)}
          hint="Auto-filled from the CIDR prefix; adjust if needed." />
      </form>
    </Modal>
  );
}

function NewVlanModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [vlanId, setVlanId] = useState('');
  const [name, setName] = useState('');
  const [vlanType, setVlanType] = useState('access');
  const [description, setDescription] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!vlanId || !name.trim()) return;
    setBusy(true);
    try {
      await createVlan({
        vlan_id: Number(vlanId),
        name: name.trim(),
        vlan_type: vlanType,
        description: description.trim() || undefined,
        branch_id: useAdminStore.getState().branchId ?? undefined,
      });
      toast('VLAN created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create VLAN', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New VLAN" subtitle="Create a virtual LAN." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create VLAN</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="VLAN ID" type="number" required value={vlanId} onChange={(e) => setVlanId(e.target.value)} />
        <TextField label="Name" required value={name} onChange={(e) => setName(e.target.value)} />
        <SelectField label="VLAN type" value={vlanType} onChange={(e) => setVlanType(e.target.value)}
          options={[
            { value: 'access', label: 'Access' },
            { value: 'trunk', label: 'Trunk' },
            { value: 'management', label: 'Management' },
          ]}
        />
        <TextField label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
      </form>
    </Modal>
  );
}

function NewMacBindingModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [macAddress, setMacAddress] = useState('');
  const [ipAddress, setIpAddress] = useState('');
  const [customerId, setCustomerId] = useState('');
  const [subscriptionId, setSubscriptionId] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!macAddress.trim() || !ipAddress.trim() || !customerId || !subscriptionId) return;
    setBusy(true);
    try {
      await createMacBinding({
        mac_address: macAddress.trim(),
        assigned_ip: ipAddress.trim(),
        customer_id: Number(customerId),
        subscription_id: Number(subscriptionId),
        branch_id: useAdminStore.getState().branchId ?? undefined,
      });
      toast('MAC binding created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create MAC binding', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New MAC binding" subtitle="Bind a MAC address to a customer subscription." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create binding</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="MAC address" required placeholder="AA:BB:CC:DD:EE:FF" value={macAddress} onChange={(e) => setMacAddress(e.target.value)} />
        <TextField label="Assigned IP" required placeholder="10.0.0.100" value={ipAddress} onChange={(e) => setIpAddress(e.target.value)} />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Customer ID" type="number" required value={customerId} onChange={(e) => setCustomerId(e.target.value)} />
          <TextField label="Subscription ID" type="number" required value={subscriptionId} onChange={(e) => setSubscriptionId(e.target.value)} />
        </div>
      </form>
    </Modal>
  );
}
