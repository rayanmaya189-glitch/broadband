import { useEffect, useState } from 'react';
import { Target, Plus, UserPlus, CheckCircle2 } from 'lucide-react';
import { assignLead, convertLead, createLead, listLeads, updateLeadStatus } from '../../api/admin/modules';
import type { Lead } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { timeAgo } from '../lib/format';
import { toast } from '../lib/toast';

export function LeadsPage() {
  const [data, setData] = useState<Lead[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [busyId, setBusyId] = useState<number | null>(null);
  const [assignTarget, setAssignTarget] = useState<Lead | null>(null);
  const [assignUserId, setAssignUserId] = useState('');

  const load = async () => {
    setLoading(true);
    try {
      setData(await listLeads());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load leads', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const setStatus = async (lead: Lead, status: string) => {
    setBusyId(lead.id);
    try {
      await updateLeadStatus(lead.id, status);
      toast(`Lead marked ${status}`);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update lead', 'error');
    } finally {
      setBusyId(null);
    }
  };

  const convert = async (lead: Lead) => {
    setBusyId(lead.id);
    try {
      await convertLead(lead.id);
      toast('Lead converted to customer');
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Conversion failed', 'error');
    } finally {
      setBusyId(null);
    }
  };

  const doAssign = async () => {
    if (!assignTarget || !assignUserId) return;
    try {
      await assignLead(assignTarget.id, Number(assignUserId));
      toast('Lead assigned');
      setAssignTarget(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to assign lead', 'error');
    }
  };

  const columns: Column<Lead>[] = [
    { key: 'name', header: 'Lead', render: (l) => (
      <div>
        <p className="font-medium text-white">{l.name}</p>
        <p className="text-xs text-dark-500">{l.phone ?? l.email ?? 'â€”'}</p>
      </div>
    )},
    { key: 'source', header: 'Source', render: (l) => <span className="text-dark-400">{l.source ?? 'â€”'}</span> },
    { key: 'status', header: 'Status', render: (l) => <StatusBadge status={l.status} /> },
    { key: 'assigned_to_name', header: 'Assignee', render: (l) => <span className="text-dark-400">{l.assigned_to_name ?? 'Unassigned'}</span> },
    { key: 'created_at', header: 'Created', render: (l) => <span className="text-xs text-dark-500">{timeAgo(l.created_at)}</span> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (l) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <select
            value={l.status}
            onChange={(e) => void setStatus(l, e.target.value)}
            disabled={busyId === l.id}
            className="rounded-lg border border-white/10 bg-dark-950/70 px-2 py-1.5 text-xs text-dark-300 focus:border-accent-500/50 focus:outline-none"
            title="Change status"
          >
            {['new', 'contacted', 'qualified', 'converted', 'lost'].map((s) => (
              <option key={s} value={s} className="bg-dark-900 text-white">{s}</option>
            ))}
          </select>
          <Button variant="ghost" size="sm" loading={busyId === l.id} onClick={() => void convert(l)} title="Convert to customer">
            <CheckCircle2 className="h-4 w-4 text-emerald-400" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => { setAssignTarget(l); setAssignUserId(''); }} title="Assign">
            <UserPlus className="h-4 w-4" />
          </Button>
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Leads"
          subtitle="Sales pipeline and lead management."
          icon={<Target className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New lead</Button>}
        />

        <DataTable
          columns={columns}
          data={data}
          rowKey={(l) => l.id}
          loading={loading}
          onRowClick={(l) => undefined}
          emptyTitle="No leads"
          emptyDescription="New sales leads will appear here."
        />

        <CreateLeadModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />

        <Modal open={assignTarget !== null} onClose={() => setAssignTarget(null)} title="Assign lead" subtitle={assignTarget?.name} width="sm"
          footer={<>
            <Button variant="ghost" onClick={() => setAssignTarget(null)}>Cancel</Button>
            <Button onClick={doAssign} disabled={!assignUserId}>Assign</Button>
          </>}>
          <TextField label="User ID" type="number" required value={assignUserId} onChange={(e) => setAssignUserId(e.target.value)} />
        </Modal>
      </div>
    </PageTransition>
  );
}

function CreateLeadModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [phone, setPhone] = useState('');
  const [email, setEmail] = useState('');
  const [source, setSource] = useState('website');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await createLead({ name: name.trim(), phone: phone.trim() || undefined, email: email.trim() || undefined, source: source || undefined });
      toast('Lead created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create lead', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New lead" subtitle="Capture a sales lead." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create lead</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Phone" value={phone} onChange={(e) => setPhone(e.target.value)} />
        <TextField label="Email" type="email" value={email} onChange={(e) => setEmail(e.target.value)} />
        <SelectField label="Source" value={source} onChange={(e) => setSource(e.target.value)}
          options={[
            { value: 'website', label: 'Website' },
            { value: 'referral', label: 'Referral' },
            { value: 'walk_in', label: 'Walk-in' },
            { value: 'call', label: 'Phone call' },
            { value: 'social', label: 'Social media' },
            { value: 'other', label: 'Other' },
          ]}
        />
      </form>
    </Modal>
  );
}
