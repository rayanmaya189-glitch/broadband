import { useEffect, useState } from 'react';
import { UserCog, Shield, ShieldOff } from 'lucide-react';
import { assignRoleToUser, revokeRoleFromUser, listRoles, listUsers } from '../../api/admin/modules';
import type { Role, UserAccount } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

export function UsersPage() {
  const [users, setUsers] = useState<UserAccount[]>([]);
  const [roles, setRoles] = useState<Role[]>([]);
  const [loading, setLoading] = useState(true);
  const [manage, setManage] = useState<UserAccount | null>(null);
  const [userRoles, setUserRoles] = useState<number[]>([]);
  const [assignRoleId, setAssignRoleId] = useState('');
  const [busy, setBusy] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const [u, r] = await Promise.all([listUsers(), listRoles()]);
      setUsers(u);
      setRoles(r);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load users', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const openManage = (user: UserAccount) => {
    setManage(user);
    // Derive role IDs from user data if available
    const userData = user as UserAccount & { role_ids?: number[] };
    setUserRoles(userData.role_ids ?? []);
    setAssignRoleId('');
  };

  const doAssign = async () => {
    if (!manage || !assignRoleId) return;
    setBusy(true);
    try {
      await assignRoleToUser(manage.id, Number(assignRoleId));
      toast('Role assigned');
      setUserRoles((prev) => [...prev, Number(assignRoleId)]);
      setAssignRoleId('');
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to assign role', 'error');
    } finally {
      setBusy(false);
    }
  };

  const doRevoke = async (roleId: number) => {
    if (!manage) return;
    setBusy(true);
    try {
      await revokeRoleFromUser(manage.id, roleId);
      toast('Role revoked');
      setUserRoles((prev) => prev.filter((id) => id !== roleId));
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to revoke role', 'error');
    } finally {
      setBusy(false);
    }
  };

  const assignedRoles = roles.filter((r) => userRoles.includes(r.id));
  const availableRoles = roles.filter((r) => !userRoles.includes(r.id));

  const columns: Column<UserAccount>[] = [
    { key: 'name', header: 'User', render: (u) => (
      <div>
        <p className="font-medium text-white">{u.name}</p>
        <p className="text-xs text-dark-500">{u.email}</p>
      </div>
    )},
    { key: 'phone', header: 'Phone', render: (u) => <span className="text-dark-400">{u.phone || '—'}</span> },
    { key: 'branch_id', header: 'Branch', render: (u) => <span className="text-dark-400">{u.branch_id ? `#${u.branch_id}` : 'Platform-wide'}</span> },
    { key: 'status', header: 'Status', render: (u) => <StatusBadge status={u.status} /> },
    { key: 'last_login_at', header: 'Last login', render: (u) => <span className="text-xs text-dark-500">{formatDateTime(u.last_login_at)}</span> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (u) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <Button variant="secondary" size="sm" onClick={() => openManage(u)}>
            <Shield className="h-4 w-4" /> Roles
          </Button>
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Users & roles"
          subtitle="Staff accounts and their access roles."
          icon={<UserCog className="h-5 w-5" />}
        />

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={users}
            rowKey={(u) => u.id}
            emptyTitle="No staff users"
            emptyDescription="Staff accounts will appear here."
          />
        )}

        <Modal open={manage !== null} onClose={() => setManage(null)} title="Manage roles" subtitle={manage?.name} width="lg">
          <div className="space-y-5">
            {/* Assigned roles */}
            <div>
              <h3 className="mb-2 text-sm font-semibold text-white">Assigned roles</h3>
              {assignedRoles.length === 0 ? (
                <p className="text-sm text-dark-500">No roles assigned.</p>
              ) : (
                <div className="space-y-1.5">
                  {assignedRoles.map((r) => (
                    <div key={r.id} className="flex items-center justify-between gap-3 rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-2.5">
                      <div>
                        <p className="text-sm font-medium text-white">{toLabel(r.name)}</p>
                        {r.description && <p className="text-xs text-dark-500">{r.description}</p>}
                      </div>
                      <Button variant="danger" size="sm" loading={busy} onClick={() => void doRevoke(r.id)}>
                        <ShieldOff className="h-3.5 w-3.5" /> Revoke
                      </Button>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* Assign new role */}
            <div className="border-t border-white/[0.06] pt-4">
              <h3 className="mb-2 text-sm font-semibold text-white">Assign new role</h3>
              <div className="flex items-end gap-2">
                <SelectField
                  label="Role"
                  value={assignRoleId}
                  onChange={(e) => setAssignRoleId(e.target.value)}
                  options={availableRoles.map((r) => ({ value: String(r.id), label: toLabel(r.name) }))}
                  placeholder={availableRoles.length === 0 ? 'All roles assigned' : 'Select a role'}
                />
                <Button onClick={doAssign} loading={busy} disabled={!assignRoleId || availableRoles.length === 0}>Assign</Button>
              </div>
              {availableRoles.length === 0 && (
                <p className="mt-1 text-xs text-dark-500">This user already has all available roles.</p>
              )}
            </div>
          </div>
        </Modal>
      </div>
    </PageTransition>
  );
}
