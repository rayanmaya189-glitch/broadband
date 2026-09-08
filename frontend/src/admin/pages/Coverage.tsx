import { useEffect, useState } from 'react';
import { MapPin, Plus, Edit3, Trash2, Check } from 'lucide-react';
import { createCoverageArea, listCoverageAreas, updateCoverageArea, deleteCoverageArea, checkCoverage } from '../../api/admin/modules';
import type { CoverageArea, CoverageCheck } from '../../types/admin';
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

export function CoveragePage() {
  const [data, setData] = useState<CoverageArea[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [editTarget, setEditTarget] = useState<CoverageArea | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<CoverageArea | null>(null);
  const [deleting, setDeleting] = useState(false);
  const [checkOpen, setCheckOpen] = useState(false);
  const [checkResult, setCheckResult] = useState<CoverageCheck | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listCoverageAreas());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load coverage areas', 'error');
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
      await deleteCoverageArea(deleteTarget.id);
      toast('Coverage area deleted');
      setDeleteTarget(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to delete coverage area', 'error');
    } finally {
      setDeleting(false);
    }
  };

  const columns: Column<CoverageArea>[] = [
    { key: 'name', header: 'Area', render: (c) => <span className="font-medium text-white">{c.name}</span> },
    { key: 'type', header: 'Type', render: (c) => <StatusBadge status={c.type ?? 'locality'} /> },
    { key: 'status', header: 'Status', render: (c) => <StatusBadge status={c.status ?? 'active'} /> },
    {
      key: 'pincodes',
      header: 'Pincodes',
      render: (c) => {
        const list = c.pincodes ?? [];
        return (
          <div className="flex max-w-md flex-wrap gap-1">
            {list.slice(0, 6).map((p) => (
              <span key={p} className="rounded-md border border-white/[0.06] bg-white/[0.03] px-1.5 py-0.5 font-mono text-[10px] text-dark-400">
                {p}
              </span>
            ))}
            {list.length > 6 && <span className="text-[10px] text-dark-500">+{list.length - 6}</span>}
          </div>
        );
      },
    },
    {
      key: 'actions', header: '', align: 'right',
      render: (c) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => setEditTarget(c)} title="Edit">
            <Edit3 className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setDeleteTarget(c)} title="Delete">
            <Trash2 className="h-4 w-4 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Coverage areas"
          subtitle="Serviceable areas and their pincodes."
          icon={<MapPin className="h-5 w-5" />}
          actions={
            <>
              <Button variant="secondary" onClick={() => { setCheckOpen(true); setCheckResult(null); }}>
                <Check className="h-4 w-4" /> Check availability
              </Button>
              <Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New area</Button>
            </>
          }
        />

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={data}
            rowKey={(c) => c.id}
            onRowClick={(c) => setEditTarget(c)}
            emptyTitle="No coverage areas"
            emptyDescription="Coverage areas will appear here."
          />
        )}

        <CreateCoverageModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />

        {editTarget && (
          <EditCoverageModal area={editTarget} open={editTarget !== null} onClose={() => setEditTarget(null)}
            onSaved={() => { setEditTarget(null); void load(); }} />
        )}

        <ConfirmDialog
          open={deleteTarget !== null}
          onClose={() => setDeleteTarget(null)}
          onConfirm={confirmDelete}
          loading={deleting}
          title="Delete coverage area?"
          description={`Permanently delete "${deleteTarget?.name}".`}
          confirmLabel="Delete area"
          danger
        />

        <CheckAvailabilityModal open={checkOpen} onClose={() => setCheckOpen(false)} result={checkResult} setResult={setCheckResult} />
      </div>
    </PageTransition>
  );
}

function CreateCoverageModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [type, setType] = useState('locality');
  const [pincodes, setPincodes] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await createCoverageArea({ name: name.trim(), area_type: type.trim() || 'locality' });
      toast('Coverage area created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create coverage area', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New coverage area" subtitle="Define a serviceable locality." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Area name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Type" value={type} onChange={(e) => setType(e.target.value)} hint="e.g. locality, ward, sector." />
        <TextField label="Pincodes" value={pincodes} onChange={(e) => setPincodes(e.target.value)} hint="Comma-separated, e.g. 400001, 400002." />
      </form>
    </Modal>
  );
}

function EditCoverageModal({ area, open, onClose, onSaved }: { area: CoverageArea; open: boolean; onClose: () => void; onSaved: () => void }) {
  const [name, setName] = useState(area.name);
  const [type, setType] = useState(area.type ?? 'locality');
  const [pincodes, setPincodes] = useState((area.pincodes ?? []).join(', '));
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await updateCoverageArea(area.id, { name: name.trim(), area_type: type.trim() || 'locality' } as Partial<CoverageArea>);
      toast('Coverage area updated');
      onSaved();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update coverage area', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Edit coverage area" subtitle={area.name} width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Save changes</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Area name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Type" value={type} onChange={(e) => setType(e.target.value)} />
        <TextField label="Pincodes" value={pincodes} onChange={(e) => setPincodes(e.target.value)} hint="Comma-separated." />
      </form>
    </Modal>
  );
}

function CheckAvailabilityModal({ open, onClose, result, setResult }: { open: boolean; onClose: () => void; result: CoverageCheck | null; setResult: (r: CoverageCheck | null) => void }) {
  const [pincode, setPincode] = useState('');
  const [busy, setBusy] = useState(false);

  const check = async () => {
    if (!pincode.trim()) return;
    setBusy(true);
    try {
      setResult(await checkCoverage({ pincode: pincode.trim() }));
    } catch (err) {
      setResult({ available: false, message: err instanceof Error ? err.message : 'Check failed' });
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Check coverage availability" width="sm"
      footer={<Button variant="ghost" onClick={onClose}>Close</Button>}>
      <div className="space-y-4">
        <div className="flex items-end gap-2">
          <TextField label="Pincode" required value={pincode} onChange={(e) => setPincode(e.target.value)} placeholder="400001" />
          <Button onClick={check} loading={busy}>Check</Button>
        </div>
        {result && (
          <div className={`rounded-xl border p-4 text-sm ${result.available ? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-300' : 'border-red-500/30 bg-red-500/10 text-red-300'}`}>
            {result.available ? (
              <p>✓ Service available{result.area_name ? ` in ${result.area_name}` : ''}.</p>
            ) : (
              <p>✗ {result.message ?? 'Service not available in this area.'}</p>
            )}
          </div>
        )}
      </div>
    </Modal>
  );
}
