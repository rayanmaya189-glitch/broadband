import { useEffect, useMemo, useState } from 'react';
import { CreditCard, Plus, Send, XCircle, Wallet, Tag, RotateCw, Zap } from 'lucide-react';
import {
  createInvoice,
  listInvoices,
  listPayments,
  listOverdueInvoices,
  listDiscounts,
  createDiscount,
  recordPayment,
  sendInvoice,
  voidInvoice,
  autoGenerateInvoices,
  requestRefund,
  getInvoice,
  listInvoiceItems,
  addInvoiceItem,
  removeInvoiceItem,
  listCustomers,
  listSubscriptions,
} from '../../api/admin/modules';
import type { Invoice, InvoiceItem, Payment, Discount, Subscription } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField, TextArea } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency, formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function BillingPage() {
  const [tab, setTab] = useState<'invoices' | 'payments' | 'overdue' | 'discounts'>('invoices');
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [payments, setPayments] = useState<Payment[]>([]);
  const [overdue, setOverdue] = useState<Invoice[]>([]);
  const [discounts, setDiscounts] = useState<Discount[]>([]);
  const [loading, setLoading] = useState(true);
  const [invoiceOpen, setInvoiceOpen] = useState(false);
  const [paymentOpen, setPaymentOpen] = useState(false);
  const [discountOpen, setDiscountOpen] = useState(false);
  const [refundOpen, setRefundOpen] = useState<Payment | null>(null);
  const [detailTarget, setDetailTarget] = useState<Invoice | null>(null);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);
  const [generating, setGenerating] = useState(false);

  const loadAll = async () => {
    setLoading(true);
    try {
      const [inv, pay, ovd, disc] = await Promise.all([listInvoices(), listPayments(), listOverdueInvoices(), listDiscounts()]);
      setInvoices(inv);
      setPayments(pay);
      setOverdue(ovd);
      setDiscounts(disc);
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

  const handleAutoGenerate = async () => {
    setGenerating(true);
    try {
      await autoGenerateInvoices();
      toast('Invoices auto-generated');
      void loadAll();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Auto-generate failed', 'error');
    } finally {
      setGenerating(false);
    }
  };

  const invoiceColumns: Column<Invoice>[] = [
    { key: 'invoice_number', header: 'Invoice', render: (i) => (
      <div>
        <p className="font-medium text-white">{i.invoice_number ?? `#${i.id}`}</p>
        <p className="text-xs text-dark-500">{i.customer_name ?? '—'}</p>
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
          <Button variant="ghost" size="sm" onClick={() => setDetailTarget(i)} title="View details">
            Details
          </Button>
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
    { key: 'customer_name', header: 'Customer', render: (p) => <span className="text-dark-200">{p.customer_name ?? '—'}</span> },
    { key: 'amount', header: 'Amount', align: 'right', render: (p) => <span className="tabular-nums text-dark-200">{formatCurrency(p.amount)}</span> },
    { key: 'method', header: 'Method', render: (p) => <span className="text-dark-300">{p.payment_method ?? p.method ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (p) => <StatusBadge status={p.status ?? 'unknown'} /> },
    { key: 'paid_at', header: 'Paid at', render: (p) => <span className="text-xs text-dark-500">{formatDateTime(p.paid_at)}</span> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (p) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          {p.status === 'completed' && (
            <Button variant="ghost" size="sm" onClick={() => setRefundOpen(p)}>
              <RotateCw className="h-4 w-4" /> Refund
            </Button>
          )}
        </div>
      ),
    },
  ];

  const overdueColumns: Column<Invoice>[] = [
    { key: 'invoice_number', header: 'Invoice', render: (i) => <span className="font-medium text-white">{i.invoice_number ?? `#${i.id}`}</span> },
    { key: 'customer_name', header: 'Customer', render: (i) => <span className="text-dark-300">{i.customer_name ?? '—'}</span> },
    { key: 'amount', header: 'Amount', align: 'right', render: (i) => <span className="tabular-nums text-red-300">{formatCurrency(i.total_amount ?? i.amount)}</span> },
    { key: 'due_date', header: 'Due', render: (i) => <span className="text-xs text-dark-500">{formatDateTime(i.due_date)}</span> },
  ];

  const discountColumns: Column<Discount>[] = [
    { key: 'name', header: 'Discount', render: (d) => <span className="font-medium text-white">{d.name}</span> },
    { key: 'type', header: 'Type', render: (d) => <StatusBadge status={d.type} /> },
    { key: 'value', header: 'Value', align: 'right', render: (d) => (
      <span className="tabular-nums text-dark-200">{d.type === 'percentage' ? `${d.value}%` : formatCurrency(d.value)}</span>
    )},
    { key: 'valid_until', header: 'Valid until', render: (d) => <span className="text-xs text-dark-500">{formatDateTime(d.valid_until)}</span> },
    { key: 'is_active', header: 'Status', render: (d) => <StatusBadge status={d.is_active !== false ? 'active' : 'inactive'} /> },
  ];

  const tabs = useMemo(
    () => [
      { key: 'invoices' as const, label: `Invoices (${invoices.length})` },
      { key: 'payments' as const, label: `Payments (${payments.length})` },
      { key: 'overdue' as const, label: `Overdue (${overdue.length})` },
      { key: 'discounts' as const, label: `Discounts (${discounts.length})` },
    ],
    [invoices.length, payments.length, overdue.length, discounts.length]
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
              <Button variant="secondary" loading={generating} onClick={handleAutoGenerate}>
                <Zap className="h-4 w-4" /> Auto-generate
              </Button>
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
          <DataTable columns={invoiceColumns} data={invoices} rowKey={(i) => i.id}
            onRowClick={(i) => setDetailTarget(i)}
            emptyTitle="No invoices" emptyDescription="Invoices will appear here." />
        ) : tab === 'payments' ? (
          <DataTable columns={paymentColumns} data={payments} rowKey={(p) => p.id} emptyTitle="No payments" emptyDescription="Recorded payments will appear here." />
        ) : tab === 'overdue' ? (
          <DataTable columns={overdueColumns} data={overdue} rowKey={(i) => i.id} emptyTitle="No overdue invoices" emptyDescription="You are all caught up." />
        ) : (
          <DataTable columns={discountColumns} data={discounts} rowKey={(d) => d.id} emptyTitle="No discounts" emptyDescription="Create discounts to apply to invoices." />
        )}

        <CreateInvoiceModal open={invoiceOpen} onClose={() => setInvoiceOpen(false)} onCreated={() => { setInvoiceOpen(false); void loadAll(); }} />
        <RecordPaymentModal open={paymentOpen} onClose={() => setPaymentOpen(false)} onCreated={() => { setPaymentOpen(false); void loadAll(); }} />
        <CreateDiscountModal open={discountOpen} onClose={() => setDiscountOpen(false)} onCreated={() => { setDiscountOpen(false); void loadAll(); }} />

        {refundOpen && (
          <RefundModal payment={refundOpen} open={refundOpen !== null} onClose={() => setRefundOpen(null)} onDone={() => { setRefundOpen(null); void loadAll(); }} />
        )}

        {detailTarget && (
          <InvoiceDetailModal invoice={detailTarget} open={detailTarget !== null} onClose={() => setDetailTarget(null)} />
        )}
      </div>
    </PageTransition>
  );
}

interface CustomerOption { id: number; name: string; phone?: string; customer_code?: string }

function CreateInvoiceModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [customers, setCustomers] = useState<CustomerOption[]>([]);
  const [subscriptions, setSubscriptions] = useState<Subscription[]>([]);
  const [customerId, setCustomerId] = useState('');
  const [subscriptionId, setSubscriptionId] = useState('');
  const [periodStart, setPeriodStart] = useState('');
  const [periodEnd, setPeriodEnd] = useState('');
  const [amount, setAmount] = useState('');
  const [busy, setBusy] = useState(false);
  const [loadingOpts, setLoadingOpts] = useState(true);

  useEffect(() => {
    if (!open) return;
    setLoadingOpts(true);
    const now = new Date();
    const first = new Date(now.getFullYear(), now.getMonth(), 1);
    const last = new Date(now.getFullYear(), now.getMonth() + 1, 0);
    setPeriodStart(first.toISOString().slice(0, 10));
    setPeriodEnd(last.toISOString().slice(0, 10));
    void Promise.all([
      listCustomers({ page: 1, page_size: 200 }).then((r) =>
        setCustomers(r.customers.map((c) => ({ id: c.id, name: c.name, phone: c.phone, customer_code: c.customer_code })))
      ).catch(() => undefined),
      listSubscriptions().then(setSubscriptions).catch(() => undefined),
    ]).finally(() => setLoadingOpts(false));
  }, [open]);

  const eligibleSubs = subscriptions.filter((s) => !customerId || s.customer_id === Number(customerId));

  const submit = async () => {
    if (!subscriptionId || !amount || !periodStart || !periodEnd) return;
    setBusy(true);
    try {
      await createInvoice({
        subscription_id: Number(subscriptionId),
        billing_period_start: periodStart,
        billing_period_end: periodEnd,
        total_amount: Number(amount),
      });
      toast('Invoice created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create invoice', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New invoice" subtitle="Generate an invoice for a subscription." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create invoice</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        {loadingOpts ? (
          <Spinner label="Loading options…" />
        ) : (
          <>
            <SelectField label="Customer" value={customerId} onChange={(e) => { setCustomerId(e.target.value); setSubscriptionId(''); }}
              options={customers.map((c) => ({ value: String(c.id), label: `${c.name} (${c.customer_code ?? `#${c.id}`})` }))}
              placeholder="Filter by customer (optional)" />
            <SelectField label="Subscription" required value={subscriptionId} onChange={(e) => setSubscriptionId(e.target.value)}
              options={eligibleSubs.map((s) => ({ value: String(s.id), label: `${s.plan_name ?? `Plan #${s.plan_id}`} — ${s.customer_name ?? `Customer #${s.customer_id}`} (#${s.id})` }))}
              placeholder="Select a subscription" />
            <div className="grid gap-4 sm:grid-cols-2">
              <TextField label="Billing period start" type="date" required value={periodStart} onChange={(e) => setPeriodStart(e.target.value)} />
              <TextField label="Billing period end" type="date" required value={periodEnd} onChange={(e) => setPeriodEnd(e.target.value)} />
            </div>
          </>
        )}
        <TextField label="Amount (₹)" type="number" required value={amount} onChange={(e) => setAmount(e.target.value)} />
      </form>
    </Modal>
  );
}

function RecordPaymentModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [invoiceId, setInvoiceId] = useState('');
  const [amount, setAmount] = useState('');
  const [paymentMethod, setPaymentMethod] = useState('cash');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!invoiceId || !amount) return;
    setBusy(true);
    try {
      await recordPayment({ invoice_id: Number(invoiceId), amount: Number(amount), payment_method: paymentMethod });
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
        <TextField label="Amount (₹)" type="number" required value={amount} onChange={(e) => setAmount(e.target.value)} />
        <SelectField label="Method" value={paymentMethod} onChange={(e) => setPaymentMethod(e.target.value)}
          options={[
            { value: 'cash', label: 'Cash' },
            { value: 'upi', label: 'UPI' },
            { value: 'card', label: 'Card' },
            { value: 'bank_transfer', label: 'Bank transfer' },
            { value: 'wallet', label: 'Wallet' },
            { value: 'online', label: 'Online' },
            { value: 'other', label: 'Other' },
          ]}
        />
      </form>
    </Modal>
  );
}

function CreateDiscountModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [type, setType] = useState('percentage');
  const [value, setValue] = useState('');
  const [validUntil, setValidUntil] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim() || !value) return;
    setBusy(true);
    try {
      await createDiscount({ name: name.trim(), type, value: Number(value), valid_until: validUntil || undefined, is_active: true });
      toast('Discount created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create discount', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New discount" subtitle="Create a discount to apply to invoices." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create discount</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Discount name" required value={name} onChange={(e) => setName(e.target.value)} />
        <SelectField label="Type" value={type} onChange={(e) => setType(e.target.value)}
          options={[{ value: 'percentage', label: 'Percentage (%)' }, { value: 'flat', label: 'Flat amount (₹)' }]} />
        <TextField label={type === 'percentage' ? 'Percentage' : 'Amount (₹)'} type="number" required value={value} onChange={(e) => setValue(e.target.value)} />
        <TextField label="Valid until" type="date" value={validUntil} onChange={(e) => setValidUntil(e.target.value)} />
      </form>
    </Modal>
  );
}

function RefundModal({ payment, open, onClose, onDone }: { payment: Payment; open: boolean; onClose: () => void; onDone: () => void }) {
  const [amount, setAmount] = useState(String(payment.amount));
  const [reason, setReason] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!payment.id || !amount) return;
    setBusy(true);
    try {
      await requestRefund({ payment_id: payment.id, amount: Number(amount), reason: reason.trim() || undefined });
      toast('Refund requested');
      onDone();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Refund failed', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Request refund" subtitle={`Payment #${payment.id} — ${formatCurrency(payment.amount)}`} width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button variant="danger" onClick={submit} loading={busy}>Request refund</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Refund amount (₹)" type="number" required value={amount} onChange={(e) => setAmount(e.target.value)} />
        <TextArea label="Reason" value={reason} onChange={(e) => setReason(e.target.value)} placeholder="Reason for refund…" />
      </form>
    </Modal>
  );
}

function InvoiceDetailModal({ invoice, open, onClose }: { invoice: Invoice; open: boolean; onClose: () => void }) {
  const [items, setItems] = useState<InvoiceItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [addItemOpen, setAddItemOpen] = useState(false);
  const [desc, setDesc] = useState('');
  const [qty, setQty] = useState('1');
  const [unitPrice, setUnitPrice] = useState('');
  const [adding, setAdding] = useState(false);

  const loadItems = async () => {
    setLoading(true);
    try {
      setItems(await listInvoiceItems(invoice.id));
    } catch {
      setItems([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (open) void loadItems();
  }, [open]);

  const handleAddItem = async () => {
    if (!desc.trim() || !unitPrice) return;
    setAdding(true);
    try {
      await addInvoiceItem(invoice.id, { description: desc.trim(), quantity: Number(qty) || 1, unit_price: Number(unitPrice), amount: (Number(qty) || 1) * Number(unitPrice) });
      toast('Item added');
      setDesc(''); setQty('1'); setUnitPrice(''); setAddItemOpen(false);
      void loadItems();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to add item', 'error');
    } finally {
      setAdding(false);
    }
  };

  const handleRemoveItem = async (itemId: number) => {
    try {
      await removeInvoiceItem(invoice.id, itemId);
      toast('Item removed');
      void loadItems();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to remove item', 'error');
    }
  };

  return (
    <Modal open={open} onClose={onClose} title={`Invoice ${invoice.invoice_number ?? `#${invoice.id}`}`} subtitle={invoice.customer_name} width="lg">
      <div className="space-y-4">
        <div className="flex flex-wrap items-center gap-3">
          <StatusBadge status={invoice.status} />
          <span className="tabular-nums text-lg font-semibold text-white">{formatCurrency(invoice.total_amount ?? invoice.amount)}</span>
          <span className="text-xs text-dark-500">Due: {formatDateTime(invoice.due_date)}</span>
        </div>

        <div>
          <div className="mb-2 flex items-center justify-between">
            <h3 className="text-sm font-semibold text-white">Line items</h3>
            <Button variant="secondary" size="sm" onClick={() => setAddItemOpen(!addItemOpen)}>
              <Plus className="h-3.5 w-3.5" /> Add item
            </Button>
          </div>

          {addItemOpen && (
            <div className="mb-3 rounded-xl border border-white/[0.08] bg-dark-950/40 p-3">
              <div className="grid gap-3 sm:grid-cols-4">
                <div className="sm:col-span-2">
                  <TextField label="Description" value={desc} onChange={(e) => setDesc(e.target.value)} />
                </div>
                <TextField label="Qty" type="number" value={qty} onChange={(e) => setQty(e.target.value)} />
                <TextField label="Unit price (₹)" type="number" value={unitPrice} onChange={(e) => setUnitPrice(e.target.value)} />
              </div>
              <div className="mt-2 flex justify-end gap-2">
                <Button variant="ghost" size="sm" onClick={() => setAddItemOpen(false)}>Cancel</Button>
                <Button size="sm" onClick={handleAddItem} loading={adding}>Add</Button>
              </div>
            </div>
          )}

          {loading ? (
            <Spinner />
          ) : items.length === 0 ? (
            <p className="py-4 text-center text-sm text-dark-500">No line items.</p>
          ) : (
            <div className="space-y-1">
              {items.map((item) => (
                <div key={item.id} className="flex items-center justify-between gap-3 rounded-lg border border-white/[0.04] px-3 py-2 text-sm">
                  <div className="min-w-0 flex-1">
                    <p className="text-dark-200">{item.description}</p>
                    <p className="text-xs text-dark-500">Qty: {item.quantity} × {formatCurrency(item.unit_price)}</p>
                  </div>
                  <span className="tabular-nums text-white">{formatCurrency(item.amount)}</span>
                  <Button variant="ghost" size="sm" onClick={() => void handleRemoveItem(item.id)}>
                    <XCircle className="h-3.5 w-3.5 text-red-400" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </Modal>
  );
}
