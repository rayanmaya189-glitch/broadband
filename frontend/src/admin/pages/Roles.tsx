import { useEffect, useState } from 'react';
import { Shield, Plus, KeyRound, Check, X } from 'lucide-react';
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
  const [rolePerms, setRolePerms] = useState<Set<number>>(new Set());
  const [permsLoading, setPermsLoading] = useState(false);

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

  const openDetail = async (role: Role) => {
    setDetail(role);
    setPermsLoading(true);
    // The role object may contain permission_ids from the API
    try {
      const roleData = role as Role & { permission_ids?: number[] };
      if (roleData.permission_ids) {
        setRolePerms(new Set(roleData.permission_ids));
      } else {
        // Fallback: derive from permissions_count
        setRolePerms(new Set());
      }
    } catch {
      setRolePerms(new Set());
    } finally {
      setPermsLoading(false);
    }
  };

  const togglePerm = (permId: number) => {
    setRolePerms((prev) => {
      const next = new Set(prev);
      if (next.has(permId)) next.delete(permId);
      else next.add(permId);
      return next;
    });
  };

  const savePerms = async () => {
    if (!detail) return;
    try {
      // Assign or revoke each permission based on current set
      const { apiPost, apiSend } = await import('../../api/admin/client');
      const roleData = detail as Role & { permission_ids?: number[] };
      const currentIds = new Set(roleData.permission_ids ?? []);

      // Add new permissions
      for (const permId of rolePerms) {
        if (!currentIds.has(permId)) {
          await apiPost(`/rbac/roles/${detail.id}/permissions`, { permission_id: permId });
        }
      }
      // Revoke removed permissions
      for (const permId of currentIds) {
        if (!rolePerms.has(permId)) {
          await apiSend('DELETE', `/rbac/roles/${detail.id}/permissions/${permId}`);
        }
      }
      toast('Permissions updated');
      setDetail(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update permissions', 'error');
    }
  };

  const permissionMap = new Map(permissions.map((p) => [p.id, p]));

  // Group permissions by module
  const groupedPerms = new Map<string, Permission[]>();
  for (const p of permissions) {
    const key = p.module ?? 'other';
    if (!groupedPerms.has(key)) groupedPerms.set(key, []);
    groupedPerms.get(key)!.push(p);
  }

  const columns: Column<Role>[] = [
    { key: 'name', header: 'Role', render: (r) => (
      <div>
        <p className="font-medium text-white">{toLabel(r.name)}</p>
        {r.description && <p className="max-w-md truncate text-xs text-dark-500">{r.description}</p>}
      </div>
    )},
    { key: 'slug', header: 'Slug', render: (r) => <span className="font-mono text-xs text-dark-400">{r.slug ?? r.name}</span> },
    { key: 'permissions_count', header: 'Permissions', render: (r) => (
      <span className="tabular-nums text-dark-300">{r.permissions_count ?? '—'}</span>
    )},
    { key: 'is_system', header: 'Type', render: (r) => (r.is_system ? <StatusBadge status="system" /> : <span className="text-dark-600">—</span>) },
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
              onRowClick={(r) => openDetail(r)}
              emptyTitle="No roles"
              emptyDescription="Roles will appear here."
            />

            <section className="glass-card rounded-2xl p-5">
              <h2 className="mb-4 flex items-center gap-2 text-sm font-semibold text-white">
                <KeyRound className="h-4 w-4 text-primary-400" /> Permission catalog ({permissions.length})
              </h2>
              <div className="space-y-3">
                {Array.from(groupedPerms.entries()).map(([module, perms]) => (
                  <div key={module}>
                    <p className="mb-1.5 text-xs font-semibold uppercase tracking-wider text-dark-500">{toLabel(module)}</p>
                    <div className="flex flex-wrap gap-1.5">
                      {perms.map((p) => (
                        <span
                          key={p.id}
                          title={`${p.resource ?? ''} ${p.action ?? ''}`}
                          className="rounded-lg border border-white/[0.08] bg-white/[0.03] px-2 py-1 text-[11px] text-dark-300"
                        >
                          {p.name}
                        </span>
                      ))}
                    </div>
                  </div>
                ))}
                {permissions.length === 0 && <p className="text-sm text-dark-500">No permissions returned.</p>}
              </div>
            </section>
          </>
        )}

        <CreateRoleModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />

        <Modal open={detail !== null} onClose={() => setDetail(null)} title="Manage permissions" subtitle={detail ? toLabel(detail.name) : undefined} width="xl">
          {detail ? (
            <div className="space-y-4">
              {detail.description && <p className="text-sm text-dark-300">{detail.description}</p>}

              {permsLoading ? (
                <Spinner />
              ) : (
                <div className="space-y-4 max-h-[60vh] overflow-y-auto pr-2">
                  {Array.from(groupedPerms.entries()).map(([module, perms]) => (
                    <div key={module}>
                      <div className="mb-2 flex items-center justify-between">
                        <p className="text-xs font-semibold uppercase tracking-wider text-dark-400">{toLabel(module)}</p>
                        <button
                          className="text-[11px] text-accent-400 hover:text-accent-300"
                          onClick={() => {
                            const allIds = perms.map((p) => p.id);
                            const allSelected = allIds.every((id) => rolePerms.has(id));
                            setRolePerms((prev) => {
                              const next = new Set(prev);
                              for (const id of allIds) {
                                if (allSelected) next.delete(id);
                                else next.add(id);
                              }
                              return next;
                            });
                          }}
                        >
                          {perms.every((p) => rolePerms.has(p.id)) ? 'Deselect all' : 'Select all'}
                        </button>
                      </div>
                      <div className="space-y-1">
                        {perms.map((p) => {
                          const selected = rolePerms.has(p.id);
                          return (
                            <button
                              key={p.id}
                              onClick={() => togglePerm(p.id)}
                              className={`flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left text-sm transition-colors ${
                                selected
                                  ? 'border border-accent-500/30 bg-accent-500/10 text-white'
                                  : 'border border-white/[0.04] bg-white/[0.02] text-dark-400 hover:bg-white/[0.04]'
                              }`}
                            >
                              <div className={`flex h-5 w-5 shrink-0 items-center justify-center rounded border ${
                                selected ? 'border-accent-500 bg-accent-500' : 'border-dark-600 bg-dark-900'
                              }`}>
                                {selected && <Check className="h-3 w-3 text-white" />}
                              </div>
                              <div className="min-w-0 flex-1">
                                <p className="font-medium">{p.name}</p>
                                {p.description && <p className="text-xs text-dark-500 truncate">{p.description}</p>}
                              </div>
                              <span className="shrink-0 text-[10px] text-dark-600">{p.resource ?? ''} {p.action ?? ''}</span>
                            </button>
                          );
                        })}
                      </div>
                    </div>
                  ))}
                </div>
              )}

              <div className="flex items-center justify-between border-t border-white/[0.06] pt-4">
                <p className="text-xs text-dark-500">{rolePerms.size} permission{rolePerms.size !== 1 ? 's' : ''} selected</p>
                <div className="flex gap-2">
                  <Button variant="ghost" onClick={() => setDetail(null)}>Cancel</Button>
                  <Button onClick={savePerms}>Save permissions</Button>
                </div>
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
      const slug = name.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '');
      await createRole({ name: name.trim(), slug, description: description.trim() || undefined });
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
