import { useEffect, useState } from 'react';
import { Zap, Plus, Trash2, Edit3, Play, BarChart3 } from 'lucide-react';
import {
  listBandwidthProfiles,
  createBandwidthProfile,
  updateBandwidthProfile,
  deleteBandwidthProfile,
  applyProfileToAll,
  applyProfileToSubscription,
  listBandwidthPolicies,
  createBandwidthPolicy,
  updateBandwidthPolicy,
  deleteBandwidthPolicy,
  listBandwidthApplications,
  getBandwidthUsage,
} from '../../api/admin/modules';
import type { BandwidthProfile, BandwidthPolicy, BandwidthApplication, BandwidthUsage } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField, TextArea, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, formatNumber, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'profiles' | 'policies' | 'applications';

function formatSpeed(kbps: number | undefined): string {
  if (!kbps) return '—';
  if (kbps >= 1000) return `${(kbps / 1000).toFixed(kbps % 1000 === 0 ? 0 : 1)} Mbps`;
  return `${kbps} Kbps`;
}

function formatBytes(bytes: number | undefined): string {
  if (!bytes) return '—';
  if (bytes >= 1073741824) return `${(bytes / 1073741824).toFixed(1)} GB`;
  if (bytes >= 1048576) return `${(bytes / 1048576).toFixed(1)} MB`;
  if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${bytes} B`;
}

export function BandwidthPage() {
  const [profiles, setProfiles] = useState<BandwidthProfile[]>([]);
  const [policies, setPolicies] = useState<BandwidthPolicy[]>([]);
  const [applications, setApplications] = useState<BandwidthApplication[]>([]);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<Tab>('profiles');
  const [profileCreateOpen, setProfileCreateOpen] = useState(false);
  const [profileEditTarget, setProfileEditTarget] = useState<BandwidthProfile | null>(null);
  const [profileDeleteTarget, setProfileDeleteTarget] = useState<BandwidthProfile | null>(null);
  const [profileApplyTarget, setProfileApplyTarget] = useState<BandwidthProfile | null>(null);
  const [policyCreateOpen, setPolicyCreateOpen] = useState(false);
  const [policyEditTarget, setPolicyEditTarget] = useState<BandwidthPolicy | null>(null);
  const [policyDeleteTarget, setPolicyDeleteTarget] = useState<BandwidthPolicy | null>(null);
  const [usageOpen, setUsageOpen] = useState<number | null>(null);
  const [usage, setUsage] = useState<BandwidthUsage | null>(null);
  const [usageLoading, setUsageLoading] = useState(false);
  const [deleting, setDeleting] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const [p, pol, app] = await Promise.all([
        listBandwidthProfiles().catch(() => []),
        listBandwidthPolicies().catch(() => []),
        listBandwidthApplications().catch(() => []),
      ]);
      setProfiles(p);
      setPolicies(pol);
      setApplications(app);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load bandwidth data', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const confirmProfileDelete = async () => {
    if (!profileDeleteTarget) return;
    setDeleting(true);
    try {
      await deleteBandwidthProfile(profileDeleteTarget.id);
      toast('Profile deleted');
      setProfileDeleteTarget(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to delete profile', 'error');
    } finally {
      setDeleting(false);
    }
  };

  const confirmPolicyDelete = async () => {
    if (!policyDeleteTarget) return;
    setDeleting(true);
    try {
      await deleteBandwidthPolicy(policyDeleteTarget.id);
      toast('Policy deleted');
      setPolicyDeleteTarget(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to delete policy', 'error');
    } finally {
      setDeleting(false);
    }
  };

  const handleApplyAll = async () => {
    if (!profileApplyTarget) return;
    try {
      await applyProfileToAll(profileApplyTarget.id);
      toast(`Profile "${profileApplyTarget.name}" applied to all eligible subscriptions`);
      setProfileApplyTarget(null);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to apply profile', 'error');
    }
  };

  const openUsage = async (subscriptionId: number) => {
    setUsageOpen(subscriptionId);
    setUsageLoading(true);
    setUsage(null);
    try {
      setUsage(await getBandwidthUsage(subscriptionId));
    } catch {
      setUsage(null);
    } finally {
      setUsageLoading(false);
    }
  };

  const profileColumns: Column<BandwidthProfile>[] = [
    { key: 'name', header: 'Profile', render: (p) => (
      <div>
        <p className="font-medium text-white">{p.name}</p>
        {p.description && <p className="max-w-md truncate text-xs text-dark-500">{p.description}</p>}
      </div>
    )},
    { key: 'download_kbps', header: 'Download', render: (p) => (
      <span className="tabular-nums text-dark-200">{formatSpeed(p.download_kbps)}</span>
    )},
    { key: 'upload_kbps', header: 'Upload', render: (p) => (
      <span className="tabular-nums text-dark-200">{formatSpeed(p.upload_kbps)}</span>
    )},
    { key: 'priority', header: 'Priority', render: (p) => (
      <span className="tabular-nums text-dark-300">{p.priority ?? '—'}</span>
    )},
    { key: 'is_active', header: 'Status', render: (p) => <StatusBadge status={p.is_active !== false ? 'active' : 'inactive'} /> },
    {
      key: 'actions', header: 'Actions', align: 'right',
      render: (p) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => setProfileApplyTarget(p)} title="Apply to all">
            <Play className="h-3.5 w-3.5" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setProfileEditTarget(p)} title="Edit">
            <Edit3 className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setProfileDeleteTarget(p)} title="Delete">
            <Trash2 className="h-4 w-4 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  const policyColumns: Column<BandwidthPolicy>[] = [
    { key: 'name', header: 'Policy', render: (p) => (
      <div>
        <p className="font-medium text-white">{p.name}</p>
        {p.description && <p className="max-w-md truncate text-xs text-dark-500">{p.description}</p>}
      </div>
    )},
    { key: 'type', header: 'Type', render: (p) => <StatusBadge status={p.type} /> },
    { key: 'is_active', header: 'Status', render: (p) => <StatusBadge status={p.is_active !== false ? 'active' : 'inactive'} /> },
    { key: 'created_at', header: 'Created', render: (p) => <span className="text-xs text-dark-500">{formatDateTime(p.created_at)}</span> },
    {
      key: 'actions', header: 'Actions', align: 'right',
      render: (p) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => setPolicyEditTarget(p)} title="Edit">
            <Edit3 className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setPolicyDeleteTarget(p)} title="Delete">
            <Trash2 className="h-4 w-4 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  const applicationColumns: Column<BandwidthApplication>[] = [
    { key: 'customer_name', header: 'Customer', render: (a) => (
      <span className="text-dark-200">{a.customer_name ?? `Sub #${a.subscription_id}`}</span>
    )},
    { key: 'profile_name', header: 'Profile', render: (a) => <span className="text-dark-300">{a.profile_name ?? `#${a.profile_id}`}</span> },
    { key: 'status', header: 'Status', render: (a) => <StatusBadge status={a.status ?? 'active'} /> },
    { key: 'applied_at', header: 'Applied', render: (a) => <span className="text-xs text-dark-500">{formatDateTime(a.applied_at)}</span> },
    {
      key: 'actions', header: '', align: 'right',
      render: (a) => (
        <div onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => void openUsage(a.subscription_id)} title="View usage">
            <BarChart3 className="h-4 w-4" />
          </Button>
        </div>
      ),
    },
  ];

  const tabs: { key: Tab; label: string }[] = [
    { key: 'profiles', label: `Profiles (${profiles.length})` },
    { key: 'policies', label: `Policies (${policies.length})` },
    { key: 'applications', label: `Applications (${applications.length})` },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Bandwidth management"
          subtitle="Speed profiles, traffic policies and per-subscription usage."
          icon={<Zap className="h-5 w-5" />}
          actions={
            <>
              {tab === 'profiles' && (
                <Button onClick={() => setProfileCreateOpen(true)}>
                  <Plus className="h-4 w-4" /> New profile
                </Button>
              )}
              {tab === 'policies' && (
                <Button onClick={() => setPolicyCreateOpen(true)}>
                  <Plus className="h-4 w-4" /> New policy
                </Button>
              )}
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
        ) : tab === 'profiles' ? (
          <DataTable columns={profileColumns} data={profiles} rowKey={(p) => p.id} emptyTitle="No profiles" emptyDescription="Create bandwidth profiles to define speed tiers." />
        ) : tab === 'policies' ? (
          <DataTable columns={policyColumns} data={policies} rowKey={(p) => p.id} emptyTitle="No policies" emptyDescription="Create traffic policies to control bandwidth allocation." />
        ) : (
          <DataTable columns={applicationColumns} data={applications} rowKey={(a) => a.id} emptyTitle="No applications" emptyDescription="Active bandwidth profile applications will appear here." />
        )}

        {/* Profile modals */}
        <CreateProfileModal open={profileCreateOpen} onClose={() => setProfileCreateOpen(false)} onCreated={() => { setProfileCreateOpen(false); void load(); }} />
        {profileEditTarget && (
          <EditProfileModal profile={profileEditTarget} open={profileEditTarget !== null} onClose={() => setProfileEditTarget(null)} onSaved={() => { setProfileEditTarget(null); void load(); }} />
        )}
        <ConfirmDialog open={profileDeleteTarget !== null} onClose={() => setProfileDeleteTarget(null)} onConfirm={confirmProfileDelete} loading={deleting}
          title="Delete profile?" description={`Delete bandwidth profile "${profileDeleteTarget?.name}". This may affect active subscriptions.`} confirmLabel="Delete profile" danger />
        <ConfirmDialog open={profileApplyTarget !== null} onClose={() => setProfileApplyTarget(null)} onConfirm={handleApplyAll} loading={false}
          title="Apply to all subscriptions?" description={`Apply "${profileApplyTarget?.name}" (${formatSpeed(profileApplyTarget?.download_kbps)}↓ / ${formatSpeed(profileApplyTarget?.upload_kbps)}↑) to all eligible active subscriptions?`} confirmLabel="Apply to all" />

        {/* Policy modals */}
        <CreatePolicyModal open={policyCreateOpen} onClose={() => setPolicyCreateOpen(false)} onCreated={() => { setPolicyCreateOpen(false); void load(); }} />
        {policyEditTarget && (
          <EditPolicyModal policy={policyEditTarget} open={policyEditTarget !== null} onClose={() => setPolicyEditTarget(null)} onSaved={() => { setPolicyEditTarget(null); void load(); }} />
        )}
        <ConfirmDialog open={policyDeleteTarget !== null} onClose={() => setPolicyDeleteTarget(null)} onConfirm={confirmPolicyDelete} loading={deleting}
          title="Delete policy?" description={`Delete policy "${policyDeleteTarget?.name}".`} confirmLabel="Delete policy" danger />

        {/* Usage modal */}
        <Modal open={usageOpen !== null} onClose={() => setUsageOpen(null)} title="Bandwidth usage" subtitle={`Subscription #${usageOpen}`} width="md">
          {usageLoading ? (
            <Spinner />
          ) : usage ? (
            <div className="space-y-4">
              <div className="grid gap-3 sm:grid-cols-2">
                <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3">
                  <p className="text-xs text-dark-500">Download speed</p>
                  <p className="mt-1 text-lg font-semibold text-white">{formatSpeed(usage.download_kbps)}</p>
                </div>
                <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3">
                  <p className="text-xs text-dark-500">Upload speed</p>
                  <p className="mt-1 text-lg font-semibold text-white">{formatSpeed(usage.upload_kbps)}</p>
                </div>
              </div>
              <div className="grid gap-3 sm:grid-cols-2">
                <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3">
                  <p className="text-xs text-dark-500">Downloaded</p>
                  <p className="mt-1 text-lg font-semibold text-white">{formatBytes(usage.bytes_downloaded)}</p>
                </div>
                <div className="rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3">
                  <p className="text-xs text-dark-500">Uploaded</p>
                  <p className="mt-1 text-lg font-semibold text-white">{formatBytes(usage.bytes_uploaded)}</p>
                </div>
              </div>
              {usage.profile_name && (
                <p className="text-xs text-dark-500">Profile: {usage.profile_name} · Period: {formatDateTime(usage.period_start)} — {formatDateTime(usage.period_end)}</p>
              )}
            </div>
          ) : (
            <p className="py-6 text-center text-sm text-dark-500">No usage data available for this subscription.</p>
          )}
        </Modal>
      </div>
    </PageTransition>
  );
}

