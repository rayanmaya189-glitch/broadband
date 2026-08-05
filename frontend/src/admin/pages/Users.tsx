import { useEffect, useState } from 'react';
import { UserCog, Shield } from 'lucide-react';
import { assignRoleToUser, listRoles, listUsers } from '../../api/admin/modules';
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

  const doAssign = async () => {
    if (!manage || !assignRoleId) return;
    setBusy(true);
    try {
      await assignRoleToUser(manage.id, Number(assignRoleId));
      toast('Role assigned');
      setManage(null);
      setAssignRoleId('');
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to assign role', 'error');
    } finally {
      setBusy(false);
    }
  };

  const columns: Column<UserAccount>[] = [
    { key: 'name', header: 'User', render: (u) => (
      <div>
        <p className="font-medium text-white">{u.name}</p>
        <p className="text-xs text-dark-500">{u.email}</p>
      </div>
    )},
    { key: 'phone', header: 'Phone', render: (u) => <span className="text-dark-400">{u.phone || 'â€”'}</span> },
    { key: 'branch_id', header: 'Branch', render: (u) => <span className="text-dark-400">{u.branch_id ? `#${u.branch_id}` : 'Platform-wide'}</span> },
    { key: 'status', header: 'Status', render: (u) => <StatusBadge status={u.status} /> },
    { key: 'last_login_at', header: 'Last login', render: (u) => <span className="text-xs text-dark-500">{formatDateTime(u.last_login_at)}</span> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (u) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <Button variant="secondary" size="sm" onClick={() => { setManage(u); setAssignRoleId(''); }}>
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

        <Modal open={manage !== null} onClose={() => setManage(null)} title="Manage roles" subtitle={manage?.name} width="md">
          <div className="space-y-4">
            <div className="flex items-end gap-2">
              <SelectField
                label="Assign role"
                value={assignRoleId}
                onChange={(e) => setAssignRoleId(e.target.value)}
                options={roles.map((r) => ({ value: String(r.id), label: toLabel(r.name) }))}
                placeholder="Select a role"
              />
              <Button onClick={doAssign} loading={busy} disabled={!assignRoleId}>Assign</Button>
            </div>
            <p className="text-xs text-dark-500">Revoking a role removes that access level immediately.</p>
          </div>
        </Modal>
      </div>
    </PageTransition>
  );
}
