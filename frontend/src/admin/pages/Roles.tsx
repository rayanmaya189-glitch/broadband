import { useEffect, useState } from 'react';
import { Shield, Plus, KeyRound } from 'lucide-react';
import { createRole, listPermissions, listRoles } from '../../api/admin/modules';
import type { Permission, Role } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, TextArea } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

export function RolesPage() {
  const [roles, setRoles] = useState<Role[]>([]);
  const [permissions, setPermissions] = useState<Permission[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [detail, setDetail] = useState<Role | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [r, p] = await Promise.all([listRoles(), listPermissions()]);
      setRoles(r);
      setPermissions(p);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load roles', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const permissionMap = new Map(permissions.map((p) => [p.id, p]));

  const columns: Column<Role>[] = [
    { key: 'name', header: 'Role', render: (r) => (
      <div>
        <p className="font-medium text-white">{toLabel(r.name)}</p>
        {r.description && <p className="max-w-md truncate text-xs text-dark-500">{r.description}</p>}
      </div>
    )},
    { key: 'slug', header: 'Slug', render: (r) => <span className="font-mono text-xs text-dark-400">{r.slug ?? r.name}</span> },
    { key: 'permissions_count', header: 'Permissions', render: (r) => (
      <span className="tabular-nums text-dark-300">{r.permissions_count ?? 'â€”'}</span>
    )},
    { key: 'is_system', header: 'System', render: (r) => (r.is_system ? <StatusBadge status="system" /> : <span className="text-dark-600">â€”</span>) },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Roles & permissions"
          subtitle="Role definitions and the permissions they grant."
          icon={<Shield className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New role</Button>}
        />

        {loading ? (
          <Spinner />
        ) : (
          <>
            <DataTable
              columns={columns}
              data={roles}
              rowKey={(r) => r.id}
              onRowClick={(r) => setDetail(r)}
              emptyTitle="No roles"
              emptyDescription="Roles will appear here."
            />

            <section className="glass-card rounded-2xl p-5">
              <h2 className="mb-4 flex items-center gap-2 text-sm font-semibold text-white">
                <KeyRound className="h-4 w-4 text-primary-400" /> Permission catalog ({permissions.length})
              </h2>
              <div className="flex max-h-72 flex-wrap gap-1.5 overflow-y-auto">
                {permissions.map((p) => (
                  <span
                    key={p.id}
                    title={`${p.module} Â· ${p.resource ?? ''} ${p.action ?? ''}`}
                    className="rounded-lg border border-white/[0.08] bg-white/[0.03] px-2 py-1 text-[11px] text-dark-300"
                  >
                    {p.name}
                  </span>
                ))}
                {permissions.length === 0 && <p className="text-sm text-dark-500">No permissions returned.</p>}
              </div>
            </section>
          </>
        )}

        <CreateRoleModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />

        <Modal open={detail !== null} onClose={() => setDetail(null)} title="Role permissions" subtitle={detail ? toLabel(detail.name) : undefined} width="lg">
          {detail ? (
            <div className="space-y-3">
              {detail.description && <p className="text-sm text-dark-300">{detail.description}</p>}
              <p className="text-xs text-dark-500">
                {detail.permissions_count != null ? `${detail.permissions_count} permissions` : 'Permission count unknown'}
              </p>
              <p className="text-xs text-dark-500">Created {formatDateTime((detail as Role & { created_at?: string }).created_at)}</p>
              <div className="flex flex-wrap gap-1.5">
                {Array.from(permissionMap.entries()).slice(0, 12).map(([id, p]) => (
                  <span key={id} className="rounded-lg border border-white/[0.08] bg-white/[0.03] px-2 py-1 text-[11px] text-dark-300">
                    {p.name}
                  </span>
                ))}
                {Array.from(permissionMap.entries()).length > 12 && (
                  <span className="px-2 py-1 text-[11px] text-dark-500">+ moreâ€¦</span>
                )}
              </div>
            </div>
          ) : (
            <Spinner />
          )}
        </Modal>
      </div>
    </PageTransition>
  );
}

function CreateRoleModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await createRole({ name: name.trim(), description: description.trim() || undefined });
      toast('Role created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create role', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New role" subtitle="Create a custom access role." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create role</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Role name" required value={name} onChange={(e) => setName(e.target.value)} hint="e.g. area_manager" />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
      </form>
    </Modal>
  );
}
