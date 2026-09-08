import { useEffect, useState } from 'react';
import { Clock, Plus, Play, Trash2, Edit3, RotateCw, CheckCircle2, XCircle } from 'lucide-react';
import {
  listSchedulerJobs,
  createSchedulerJob,
  updateSchedulerJob,
  deleteSchedulerJob,
  triggerSchedulerJob,
  listSchedulerExecutions,
  getSchedulerStats,
} from '../../api/admin/modules';
import type { SchedulerJob, SchedulerExecution, SchedulerStats } from '../../types/admin';
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

type Tab = 'jobs' | 'executions' | 'stats';

export function SchedulerPage() {
  const [jobs, setJobs] = useState<SchedulerJob[]>([]);
  const [executions, setExecutions] = useState<SchedulerExecution[]>([]);
  const [stats, setStats] = useState<SchedulerStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<Tab>('jobs');
  const [createOpen, setCreateOpen] = useState(false);
  const [editTarget, setEditTarget] = useState<SchedulerJob | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<SchedulerJob | null>(null);
  const [deleting, setDeleting] = useState(false);
  const [triggering, setTriggering] = useState<number | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [j, e, s] = await Promise.all([
        listSchedulerJobs().catch(() => []),
        listSchedulerExecutions().catch(() => []),
        getSchedulerStats().catch(() => null),
      ]);
      setJobs(j);
      setExecutions(e);
      if (s) setStats(s);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load scheduler data', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    setDeleting(true);
    try {
      await deleteSchedulerJob(deleteTarget.id);
      toast('Job deleted');
      setDeleteTarget(null);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to delete job', 'error');
    } finally {
      setDeleting(false);
    }
  };

  const handleTrigger = async (job: SchedulerJob) => {
    setTriggering(job.id);
    try {
      await triggerSchedulerJob(job.id);
      toast(`Job "${job.name}" triggered`);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to trigger job', 'error');
    } finally {
      setTriggering(null);
    }
  };

  const jobColumns: Column<SchedulerJob>[] = [
    { key: 'name', header: 'Job', render: (j) => (
      <div>
        <p className="font-medium text-white">{j.name}</p>
        {j.description && <p className="max-w-md truncate text-xs text-dark-500">{j.description}</p>}
      </div>
    )},
    { key: 'job_type', header: 'Type', render: (j) => <StatusBadge status={j.job_type} /> },
    { key: 'schedule', header: 'Schedule', render: (j) => (
      <span className="font-mono text-xs text-dark-300">{j.schedule ?? '—'}</span>
    )},
    { key: 'target_module', header: 'Module', render: (j) => <span className="text-dark-400">{j.target_module ?? '—'}</span> },
    { key: 'is_active', header: 'Status', render: (j) => <StatusBadge status={j.is_active !== false ? 'active' : 'inactive'} /> },
    { key: 'last_run_at', header: 'Last run', render: (j) => (
      <div>
        <span className="text-xs text-dark-500">{formatDateTime(j.last_run_at)}</span>
        {j.last_status && <StatusBadge status={j.last_status} />}
      </div>
    )},
    { key: 'next_run_at', header: 'Next run', render: (j) => <span className="text-xs text-dark-500">{formatDateTime(j.next_run_at)}</span> },
    {
      key: 'actions', header: 'Actions', align: 'right',
      render: (j) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="accent" size="sm" loading={triggering === j.id} onClick={() => void handleTrigger(j)} title="Trigger now">
            <Play className="h-3.5 w-3.5" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setEditTarget(j)} title="Edit">
            <Edit3 className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={() => setDeleteTarget(j)} title="Delete">
            <Trash2 className="h-4 w-4 text-red-400" />
          </Button>
        </div>
      ),
    },
  ];

  const executionColumns: Column<SchedulerExecution>[] = [
    { key: 'job_name', header: 'Job', render: (e) => <span className="font-medium text-white">{e.job_name ?? `#${e.job_id}`}</span> },
    { key: 'status', header: 'Status', render: (e) => <StatusBadge status={e.status} /> },
    { key: 'started_at', header: 'Started', render: (e) => <span className="text-xs text-dark-500">{formatDateTime(e.started_at)}</span> },
    { key: 'finished_at', header: 'Finished', render: (e) => <span className="text-xs text-dark-500">{formatDateTime(e.finished_at)}</span> },
    { key: 'duration_ms', header: 'Duration', render: (e) => (
      <span className="tabular-nums text-dark-300">{e.duration_ms != null ? `${e.duration_ms}ms` : '—'}</span>
    )},
    { key: 'error_message', header: 'Error', render: (e) => (
      <span className="max-w-xs truncate text-xs text-red-400">{e.error_message ?? '—'}</span>
    )},
  ];

  const tabs: { key: Tab; label: string }[] = [
    { key: 'jobs', label: `Jobs (${jobs.length})` },
    { key: 'executions', label: `Executions (${executions.length})` },
    { key: 'stats', label: 'Stats' },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Scheduler & jobs"
          subtitle="Background job scheduling, execution history and statistics."
          icon={<Clock className="h-5 w-5" />}
          actions={
            <Button onClick={() => setCreateOpen(true)}>
              <Plus className="h-4 w-4" /> New job
            </Button>
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
        ) : tab === 'jobs' ? (
          <DataTable columns={jobColumns} data={jobs} rowKey={(j) => j.id} emptyTitle="No jobs" emptyDescription="Create scheduled jobs to automate background tasks." />
        ) : tab === 'executions' ? (
          <DataTable columns={executionColumns} data={executions} rowKey={(e) => e.id} emptyTitle="No executions" emptyDescription="Job execution history will appear here." />
        ) : (
          <StatsView stats={stats} />
        )}

        <CreateJobModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />
        {editTarget && (
          <EditJobModal job={editTarget} open={editTarget !== null} onClose={() => setEditTarget(null)} onSaved={() => { setEditTarget(null); void load(); }} />
        )}
        <ConfirmDialog open={deleteTarget !== null} onClose={() => setDeleteTarget(null)} onConfirm={confirmDelete} loading={deleting}
          title="Delete job?" description={`Permanently delete job "${deleteTarget?.name}".`} confirmLabel="Delete job" danger />
      </div>
    </PageTransition>
  );
}

