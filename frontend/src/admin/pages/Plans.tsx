import { useEffect, useState } from 'react';
import { Gauge, Plus, Eye, EyeOff, Trash2, Copy, DollarSign, Edit3 } from 'lucide-react';
import {
  createPlan,
  listPlans,
  updatePlan,
  updatePlanPricing,
  publishPlan,
  deactivatePlan,
  clonePlan,
  getPlanHistory,
} from '../../api/admin/modules';
import type { Plan, PlanPrice } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField, TextArea, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency, formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function PlansPage() {
  const [plans, setPlans] = useState<Plan[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [editTarget, setEditTarget] = useState<Plan | null>(null);
  const [pricingTarget, setPricingTarget] = useState<Plan | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<Plan | null>(null);
  const [deleting, setDeleting] = useState(false);
  const [cloning, setCloning] = useState<number | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      setPlans(await listPlans());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load plans', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const togglePublish = async (plan: Plan) => {
    try {
      const next = plan.is_published ? false : true;
      await publishPlan(plan.id, next);
      toast(`Plan ${next ? 'published' : 'unpublished'}`);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Action failed', 'error');
    }
  };

  const handleClone = async (plan: Plan) => {
    setCloning(plan.id);
    try {
      await clonePlan(plan.id);
      toast('Plan cloned');
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to clone plan', 'error');
    } finally {
      setCloning(null);
    }
  };

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    setDeleting(true);
    try {
      await deactivatePlan(deleteTarget.id);
      toast('Plan deactivated', 'info');
      setDeleteTarget(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to deactivate plan', 'error');
    } finally {
      setDeleting(false);
    }
  };

  const columns: Column<Plan>[] = [
    { key: 'name', header: 'Plan', render: (p) => (
      <div>
        <p className="font-medium text-white">{p.name}</p>
        {p.description && <p className="max-w-md truncate text-xs text-dark-500">{p.description}</p>}
      </div>
    )},
    { key: 'slug', header: 'Slug', render: (p) => <span className="font-mono text-xs text-dark-400">{p.slug ?? '—'}</span> },
    {
      key: 'pricing',
      header: 'Starting at',
      render: (p) => {
        const prices = p.pricing ?? [];
        const min = prices.length ? Math.min(...prices.map((pr) => Number(pr.monthly_price) || 0)) : 0;
        return <span className="tabular-nums text-dark-200">{prices.length ? `${formatCurrency(min)}/mo` : '—'}</span>;
      },
    },
    { key: 'status', header: 'Status', render: (p) => <StatusBadge status={p.status} /> },
    {
      key: 'is_published',
      header: 'Visibility',
      render: (p) =>
        p.is_published ? (
          <StatusBadge status="published" />
        ) : (
          <StatusBadge status="draft" />
        ),
    },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (p) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={() => setEditTarget(p)} title="Edit plan">
            <Edit3 className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setPricingTarget(p)} title="Manage pricing">
            <DollarSign className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" loading={cloning === p.id} onClick={() => void handleClone(p)} title="Clone plan">
            <Copy className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => void togglePublish(p)} title={p.is_published ? 'Unpublish' : 'Publish'}>
            {p.is_published ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setDeleteTarget(p)} title="Deactivate">
            <Trash2 className="h-4 w-4 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Plans & pricing"
          subtitle="Broadband plans available to customers."
          icon={<Gauge className="h-5 w-5" />}
          actions={
            <Button onClick={() => setCreateOpen(true)}>
              <Plus className="h-4 w-4" /> New plan
            </Button>
          }
        />

        <DataTable
          columns={columns}
          data={plans}
          rowKey={(p) => p.id}
          loading={loading}
          onRowClick={(p) => setEditTarget(p)}
          emptyTitle="No plans yet"
          emptyDescription="Create your first broadband plan to start selling."
        />

        <CreatePlanModal
          open={createOpen}
          onClose={() => setCreateOpen(false)}
          onCreated={() => {
            setCreateOpen(false);
            void load();
          }}
        />

        {editTarget && (
          <EditPlanModal
            plan={editTarget}
            open={editTarget !== null}
            onClose={() => setEditTarget(null)}
            onSaved={() => {
              setEditTarget(null);
              void load();
            }}
          />
        )}

        {pricingTarget && (
          <PricingModal
            plan={pricingTarget}
            open={pricingTarget !== null}
            onClose={() => setPricingTarget(null)}
            onSaved={() => {
              setPricingTarget(null);
              void load();
            }}
          />
        )}

        <ConfirmDialog
          open={deleteTarget !== null}
          onClose={() => setDeleteTarget(null)}
          onConfirm={confirmDelete}
          loading={deleting}
          title="Deactivate plan?"
          description={`This deactivates "${deleteTarget?.name}". Existing subscriptions continue, but the plan can no longer be sold.`}
          confirmLabel="Deactivate plan"
          danger
        />
      </div>
    </PageTransition>
  );
}

