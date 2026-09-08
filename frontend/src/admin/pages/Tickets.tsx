import { useEffect, useState } from 'react';
import { LifeBuoy, Plus, MessageSquare, UserPlus, Send } from 'lucide-react';
import { createTicket, listTickets, ticketAction, listTicketComments, addTicketComment, getTicketMetrics, listUsers } from '../../api/admin/modules';
import type { Ticket, TicketComment, TicketMetrics, UserAccount } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, TextArea, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { timeAgo, formatDateTime } from '../lib/format';
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
  const [metrics, setMetrics] = useState<TicketMetrics | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [tickets, m] = await Promise.all([listTickets(), getTicketMetrics().catch(() => null)]);
      setData(tickets);
      if (m) setMetrics(m);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load tickets', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const runAction = async (action: string, id: number, escalatedTo?: number) => {
    if (action === 'escalate' && !escalatedTo) {
      toast('Select a staff member to escalate to first.', 'error');
      return;
    }
    setActing({ id, action });
    try {
      const payload =
        action === 'escalate' ? { escalated_to: escalatedTo } :
        action === 'assign' ? { assigned_to: escalatedTo } :
        undefined;
      await ticketAction(id, action as 'assign' | 'resolve' | 'escalate' | 'close' | 'reopen', payload);
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
        <p className="text-xs text-dark-500">{t.ticket_number ?? `#${t.id}`} · {t.customer_name ?? '—'}</p>
      </div>
    )},
    { key: 'category', header: 'Category', render: (t) => <span className="text-dark-400">{t.category ?? '—'}</span> },
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

        {metrics && (
          <div className="grid gap-4 sm:grid-cols-5">
            {[
              { label: 'Open', value: metrics.open, color: 'text-blue-400' },
              { label: 'In progress', value: metrics.in_progress, color: 'text-amber-400' },
              { label: 'Resolved', value: metrics.resolved, color: 'text-emerald-400' },
              { label: 'Closed', value: metrics.closed, color: 'text-dark-500' },
              { label: 'Avg resolution', value: metrics.avg_resolution_hours != null ? `${Math.round(metrics.avg_resolution_hours)}h` : '—', color: 'text-dark-300' },
            ].map((m) => (
              <div key={m.label} className="glass-card rounded-xl px-4 py-3 text-center">
                <p className={`text-2xl font-semibold tabular-nums ${m.color}`}>{typeof m.value === 'number' ? m.value.toLocaleString() : m.value}</p>
                <p className="text-xs text-dark-500">{m.label}</p>
              </div>
            ))}
          </div>
        )}

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

        {detail && (
          <TicketDetailModal
            ticket={detail}
            open={detail !== null}
            onClose={() => setDetail(null)}
            onAction={async (action, escalatedTo) => {
              await runAction(action, detail.id, escalatedTo);
              setDetail(null);
            }}
            acting={acting}
          />
        )}
      </div>
    </PageTransition>
  );
}

function CreateTicketModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [customerId, setCustomerId] = useState('');
  const [subject, setSubject] = useState('');
  const [description, setDescription] = useState('');
  const [category, setCategory] = useState('billing');
  const [priority, setPriority] = useState('medium');
  const [source, setSource] = useState('portal');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!subject.trim() || !description.trim()) return;
    setBusy(true);
    try {
      await createTicket({
        subject: subject.trim(),
        description: description.trim(),
        customer_id: customerId ? Number(customerId) : undefined,
        category: category || 'general',
        priority,
        source,
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
        <TextArea label="Description" required value={description} onChange={(e) => setDescription(e.target.value)} placeholder="Describe the issue…" />
        <TextField label="Customer ID" type="number" value={customerId} onChange={(e) => setCustomerId(e.target.value)} />
        <div className="grid gap-4 sm:grid-cols-3">
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
          <SelectField label="Source" value={source} onChange={(e) => setSource(e.target.value)}
            options={[
              { value: 'portal', label: 'Customer portal' },
              { value: 'phone', label: 'Phone call' },
              { value: 'email', label: 'Email' },
              { value: 'walk_in', label: 'Walk-in' },
              { value: 'whatsapp', label: 'WhatsApp' },
              { value: 'internal', label: 'Internal' },
            ]}
          />
        </div>
      </form>
    </Modal>
  );
}

function TicketDetailModal({
  ticket,
  open,
  onClose,
  onAction,
  acting,
}: {
  ticket: Ticket;
  open: boolean;
  onClose: () => void;
  onAction: (action: string, escalatedTo?: number) => void;
  acting: { id: number; action: string } | null;
}) {
  const [comments, setComments] = useState<TicketComment[]>([]);
  const [loadingComments, setLoadingComments] = useState(true);
  const [newComment, setNewComment] = useState('');
  const [isInternal, setIsInternal] = useState(false);
  const [posting, setPosting] = useState(false);
  const [users, setUsers] = useState<UserAccount[]>([]);
  const [assignUserId, setAssignUserId] = useState('');
  const [assigning, setAssigning] = useState(false);

  const loadComments = async () => {
    setLoadingComments(true);
    try {
      setComments(await listTicketComments(ticket.id));
    } catch {
      setComments([]);
    } finally {
      setLoadingComments(false);
    }
  };

  useEffect(() => {
    if (open) {
      void loadComments();
      void listUsers().then(setUsers).catch(() => undefined);
    }
  }, [open]);

  const postComment = async () => {
    if (!newComment.trim()) return;
    setPosting(true);
    try {
      await addTicketComment(ticket.id, { content: newComment.trim(), is_internal: isInternal });
      toast('Comment added');
      setNewComment('');
      setIsInternal(false);
      void loadComments();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to post comment', 'error');
    } finally {
      setPosting(false);
    }
  };

  const assignTicket = async () => {
    if (!assignUserId) return;
    setAssigning(true);
    try {
      await ticketAction(ticket.id, 'assign', { assigned_to: Number(assignUserId) });
      toast('Ticket assigned');
      setAssignUserId('');
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to assign ticket', 'error');
    } finally {
      setAssigning(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Ticket details" subtitle={ticket.ticket_number ?? `#${ticket.id}`} width="lg">
      <div className="space-y-5">
        <div className="flex flex-wrap items-center gap-2">
          <StatusBadge status={ticket.priority} />
          <StatusBadge status={ticket.status} />
          <span className="text-xs text-dark-500">Created {timeAgo(ticket.created_at)}</span>
        </div>
        <div>
          <h3 className="mb-1 text-sm font-semibold text-white">{ticket.subject}</h3>
          {ticket.category && <p className="text-sm text-dark-400">Category: {ticket.category}</p>}
          <p className="mt-1 text-xs text-dark-500">
            Customer: {ticket.customer_name ?? `#${ticket.customer_id ?? '—'}`} · Assignee: {ticket.assignee_name ?? 'Unassigned'}
          </p>
        </div>

        <div className="flex flex-wrap gap-2 border-t border-white/[0.06] pt-4">
          {ACTIONS.map((a) => (
            <Button
              key={a.action}
              variant={a.variant}
              size="sm"
              loading={acting?.id === ticket.id && acting.action === a.action}
              disabled={acting !== null}
              onClick={() => onAction(a.action, a.action === 'escalate' && assignUserId ? Number(assignUserId) : undefined)}
            >
              {a.label}
            </Button>
          ))}
        </div>

        {/* Assignment / escalation target */}
        <div className="border-t border-white/[0.06] pt-4">
          <h4 className="mb-2 text-xs font-semibold uppercase tracking-wider text-dark-500">Assign / escalate to</h4>
          <div className="flex items-end gap-2">
            <SelectField
              value={assignUserId}
              onChange={(e) => setAssignUserId(e.target.value)}
              options={users.map((u) => ({ value: String(u.id), label: `${u.name} (${u.email})` }))}
              placeholder="Select staff member"
            />
            <Button size="sm" onClick={assignTicket} loading={assigning} disabled={!assignUserId}>
              <UserPlus className="h-3.5 w-3.5" /> Assign
            </Button>
          </div>
        </div>

        {/* Comments */}
        <div className="border-t border-white/[0.06] pt-4">
          <h4 className="mb-3 text-xs font-semibold uppercase tracking-wider text-dark-500">
            Comments ({comments.length})
          </h4>

          {loadingComments ? (
            <Spinner />
          ) : comments.length === 0 ? (
            <p className="py-4 text-center text-sm text-dark-500">No comments yet.</p>
          ) : (
            <div className="space-y-2 max-h-64 overflow-y-auto">
              {comments.map((c) => (
                <div key={c.id} className={`rounded-xl px-4 py-3 text-sm ${c.is_internal ? 'border border-amber-500/20 bg-amber-500/5' : 'border border-white/[0.06] bg-white/[0.02]'}`}>
                  <div className="flex items-center justify-between gap-2">
                    <span className="text-xs font-medium text-white">{c.user_name ?? 'Staff'}</span>
                    <div className="flex items-center gap-2">
                      {c.is_internal && <StatusBadge status="internal" />}
                      <span className="text-[11px] text-dark-500">{timeAgo(c.created_at)}</span>
                    </div>
                  </div>
                  <p className="mt-1 text-dark-300 whitespace-pre-wrap">{c.content}</p>
                </div>
              ))}
            </div>
          )}

          <div className="mt-3 space-y-2">
            <TextArea
              value={newComment}
              onChange={(e) => setNewComment(e.target.value)}
              placeholder="Add a comment…"
            />
            <div className="flex items-center justify-between">
              <label className="flex items-center gap-2 text-xs text-dark-400">
                <input type="checkbox" checked={isInternal} onChange={(e) => setIsInternal(e.target.checked)}
                  className="rounded border-dark-600 bg-dark-900 text-accent-500" />
                Internal note (not visible to customer)
              </label>
              <Button size="sm" onClick={postComment} loading={posting} disabled={!newComment.trim()}>
                <Send className="h-3.5 w-3.5" /> Post
              </Button>
            </div>
          </div>
        </div>
      </div>
    </Modal>
  );
}
