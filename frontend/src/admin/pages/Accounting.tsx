import { useEffect, useState } from 'react';
import { BookOpen, Landmark, NotebookTabs } from 'lucide-react';
import { getTrialBalance, listAccounts, listJournalEntries } from '../../api/admin/modules';
import type { AccountingAccount, JournalEntry } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { DataTable, type Column } from '../components/ui/DataTable';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency, formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function AccountingPage() {
  const [tab, setTab] = useState<'accounts' | 'journal' | 'trial'>('accounts');
  const [accounts, setAccounts] = useState<AccountingAccount[]>([]);
  const [journal, setJournal] = useState<JournalEntry[]>([]);
  const [trial, setTrial] = useState<unknown | null>(null);
  const [loading, setLoading] = useState(true);

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

  const accountColumns: Column<AccountingAccount>[] = [
    { key: 'code', header: 'Code', render: (a) => <span className="font-mono text-xs text-accent-300">{a.code ?? `#${a.id}`}</span> },
    { key: 'name', header: 'Account', render: (a) => <span className="font-medium text-white">{a.name}</span> },
    { key: 'account_type', header: 'Type', render: (a) => <StatusBadge status={a.account_type} /> },
    { key: 'balance', header: 'Balance', align: 'right', render: (a) => (
      <span className="tabular-nums text-dark-200">{formatCurrency(a.balance)}</span>
    )},
    { key: 'status', header: 'Status', render: (a) => <StatusBadge status={a.status} /> },
  ];

  const journalColumns: Column<JournalEntry>[] = [
    { key: 'entry_number', header: 'Entry', render: (j) => <span className="font-medium text-white">{j.entry_number ?? `#${j.id}`}</span> },
    { key: 'description', header: 'Description', render: (j) => <span className="max-w-md truncate text-dark-300">{j.description ?? 'â€”'}</span> },
    { key: 'entry_type', header: 'Type', render: (j) => <StatusBadge status={j.entry_type} /> },
    { key: 'amount', header: 'Amount', align: 'right', render: (j) => <span className="tabular-nums text-dark-200">{formatCurrency(j.amount)}</span> },
    { key: 'status', header: 'Status', render: (j) => <StatusBadge status={j.status} /> },
    { key: 'entry_date', header: 'Date', render: (j) => <span className="text-xs text-dark-500">{formatDateTime(j.entry_date)}</span> },
  ];

  const tabs = [
    { key: 'accounts' as const, label: `Chart of accounts (${accounts.length})` },
    { key: 'journal' as const, label: `Journal (${journal.length})` },
    { key: 'trial' as const, label: 'Trial balance' },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Accounting"
          subtitle="Chart of accounts, journal entries and financial reports."
          icon={<BookOpen className="h-5 w-5" />}
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
        ) : tab === 'accounts' ? (
          <DataTable columns={accountColumns} data={accounts} rowKey={(a) => a.id} emptyTitle="No accounts" emptyDescription="Chart of accounts will appear here." />
        ) : tab === 'journal' ? (
          <DataTable columns={journalColumns} data={journal} rowKey={(j) => j.id} emptyTitle="No journal entries" emptyDescription="Journal entries will appear here." />
        ) : (
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
        )}

        <div className="flex items-center gap-2 text-xs text-dark-500">
          <NotebookTabs className="h-3.5 w-3.5" />
          Full statements (P&L, balance sheet, GST returns) are exposed via /accounting endpoints.
        </div>
      </div>
    </PageTransition>
  );
}
