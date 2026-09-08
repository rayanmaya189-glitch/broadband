import { useEffect, useState } from 'react';
import { Building2, Plus, Edit3, Trash2, Clock } from 'lucide-react';
import { createBranch, listBranches, updateBranch, deleteBranch, getBranchWorkingHours, updateBranchWorkingHours } from '../../api/admin/modules';
import type { Branch, WorkingHours } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { toast } from '../lib/toast';

const DAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

export function BranchesPage() {
  const [data, setData] = useState<Branch[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [editTarget, setEditTarget] = useState<Branch | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<Branch | null>(null);
  const [deleting, setDeleting] = useState(false);
  const [hoursTarget, setHoursTarget] = useState<Branch | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listBranches());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load branches', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    setDeleting(true);
    try {
      await deleteBranch(deleteTarget.id);
      toast('Branch deleted');
      setDeleteTarget(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to delete branch', 'error');
    } finally {
      setDeleting(false);
    }
  };

  const columns: Column<Branch>[] = [
    { key: 'name', header: 'Branch', render: (b) => (
      <div>
        <p className="font-medium text-white">{b.name}</p>
        {b.code && <p className="text-xs font-mono text-dark-500">{b.code}</p>}
      </div>
    )},
    { key: 'city', header: 'City', render: (b) => <span className="text-dark-300">{b.city ?? '—'}</span> },
    { key: 'state', header: 'State', render: (b) => <span className="text-dark-300">{b.state ?? '—'}</span> },
    { key: 'parent_branch_id', header: 'Parent', render: (b) => <span className="text-dark-400">{b.parent_branch_id ? `#${b.parent_branch_id}` : 'Head office'}</span> },
    { key: 'status', header: 'Status', render: (b) => <StatusBadge status={b.status ?? 'active'} /> },
    { key: 'is_head_office', header: 'Type', render: (b) => (b.is_head_office ? <StatusBadge status="head_office" /> : <span className="text-dark-600">Branch</span>) },
    {
      key: 'actions', header: '', align: 'right',
      render: (b) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => void openHours(b)} title="Working hours">
            <Clock className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setEditTarget(b)} title="Edit">
            <Edit3 className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setDeleteTarget(b)} title="Delete">
            <Trash2 className="h-4 w-4 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  const openHours = async (branch: Branch) => {
    setHoursTarget(branch);
  };

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Branches"
          subtitle="Branch offices across the service area."
          icon={<Building2 className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New branch</Button>}
        />

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={data}
            rowKey={(b) => b.id}
            emptyTitle="No branches"
            emptyDescription="Branches will appear here."
          />
        )}

        <CreateBranchModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />

        {editTarget && (
          <EditBranchModal branch={editTarget} open={editTarget !== null} onClose={() => setEditTarget(null)}
            onSaved={() => { setEditTarget(null); void load(); }} />
        )}

        <ConfirmDialog
          open={deleteTarget !== null}
          onClose={() => setDeleteTarget(null)}
          onConfirm={confirmDelete}
          loading={deleting}
          title="Delete branch?"
          description={`Permanently delete "${deleteTarget?.name}". This cannot be undone.`}
          confirmLabel="Delete branch"
          danger
        />

        {hoursTarget && (
          <WorkingHoursModal branch={hoursTarget} open={hoursTarget !== null} onClose={() => setHoursTarget(null)} />
        )}
      </div>
    </PageTransition>
  );
}

function CreateBranchModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [code, setCode] = useState('');
  const [city, setCity] = useState('');
  const [state, setState] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim() || !code.trim() || !city.trim()) return;
    setBusy(true);
    try {
      await createBranch({
        name: name.trim(),
        slug: name.trim().toLowerCase().replace(/[^a-z0-9\s-]/g, '').replace(/[\s_]+/g, '-').replace(/-+/g, '-'),
        code: code.trim(),
        city: city.trim(),
        state: state.trim() || undefined,
      });
      toast('Branch created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create branch', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New branch" subtitle="Add a branch office." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create branch</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Branch name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Code" required value={code} onChange={(e) => setCode(e.target.value)} hint="Short branch code, e.g. MUM01." />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="City" required value={city} onChange={(e) => setCity(e.target.value)} />
          <TextField label="State" value={state} onChange={(e) => setState(e.target.value)} />
        </div>
      </form>
    </Modal>
  );
}

