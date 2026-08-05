import { useEffect, useState } from 'react';
import { Gauge, Plus, Eye, EyeOff, Trash2 } from 'lucide-react';
import { createPlan, listPlans, publishPlan, deactivatePlan } from '../../api/admin/modules';
import type { Plan } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField, TextArea } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatCurrency } from '../lib/format';
import { toast } from '../lib/toast';

export function PlansPage() {
  const [plans, setPlans] = useState<Plan[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<Plan | null>(null);
  const [deleting, setDeleting] = useState(false);

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
    { key: 'slug', header: 'Slug', render: (p) => <span className="font-mono text-xs text-dark-400">{p.slug ?? 'â€”'}</span> },
    {
      key: 'pricing',
      header: 'Starting at',
      render: (p) => {
        const prices = p.pricing ?? [];
        const min = prices.length ? Math.min(...prices.map((pr) => Number(pr.monthly_price) || 0)) : 0;
        return <span className="tabular-nums text-dark-200">{prices.length ? `${formatCurrency(min)}/mo` : 'â€”'}</span>;
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
  const [busy, setBusy] = useState(false);
  const [touched, setTouched] = useState(false);

  const reset = () => {
    setName('');
    setSlug('');
    setDescription('');
    setTouched(false);
  };

  const submit = async () => {
    setTouched(true);
    if (!name.trim()) return;
    setBusy(true);
    try {
      await createPlan({ name: name.trim(), slug: slug.trim() || undefined, description: description.trim() || undefined });
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
      onClose={() => {
        reset();
        onClose();
      }}
      title="New plan"
      subtitle="Create a broadband plan. Pricing tiers are configured by plan administrators."
      width="md"
      footer={
        <>
          <Button variant="ghost" onClick={onClose}>Cancel</Button>
          <Button onClick={submit} loading={busy}>Create plan</Button>
        </>
      }
    >
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void submit();
        }}
        className="space-y-4"
        noValidate
      >
        <TextField
          label="Plan name"
          required
          value={name}
          onChange={(e) => setName(e.target.value)}
          error={touched && !name.trim() ? 'Plan name is required.' : undefined}
        />
        <TextField
          label="Slug"
          value={slug}
          onChange={(e) => setSlug(e.target.value)}
          hint="URL-friendly identifier, e.g. home-200."
        />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
      </form>
    </Modal>
  );
}
