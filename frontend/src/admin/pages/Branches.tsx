import { useEffect, useState } from 'react';
import { Building2, Plus } from 'lucide-react';
import { createBranch, listBranches } from '../../api/admin/modules';
import type { Branch } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { toast } from '../lib/toast';

export function BranchesPage() {
  const [data, setData] = useState<Branch[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);

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

  const columns: Column<Branch>[] = [
    { key: 'name', header: 'Branch', render: (b) => (
      <div>
        <p className="font-medium text-white">{b.name}</p>
        {b.code && <p className="text-xs font-mono text-dark-500">{b.code}</p>}
      </div>
    )},
    { key: 'city', header: 'City', render: (b) => <span className="text-dark-300">{b.city ?? 'â€”'}</span> },
    { key: 'state', header: 'State', render: (b) => <span className="text-dark-300">{b.state ?? 'â€”'}</span> },
    { key: 'parent_branch_id', header: 'Parent', render: (b) => <span className="text-dark-400">{b.parent_branch_id ? `#${b.parent_branch_id}` : 'Head office'}</span> },
    { key: 'status', header: 'Status', render: (b) => <StatusBadge status={b.status} /> },
    { key: 'is_head_office', header: 'Type', render: (b) => (b.is_head_office ? <StatusBadge status="head_office" /> : <span className="text-dark-600">Branch</span>) },
  ];

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
    if (!name.trim()) return;
    setBusy(true);
    try {
      await createBranch({
        name: name.trim(),
        code: code.trim() || undefined,
        city: city.trim() || undefined,
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
        <TextField label="Code" value={code} onChange={(e) => setCode(e.target.value)} />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="City" value={city} onChange={(e) => setCity(e.target.value)} />
          <TextField label="State" value={state} onChange={(e) => setState(e.target.value)} />
        </div>
      </form>
    </Modal>
  );
}