function EditBranchModal({ branch, open, onClose, onSaved }: { branch: Branch; open: boolean; onClose: () => void; onSaved: () => void }) {
  const [name, setName] = useState(branch.name);
  const [code, setCode] = useState(branch.code ?? '');
  const [city, setCity] = useState(branch.city ?? '');
  const [state, setState] = useState(branch.state ?? '');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await updateBranch(branch.id, { name: name.trim(), code: code.trim() || undefined, city: city.trim() || undefined, state: state.trim() || undefined });
      toast('Branch updated');
      onSaved();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update branch', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Edit branch" subtitle={branch.name} width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Save changes</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Branch name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Code" value={code} onChange={(e) => setCode(e.target.value)} />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="City" value={city} onChange={(e) => setCity(e.target.value)} />
          <TextField label="State" value={state} onChange={(e) => setState(e.target.value)} />
        </div>
      </form>
    </Modal>
  );
}

function WorkingHoursModal({ branch, open, onClose }: { branch: Branch; open: boolean; onClose: () => void }) {
  const [hours, setHours] = useState<WorkingHours[]>([]);
  const [loading, setLoading] = useState(true);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    if (!open) return;
    setLoading(true);
    void getBranchWorkingHours(branch.id)
      .then((h) => {
        if (h.length === 0) {
          setHours(DAYS.map((_, i) => ({ branch_id: branch.id, day_of_week: i, open_time: '09:00', close_time: '18:00', is_closed: i === 0 || i === 6 })));
        } else {
          setHours(h);
        }
      })
      .catch(() => setHours(DAYS.map((_, i) => ({ branch_id: branch.id, day_of_week: i, open_time: '09:00', close_time: '18:00', is_closed: i === 0 || i === 6 }))))
      .finally(() => setLoading(false));
  }, [open, branch.id]);

  const updateHour = (dayIndex: number, field: keyof WorkingHours, value: string | boolean) => {
    setHours((prev) => prev.map((h) => h.day_of_week === dayIndex ? { ...h, [field]: value } : h));
  };

  const save = async () => {
    setBusy(true);
    try {
      await updateBranchWorkingHours(branch.id, hours);
      toast('Working hours updated');
      onClose();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update hours', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title={`Working hours — ${branch.name}`} width="lg"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={save} loading={busy}>Save hours</Button>
      </>}>
      {loading ? (
        <Spinner />
      ) : (
        <div className="space-y-2">
          {hours.map((h) => (
            <div key={h.day_of_week} className="flex items-center gap-3 rounded-lg border border-white/[0.06] bg-white/[0.02] px-4 py-2.5">
              <span className="w-10 text-sm font-medium text-white">{DAYS[h.day_of_week]}</span>
              <label className="flex items-center gap-2 text-xs text-dark-400">
                <input type="checkbox" checked={!h.is_closed} onChange={(e) => updateHour(h.day_of_week, 'is_closed', !e.target.checked)}
                  className="rounded border-dark-600 bg-dark-900 text-accent-500" />
                Open
              </label>
              {!h.is_closed && (
                <>
                  <input type="time" value={h.open_time} onChange={(e) => updateHour(h.day_of_week, 'open_time', e.target.value)}
                    className="rounded-lg border border-white/10 bg-dark-950/70 px-2 py-1.5 text-xs text-dark-300 focus:border-accent-500/50 focus:outline-none" />
                  <span className="text-dark-500">to</span>
                  <input type="time" value={h.close_time} onChange={(e) => updateHour(h.day_of_week, 'close_time', e.target.value)}
                    className="rounded-lg border border-white/10 bg-dark-950/70 px-2 py-1.5 text-xs text-dark-300 focus:border-accent-500/50 focus:outline-none" />
                </>
              )}
              {h.is_closed && <span className="text-xs text-dark-500">Closed</span>}
            </div>
          ))}
        </div>
      )}
    </Modal>
  );
}
