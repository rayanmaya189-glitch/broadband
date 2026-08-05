import { useEffect, useState } from 'react';
import { LifeBuoy, Plus, MessageSquare } from 'lucide-react';
import { createTicket, listTickets, ticketAction } from '../../api/admin/modules';
import type { Ticket } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, TextArea, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { timeAgo } from '../lib/format';
import { toast } from '../lib/toast';

const ACTIONS: { action: 'resolve' | 'escalate' | 'close' | 'reopen'; label: string; variant: 'success' | 'warning' | 'danger' | 'accent' }[] = [
  { action: 'resolve', label: 'Resolve', variant: 'success' },
  { action: 'escalate', label: 'Escalate', variant: 'warning' },
  { action: 'close', label: 'Close', variant: 'danger' },
  { action: 'reopen', label: 'Reopen', variant: 'accent' },
];

export function TicketsPage() {
  const [data, setData] = useState<Ticket[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [detail, setDetail] = useState<Ticket | null>(null);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listTickets());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load tickets', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const runAction = async (action: string, id: number) => {
    setActing({ id, action });
    try {
      await ticketAction(id, action as 'resolve' | 'escalate' | 'close' | 'reopen');
      toast(`Ticket ${action}d`);
      if (detail?.id === id) setDetail(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : `Failed to ${action} ticket`, 'error');
    } finally {
      setActing(null);
    }
  };

  const columns: Column<Ticket>[] = [
    { key: 'subject', header: 'Subject', render: (t) => (
      <div>
        <p className="font-medium text-white">{t.subject}</p>
        <p className="text-xs text-dark-500">{t.ticket_number ?? `#${t.id}`} Â· {t.customer_name ?? 'â€”'}</p>
      </div>
    )},
    { key: 'category', header: 'Category', render: (t) => <span className="text-dark-400">{t.category ?? 'â€”'}</span> },
    { key: 'priority', header: 'Priority', render: (t) => <StatusBadge status={t.priority} /> },
    { key: 'status', header: 'Status', render: (t) => <StatusBadge status={t.status} pulse={t.status === 'open'} /> },
    { key: 'assignee_name', header: 'Assignee', render: (t) => <span className="text-dark-400">{t.assignee_name ?? 'Unassigned'}</span> },
    { key: 'created_at', header: 'Created', render: (t) => <span className="text-xs text-dark-500">{timeAgo(t.created_at)}</span> },
    { key: 'actions', header: 'Actions', align: 'right', render: (t) => (
      <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
        <Button variant="ghost" size="sm" onClick={() => setDetail(t)}><MessageSquare className="h-4 w-4" /> View</Button>
      </div>
    )},
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Tickets & support"
          subtitle="Customer support and incident tickets."
          icon={<LifeBuoy className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New ticket</Button>}
        />

        <DataTable
          columns={columns}
          data={data}
          rowKey={(t) => t.id}
          loading={loading}
          onRowClick={(t) => setDetail(t)}
          emptyTitle="No tickets"
          emptyDescription="Support tickets will appear here."
        />

        <CreateTicketModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />

        <Modal open={detail !== null} onClose={() => setDetail(null)} title="Ticket details" subtitle={detail ? `${detail.ticket_number ?? `#${detail.id}`}` : undefined} width="lg">
          {detail ? (
            <div className="space-y-5">
              <div className="flex flex-wrap items-center gap-2">
                <StatusBadge status={detail.priority} />
                <StatusBadge status={detail.status} />
                <span className="text-xs text-dark-500">Created {timeAgo(detail.created_at)}</span>
              </div>
              <div>
                <h3 className="mb-1 text-sm font-semibold text-white">{detail.subject}</h3>
                {detail.category && <p className="text-sm text-dark-400">Category: {detail.category}</p>}
                <p className="mt-1 text-xs text-dark-500">
                  Customer: {detail.customer_name ?? `#${detail.customer_id ?? 'â€”'}`} Â· Assignee:{' '}
                  {detail.assignee_name ?? 'Unassigned'}
                </p>
              </div>
              <div className="flex flex-wrap gap-2 border-t border-white/[0.06] pt-4">
                {ACTIONS.map((a) => (
                  <Button
                    key={a.action}
                    variant={a.variant}
                    size="sm"
                    loading={acting?.id === detail.id && acting.action === a.action}
                    disabled={acting !== null}
                    onClick={() => void runAction(a.action, detail.id)}
                  >
                    {a.label}
                  </Button>
                ))}
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

function CreateTicketModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [customerId, setCustomerId] = useState('');
  const [subject, setSubject] = useState('');
  const [category, setCategory] = useState('billing');
  const [priority, setPriority] = useState('medium');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!subject.trim()) return;
    setBusy(true);
    try {
      await createTicket({
        subject: subject.trim(),
        customer_id: customerId ? Number(customerId) : undefined,
        category: category || undefined,
        priority,
      });
      toast('Ticket created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create ticket', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New ticket" subtitle="Raise a support or incident ticket." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create ticket</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Subject" required value={subject} onChange={(e) => setSubject(e.target.value)} />
        <TextField label="Customer ID" type="number" value={customerId} onChange={(e) => setCustomerId(e.target.value)} />
        <div className="grid gap-4 sm:grid-cols-2">
          <SelectField label="Category" value={category} onChange={(e) => setCategory(e.target.value)}
            options={[
              { value: 'billing', label: 'Billing' },
              { value: 'connection', label: 'Connection' },
              { value: 'network', label: 'Network' },
              { value: 'hardware', label: 'Hardware' },
              { value: 'complaint', label: 'Complaint' },
              { value: 'other', label: 'Other' },
            ]}
          />
          <SelectField label="Priority" value={priority} onChange={(e) => setPriority(e.target.value)}
            options={[
              { value: 'low', label: 'Low' },
              { value: 'medium', label: 'Medium' },
              { value: 'high', label: 'High' },
              { value: 'critical', label: 'Critical' },
            ]}
          />
        </div>
      </form>
    </Modal>
  );
}