// ─── Profile modals ──────────────────────────────────────────────────────────

function CreateProfileModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [download, setDownload] = useState('');
  const [upload, setUpload] = useState('');
  const [priority, setPriority] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim() || !download) return;
    setBusy(true);
    try {
      await createBandwidthProfile({
        name: name.trim(),
        download_kbps: Number(download),
        upload_kbps: Number(upload) || 0,
      });
      toast('Profile created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create profile', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New bandwidth profile" subtitle="Define a speed tier for subscriptions." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create profile</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Profile name" required value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. Home 100 Mbps" />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Download (Kbps)" type="number" required value={download} onChange={(e) => setDownload(e.target.value)} placeholder="100000" />
          <TextField label="Upload (Kbps)" type="number" value={upload} onChange={(e) => setUpload(e.target.value)} placeholder="50000" />
        </div>
        <TextField label="Priority" type="number" value={priority} onChange={(e) => setPriority(e.target.value)} hint="1 = highest, 8 = lowest. Blank = default." />
      </form>
    </Modal>
  );
}

function EditProfileModal({ profile, open, onClose, onSaved }: { profile: BandwidthProfile; open: boolean; onClose: () => void; onSaved: () => void }) {
  const [name, setName] = useState(profile.name);
  const [description, setDescription] = useState(profile.description ?? '');
  const [download, setDownload] = useState(String(profile.download_kbps));
  const [upload, setUpload] = useState(String(profile.upload_kbps));
  const [priority, setPriority] = useState(profile.priority != null ? String(profile.priority) : '');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim() || !download) return;
    setBusy(true);
    try {
      await updateBandwidthProfile(profile.id, {
        name: name.trim(),
        description: description.trim() || undefined,
        download_kbps: Number(download),
        upload_kbps: Number(upload) || 0,
        priority: priority ? Number(priority) : undefined,
      });
      toast('Profile updated');
      onSaved();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update profile', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Edit profile" subtitle={profile.name} width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Save changes</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Profile name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Download (Kbps)" type="number" required value={download} onChange={(e) => setDownload(e.target.value)} />
          <TextField label="Upload (Kbps)" type="number" value={upload} onChange={(e) => setUpload(e.target.value)} />
        </div>
        <TextField label="Priority" type="number" value={priority} onChange={(e) => setPriority(e.target.value)} hint="1 = highest, 8 = lowest." />
      </form>
    </Modal>
  );
}

