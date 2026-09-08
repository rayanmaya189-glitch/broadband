import { useEffect, useState } from 'react';
import { Gift, Plus, TrendingUp, Wallet } from 'lucide-react';
import {
  listReferralPrograms,
  createReferralProgram,
  deleteReferralProgram,
  listReferrals,
  listReferralWallets,
  adjustReferralWallet,
  getReferralAnalytics,
} from '../../api/admin/modules';
import type { ReferralProgram, ReferralRecord, ReferralWallet, ReferralAnalytics } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency, formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'programs' | 'referrals' | 'wallets' | 'analytics';

export function ReferralsPage() {
  const [programs, setPrograms] = useState<ReferralProgram[]>([]);
  const [referrals, setReferrals] = useState<ReferralRecord[]>([]);
  const [wallets, setWallets] = useState<ReferralWallet[]>([]);
  const [analytics, setAnalytics] = useState<ReferralAnalytics | null>(null);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<Tab>('programs');
  const [createOpen, setCreateOpen] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<ReferralProgram | null>(null);
  const [adjustTarget, setAdjustTarget] = useState<ReferralWallet | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [p, r, w, a] = await Promise.all([
        listReferralPrograms().catch(() => []),
        listReferrals().catch(() => []),
        listReferralWallets().catch(() => []),
        getReferralAnalytics().catch(() => null),
      ]);
      setPrograms(p);
      setReferrals(r);
      setWallets(w);
      if (a) setAnalytics(a);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed', 'error');
    } finally { setLoading(false); }
  };
  useEffect(() => { void load(); }, []);

  const programColumns: Column<ReferralProgram>[] = [
    { key: 'name', header: 'Program', render: (p) => <div><p className="font-medium text-white">{p.name}</p>{p.description && <p className="max-w-md truncate text-xs text-dark-500">{p.description}</p>}</div> },
    { key: 'reward_type', header: 'Reward', render: (p) => <StatusBadge status={p.reward_type} /> },
    { key: 'reward_amount', header: 'Amount', align: 'right', render: (p) => <span className="tabular-nums text-dark-200">{formatCurrency(p.reward_amount)}</span> },
    { key: 'is_active', header: 'Status', render: (p) => <StatusBadge status={p.is_active !== false ? 'active' : 'inactive'} /> },
    { key: 'valid_until', header: 'Valid until', render: (p) => <span className="text-xs text-dark-500">{formatDateTime(p.valid_until)}</span> },
    { key: 'actions', header: '', align: 'right', render: (p) => (
      <div onClick={(e) => e.stopPropagation()}><Button variant="ghost" size="sm" onClick={() => setDeleteTarget(p)}><span className="text-red-400">Delete</span></Button></div>
    )},
  ];

  const referralColumns: Column<ReferralRecord>[] = [
    { key: 'referrer_name', header: 'Referrer', render: (r) => <span className="text-dark-200">{r.referrer_name ?? `#${r.referrer_id}`}</span> },
    { key: 'referred_name', header: 'Referred', render: (r) => <span className="text-dark-200">{r.referred_name ?? `#${r.referred_id}`}</span> },
    { key: 'program_name', header: 'Program', render: (r) => <span className="text-dark-300">{r.program_name ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (r) => <StatusBadge status={r.status} /> },
    { key: 'reward_amount', header: 'Reward', align: 'right', render: (r) => <span className="tabular-nums text-dark-200">{r.reward_amount != null ? formatCurrency(r.reward_amount) : '—'}</span> },
    { key: 'created_at', header: 'Date', render: (r) => <span className="text-xs text-dark-500">{formatDateTime(r.created_at)}</span> },
  ];

  const walletColumns: Column<ReferralWallet>[] = [
    { key: 'customer_name', header: 'Customer', render: (w) => <span className="font-medium text-white">{w.customer_name ?? `#${w.customer_id}`}</span> },
    { key: 'balance', header: 'Balance', align: 'right', render: (w) => <span className="tabular-nums text-emerald-400">{formatCurrency(w.balance)}</span> },
    { key: 'total_earned', header: 'Earned', align: 'right', render: (w) => <span className="tabular-nums text-dark-200">{formatCurrency(w.total_earned)}</span> },
    { key: 'total_redeemed', header: 'Redeemed', align: 'right', render: (w) => <span className="tabular-nums text-dark-300">{formatCurrency(w.total_redeemed)}</span> },
    { key: 'actions', header: '', align: 'right', render: (w) => (
      <div onClick={(e) => e.stopPropagation()}><Button variant="secondary" size="sm" onClick={() => setAdjustTarget(w)}>Adjust</Button></div>
    )},
  ];

  const tabs: { key: Tab; label: string }[] = [
    { key: 'programs', label: `Programs (${programs.length})` },
    { key: 'referrals', label: `Referrals (${referrals.length})` },
    { key: 'wallets', label: `Wallets (${wallets.length})` },
    { key: 'analytics', label: 'Analytics' },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader title="Referrals" subtitle="Referral programs, tracking and wallet management." icon={<Gift className="h-5 w-5" />}
          actions={tab === 'programs' ? <Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> New program</Button> : undefined}
        />
        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {tabs.map((t) => (
            <button key={t.key} onClick={() => setTab(t.key)}
              className={`rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors ${tab === t.key ? 'bg-primary-500/20 text-primary-200' : 'text-dark-400 hover:text-white'}`}>
              {t.label}
            </button>
          ))}
        </div>
        {loading ? <Spinner /> : tab === 'programs' ? (
          <DataTable columns={programColumns} data={programs} rowKey={(p) => p.id} emptyTitle="No programs" emptyDescription="Create a referral program to start." />
        ) : tab === 'referrals' ? (
          <DataTable columns={referralColumns} data={referrals} rowKey={(r) => r.id} emptyTitle="No referrals" emptyDescription="Referral records will appear here." />
        ) : tab === 'wallets' ? (
          <DataTable columns={walletColumns} data={wallets} rowKey={(w) => w.id} emptyTitle="No wallets" emptyDescription="Referral wallets will appear here." />
        ) : analytics ? (
          <div className="grid gap-4 sm:grid-cols-3">
            {[
              { label: 'Total referrals', value: analytics.total_referrals, icon: <Gift className="h-5 w-5" />, color: 'text-primary-400' },
              { label: 'Conversions', value: analytics.successful_conversions, icon: <TrendingUp className="h-5 w-5" />, color: 'text-emerald-400' },
              { label: 'Rewards paid', value: formatCurrency(analytics.total_rewards_paid), icon: <Wallet className="h-5 w-5" />, color: 'text-accent-400' },
            ].map((c) => (
              <div key={c.label} className="glass-card flex items-center gap-4 rounded-2xl p-5">
                <div className={`flex h-10 w-10 items-center justify-center rounded-xl border border-white/[0.08] bg-white/[0.03] ${c.color}`}>{c.icon}</div>
                <div><p className="text-2xl font-semibold tabular-nums text-white">{typeof c.value === 'number' ? c.value.toLocaleString() : c.value}</p><p className="text-xs text-dark-400">{c.label}</p></div>
              </div>
            ))}
            <div className="glass-card rounded-2xl p-5 sm:col-span-3">
              <p className="text-sm text-dark-300">Conversion rate: <span className="text-white font-semibold">{analytics.conversion_rate}%</span> · Pending rewards: <span className="text-white">{formatCurrency(analytics.pending_rewards)}</span></p>
            </div>
          </div>
        ) : <Spinner />}

        <CreateProgramModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />
        <ConfirmDialog open={deleteTarget !== null} onClose={() => setDeleteTarget(null)} onConfirm={async () => { if (!deleteTarget) return; try { await deleteReferralProgram(deleteTarget.id); toast('Deleted'); setDeleteTarget(null); void load(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } }} title="Delete program?" description={`Delete "${deleteTarget?.name}".`} confirmLabel="Delete" danger />
        {adjustTarget && <AdjustWalletModal wallet={adjustTarget} open={adjustTarget !== null} onClose={() => setAdjustTarget(null)} onDone={() => { setAdjustTarget(null); void load(); }} />}
      </div>
    </PageTransition>
  );
}

function CreateProgramModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState(''); const [rewardType, setRewardType] = useState('fixed'); const [rewardValue, setRewardValue] = useState(''); const [validFrom, setValidFrom] = useState(new Date().toISOString().slice(0, 10)); const [validUntil, setValidUntil] = useState(''); const [busy, setBusy] = useState(false);
  const submit = async () => { if (!name.trim()) return; setBusy(true); try { await createReferralProgram({ name: name.trim(), reward_type: rewardType, reward_value: Number(rewardValue) || 0, valid_from: validFrom, valid_until: validUntil || validFrom }); toast('Program created'); onCreated(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setBusy(false); } };
  return (
    <Modal open={open} onClose={onClose} title="New referral program" width="md" footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Create</Button></>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Name" required value={name} onChange={(e) => setName(e.target.value)} />
        <SelectField label="Reward type" value={rewardType} onChange={(e) => setRewardType(e.target.value)} options={[{ value: 'fixed', label: 'Fixed amount' }, { value: 'percentage', label: 'Percentage' }, { value: 'credit', label: 'Wallet credit' }]} />
        <TextField label="Reward value" type="number" required value={rewardValue} onChange={(e) => setRewardValue(e.target.value)} placeholder="e.g. 100 for fixed, 5 for percentage" />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Valid from" type="date" required value={validFrom} onChange={(e) => setValidFrom(e.target.value)} />
          <TextField label="Valid until" type="date" value={validUntil} onChange={(e) => setValidUntil(e.target.value)} />
        </div>
      </form>
    </Modal>
  );
}

function AdjustWalletModal({ wallet, open, onClose, onDone }: { wallet: ReferralWallet; open: boolean; onClose: () => void; onDone: () => void }) {
  const [amount, setAmount] = useState(''); const [reason, setReason] = useState(''); const [busy, setBusy] = useState(false);
  const submit = async () => { if (!amount) return; setBusy(true); try { await adjustReferralWallet(wallet.id, { amount: Number(amount), reason: reason.trim() || undefined }); toast('Wallet adjusted'); onDone(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setBusy(false); } };
  return (
    <Modal open={open} onClose={onClose} title={`Adjust wallet — ${wallet.customer_name ?? `#${wallet.customer_id}`}`} width="md" footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Adjust</Button></>}>
      <div className="mb-3 text-sm text-dark-300">Current balance: <span className="text-white font-semibold">{formatCurrency(wallet.balance)}</span></div>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Amount (negative to deduct)" type="number" required value={amount} onChange={(e) => setAmount(e.target.value)} />
        <TextField label="Reason" value={reason} onChange={(e) => setReason(e.target.value)} placeholder="Adjustment reason…" />
      </form>
    </Modal>
  );
}
