import { useEffect, useState } from 'react';
import { Key, Plus, Trash2, Eye, EyeOff, Activity } from 'lucide-react';
import { listApiKeys, createApiKey, revokeApiKey, listRateLimitRules, createRateLimitRule, deleteRateLimitRule, listRequestLogs, getGatewayStats } from '../../api/admin/modules';
import type { ApiKey, RateLimitRule, RequestLog, GatewayStats } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'apikeys' | 'ratelimits' | 'logs' | 'stats';

export function GatewayPage() {
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([]);
  const [rateLimits, setRateLimits] = useState<RateLimitRule[]>([]);
  const [logs, setLogs] = useState<RequestLog[]>([]);
  const [stats, setStats] = useState<GatewayStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<Tab>('apikeys');
  const [keyCreateOpen, setKeyCreateOpen] = useState(false);
  const [keyDeleteTarget, setKeyDeleteTarget] = useState<ApiKey | null>(null);
  const [ruleCreateOpen, setRuleCreateOpen] = useState(false);
  const [ruleDeleteTarget, setRuleDeleteTarget] = useState<RateLimitRule | null>(null);
  const [deleting, setDeleting] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const [k, r, l, s] = await Promise.all([
        listApiKeys().catch(() => []), listRateLimitRules().catch(() => []),
        listRequestLogs().catch(() => []), getGatewayStats().catch(() => null),
      ]);
      setApiKeys(k); setRateLimits(r); setLogs(l); if (s) setStats(s);
    } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
    finally { setLoading(false); }
  };
  useEffect(() => { void load(); }, []);

  const handleRevokeKey = async () => { if (!keyDeleteTarget) return; setDeleting(true); try { await revokeApiKey(keyDeleteTarget.id); toast('API key revoked'); setKeyDeleteTarget(null); void load(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setDeleting(false); } };
  const handleDeleteRule = async () => { if (!ruleDeleteTarget) return; setDeleting(true); try { await deleteRateLimitRule(ruleDeleteTarget.id); toast('Rule deleted'); setRuleDeleteTarget(null); void load(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setDeleting(false); } };

  const keyColumns: Column<ApiKey>[] = [
    { key: 'name', header: 'Key', render: (k) => <div><p className="font-medium text-white">{k.name}</p><p className="font-mono text-xs text-dark-500">{k.key_prefix ?? '—'}…</p></div> },
    { key: 'scopes', header: 'Scopes', render: (k) => <div className="flex flex-wrap gap-1">{(k.scopes ?? []).map((s) => <StatusBadge key={s} status={s} />)}</div> },
    { key: 'is_active', header: 'Status', render: (k) => <StatusBadge status={k.is_active !== false ? 'active' : 'revoked'} /> },
    { key: 'last_used_at', header: 'Last used', render: (k) => <span className="text-xs text-dark-500">{formatDateTime(k.last_used_at)}</span> },
    { key: 'expires_at', header: 'Expires', render: (k) => <span className="text-xs text-dark-500">{formatDateTime(k.expires_at)}</span> },
    { key: 'actions', header: '', align: 'right', render: (k) => k.is_active !== false ? (
      <div onClick={(e) => e.stopPropagation()}><Button variant="danger" size="sm" onClick={() => setKeyDeleteTarget(k)}>Revoke</Button></div>
    ) : null },
  ];

  const ruleColumns: Column<RateLimitRule>[] = [
    { key: 'route_pattern', header: 'Rule / Path', render: (r) => <span className="font-mono text-xs text-dark-200">{r.route_pattern}</span> },
    { key: 'max_requests', header: 'Max req', render: (r) => <span className="tabular-nums text-dark-200">{r.max_requests}</span> },
    { key: 'window_seconds', header: 'Window', render: (r) => <span className="tabular-nums text-dark-200">{r.window_seconds}s</span> },
    { key: 'is_active', header: 'Status', render: (r) => <StatusBadge status={r.is_active !== false ? 'active' : 'inactive'} /> },
    { key: 'actions', header: '', align: 'right', render: (r) => (
      <div onClick={(e) => e.stopPropagation()}><Button variant="ghost" size="sm" onClick={() => setRuleDeleteTarget(r)}><Trash2 className="h-3.5 w-3.5 text-red-400" /></Button></div>
    ) },
  ];

  const logColumns: Column<RequestLog>[] = [
    { key: 'method', header: 'Method', render: (l) => <StatusBadge status={l.method} /> },
    { key: 'path', header: 'Path', render: (l) => <span className="font-mono text-xs text-dark-300 truncate max-w-xs block">{l.path}</span> },
    { key: 'status_code', header: 'Status', render: (l) => <StatusBadge status={l.status_code >= 200 && l.status_code < 300 ? 'success' : l.status_code >= 400 ? 'error' : 'warning'} /> },
    { key: 'ip_address', header: 'IP', render: (l) => <span className="font-mono text-xs text-dark-400">{l.ip_address ?? '—'}</span> },
    { key: 'duration_ms', header: 'Duration', render: (l) => <span className="tabular-nums text-dark-300">{l.duration_ms != null ? `${l.duration_ms}ms` : '—'}</span> },
    { key: 'created_at', header: 'Time', render: (l) => <span className="text-xs text-dark-500">{formatDateTime(l.created_at)}</span> },
  ];

  const tabs: { key: Tab; label: string }[] = [
    { key: 'apikeys', label: `API keys (${apiKeys.length})` },
    { key: 'ratelimits', label: `Rate limits (${rateLimits.length})` },
    { key: 'logs', label: `Request logs (${logs.length})` },
    { key: 'stats', label: 'Stats' },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader title="API Gateway" subtitle="API keys, rate limiting, request logs and usage stats." icon={<Key className="h-5 w-5" />}
          actions={
            <>
              {tab === 'apikeys' && <Button onClick={() => setKeyCreateOpen(true)}><Plus className="h-4 w-4" /> New API key</Button>}
              {tab === 'ratelimits' && <Button onClick={() => setRuleCreateOpen(true)}><Plus className="h-4 w-4" /> New rule</Button>}
            </>
          }
        />
        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {tabs.map((t) => (
            <button key={t.key} onClick={() => setTab(t.key)} className={`rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors ${tab === t.key ? 'bg-primary-500/20 text-primary-200' : 'text-dark-400 hover:text-white'}`}>{t.label}</button>
          ))}
        </div>
        {loading ? <Spinner /> : tab === 'apikeys' ? (
          <DataTable columns={keyColumns} data={apiKeys} rowKey={(k) => k.id} emptyTitle="No API keys" emptyDescription="Create API keys for external integrations." />
        ) : tab === 'ratelimits' ? (
          <DataTable columns={ruleColumns} data={rateLimits} rowKey={(r) => r.id} emptyTitle="No rate limit rules" emptyDescription="Add rules to protect endpoints." />
        ) : tab === 'logs' ? (
          <DataTable columns={logColumns} data={logs} rowKey={(l) => l.id} emptyTitle="No request logs" emptyDescription="API request logs will appear here." />
        ) : stats ? (
          <div className="grid gap-4 sm:grid-cols-3">
            {[
              { label: 'Total requests', value: stats.total_requests.toLocaleString() },
              { label: 'Successful', value: stats.successful_requests.toLocaleString() },
              { label: 'Failed', value: stats.failed_requests.toLocaleString() },
              { label: 'Avg response', value: `${stats.avg_response_time_ms}ms` },
              { label: 'Active keys', value: stats.active_api_keys.toLocaleString() },
              { label: 'Rate limited', value: stats.rate_limited_requests.toLocaleString() },
            ].map((c) => (
              <div key={c.label} className="glass-card rounded-2xl p-5 text-center">
                <p className="text-2xl font-semibold tabular-nums text-white">{c.value}</p>
                <p className="text-xs text-dark-400">{c.label}</p>
              </div>
            ))}
          </div>
        ) : <Spinner />}

        <CreateKeyModal open={keyCreateOpen} onClose={() => setKeyCreateOpen(false)} onCreated={() => { setKeyCreateOpen(false); void load(); }} />
        <ConfirmDialog open={keyDeleteTarget !== null} onClose={() => setKeyDeleteTarget(null)} onConfirm={handleRevokeKey} loading={deleting} title="Revoke API key?" description={`Revoke "${keyDeleteTarget?.name}". This cannot be undone.`} confirmLabel="Revoke" danger />
        <CreateRuleModal open={ruleCreateOpen} onClose={() => setRuleCreateOpen(false)} onCreated={() => { setRuleCreateOpen(false); void load(); }} />
        <ConfirmDialog open={ruleDeleteTarget !== null} onClose={() => setRuleDeleteTarget(null)} onConfirm={handleDeleteRule} loading={deleting} title="Delete rule?" description={`Delete rate limit rule "${ruleDeleteTarget?.name}".`} confirmLabel="Delete" danger />
      </div>
    </PageTransition>
  );
}

function CreateKeyModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState(''); const [busy, setBusy] = useState(false);
  const submit = async () => { if (!name.trim()) return; setBusy(true); try { await createApiKey({ name: name.trim(), permissions: 'read' }); toast('API key created'); onCreated(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setBusy(false); } };
  return (
    <Modal open={open} onClose={onClose} title="New API key" width="sm" footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Create</Button></>}>
      <TextField label="Key name" required value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. Payment webhook" />
    </Modal>
  );
}

function CreateRuleModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState(''); const [path, setPath] = useState(''); const [maxReqs, setMaxReqs] = useState(''); const [windowSecs, setWindowSecs] = useState('60'); const [busy, setBusy] = useState(false);
  const submit = async () => { if (!path.trim()) return; setBusy(true); try { await createRateLimitRule({ route_pattern: path.trim(), methods: 'ALL', max_requests: Number(maxReqs) || 100, window_seconds: Number(windowSecs) || 60 }); toast('Rule created'); onCreated(); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); } finally { setBusy(false); } };
  return (
    <Modal open={open} onClose={onClose} title="New rate limit rule" width="md" footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Create rule</Button></>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Rule name" required value={name} onChange={(e) => setName(e.target.value)} />
        <TextField label="Path pattern" required value={path} onChange={(e) => setPath(e.target.value)} placeholder="/api/v1/auth/*" />
        <div className="grid gap-4 sm:grid-cols-2">
          <TextField label="Max requests" type="number" value={maxReqs} onChange={(e) => setMaxReqs(e.target.value)} placeholder="100" />
          <TextField label="Window (seconds)" type="number" value={windowSecs} onChange={(e) => setWindowSecs(e.target.value)} />
        </div>
      </form>
    </Modal>
  );
}
