import { useEffect, useState } from 'react';
import { BookOpen, Landmark, NotebookTabs, Plus, FileText } from 'lucide-react';
import {
  getTrialBalance,
  listAccounts,
  listJournalEntries,
  createJournalEntry,
  postJournalEntry,
  voidJournalEntry,
  getProfitAndLoss,
  getBalanceSheet,
  getGstinReturn,
} from '../../api/admin/modules';
import type { AccountingAccount, JournalEntry, JournalEntryLine, ProfitAndLoss, BalanceSheet, GstinReturn } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField, TextArea } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency, formatDateTime, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'accounts' | 'journal' | 'trial' | 'pl' | 'bs' | 'gst';

export function AccountingPage() {
  const [tab, setTab] = useState<Tab>('accounts');
  const [accounts, setAccounts] = useState<AccountingAccount[]>([]);
  const [journal, setJournal] = useState<JournalEntry[]>([]);
  const [trial, setTrial] = useState<unknown | null>(null);
  const [pl, setPl] = useState<ProfitAndLoss | null>(null);
  const [bs, setBs] = useState<BalanceSheet | null>(null);
  const [gst, setGst] = useState<GstinReturn | null>(null);
  const [loading, setLoading] = useState(true);
  const [journalOpen, setJournalOpen] = useState(false);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [a, j, t] = await Promise.all([
        listAccounts(),
        listJournalEntries(),
        getTrialBalance().catch(() => null),
      ]);
      setAccounts(a);
      setJournal(j);
      setTrial(t);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load accounting data', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const loadTabData = async (target: Tab) => {
    if (target === 'pl' && !pl) {
      try { setPl(await getProfitAndLoss()); } catch { /* ignore */ }
    }
    if (target === 'bs' && !bs) {
      try { setBs(await getBalanceSheet()); } catch { /* ignore */ }
    }
    if (target === 'gst' && !gst) {
      try { setGst(await getGstinReturn('gstr1')); } catch { /* ignore */ }
    }
  };

  const handleTab = (t: Tab) => {
    setTab(t);
    void loadTabData(t);
  };

  const act = async (action: string, id: number, fn: () => Promise<unknown>, msg: string) => {
    setActing({ id, action });
    try {
      await fn();
      toast(msg);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Action failed', 'error');
    } finally {
      setActing(null);
    }
  };

  const accountColumns: Column<AccountingAccount>[] = [
    { key: 'code', header: 'Code', render: (a) => <span className="font-mono text-xs text-accent-300">{a.code ?? `#${a.id}`}</span> },
    { key: 'name', header: 'Account', render: (a) => <span className="font-medium text-white">{a.name}</span> },
    { key: 'account_type', header: 'Type', render: (a) => <StatusBadge status={a.account_type} /> },
    { key: 'balance', header: 'Balance', align: 'right', render: (a) => (
      <span className="tabular-nums text-dark-200">{formatCurrency(a.balance)}</span>
    )},
    { key: 'status', header: 'Status', render: (a) => <StatusBadge status={a.status ?? 'active'} /> },
  ];

  const journalColumns: Column<JournalEntry>[] = [
    { key: 'entry_number', header: 'Entry', render: (j) => <span className="font-medium text-white">{j.entry_number ?? `#${j.id}`}</span> },
    { key: 'description', header: 'Description', render: (j) => <span className="max-w-md truncate text-dark-300">{j.description ?? '—'}</span> },
    { key: 'entry_type', header: 'Type', render: (j) => <StatusBadge status={j.entry_type ?? 'unknown'} /> },
    { key: 'amount', header: 'Amount', align: 'right', render: (j) => <span className="tabular-nums text-dark-200">{formatCurrency(j.amount)}</span> },
    { key: 'status', header: 'Status', render: (j) => <StatusBadge status={j.status ?? 'draft'} /> },
    { key: 'entry_date', header: 'Date', render: (j) => <span className="text-xs text-dark-500">{formatDateTime(j.entry_date)}</span> },
    {
      key: 'actions', header: '', align: 'right',
      render: (j) => j.status === 'draft' ? (
        <div className="flex gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="success" size="sm" loading={acting?.id === j.id && acting.action === 'post'}
            onClick={() => void act('post', j.id, () => postJournalEntry(j.id), 'Entry posted')}>Post</Button>
          <Button variant="ghost" size="sm" loading={acting?.id === j.id && acting.action === 'void'}
            onClick={() => void act('void', j.id, () => voidJournalEntry(j.id), 'Entry voided')}>Void</Button>
        </div>
      ) : null,
    },
  ];

  const tabs: { key: Tab; label: string }[] = [
    { key: 'accounts', label: `Accounts (${accounts.length})` },
    { key: 'journal', label: `Journal (${journal.length})` },
    { key: 'trial', label: 'Trial balance' },
    { key: 'pl', label: 'P&L' },
    { key: 'bs', label: 'Balance sheet' },
    { key: 'gst', label: 'GST' },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Accounting"
          subtitle="Chart of accounts, journal entries and financial reports."
          icon={<BookOpen className="h-5 w-5" />}
          actions={
            <Button onClick={() => setJournalOpen(true)}>
              <Plus className="h-4 w-4" /> Journal entry
            </Button>
          }
        />

        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {tabs.map((t) => (
            <button
              key={t.key}
              onClick={() => handleTab(t.key)}
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
        ) : tab === 'accounts' ? (
          <DataTable columns={accountColumns} data={accounts} rowKey={(a) => a.id} emptyTitle="No accounts" emptyDescription="Chart of accounts will appear here." />
        ) : tab === 'journal' ? (
          <DataTable columns={journalColumns} data={journal} rowKey={(j) => j.id} emptyTitle="No journal entries" emptyDescription="Journal entries will appear here." />
        ) : tab === 'trial' ? (
          <div className="glass-card rounded-2xl p-6">
            {trial ? (
              <pre className="overflow-x-auto text-xs text-dark-400">{JSON.stringify(trial, null, 2)}</pre>
            ) : (
              <div className="flex flex-col items-center gap-3 py-12 text-dark-400">
                <Landmark className="h-8 w-8" />
                <p className="text-sm">Trial balance unavailable.</p>
              </div>
            )}
          </div>
        ) : tab === 'pl' ? (
          <div className="glass-card rounded-2xl p-6">
            {pl ? (
              <div className="space-y-4">
                <div className="flex items-center gap-2 text-xs text-dark-500">
                  <FileText className="h-3.5 w-3.5" />
                  Period: {formatDateTime(pl.period_start)} — {formatDateTime(pl.period_end)}
                </div>
                <div className="grid gap-6 sm:grid-cols-2">
                  <div>
                    <h3 className="mb-2 text-sm font-semibold text-emerald-400">Revenue</h3>
                    {pl.revenue.map((r, i) => (
                      <div key={i} className="flex justify-between border-b border-white/[0.04] py-1.5 text-sm">
                        <span className="text-dark-300">{r.account}</span>
                        <span className="tabular-nums text-white">{formatCurrency(r.amount)}</span>
                      </div>
                    ))}
                    <div className="mt-2 flex justify-between border-t border-white/10 pt-2 text-sm font-semibold">
                      <span className="text-dark-200">Total revenue</span>
                      <span className="tabular-nums text-emerald-400">{formatCurrency(pl.total_revenue)}</span>
                    </div>
                  </div>
                  <div>
                    <h3 className="mb-2 text-sm font-semibold text-red-400">Expenses</h3>
                    {pl.expenses.map((e, i) => (
                      <div key={i} className="flex justify-between border-b border-white/[0.04] py-1.5 text-sm">
                        <span className="text-dark-300">{e.account}</span>
                        <span className="tabular-nums text-white">{formatCurrency(e.amount)}</span>
                      </div>
                    ))}
                    <div className="mt-2 flex justify-between border-t border-white/10 pt-2 text-sm font-semibold">
                      <span className="text-dark-200">Total expenses</span>
                      <span className="tabular-nums text-red-400">{formatCurrency(pl.total_expenses)}</span>
                    </div>
                  </div>
                </div>
                <div className="flex justify-between rounded-xl border border-white/[0.08] bg-white/[0.03] p-4 text-lg font-semibold">
                  <span className="text-white">Net profit</span>
                  <span className={`tabular-nums ${pl.net_profit >= 0 ? 'text-emerald-400' : 'text-red-400'}`}>
                    {formatCurrency(pl.net_profit)}
                  </span>
                </div>
              </div>
            ) : (
              <div className="flex flex-col items-center gap-3 py-12 text-dark-400">
                <BookOpen className="h-8 w-8" />
                <p className="text-sm">P&L statement unavailable.</p>
              </div>
            )}
          </div>
        ) : tab === 'bs' ? (
          <div className="glass-card rounded-2xl p-6">
            {bs ? (
              <div className="space-y-4">
                <div className="grid gap-6 sm:grid-cols-3">
                  {[
                    { title: 'Assets', items: bs.assets, total: bs.total_assets, color: 'text-blue-400' },
                    { title: 'Liabilities', items: bs.liabilities, total: bs.total_liabilities, color: 'text-red-400' },
                    { title: 'Equity', items: bs.equity, total: bs.total_equity, color: 'text-emerald-400' },
                  ].map((section) => (
                    <div key={section.title}>
                      <h3 className={`mb-2 text-sm font-semibold ${section.color}`}>{section.title}</h3>
                      {section.items.map((item, i) => (
                        <div key={i} className="flex justify-between border-b border-white/[0.04] py-1.5 text-sm">
                          <span className="text-dark-300">{item.account}</span>
                          <span className="tabular-nums text-white">{formatCurrency(item.amount)}</span>
                        </div>
                      ))}
                      <div className="mt-2 flex justify-between border-t border-white/10 pt-2 text-sm font-semibold">
                        <span className="text-dark-200">Total</span>
                        <span className={`tabular-nums ${section.color}`}>{formatCurrency(section.total)}</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            ) : (
              <div className="flex flex-col items-center gap-3 py-12 text-dark-400">
                <Landmark className="h-8 w-8" />
                <p className="text-sm">Balance sheet unavailable.</p>
              </div>
            )}
          </div>
        ) : (
          <div className="glass-card rounded-2xl p-6">
            {gst ? (
              <div className="space-y-4">
                <div className="flex items-center gap-3">
                  <StatusBadge status={gst.filing_status ?? 'pending'} />
                  <span className="text-xs text-dark-500">Period: {gst.period ?? '—'}</span>
                </div>
                <div className="grid gap-4 sm:grid-cols-5">
                  {[
                    { label: 'Taxable', value: gst.total_taxable },
                    { label: 'CGST', value: gst.cgst },
                    { label: 'SGST', value: gst.sgst },
                    { label: 'IGST', value: gst.igst },
                    { label: 'Total', value: gst.total },
                  ].map((item) => (
                    <div key={item.label} className="rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3 text-center">
                      <p className="text-lg font-semibold tabular-nums text-white">{formatCurrency(item.value)}</p>
                      <p className="text-xs text-dark-500">{item.label}</p>
                    </div>
                  ))}
                </div>
              </div>
            ) : (
              <div className="flex flex-col items-center gap-3 py-12 text-dark-400">
                <FileText className="h-8 w-8" />
                <p className="text-sm">GST return data unavailable.</p>
              </div>
            )}
          </div>
        )}

        <CreateJournalModal open={journalOpen} onClose={() => setJournalOpen(false)} onCreated={() => { setJournalOpen(false); void load(); }} accounts={accounts} />
      </div>
    </PageTransition>
  );
}

function CreateJournalModal({
  open,
  onClose,
  onCreated,
  accounts,
}: {
  open: boolean;
  onClose: () => void;
  onCreated: () => void;
  accounts: AccountingAccount[];
}) {
  const [description, setDescription] = useState('');
  const [entryType, setEntryType] = useState('general');
  const [lines, setLines] = useState<JournalEntryLine[]>([
    { account_id: 0, debit: 0, credit: 0 },
    { account_id: 0, debit: 0, credit: 0 },
  ]);
  const [busy, setBusy] = useState(false);

  const updateLine = (index: number, field: keyof JournalEntryLine, value: string | number) => {
    setLines((prev) => {
      const next = [...prev];
      next[index] = { ...next[index], [field]: value };
      return next;
    });
  };

  const addLine = () => {
    setLines([...lines, { account_id: 0, debit: 0, credit: 0 }]);
  };

  const removeLine = (index: number) => {
    if (lines.length <= 2) return;
    setLines(lines.filter((_, i) => i !== index));
  };

  const totalDebit = lines.reduce((s, l) => s + (Number(l.debit) || 0), 0);
  const totalCredit = lines.reduce((s, l) => s + (Number(l.credit) || 0), 0);
  const balanced = Math.abs(totalDebit - totalCredit) < 0.01 && totalDebit > 0;

  const submit = async () => {
    if (!description.trim() || !balanced) return;
    setBusy(true);
    try {
      await createJournalEntry({
        description: description.trim(),
        entry_type: entryType,
        lines: lines.filter((l) => l.account_id > 0),
      });
      toast('Journal entry created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create entry', 'error');
    } finally {
      setBusy(false);
    }
  };

  const accountOptions = accounts.map((a) => ({ value: String(a.id), label: `${a.code ?? `#${a.id}`} — ${a.name}` }));

  return (
    <Modal open={open} onClose={onClose} title="New journal entry" subtitle="Create a double-entry journal record." width="xl"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy} disabled={!balanced}>Create entry</Button>
      </>}>
      <div className="space-y-4">
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Description" required value={description} onChange={(e) => setDescription(e.target.value)} />
          <SelectField label="Type" value={entryType} onChange={(e) => setEntryType(e.target.value)}
            options={[
              { value: 'general', label: 'General' },
              { value: 'sales', label: 'Sales' },
              { value: 'purchase', label: 'Purchase' },
              { value: 'adjustment', label: 'Adjustment' },
            ]}
          />
        </div>

        <div className="space-y-2">
          {lines.map((line, i) => (
            <div key={i} className="grid items-end gap-2 sm:grid-cols-[1fr_120px_120px_32px]">
              <SelectField
                label={i === 0 ? 'Account' : undefined}
                value={line.account_id ? String(line.account_id) : ''}
                onChange={(e) => updateLine(i, 'account_id', Number(e.target.value))}
                options={accountOptions}
                placeholder="Select account"
              />
              <TextField
                label={i === 0 ? 'Debit (₹)' : undefined}
                type="number"
                value={line.debit ? String(line.debit) : ''}
                onChange={(e) => updateLine(i, 'debit', e.target.value)}
              />
              <TextField
                label={i === 0 ? 'Credit (₹)' : undefined}
                type="number"
                value={line.credit ? String(line.credit) : ''}
                onChange={(e) => updateLine(i, 'credit', e.target.value)}
              />
              <Button variant="ghost" size="sm" onClick={() => removeLine(i)} disabled={lines.length <= 2}>×</Button>
            </div>
          ))}
          <Button variant="secondary" size="sm" onClick={addLine}>+ Add line</Button>
        </div>

        <div className="flex items-center justify-between rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3 text-sm">
          <div className="flex gap-6">
            <span className="text-dark-400">Debits: <span className="tabular-nums text-white">{formatCurrency(totalDebit)}</span></span>
            <span className="text-dark-400">Credits: <span className="tabular-nums text-white">{formatCurrency(totalCredit)}</span></span>
          </div>
          {balanced ? (
            <StatusBadge status="balanced" />
          ) : (
            <span className="text-xs text-red-400">Debits and credits must balance</span>
          )}
        </div>
      </div>
    </Modal>
  );
}
