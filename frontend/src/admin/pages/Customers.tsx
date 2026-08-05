import { useEffect, useMemo, useState } from 'react';
import { Users, Plus, Phone, Mail, MapPin } from 'lucide-react';
import {
  listCustomers,
  getCustomer,
  createCustomer,
  updateCustomerStatus,
  deleteCustomer,
  listCustomerAddresses,
  getCustomerHistory,
  type CustomerListItem,
  type ListCustomersOptions,
} from '../../api/admin/modules';
import { listBranches } from '../../api/admin/modules';
import type { Branch, Paginated } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { SearchInput } from '../components/ui/SearchInput';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Pagination } from '../components/ui/Pagination';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { SelectField, TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, timeAgo } from '../lib/format';
import { toast } from '../lib/toast';

const PAGE_SIZE = 20;
const STATUSES = ['', 'active', 'pending', 'suspended', 'terminated'];

export function CustomersPage() {
  const [data, setData] = useState<Paginated<CustomerListItem>>({
    items: [],
    total: 0,
    page: 1,
    limit: PAGE_SIZE,
  });
  const [loading, setLoading] = useState(true);
  const [query, setQuery] = useState('');
  const [status, setStatus] = useState('');
  const [page, setPage] = useState(1);
  const [branches, setBranches] = useState<Branch[]>([]);

  const [createOpen, setCreateOpen] = useState(false);
  const [detail, setDetail] = useState<CustomerListItem | null>(null);
  const [detailLoading, setDetailLoading] = useState(false);
  const [addresses, setAddresses] = useState<Awaited<ReturnType<typeof listCustomerAddresses>>>([]);
  const [history, setHistory] = useState<Awaited<ReturnType<typeof getCustomerHistory>> | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<CustomerListItem | null>(null);
  const [deleting, setDeleting] = useState(false);

  const load = async (opts: ListCustomersOptions) => {
    setLoading(true);
    try {
      const res = await listCustomers(opts);
      setData({ items: res.customers, total: res.total_count, page: res.page, limit: res.page_size || PAGE_SIZE });
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load customers', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load({ page, page_size: PAGE_SIZE, status: status || undefined });
  }, [page, status]);

  useEffect(() => {
    const t = setTimeout(() => {
      void listCustomers({ page: 1, page_size: PAGE_SIZE, query: query.trim() || undefined, status: status || undefined })
        .then((res) => {
          setData({ items: res.customers, total: res.total_count, page: res.page, limit: res.page_size || PAGE_SIZE });
          setPage(1);
        })
        .catch(() => undefined);
    }, 400);
    return () => clearTimeout(t);
  }, [query]);

  useEffect(() => {
    void listBranches().then(setBranches).catch(() => undefined);
  }, []);

  const openDetail = async (customer: CustomerListItem) => {
    setDetail(customer);
    setDetailLoading(true);
    setAddresses([]);
    setHistory(null);
    try {
      const [full, addrs, hist] = await Promise.all([
        getCustomer(customer.id),
        listCustomerAddresses(customer.id),
        getCustomerHistory(customer.id),
      ]);
      setDetail(full);
      setAddresses(addrs);
      setHistory(hist);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load customer details', 'error');
    } finally {
      setDetailLoading(false);
    }
  };

  const setStatusFor = async (customerId: number, newStatus: string) => {
    try {
      await updateCustomerStatus(customerId, newStatus);
      toast(`Customer marked ${newStatus}`);
      if (detail?.id === customerId) setDetail((d) => (d ? { ...d, status: newStatus } : d));
      void load({ page, page_size: PAGE_SIZE, status: status || undefined });
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update status', 'error');
    }
  };

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    setDeleting(true);
    try {
      await deleteCustomer(deleteTarget.id);
      toast('Customer deleted', 'info');
      setDeleteTarget(null);
      if (detail?.id === deleteTarget.id) setDetail(null);
      void load({ page, page_size: PAGE_SIZE, status: status || undefined });
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to delete customer', 'error');
    } finally {
      setDeleting(false);
    }
  };

  const columns: Column<CustomerListItem>[] = [
    {
      key: 'customer_code',
      header: 'Code',
      render: (c) => <span className="font-mono text-xs text-accent-300">{c.customer_code ?? `#${c.id}`}</span>,
    },
    { key: 'name', header: 'Name', render: (c) => <span className="font-medium text-white">{c.name}</span> },
    { key: 'phone', header: 'Phone', render: (c) => (
      <span className="flex items-center gap-1.5 text-dark-300"><Phone className="h-3.5 w-3.5 text-dark-500" />{c.phone}</span>
    )},
    { key: 'email', header: 'Email', render: (c) => <span className="text-dark-400">{c.email ?? 'â€”'}</span> },
    { key: 'status', header: 'Status', render: (c) => <StatusBadge status={c.status} /> },
    {
      key: 'created_at',
      header: 'Created',
      render: (c) => <span className="text-xs text-dark-500">{timeAgo(c.created_at)}</span>,
    },
  ];

  const branchOptions = useMemo(
    () => branches.map((b) => ({ value: String(b.id), label: b.name })),
    [branches]
  );

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Customers"
          subtitle="Manage customer accounts across branches."
          icon={<Users className="h-5 w-5" />}
          actions={
            <Button onClick={() => setCreateOpen(true)}>
              <Plus className="h-4 w-4" /> Add customer
            </Button>
          }
        />

        <div className="flex flex-col gap-3 sm:flex-row">
          <SearchInput
            value={query}
            onChange={setQuery}
            placeholder="Search by name, phone, email or codeâ€¦"
            className="w-full sm:max-w-sm"
          />
          <SelectField
            value={status}
            onChange={(e) => setStatus(e.target.value)}
            options={STATUSES.map((s) => ({ value: s, label: s ? s.charAt(0).toUpperCase() + s.slice(1) : 'All statuses' }))}
            placeholder=""
            className="sm:w-44"
          />
        </div>

        <DataTable
          columns={columns}
          data={data.items}
          rowKey={(c) => c.id}
          loading={loading}
          onRowClick={openDetail}
          emptyTitle="No customers found"
          emptyDescription="Try adjusting your search or add a new customer."
          footer={
            <Pagination
              page={data.page}
              pageSize={data.limit}
              total={data.total}
              onPageChange={(p) => setPage(p)}
            />
          }
        />

        {/* Create modal */}
        <CreateCustomerModal
          open={createOpen}
          onClose={() => setCreateOpen(false)}
          branchOptions={branchOptions}
          onCreated={() => {
            setCreateOpen(false);
            setPage(1);
            setQuery('');
            void load({ page: 1, page_size: PAGE_SIZE, status: status || undefined });
          }}
        />

        {/* Detail modal */}
        <Modal
          open={detail !== null}
          onClose={() => setDetail(null)}
          title="Customer details"
          subtitle={detail ? `${detail.customer_code ?? `#${detail.id}`} Â· ${detail.name}` : undefined}
          width="lg"
        >
          {detailLoading ? (
            <Spinner />
          ) : detail ? (
            <div className="space-y-6">
              <div className="flex flex-wrap items-center gap-3">
                <StatusBadge status={detail.status} pulse={detail.status === 'active'} />
                <SelectField
                  value={detail.status}
                  onChange={(e) => void setStatusFor(detail.id, e.target.value)}
                  options={STATUSES.filter((s) => s).map((s) => ({ value: s, label: s.charAt(0).toUpperCase() + s.slice(1) }))}
                  className="w-40"
                  placeholder="Change status"
                />
                <Button
                  variant="danger"
                  size="sm"
                  className="ml-auto"
                  onClick={() => setDeleteTarget(detail)}
                >
                  Delete
                </Button>
              </div>

              <div className="grid gap-3 rounded-xl border border-white/[0.06] bg-dark-950/40 p-4 text-sm sm:grid-cols-2">
                <div className="flex items-center gap-2 text-dark-300">
                  <Phone className="h-4 w-4 text-dark-500" /> {detail.phone}
                </div>
                <div className="flex items-center gap-2 text-dark-300">
                  <Mail className="h-4 w-4 text-dark-500" /> {detail.email ?? 'â€”'}
                </div>
                <div className="text-dark-300">
                  Branch: <span className="text-dark-200">{detail.branch_id ? `#${detail.branch_id}` : 'â€”'}</span>
                </div>
                <div className="text-dark-300">
                  Created: <span className="text-dark-200">{formatDateTime(detail.created_at)}</span>
                </div>
              </div>

              <div>
                <h3 className="mb-2 flex items-center gap-2 text-sm font-semibold text-white">
                  <MapPin className="h-4 w-4 text-primary-400" /> Addresses
                </h3>
                {addresses.length === 0 ? (
                  <p className="text-sm text-dark-500">No addresses on file.</p>
                ) : (
                  <div className="space-y-2">
                    {addresses.map((a) => (
                      <div key={a.id} className="flex items-start justify-between gap-3 rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3 text-sm">
                        <div>
                          <p className="text-dark-200">
                            {a.line1}
                            {a.line2 ? `, ${a.line2}` : ''}
                          </p>
                          <p className="text-xs text-dark-500">
                            {a.city}, {a.state} {a.pincode}
                          </p>
                        </div>
                        {a.is_primary && <StatusBadge status="primary" />}
                      </div>
                    ))}
                  </div>
                )}
              </div>

              <div>
                <h3 className="mb-2 text-sm font-semibold text-white">Recent activity</h3>
                {history && history.items.length === 0 ? (
                  <p className="text-sm text-dark-500">No recorded activity.</p>
                ) : (
                  <div className="space-y-1.5">
                    {history?.items.slice(0, 8).map((h) => (
                      <div key={h.id} className="flex items-center justify-between gap-3 rounded-lg border border-white/[0.04] px-3 py-2 text-xs">
                        <span className="text-dark-300">{h.action}</span>
                        <span className="shrink-0 text-dark-500">{timeAgo(h.created_at)}</span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          ) : null}
        </Modal>

        <ConfirmDialog
          open={deleteTarget !== null}
          onClose={() => setDeleteTarget(null)}
          onConfirm={confirmDelete}
          loading={deleting}
          title="Delete customer?"
          description={`This permanently removes ${deleteTarget?.name ?? 'this customer'} and their record. This action cannot be undone.`}
          confirmLabel="Delete customer"
          danger
        />
      </div>
    </PageTransition>
  );
}

function CreateCustomerModal({
  open,
  onClose,
  branchOptions,
  onCreated,
}: {
  open: boolean;
  onClose: () => void;
  branchOptions: { value: string; label: string }[];
  onCreated: () => void;
}) {
  const [name, setName] = useState('');
  const [phone, setPhone] = useState('');
  const [email, setEmail] = useState('');
  const [alternatePhone, setAlternatePhone] = useState('');
  const [branchId, setBranchId] = useState('');
  const [busy, setBusy] = useState(false);
  const [touched, setTouched] = useState(false);

  const reset = () => {
    setName('');
    setPhone('');
    setEmail('');
    setAlternatePhone('');
    setBranchId('');
    setTouched(false);
  };

  const submit = async () => {
    setTouched(true);
    if (!name.trim() || !phone.trim()) return;
    setBusy(true);
    try {
      await createCustomer({
        branch_id: branchId ? Number(branchId) : 0,
        name: name.trim(),
        phone: phone.trim(),
        email: email.trim() || undefined,
        alternate_phone: alternatePhone.trim() || undefined,
      });
      toast('Customer created');
      reset();
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create customer', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open={open}
      onClose={() => {
        reset();
        onClose();
      }}
      title="Add customer"
      subtitle="Create a new customer account."
      width="md"
      footer={
        <>
          <Button variant="ghost" onClick={onClose}>Cancel</Button>
          <Button onClick={submit} loading={busy}>Create customer</Button>
        </>
      }
    >
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void submit();
        }}
        className="space-y-4"
        noValidate
      >
        <TextField
          label="Full name"
          required
          value={name}
          onChange={(e) => setName(e.target.value)}
          error={touched && !name.trim() ? 'Name is required.' : undefined}
        />
        <TextField
          label="Phone"
          required
          value={phone}
          onChange={(e) => setPhone(e.target.value)}
          error={touched && !phone.trim() ? 'Phone is required.' : undefined}
        />
        <TextField
          label="Email"
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <TextField
          label="Alternate phone"
          value={alternatePhone}
          onChange={(e) => setAlternatePhone(e.target.value)}
        />
        <SelectField
          label="Branch"
          value={branchId}
          onChange={(e) => setBranchId(e.target.value)}
          options={branchOptions}
          placeholder="Select a branch"
        />
      </form>
    </Modal>
  );
}
