import { useEffect, useState } from 'react';
import { Bell, Plus, RotateCw, Send, FileText, Trash2 } from 'lucide-react';
import {
  listNotifications,
  retryNotification,
  sendNotification,
  listNotificationTemplates,
  createNotificationTemplate,
  deleteNotificationTemplate,
} from '../../api/admin/modules';
import type { Notification, NotificationTemplate } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField, TextArea, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'history' | 'templates';

export function NotificationsPage() {
  const [data, setData] = useState<Notification[]>([]);
  const [templates, setTemplates] = useState<NotificationTemplate[]>([]);
  const [loading, setLoading] = useState(true);
  const [sendOpen, setSendOpen] = useState(false);
  const [templateOpen, setTemplateOpen] = useState(false);
  const [busyId, setBusyId] = useState<number | null>(null);
  const [tab, setTab] = useState<Tab>('history');
  const [deleteTarget, setDeleteTarget] = useState<NotificationTemplate | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [n, t] = await Promise.all([listNotifications(), listNotificationTemplates()]);
      setData(n);
      setTemplates(t);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load notifications', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const retry = async (n: Notification) => {
    setBusyId(n.id);
    try {
      await retryNotification(n.id);
      toast('Notification retried');
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Retry failed', 'error');
    } finally {
      setBusyId(null);
    }
  };

  const historyColumns: Column<Notification>[] = [
    { key: 'channel', header: 'Channel', render: (n) => <StatusBadge status={n.channel} /> },
    { key: 'recipient_name', header: 'Recipient', render: (n) => (
      <div>
        <p className="text-dark-200">{n.recipient_name ?? '—'}</p>
        <p className="text-xs text-dark-500">{n.recipient ?? ''}</p>
      </div>
    )},
    { key: 'subject', header: 'Subject', render: (n) => (
      <div className="max-w-md">
        <p className="truncate text-dark-200">{n.subject ?? '—'}</p>
        {n.body && <p className="truncate text-xs text-dark-500">{n.body}</p>}
      </div>
    )},
    { key: 'status', header: 'Status', render: (n) => <StatusBadge status={n.status} /> },
    { key: 'created_at', header: 'Sent at', render: (n) => <span className="text-xs text-dark-500">{formatDateTime(n.created_at)}</span> },
    {
      key: 'actions', header: 'Actions', align: 'right',
      render: (n) =>
        n.status === 'failed' ? (
          <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
            <Button variant="secondary" size="sm" loading={busyId === n.id} onClick={() => void retry(n)}>
              <RotateCw className="h-4 w-4" /> Retry
            </Button>
          </div>
        ) : (
          <span className="text-xs text-dark-600">{toLabel(n.status)}</span>
        ),
    },
  ];

  const templateColumns: Column<NotificationTemplate>[] = [
    { key: 'name', header: 'Template', render: (t) => <span className="font-medium text-white">{t.name}</span> },
    { key: 'channel', header: 'Channel', render: (t) => <StatusBadge status={t.channel} /> },
    { key: 'subject', header: 'Subject', render: (t) => <span className="text-dark-300">{t.subject_template ?? t.subject ?? '—'}</span> },
    { key: 'body', header: 'Body', render: (t) => <span className="max-w-md truncate text-dark-400">{t.body_template ?? t.body ?? '—'}</span> },
    { key: 'is_active', header: 'Status', render: (t) => <StatusBadge status={t.is_active !== false ? 'active' : 'inactive'} /> },
    {
      key: 'actions', header: '', align: 'right',
      render: (t) => (
        <div onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => setDeleteTarget(t)} title="Delete template">
            <Trash2 className="h-3.5 w-3.5 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Notifications"
          subtitle="Outbound notifications, templates and delivery history."
          icon={<Bell className="h-5 w-5" />}
          actions={
            <>
              <Button variant="secondary" onClick={() => setTemplateOpen(true)}>
                <FileText className="h-4 w-4" /> New template
              </Button>
              <Button onClick={() => setSendOpen(true)}>
                <Plus className="h-4 w-4" /> Send notification
              </Button>
            </>
          }
        />

        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {[
            { key: 'history' as const, label: `Delivery history (${data.length})` },
            { key: 'templates' as const, label: `Templates (${templates.length})` },
          ].map((t) => (
            <button
              key={t.key}
              onClick={() => setTab(t.key)}
              className={`rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors ${
                tab === t.key ? 'bg-primary-500/20 text-primary-200' : 'text-dark-400 hover:text-white'
              }`}
            >
              {t.label}
            </button>
          ))}
        </div>

        {loading ? (
          <Spinner />
        ) : tab === 'history' ? (
          <DataTable columns={historyColumns} data={data} rowKey={(n) => n.id} emptyTitle="No notifications sent" emptyDescription="Outbound notifications will appear here." />
        ) : (
          <DataTable columns={templateColumns} data={templates} rowKey={(t) => t.id} emptyTitle="No templates" emptyDescription="Create notification templates for common messages." />
        )}

        <SendNotificationModal open={sendOpen} onClose={() => setSendOpen(false)} onSent={() => { setSendOpen(false); void load(); }} />
        <CreateTemplateModal open={templateOpen} onClose={() => setTemplateOpen(false)} onCreated={() => { setTemplateOpen(false); void load(); }} />

        <ConfirmDialog
          open={deleteTarget !== null}
          onClose={() => setDeleteTarget(null)}
          onConfirm={async () => {
            if (!deleteTarget) return;
            try { await deleteNotificationTemplate(deleteTarget.id); toast('Template deleted'); setDeleteTarget(null); void load(); }
            catch (err) { toast(err instanceof Error ? err.message : 'Failed to delete template', 'error'); }
          }}
          title="Delete template?"
          description={`Permanently delete template "${deleteTarget?.name}".`}
          confirmLabel="Delete template"
          danger
        />
      </div>
    </PageTransition>
  );
}