function slugify(value: string): string {
  return value
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9\s-]/g, '')
    .replace(/[\s_]+/g, '-')
    .replace(/-+/g, '-');
}

function CreatePlanModal({
  open,
  onClose,
  onCreated,
}: {
  open: boolean;
  onClose: () => void;
  onCreated: () => void;
}) {
  const [name, setName] = useState('');
  const [slug, setSlug] = useState('');
  const [description, setDescription] = useState('');
  const [speedLabel, setSpeedLabel] = useState('');
  const [downloadMbps, setDownloadMbps] = useState('');
  const [uploadMbps, setUploadMbps] = useState('');
  const [isBusiness, setIsBusiness] = useState(false);
  const [busy, setBusy] = useState(false);
  const [touched, setTouched] = useState(false);

  const reset = () => {
    setName('');
    setSlug('');
    setDescription('');
    setSpeedLabel('');
    setDownloadMbps('');
    setUploadMbps('');
    setIsBusiness(false);
    setTouched(false);
  };

  const submit = async () => {
    setTouched(true);
    if (!name.trim() || !downloadMbps || !uploadMbps || !speedLabel.trim()) return;
    setBusy(true);
    try {
      await createPlan({
        name: name.trim(),
        slug: slug.trim() || slugify(name),
        description: description.trim() || undefined,
        speed_label: speedLabel.trim(),
        download_mbps: Number(downloadMbps),
        upload_mbps: Number(uploadMbps),
        is_business: isBusiness,
      });
      toast('Plan created');
      reset();
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create plan', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open={open}
      onClose={() => { reset(); onClose(); }}
      title="New plan"
      subtitle="Create a broadband plan."
      width="md"
      footer={
        <>
          <Button variant="ghost" onClick={onClose}>Cancel</Button>
          <Button onClick={submit} loading={busy}>Create plan</Button>
        </>
      }
    >
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4" noValidate>
        <TextField label="Plan name" required value={name} onChange={(e) => setName(e.target.value)}
          error={touched && !name.trim() ? 'Plan name is required.' : undefined} />
        <TextField label="Slug" value={slug} onChange={(e) => setSlug(e.target.value)} hint="Leave blank to auto-generate from the name." />
        <TextField label="Speed label" required value={speedLabel} onChange={(e) => setSpeedLabel(e.target.value)}
          placeholder="e.g. Home 200 Mbps"
          error={touched && !speedLabel.trim() ? 'Speed label is required.' : undefined} />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Download (Mbps)" type="number" required value={downloadMbps} onChange={(e) => setDownloadMbps(e.target.value)}
            error={touched && !downloadMbps ? 'Download speed is required.' : undefined} />
          <TextField label="Upload (Mbps)" type="number" required value={uploadMbps} onChange={(e) => setUploadMbps(e.target.value)}
            error={touched && !uploadMbps ? 'Upload speed is required.' : undefined} />
        </div>
        <label className="flex items-center gap-2 text-xs text-dark-400">
          <input type="checkbox" checked={isBusiness} onChange={(e) => setIsBusiness(e.target.checked)}
            className="rounded border-dark-600 bg-dark-900 text-accent-500" />
          Business plan
        </label>
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
      </form>
    </Modal>
  );
}

function EditPlanModal({
  plan,
  open,
  onClose,
  onSaved,
}: {
  plan: Plan;
  open: boolean;
  onClose: () => void;
  onSaved: () => void;
}) {
  const [name, setName] = useState(plan.name);
  const [slug, setSlug] = useState(plan.slug ?? '');
  const [description, setDescription] = useState(plan.description ?? '');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await updatePlan(plan.id, { name: name.trim(), slug: slug.trim() || undefined, description: description.trim() || undefined });
      toast('Plan updated');
      onSaved();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update plan', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Edit plan" subtitle={plan.name} width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Save changes</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Plan name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Slug" value={slug} onChange={(e) => setSlug(e.target.value)} />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
      </form>
    </Modal>
  );
}

