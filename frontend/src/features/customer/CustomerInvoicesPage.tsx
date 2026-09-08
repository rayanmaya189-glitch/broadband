import { useEffect, useState } from 'react';
import { CreditCard, ExternalLink } from 'lucide-react';
import { getMyInvoices, createPaymentLink } from '../../api/customer';
import { formatCurrency, formatDate } from '../../utils/format';
import type { CustomerInvoice } from '../../types';

export default function CustomerInvoicesPage() {
  const [invoices, setInvoices] = useState<CustomerInvoice[]>([]);
  const [loading, setLoading] = useState(true);
  const [paying, setPaying] = useState<number | null>(null);

  useEffect(() => {
    void getMyInvoices()
      .then((data) => setInvoices(data as unknown as CustomerInvoice[]))
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  const handlePay = async (invoice: CustomerInvoice) => {
    setPaying(invoice.id);
    try {
      const { payment_url } = await createPaymentLink(invoice.id);
      window.open(payment_url, '_blank');
    } catch { /* ignore */ }
    finally { setPaying(null); }
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-white">Invoices</h1>
        <p className="mt-1 text-sm text-dark-400">View and pay your invoices.</p>
      </div>

      {loading ? (
        <div className="py-12 text-center text-sm text-dark-500">Loading invoices…</div>
      ) : invoices.length === 0 ? (
        <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] py-12 text-center">
          <CreditCard className="mx-auto mb-3 h-8 w-8 text-dark-500" />
          <p className="text-sm text-dark-400">No invoices yet.</p>
        </div>
      ) : (
        <div className="space-y-3">
          {invoices.map((inv) => (
            <div key={inv.id} className="flex items-center justify-between gap-4 rounded-xl border border-white/[0.06] bg-white/[0.02] px-5 py-4">
              <div className="min-w-0">
                <p className="font-medium text-white">{inv.invoice_number}</p>
                <div className="mt-0.5 flex items-center gap-3 text-xs text-dark-500">
                  <span>Issued: {formatDate(inv.issued_at)}</span>
                  <span>Due: {formatDate(inv.due_date)}</span>
                </div>
              </div>
              <div className="flex items-center gap-4">
                <div className="text-right">
                  <p className="tabular-nums text-lg font-semibold text-white">{formatCurrency(inv.total_amount ?? inv.amount)}</p>
                  <span className={`text-xs ${inv.status === 'paid' ? 'text-emerald-400' : inv.status === 'overdue' ? 'text-red-400' : 'text-dark-500'}`}>
                    {inv.status}
                  </span>
                </div>
                {inv.status !== 'paid' && inv.status !== 'void' && (
                  <button
                    onClick={() => void handlePay(inv)}
                    disabled={paying === inv.id}
                    className="flex items-center gap-1.5 rounded-lg bg-primary-600 px-3 py-1.5 text-xs font-medium text-white transition-colors hover:bg-primary-500 disabled:opacity-50"
                  >
                    {paying === inv.id ? 'Redirecting…' : <>Pay now <ExternalLink className="h-3 w-3" /></>}
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