function SendNotificationModal({ open, onClose, onSent }: { open: boolean; onClose: () => void; onSent: () => void }) {
  const [channel, setChannel] = useState('sms');
  const [recipientType, setRecipientType] = useState('customer');
  const [recipientId, setRecipientId] = useState('');
  const [recipientAddress, setRecipientAddress] = useState('');
  const [subject, setSubject] = useState('');
  const [body, setBody] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!body.trim() || !recipientAddress.trim() || !recipientId) return;
    setBusy(true);
    try {
      await sendNotification({
        channel,
        recipient_type: recipientType,
        recipient_id: Number(recipientId),
        recipient_address: recipientAddress.trim(),
        subject: subject.trim() || undefined,
        body: body.trim(),
      });
      toast('Notification sent');
      onSent();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to send notification', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Send notification" subtitle="Send a message through a configured channel." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}><Send className="h-4 w-4" /> Send</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <SelectField label="Channel" value={channel} onChange={(e) => setChannel(e.target.value)}
          options={[
            { value: 'sms', label: 'SMS' },
            { value: 'email', label: 'Email' },
            { value: 'whatsapp', label: 'WhatsApp' },
            { value: 'telegram', label: 'Telegram' },
            { value: 'push', label: 'Push' },
          ]}
        />
        <SelectField label="Recipient type" value={recipientType} onChange={(e) => setRecipientType(e.target.value)}
          options={[
            { value: 'customer', label: 'Customer' },
            { value: 'user', label: 'User / staff' },
          ]}
        />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Recipient ID" type="number" required value={recipientId} onChange={(e) => setRecipientId(e.target.value)} />
          <TextField label="Recipient address" required value={recipientAddress} onChange={(e) => setRecipientAddress(e.target.value)} hint="Phone or email depending on channel." />
        </div>
        <TextField label="Subject" value={subject} onChange={(e) => setSubject(e.target.value)} />
        <TextArea label="Body" required value={body} onChange={(e) => setBody(e.target.value)} />
      </form>
    </Modal>
  );
}

function CreateTemplateModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [channel, setChannel] = useState('email');
  const [subject, setSubject] = useState('');
  const [body, setBody] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim() || !body.trim()) return;
    setBusy(true);
    try {
      await createNotificationTemplate({
        name: name.trim(),
        channel,
        body_template: body.trim(),
        subject_template: subject.trim() || undefined,
      });
      toast('Template created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create template', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New notification template" subtitle="Create a reusable message template." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create template</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Template name" required value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. Welcome message" />
        <SelectField label="Channel" value={channel} onChange={(e) => setChannel(e.target.value)}
          options={[
            { value: 'email', label: 'Email' },
            { value: 'sms', label: 'SMS' },
            { value: 'whatsapp', label: 'WhatsApp' },
            { value: 'telegram', label: 'Telegram' },
            { value: 'push', label: 'Push' },
          ]}
        />
        <TextField label="Subject" value={subject} onChange={(e) => setSubject(e.target.value)} />
        <TextArea label="Body" required value={body} onChange={(e) => setBody(e.target.value)} placeholder="Use {{variable}} for template variables." />
      </form>
    </Modal>
  );
}
