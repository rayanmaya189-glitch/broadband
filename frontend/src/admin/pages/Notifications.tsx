import { useEffect, useState } from 'react';
import { Bell, Plus, RotateCw, Send } from 'lucide-react';
import { listNotifications, retryNotification, sendNotification } from '../../api/admin/modules';
import type { Notification } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, TextArea, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

export function NotificationsPage() {
  const [data, setData] = useState<Notification[]>([]);
  const [loading, setLoading] = useState(true);
  const [sendOpen, setSendOpen] = useState(false);
  const [busyId, setBusyId] = useState<number | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listNotifications());
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

  const columns: Column<Notification>[] = [
    { key: 'channel', header: 'Channel', render: (n) => <StatusBadge status={n.channel} /> },
    { key: 'recipient_name', header: 'Recipient', render: (n) => (
      <div>
        <p className="text-dark-200">{n.recipient_name ?? 'â€”'}</p>
        <p className="text-xs text-dark-500">{n.recipient ?? ''}</p>
      </div>
    )},
    { key: 'subject', header: 'Subject', render: (n) => (
      <div className="max-w-md">
        <p className="truncate text-dark-200">{n.subject ?? 'â€”'}</p>
        {n.body && <p className="truncate text-xs text-dark-500">{n.body}</p>}
      </div>
    )},
    { key: 'status', header: 'Status', render: (n) => <StatusBadge status={n.status} /> },
    { key: 'created_at', header: 'Sent at', render: (n) => <span className="text-xs text-dark-500">{formatDateTime(n.created_at)}</span> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
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

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Notifications"
          subtitle="Outbound notifications and delivery history."
          icon={<Bell className="h-5 w-5" />}
          actions={<Button onClick={() => setSendOpen(true)}><Plus className="h-4 w-4" /> Send notification</Button>}
        />

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={data}
            rowKey={(n) => n.id}
            emptyTitle="No notifications sent"
            emptyDescription="Outbound notifications will appear here."
          />
        )}

        <SendNotificationModal open={sendOpen} onClose={() => setSendOpen(false)} onSent={() => { setSendOpen(false); void load(); }} />
      </div>
    </PageTransition>
  );
}

function SendNotificationModal({ open, onClose, onSent }: { open: boolean; onClose: () => void; onSent: () => void }) {
  const [channel, setChannel] = useState('sms');
  const [recipient, setRecipient] = useState('');
  const [subject, setSubject] = useState('');
  const [body, setBody] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!body.trim()) return;
    setBusy(true);
    try {
      await sendNotification({
        channel,
        recipient: recipient.trim() || undefined,
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
        <TextField label="Recipient" value={recipient} onChange={(e) => setRecipient(e.target.value)} hint="Phone or email depending on channel." />
        <TextField label="Subject" value={subject} onChange={(e) => setSubject(e.target.value)} />
        <TextArea label="Body" required value={body} onChange={(e) => setBody(e.target.value)} />
      </form>
    </Modal>
  );
}