function StatsView({ stats }: { stats: SchedulerStats | null }) {
  if (!stats) {
    return (
      <div className="glass-card flex flex-col items-center gap-3 rounded-2xl py-12 text-dark-400">
        <Clock className="h-8 w-8" />
        <p className="text-sm">Stats unavailable.</p>
      </div>
    );
  }

  const cards = [
    { label: 'Total jobs', value: stats.total_jobs, icon: <Clock className="h-5 w-5" />, color: 'primary' },
    { label: 'Active jobs', value: stats.active_jobs, icon: <CheckCircle2 className="h-5 w-5" />, color: 'success' },
    { label: 'Total executions', value: stats.total_executions, icon: <RotateCw className="h-5 w-5" />, color: 'accent' },
    { label: 'Successful', value: stats.successful_executions, icon: <CheckCircle2 className="h-5 w-5" />, color: 'success' },
    { label: 'Failed', value: stats.failed_executions, icon: <XCircle className="h-5 w-5" />, color: 'danger' },
    { label: 'Avg duration', value: stats.avg_duration_ms != null ? `${stats.avg_duration_ms}ms` : '—', icon: <Clock className="h-5 w-5" />, color: 'accent' },
  ];

  return (
    <div className="grid gap-4 sm:grid-cols-3">
      {cards.map((card) => (
        <div key={card.label} className="glass-card flex items-center gap-4 rounded-2xl p-5">
          <div className={`flex h-10 w-10 items-center justify-center rounded-xl border ${
            card.color === 'success' ? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-400' :
            card.color === 'danger' ? 'border-red-500/30 bg-red-500/10 text-red-400' :
            card.color === 'accent' ? 'border-accent-500/30 bg-accent-500/10 text-accent-400' :
            'border-primary-500/30 bg-primary-500/10 text-primary-400'
          }`}>
            {card.icon}
          </div>
          <div>
            <p className="text-2xl font-semibold tabular-nums text-white">
              {typeof card.value === 'number' ? card.value.toLocaleString() : card.value}
            </p>
            <p className="text-xs text-dark-400">{card.label}</p>
          </div>
        </div>
      ))}
    </div>
  );
}