function PricingModal({
  plan,
  open,
  onClose,
  onSaved,
}: {
  plan: Plan;
  open: boolean;
  onClose: () => void;
  onSaved: () => void;
}) {
  const [pricing, setPricing] = useState<PlanPrice[]>(plan.pricing ?? []);
  const [busy, setBusy] = useState(false);
  const [addOpen, setAddOpen] = useState(false);
  const [newPeriod, setNewPeriod] = useState('1');
  const [newPrice, setNewPrice] = useState('');
  const [newSetupFee, setNewSetupFee] = useState('');

  const addTier = () => {
    const period = Number(newPeriod);
    const price = Number(newPrice);
    if (!period || !price) return;
    const tier: PlanPrice = {
      billing_period_months: period,
      monthly_price: price,
      setup_fee: newSetupFee ? Number(newSetupFee) : undefined,
      is_active: true,
    };
    setPricing([...pricing, tier]);
    setNewPeriod('1');
    setNewPrice('');
    setNewSetupFee('');
    setAddOpen(false);
  };

  const removeTier = (index: number) => {
    setPricing(pricing.filter((_, i) => i !== index));
  };

  const save = async () => {
    setBusy(true);
    try {
      await updatePlanPricing(plan.id, { pricing });
      toast('Pricing updated');
      onSaved();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update pricing', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title={`Pricing — ${plan.name}`} subtitle="Manage billing tiers for this plan." width="lg"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={save} loading={busy}>Save pricing</Button>
      </>}>
      <div className="space-y-4">
        {pricing.length > 0 ? (
          <div className="space-y-2">
            {pricing.map((tier, i) => (
              <div key={i} className="flex items-center justify-between gap-4 rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3">
                <div className="flex items-center gap-4">
                  <span className="text-sm font-medium text-white">{tier.billing_period_months} month{tier.billing_period_months !== 1 ? 's' : ''}</span>
                  <span className="tabular-nums text-dark-200">{formatCurrency(tier.monthly_price)}/mo</span>
                  {tier.setup_fee != null && tier.setup_fee > 0 && (
                    <span className="text-xs text-dark-500">Setup: {formatCurrency(tier.setup_fee)}</span>
                  )}
                </div>
                <div className="flex items-center gap-2">
                  <StatusBadge status={tier.is_active !== false ? 'active' : 'inactive'} />
                  <Button variant="ghost" size="sm" onClick={() => removeTier(i)}>
                    <Trash2 className="h-3.5 w-3.5 text-red-400" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <p className="py-6 text-center text-sm text-dark-500">No pricing tiers configured. Add one below.</p>
        )}

        {addOpen ? (
          <div className="rounded-xl border border-white/[0.08] bg-dark-950/40 p-4">
            <p className="mb-3 text-sm font-medium text-white">Add pricing tier</p>
            <div className="grid gap-3 sm:grid-cols-3">
              <SelectField label="Billing period" value={newPeriod} onChange={(e) => setNewPeriod(e.target.value)}
                options={[
                  { value: '1', label: 'Monthly' },
                  { value: '3', label: 'Quarterly' },
                  { value: '6', label: 'Half-yearly' },
                  { value: '12', label: 'Annual' },
                ]} />
              <TextField label="Monthly price (₹)" type="number" required value={newPrice} onChange={(e) => setNewPrice(e.target.value)} />
              <TextField label="Setup fee (₹)" type="number" value={newSetupFee} onChange={(e) => setNewSetupFee(e.target.value)} />
            </div>
            <div className="mt-3 flex justify-end gap-2">
              <Button variant="ghost" size="sm" onClick={() => setAddOpen(false)}>Cancel</Button>
              <Button size="sm" onClick={addTier}>Add tier</Button>
            </div>
          </div>
        ) : (
          <Button variant="secondary" onClick={() => setAddOpen(true)}>
            <Plus className="h-4 w-4" /> Add pricing tier
          </Button>
        )}
      </div>
    </Modal>
  );
}
