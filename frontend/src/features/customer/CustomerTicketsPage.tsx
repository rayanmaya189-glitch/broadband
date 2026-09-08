import { useEffect, useState } from 'react';
import { LifeBuoy, Plus, Send, ArrowLeft } from 'lucide-react';
import { getMyTickets, createTicket, getTicketComments, addTicketComment } from '../../api/customer';
import { timeAgo } from '../../utils/format';
import type { CustomerTicket, CustomerTicketComment } from '../../types';

export default function CustomerTicketsPage() {
  const [tickets, setTickets] = useState<CustomerTicket[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [detail, setDetail] = useState<CustomerTicket | null>(null);

  useEffect(() => {
    void getMyTickets()
      .then((data) => setTickets(data as unknown as CustomerTicket[]))
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  const handleCreated = () => {
    setCreateOpen(false);
    void getMyTickets().then((data) => setTickets(data as unknown as CustomerTicket[])).catch(() => {});
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-white">Support tickets</h1>
          <p className="mt-1 text-sm text-dark-400">Get help from our support team.</p>
        </div>
        {!createOpen && !detail && (
          <button onClick={() => setCreateOpen(true)} className="flex items-center gap-1.5 rounded-lg bg-primary-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-primary-500">
            <Plus className="h-4 w-4" /> New ticket
          </button>
        )}
      </div>

      {createOpen ? (
        <CreateTicketForm onCreated={handleCreated} onCancel={() => setCreateOpen(false)} />
      ) : detail ? (
        <TicketDetail ticket={detail} onBack={() => setDetail(null)} />
      ) : loading ? (
        <div className="py-12 text-center text-sm text-dark-500">Loading tickets…</div>
      ) : tickets.length === 0 ? (
        <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] py-12 text-center">
          <LifeBuoy className="mx-auto mb-3 h-8 w-8 text-dark-500" />
          <p className="text-sm text-dark-400">No tickets yet.</p>
          <button onClick={() => setCreateOpen(true)} className="mt-3 text-sm text-accent-400 hover:text-accent-300">Submit your first ticket →</button>
        </div>
      ) : (
        <div className="space-y-2">
          {tickets.map((t) => (
            <button key={t.id} onClick={() => setDetail(t)} className="flex w-full items-center justify-between gap-4 rounded-xl border border-white/[0.06] bg-white/[0.02] px-5 py-4 text-left transition-colors hover:bg-white/[0.04]">
              <div className="min-w-0">
                <p className="font-medium text-white">{t.subject}</p>
                <p className="mt-0.5 text-xs text-dark-500">{t.ticket_number ?? `#${t.id}`} · {t.category ?? 'General'} · {timeAgo(t.created_at)}</p>
              </div>
              <span className={`shrink-0 rounded-full px-2.5 py-0.5 text-xs font-medium ${
                t.status === 'open' ? 'bg-blue-500/10 text-blue-400' :
                t.status === 'resolved' ? 'bg-emerald-500/10 text-emerald-400' :
                t.status === 'closed' ? 'bg-dark-700 text-dark-400' :
                'bg-amber-500/10 text-amber-400'
              }`}>{t.status}</span>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

function CreateTicketForm({ onCreated, onCancel }: { onCreated: () => void; onCancel: () => void }) {
  const [subject, setSubject] = useState('');
  const [category, setCategory] = useState('billing');
  const [priority, setPriority] = useState('medium');
  const [description, setDescription] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');

  const submit = async () => {
    if (!subject.trim()) return;
    setBusy(true);
    setError('');
    try {
      await createTicket({ subject: subject.trim(), category, priority, description: description.trim() || undefined });
      onCreated();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create ticket');
    } finally { setBusy(false); }
  };

  return (
    <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] p-6">
      <div className="mb-4 flex items-center gap-2">
        <button onClick={onCancel} className="rounded-lg p-1 text-dark-400 hover:text-white"><ArrowLeft className="h-4 w-4" /></button>
        <h2 className="text-lg font-semibold text-white">New support ticket</h2>
      </div>

      <div className="space-y-4">
        <div>
          <label className="mb-1.5 block text-xs font-medium text-dark-300">Subject</label>
          <input value={subject} onChange={(e) => setSubject(e.target.value)} placeholder="Brief description of your issue"
            className="w-full rounded-xl border border-white/10 bg-dark-950/60 px-3.5 py-2.5 text-sm text-white placeholder-dark-500 focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30" />
        </div>
        <div className="grid gap-4 sm:grid-cols-2">
          <div>
            <label className="mb-1.5 block text-xs font-medium text-dark-300">Category</label>
            <select value={category} onChange={(e) => setCategory(e.target.value)}
              className="w-full rounded-xl border border-white/10 bg-dark-950/60 px-3.5 py-2.5 text-sm text-white focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30">
              <option value="billing">Billing</option>
              <option value="connection">Connection</option>
              <option value="network">Network</option>
              <option value="hardware">Hardware</option>
              <option value="complaint">Complaint</option>
              <option value="other">Other</option>
            </select>
          </div>
          <div>
            <label className="mb-1.5 block text-xs font-medium text-dark-300">Priority</label>
            <select value={priority} onChange={(e) => setPriority(e.target.value)}
              className="w-full rounded-xl border border-white/10 bg-dark-950/60 px-3.5 py-2.5 text-sm text-white focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30">
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
              <option value="critical">Critical</option>
            </select>
          </div>
        </div>
        <div>
          <label className="mb-1.5 block text-xs font-medium text-dark-300">Description</label>
          <textarea value={description} onChange={(e) => setDescription(e.target.value)} rows={4} placeholder="Describe your issue in detail…"
            className="w-full rounded-xl border border-white/10 bg-dark-950/60 px-3.5 py-2.5 text-sm text-white placeholder-dark-500 focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30 resize-y" />
        </div>
        {error && <p className="rounded-lg border border-red-500/30 bg-red-500/10 px-3 py-2 text-xs text-red-300">{error}</p>}
        <button onClick={void submit} disabled={busy || !subject.trim()} className="rounded-xl bg-primary-600 px-5 py-2.5 text-sm font-medium text-white transition-colors hover:bg-primary-500 disabled:opacity-50">
          {busy ? 'Submitting…' : 'Submit ticket'}
        </button>
      </div>
    </div>
  );
}

function TicketDetail({ ticket, onBack }: { ticket: CustomerTicket; onBack: () => void }) {
  const [comments, setComments] = useState<CustomerTicketComment[]>([]);
  const [loadingComments, setLoadingComments] = useState(true);
  const [newComment, setNewComment] = useState('');
  const [posting, setPosting] = useState(false);

  useEffect(() => {
    void getTicketComments(ticket.id)
      .then((data) => setComments(data as unknown as CustomerTicketComment[]))
      .catch(() => {})
      .finally(() => setLoadingComments(false));
  }, [ticket.id]);

  const postComment = async () => {
    if (!newComment.trim()) return;
    setPosting(true);
    try {
      await addTicketComment(ticket.id, newComment.trim());
      setNewComment('');
      const data = await getTicketComments(ticket.id);
      setComments(data as unknown as CustomerTicketComment[]);
    } catch { /* ignore */ }
    finally { setPosting(false); }
  };

  return (
    <div className="space-y-4">
      <button onClick={onBack} className="flex items-center gap-1.5 text-sm text-dark-400 hover:text-white">
        <ArrowLeft className="h-4 w-4" /> Back to tickets
      </button>

      <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] p-5">
        <div className="flex items-center gap-3">
          <h2 className="text-lg font-semibold text-white">{ticket.subject}</h2>
          <span className={`rounded-full px-2.5 py-0.5 text-xs font-medium ${
            ticket.status === 'open' ? 'bg-blue-500/10 text-blue-400' :
            ticket.status === 'resolved' ? 'bg-emerald-500/10 text-emerald-400' :
            'bg-dark-700 text-dark-400'
          }`}>{ticket.status}</span>
        </div>
        <p className="mt-1 text-xs text-dark-500">{ticket.ticket_number ?? `#${ticket.id}`} · {ticket.category ?? 'General'} · {ticket.priority}</p>
      </div>

      {/* Comments */}
      <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] p-5">
        <h3 className="mb-3 text-sm font-semibold text-white">Conversation</h3>
        {loadingComments ? (
          <p className="py-4 text-center text-sm text-dark-500">Loading…</p>
        ) : comments.length === 0 ? (
          <p className="py-4 text-center text-sm text-dark-500">No messages yet.</p>
        ) : (
          <div className="space-y-3 max-h-96 overflow-y-auto">
            {comments.map((c) => (
              <div key={c.id} className={`rounded-lg px-4 py-3 text-sm ${c.is_internal ? 'border border-amber-500/20 bg-amber-500/5' : 'border border-white/[0.04] bg-white/[0.02]'}`}>
                <div className="flex items-center justify-between">
                  <span className="text-xs font-medium text-white">{c.user_name ?? 'Support'}</span>
                  <span className="text-[11px] text-dark-500">{timeAgo(c.created_at)}</span>
                </div>
                <p className="mt-1 text-dark-300 whitespace-pre-wrap">{c.content}</p>
              </div>
            ))}
          </div>
        )}

        {ticket.status !== 'closed' && (
          <div className="mt-4 flex gap-2">
            <input value={newComment} onChange={(e) => setNewComment(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); void postComment(); } }}
              placeholder="Type a reply…"
              className="flex-1 rounded-xl border border-white/10 bg-dark-950/60 px-3.5 py-2.5 text-sm text-white placeholder-dark-500 focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30" />
            <button onClick={void postComment} disabled={posting || !newComment.trim()}
              className="rounded-xl bg-primary-600 px-4 py-2.5 text-sm font-medium text-white transition-colors hover:bg-primary-500 disabled:opacity-50">
              <Send className="h-4 w-4" />
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