const TARGET_MODULES = [
  { value: 'billing', label: 'Billing worker' },
  { value: 'notification', label: 'Notification worker' },
  { value: 'device_sync', label: 'Device sync worker' },
  { value: 'bandwidth', label: 'Bandwidth worker' },
  { value: 'monitoring', label: 'Monitoring worker' },
  { value: 'radius', label: 'RADIUS accounting worker' },
  { value: 'subscription', label: 'Subscription renewal sweep' },
  { value: 'cleanup', label: 'Outbox cleanup' },
];

function CreateJobModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [jobType, setJobType] = useState('cron');
  const [cronExpr, setCronExpr] = useState('');
  const [intervalMins, setIntervalMins] = useState('');
  const [targetModule, setTargetModule] = useState('billing');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    const schedule = jobType === 'cron' ? cronExpr.trim() : `${Number(intervalMins) || 0} minutes`;
    if (!name.trim() || !schedule) return;
    setBusy(true);
    try {
      await createSchedulerJob({
        name: name.trim(),
        description: description.trim() || undefined,
        job_type: jobType,
        schedule,
        target_module: targetModule,
        action: 'run_cycle',
      });
      toast('Job created');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to create job', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="New scheduled job" subtitle="Create a background job with a schedule." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Create job</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Job name" required value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. Billing cycle" />
        <TextArea label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
        <SelectField label="Target module" value={targetModule} onChange={(e) => setTargetModule(e.target.value)}
          options={TARGET_MODULES} />
        <SelectField label="Schedule type" value={jobType} onChange={(e) => setJobType(e.target.value)}
          options={[
            { value: 'cron', label: 'Cron expression' },
            { value: 'interval', label: 'Fixed interval' },
          ]}
        />
        {jobType === 'cron' ? (
          <TextField label="Cron expression" required value={cronExpr} onChange={(e) => setCronExpr(e.target.value)}
            placeholder="0 */5 * * *" hint="e.g. 0 */5 * * * = every 5 minutes" />
        ) : (
          <TextField label="Interval (minutes)" type="number" required value={intervalMins} onChange={(e) => setIntervalMins(e.target.value)}
            placeholder="5" hint="e.g. 5 = every 5 minutes" />
        )}
      </form>
    </Modal>
  );
}

function EditJobModal({ job, open, onClose, onSaved }: { job: SchedulerJob; open: boolean; onClose: () => void; onSaved: () => void }) {
  const rawSchedule = job.schedule ?? '';
  const isInterval = /(minutes|hours)/.test(rawSchedule);
  const [jobType, setJobType] = useState(job.job_type === 'interval' || isInterval ? 'interval' : 'cron');
  const [cronExpr, setCronExpr] = useState(isInterval ? '' : rawSchedule);
  const [intervalMins, setIntervalMins] = useState(isInterval ? (rawSchedule.match(/(\d+)/)?.[1] ?? '') : '');
  const [isActive, setIsActive] = useState(job.is_active !== false);
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    const schedule = jobType === 'cron' ? cronExpr.trim() : `${Number(intervalMins) || 0} minutes`;
    if (!schedule) return;
    setBusy(true);
    try {
      await updateSchedulerJob(job.id, { schedule, is_active: isActive });
      toast('Job updated');
      onSaved();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update job', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Edit job" subtitle={job.name} width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Save changes</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <p className="rounded-lg border border-white/[0.06] bg-white/[0.02] px-3 py-2 text-xs text-dark-400">
          Module: <span className="text-dark-200">{job.target_module ?? '—'}</span> · Action: <span className="text-dark-200">{job.action ?? '—'}</span>
        </p>
        <SelectField label="Schedule type" value={jobType} onChange={(e) => setJobType(e.target.value)}
          options={[
            { value: 'cron', label: 'Cron expression' },
            { value: 'interval', label: 'Fixed interval' },
          ]}
        />
        {jobType === 'cron' ? (
          <TextField label="Cron expression" required value={cronExpr} onChange={(e) => setCronExpr(e.target.value)} />
        ) : (
          <TextField label="Interval (minutes)" type="number" required value={intervalMins} onChange={(e) => setIntervalMins(e.target.value)} />
        )}
        <label className="flex items-center gap-2 text-sm text-dark-300">
          <input type="checkbox" checked={isActive} onChange={(e) => setIsActive(e.target.checked)}
            className="rounded border-dark-600 bg-dark-900 text-accent-500" />
          Active
        </label>
      </form>
    </Modal>
  );
}
