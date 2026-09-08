import { useEffect, useState } from 'react';
import { Package, Plus, UserPlus } from 'lucide-react';
import { listInventory, createInventoryItem, assignInventoryItem } from '../../api/admin/modules';
import type { InventoryItem } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function InventoryPage() {
  const [data, setData] = useState<InventoryItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [assignTarget, setAssignTarget] = useState<InventoryItem | null>(null);

  const load = async () => {
    setLoading(true);
    try { setData(await listInventory()); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
    finally { setLoading(false); }
  };
  useEffect(() => { void load(); }, []);

  const columns: Column<InventoryItem>[] = [
    { key: 'item_type', header: 'Item', render: (i) => <div><p className="font-medium text-white">{i.item_type ?? i.name ?? `#${i.id}`}</p>{i.description && <p className="max-w-md truncate text-xs text-dark-500">{i.description}</p>}</div> },
    { key: 'serial_number', header: 'Serial', render: (i) => <span className="font-mono text-xs text-dark-400">{i.serial_number ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (i) => <StatusBadge status={i.status} /> },
    { key: 'assigned_to_name', header: 'Assigned to', render: (i) => <span className="text-dark-300">{i.assigned_to_name ?? 'Unassigned'}</span> },
    { key: 'created_at', header: 'Added', render: (i) => <span className="text-xs text-dark-500">{formatDateTime(i.created_at)}</span> },
    { key: 'actions', header: '', align: 'right', render: (i) => i.status !== 'assigned' ? (
      <div onClick={(e) => e.stopPropagation()}>
        <Button variant="secondary" size="sm" onClick={() => setAssignTarget(i)}><UserPlus className="h-3.5 w-3.5" /> Assign</Button>
      </div>
    ) : null },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader title="Inventory" subtitle="Equipment tracking and technician assignments." icon={<Package className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> Add item</Button>}
        />
        {loading ? <Spinner /> : (
          <DataTable columns={columns} data={data} rowKey={(i) => i.id} emptyTitle="No inventory items" emptyDescription="Add equipment to start tracking." />
        )}
        <CreateItemModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />
        {assignTarget && <AssignModal item={assignTarget} open={assignTarget !== null} onClose={() => setAssignTarget(null)} onDone={() => { setAssignTarget(null); void load(); }} />}
      </div>
    </PageTransition>
  );
}

function CreateItemModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState(''); const [serial, setSerial] = useState(''); const [busy, setBusy] = useState(false);
  const submit = async () => { if (!name.trim()) return; setBusy(true); try { await createInventoryItem({ item_type: name.trim(), serial_number: serial.trim() || undefined }); toast('Item added'); onCreated(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setBusy(false); } };
  return (
    <Modal open={open} onClose={onClose} title="Add inventory item" width="md" footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Add item</Button></>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Item type" required value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. router, ONT, switch, cable" />
        <TextField label="Serial number" value={serial} onChange={(e) => setSerial(e.target.value)} />
      </form>
    </Modal>
  );
}

function AssignModal({ item, open, onClose, onDone }: { item: InventoryItem; open: boolean; onClose: () => void; onDone: () => void }) {
  const [userId, setUserId] = useState(''); const [busy, setBusy] = useState(false);
  const submit = async () => { if (!userId) return; setBusy(true); try { await assignInventoryItem(item.id, { assigned_to: Number(userId) }); toast('Item assigned'); onDone(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setBusy(false); } };
  return (
    <Modal open={open} onClose={onClose} title={`Assign — ${item.name}`} width="sm" footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Assign</Button></>}>
      <TextField label="Technician / User ID" type="number" required value={userId} onChange={(e) => setUserId(e.target.value)} />
    </Modal>
  );
}