// ─── Policy modals ───────────────────────────────────────────────────────────

function CreatePolicyModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [type, setType] = useState('rate_limit');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await createBandwidthPolicy({ name: name.trim(), policy_type: type, config: {} });
      toast('Policy created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create policy', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New traffic policy" subtitle="Create a bandwidth control policy." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create policy</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Policy name" required value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. Peak hour throttle" />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
        <SelectField label="Type" value={type} onChange={(e) => setType(e.target.value)}
          options={[
            { value: 'rate_limit', label: 'Rate limiting' },
            { value: 'burst_control', label: 'Burst control' },
            { value: 'time_based', label: 'Time-based' },
            { value: 'fair_use', label: 'Fair use' },
            { value: 'qos', label: 'QoS' },
          ]}
        />
      </form>
    </Modal>
  );
}

function EditPolicyModal({ policy, open, onClose, onSaved }: { policy: BandwidthPolicy; open: boolean; onClose: () => void; onSaved: () => void }) {
  const [name, setName] = useState(policy.name);
  const [description, setDescription] = useState(policy.description ?? '');
  const [type, setType] = useState(policy.type);
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await updateBandwidthPolicy(policy.id, { name: name.trim(), config: {} });
      toast('Policy updated');
      onSaved();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update policy', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Edit policy" subtitle={policy.name} width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Save changes</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Policy name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
        <SelectField label="Type" value={type} onChange={(e) => setType(e.target.value)}
          options={[
            { value: 'rate_limit', label: 'Rate limiting' },
            { value: 'burst_control', label: 'Burst control' },
            { value: 'time_based', label: 'Time-based' },
            { value: 'fair_use', label: 'Fair use' },
            { value: 'qos', label: 'QoS' },
          ]}
        />
      </form>
    </Modal>
  );
}
