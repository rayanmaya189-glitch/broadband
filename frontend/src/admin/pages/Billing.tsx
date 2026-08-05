import { useEffect, useMemo, useState } from 'react';
import { CreditCard, Plus, Send, XCircle, Wallet } from 'lucide-react';
import {
  createInvoice,
  listInvoices,
  listPayments,
  listOverdueInvoices,
  recordPayment,
  sendInvoice,
  voidInvoice,
} from '../../api/admin/modules';
import type { Invoice, Payment } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency, formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function BillingPage() {
  const [tab, setTab] = useState<'invoices' | 'payments' | 'overdue'>('invoices');
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [payments, setPayments] = useState<Payment[]>([]);
  const [overdue, setOverdue] = useState<Invoice[]>([]);
  const [loading, setLoading] = useState(true);
  const [invoiceOpen, setInvoiceOpen] = useState(false);
  const [paymentOpen, setPaymentOpen] = useState(false);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);

  const loadAll = async () => {
    setLoading(true);
    try {
      const [inv, pay, ovd] = await Promise.all([listInvoices(), listPayments(), listOverdueInvoices()]);
      setInvoices(inv);
      setPayments(pay);
      setOverdue(ovd);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load billing data', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void loadAll();
  }, []);

  const act = async (action: string, id: number, fn: () => Promise<unknown>, doneMsg: string) => {
    setActing({ id, action });
    try {
      await fn();
      toast(doneMsg);
      void loadAll();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Action failed', 'error');
    } finally {
      setActing(null);
    }
  };

  const invoiceColumns: Column<Invoice>[] = [
    { key: 'invoice_number', header: 'Invoice', render: (i) => (
      <div>
        <p className="font-medium text-white">{i.invoice_number ?? `#${i.id}`}</p>
        <p className="text-xs text-dark-500">{i.customer_name ?? 'â€”'}</p>
      </div>
    )},
    { key: 'amount', header: 'Amount', align: 'right', render: (i) => (
      <span className="tabular-nums text-dark-200">{formatCurrency(i.total_amount ?? i.amount)}</span>
    )},
    { key: 'due_date', header: 'Due', render: (i) => <span className="text-xs text-dark-500">{formatDateTime(i.due_date)}</span> },
    { key: 'status', header: 'Status', render: (i) => <StatusBadge status={i.status} /> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (i) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button
            variant="ghost"
            size="sm"
            loading={acting?.id === i.id && acting.action === 'send'}
            disabled={acting !== null}
            onClick={() => void act('send', i.id, () => sendInvoice(i.id), 'Invoice sent')}
            title="Send invoice"
          >
            <Send className="h-4 w-4" />
          </Button>
          {i.status !== 'void' && i.status !== 'paid' && (
            <Button
              variant="ghost"
              size="sm"
              loading={acting?.id === i.id && acting.action === 'void'}
              disabled={acting !== null}
              onClick={() => void act('void', i.id, () => voidInvoice(i.id), 'Invoice voided')}
              title="Void invoice"
            >
              <XCircle className="h-4 w-4 text-red-400" />
            </Button>
          )}
        </div>
      ),
    },
  ];

  const paymentColumns: Column<Payment>[] = [
    { key: 'customer_name', header: 'Customer', render: (p) => <span className="text-dark-200">{p.customer_name ?? 'â€”'}</span> },
    { key: 'amount', header: 'Amount', align: 'right', render: (p) => <span className="tabular-nums text-dark-200">{formatCurrency(p.amount)}</span> },
    { key: 'method', header: 'Method', render: (p) => <span className="text-dark-300">{p.method ?? 'â€”'}</span> },
    { key: 'status', header: 'Status', render: (p) => <StatusBadge status={p.status} /> },
    { key: 'paid_at', header: 'Paid at', render: (p) => <span className="text-xs text-dark-500">{formatDateTime(p.paid_at)}</span> },
  ];

  const overdueColumns: Column<Invoice>[] = [
    { key: 'invoice_number', header: 'Invoice', render: (i) => <span className="font-medium text-white">{i.invoice_number ?? `#${i.id}`}</span> },
    { key: 'customer_name', header: 'Customer', render: (i) => <span className="text-dark-300">{i.customer_name ?? 'â€”'}</span> },
    { key: 'amount', header: 'Amount', align: 'right', render: (i) => <span className="tabular-nums text-red-300">{formatCurrency(i.total_amount ?? i.amount)}</span> },
    { key: 'due_date', header: 'Due', render: (i) => <span className="text-xs text-dark-500">{formatDateTime(i.due_date)}</span> },
  ];

  const tabs = useMemo(
    () => [
      { key: 'invoices' as const, label: `Invoices (${invoices.length})` },
      { key: 'payments' as const, label: `Payments (${payments.length})` },
      { key: 'overdue' as const, label: `Overdue (${overdue.length})` },
    ],
    [invoices.length, payments.length, overdue.length]
  );

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Billing"
          subtitle="Invoices, payments and collections."
          icon={<CreditCard className="h-5 w-5" />}
          actions={
            <>
              <Button variant="secondary" onClick={() => setPaymentOpen(true)}>
                <Wallet className="h-4 w-4" /> Record payment
              </Button>
              <Button onClick={() => setInvoiceOpen(true)}>
                <Plus className="h-4 w-4" /> New invoice
              </Button>
            </>
          }
        />

        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {tabs.map((t) => (
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
        ) : tab === 'invoices' ? (
          <DataTable columns={invoiceColumns} data={invoices} rowKey={(i) => i.id} emptyTitle="No invoices" emptyDescription="Invoices will appear here." />
        ) : tab === 'payments' ? (
          <DataTable columns={paymentColumns} data={payments} rowKey={(p) => p.id} emptyTitle="No payments" emptyDescription="Recorded payments will appear here." />
        ) : (
          <DataTable columns={overdueColumns} data={overdue} rowKey={(i) => i.id} emptyTitle="No overdue invoices" emptyDescription="You are all caught up." />
        )}

        <CreateInvoiceModal open={invoiceOpen} onClose={() => setInvoiceOpen(false)} onCreated={() => { setInvoiceOpen(false); void loadAll(); }} />
        <RecordPaymentModal open={paymentOpen} onClose={() => setPaymentOpen(false)} onCreated={() => { setPaymentOpen(false); void loadAll(); }} />
      </div>
    </PageTransition>
  );
}

function CreateInvoiceModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [customerId, setCustomerId] = useState('');
  const [amount, setAmount] = useState('');
  const [dueDate, setDueDate] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!customerId || !amount) return;
    setBusy(true);
    try {
      await createInvoice({ customer_id: Number(customerId), amount: Number(amount), due_date: dueDate || undefined });
      toast('Invoice created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create invoice', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New invoice" subtitle="Generate an invoice for a customer." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create invoice</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Customer ID" type="number" required value={customerId} onChange={(e) => setCustomerId(e.target.value)} />
        <TextField label="Amount (â‚¹)" type="number" required value={amount} onChange={(e) => setAmount(e.target.value)} />
        <TextField label="Due date" type="date" value={dueDate} onChange={(e) => setDueDate(e.target.value)} />
      </form>
    </Modal>
  );
}

function RecordPaymentModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [invoiceId, setInvoiceId] = useState('');
  const [amount, setAmount] = useState('');
  const [method, setMethod] = useState('cash');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!invoiceId || !amount) return;
    setBusy(true);
    try {
      await recordPayment({ invoice_id: Number(invoiceId), amount: Number(amount), method });
      toast('Payment recorded');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to record payment', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Record payment" subtitle="Log a payment against an invoice." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Record payment</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Invoice ID" type="number" required value={invoiceId} onChange={(e) => setInvoiceId(e.target.value)} />
        <TextField label="Amount (â‚¹)" type="number" required value={amount} onChange={(e) => setAmount(e.target.value)} />
        <SelectField label="Method" value={method} onChange={(e) => setMethod(e.target.value)}
          options={[
            { value: 'cash', label: 'Cash' },
            { value: 'upi', label: 'UPI' },
            { value: 'card', label: 'Card' },
            { value: 'bank_transfer', label: 'Bank transfer' },
            { value: 'wallet', label: 'Wallet' },
            { value: 'other', label: 'Other' },
          ]}
        />
      </form>
    </Modal>
  );
}
