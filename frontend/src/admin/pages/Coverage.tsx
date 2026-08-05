import { useEffect, useState } from 'react';
import { MapPin, Plus } from 'lucide-react';
import { createCoverageArea, listCoverageAreas } from '../../api/admin/modules';
import type { CoverageArea } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { toast } from '../lib/toast';

export function CoveragePage() {
  const [data, setData] = useState<CoverageArea[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);

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

  const columns: Column<CoverageArea>[] = [
    { key: 'name', header: 'Area', render: (c) => <span className="font-medium text-white">{c.name}</span> },
    { key: 'type', header: 'Type', render: (c) => <StatusBadge status={c.type} /> },
    { key: 'status', header: 'Status', render: (c) => <StatusBadge status={c.status} /> },
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
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Coverage areas"
          subtitle="Serviceable areas and their pincodes."
          icon={<MapPin className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New area</Button>}
        />

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={data}
            rowKey={(c) => c.id}
            emptyTitle="No coverage areas"
            emptyDescription="Coverage areas will appear here."
          />
        )}

        <CreateCoverageModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />
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
      await createCoverageArea({
        name: name.trim(),
        type: type || undefined,
        pincodes: pincodes.split(',').map((p) => p.trim()).filter(Boolean),
      });
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
