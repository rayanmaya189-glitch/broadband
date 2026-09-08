import { useEffect, useState } from 'react';
import { FileCheck, ShieldCheck, Database, Plus } from 'lucide-react';
import {
  listKycVerifications,
  updateKycStatus,
  createKycVerification,
  listConsents,
  grantConsent,
  revokeConsent,
  listRetentionPolicies,
  createRetentionPolicy,
} from '../../api/admin/modules';
import type { KycVerification, Consent, RetentionPolicy } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

type Tab = 'kyc' | 'consents' | 'retention';

export function CompliancePage() {
  const [tab, setTab] = useState<Tab>('kyc');
  const [kycData, setKycData] = useState<KycVerification[]>([]);
  const [consents, setConsents] = useState<Consent[]>([]);
  const [retentionPolicies, setRetentionPolicies] = useState<RetentionPolicy[]>([]);
  const [loading, setLoading] = useState(true);
  const [kycCreateOpen, setKycCreateOpen] = useState(false);
  const [consentCustomerId, setConsentCustomerId] = useState('');
  const [consentOpen, setConsentOpen] = useState(false);
  const [retentionOpen, setRetentionOpen] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const [k, r] = await Promise.all([
        listKycVerifications().catch(() => []),
        listRetentionPolicies().catch(() => []),
      ]);
      setKycData(k);
      setRetentionPolicies(r);
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load compliance data', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { void load(); }, []);

  const loadConsents = async (customerId: number) => {
    try { setConsents(await listConsents(customerId)); } catch { setConsents([]); }
  };

  const handleKycStatus = async (id: number, status: string) => {
    try { await updateKycStatus(id, status); toast(`KYC ${status}`); void load(); }
    catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
  };

  const kycColumns: Column<KycVerification>[] = [
    { key: 'customer_name', header: 'Customer', render: (k) => <span className="font-medium text-white">{k.customer_name ?? `#${k.customer_id}`}</span> },
    { key: 'document_type', header: 'Document', render: (k) => <StatusBadge status={k.document_type} /> },
    { key: 'document_number', header: 'Number', render: (k) => <span className="font-mono text-xs text-dark-300">{k.document_number ?? '—'}</span> },
    { key: 'status', header: 'Status', render: (k) => <StatusBadge status={k.status} pulse={k.status === 'pending'} /> },
    { key: 'verified_at', header: 'Verified', render: (k) => <span className="text-xs text-dark-500">{formatDateTime(k.verified_at)}</span> },
    {
      key: 'actions', header: '', align: 'right',
      render: (k) => k.status === 'pending' ? (
        <div className="flex gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="success" size="sm" onClick={() => void handleKycStatus(k.id, 'approved')}>Approve</Button>
          <Button variant="danger" size="sm" onClick={() => void handleKycStatus(k.id, 'rejected')}>Reject</Button>
        </div>
      ) : null,
    },
  ];

  const consentColumns: Column<Consent>[] = [
    { key: 'customer_name', header: 'Customer', render: (c) => <span className="text-dark-200">{c.customer_name ?? `#${c.customer_id}`}</span> },
    { key: 'consent_type', header: 'Type', render: (c) => <StatusBadge status={c.consent_type} /> },
    { key: 'granted', header: 'Status', render: (c) => <StatusBadge status={c.granted ? 'granted' : 'revoked'} /> },
    { key: 'granted_at', header: 'Granted', render: (c) => <span className="text-xs text-dark-500">{formatDateTime(c.granted_at)}</span> },
    { key: 'revoked_at', header: 'Revoked', render: (c) => <span className="text-xs text-dark-500">{formatDateTime(c.revoked_at)}</span> },
  ];

  const retentionColumns: Column<RetentionPolicy>[] = [
    { key: 'name', header: 'Policy', render: (r) => <span className="font-medium text-white">{r.name}</span> },
    { key: 'entity_type', header: 'Entity', render: (r) => <StatusBadge status={r.entity_type} /> },
    { key: 'retention_days', header: 'Retention', render: (r) => <span className="tabular-nums text-dark-200">{r.retention_days} days</span> },
    { key: 'action', header: 'Action', render: (r) => <StatusBadge status={r.action} /> },
    { key: 'is_active', header: 'Status', render: (r) => <StatusBadge status={r.is_active !== false ? 'active' : 'inactive'} /> },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader title="Compliance" subtitle="KYC verifications, consent management and data retention." icon={<FileCheck className="h-5 w-5" />}
          actions={
            <>
              {tab === 'kyc' && <Button onClick={() => setKycCreateOpen(true)}><Plus className="h-4 w-4" /> New KYC</Button>}
              {tab === 'consents' && <Button onClick={() => setConsentOpen(true)}><Plus className="h-4 w-4" /> Load consents</Button>}
              {tab === 'retention' && <Button onClick={() => setRetentionOpen(true)}><Plus className="h-4 w-4" /> New policy</Button>}
            </>
          }
        />

        <div className="flex flex-wrap gap-1 rounded-xl border border-white/[0.06] bg-white/[0.02] p-1">
          {([['kyc', 'KYC'], ['consents', 'Consents'], ['retention', 'Retention']] as [Tab, string][]).map(([key, label]) => (
            <button key={key} onClick={() => setTab(key)}
              className={`rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors ${tab === key ? 'bg-primary-500/20 text-primary-200' : 'text-dark-400 hover:text-white'}`}>
              {label}
            </button>
          ))}
        </div>

        {loading ? <Spinner /> : tab === 'kyc' ? (
          <DataTable columns={kycColumns} data={kycData} rowKey={(k) => k.id} emptyTitle="No KYC records" emptyDescription="KYC verifications will appear here." />
        ) : tab === 'consents' ? (
          <DataTable columns={consentColumns} data={consents} rowKey={(c) => c.id} emptyTitle="No consents" emptyDescription="Load consents by customer ID." />
        ) : (
          <DataTable columns={retentionColumns} data={retentionPolicies} rowKey={(r) => r.id} emptyTitle="No retention policies" emptyDescription="Create data retention policies." />
        )}

        <KycCreateModal open={kycCreateOpen} onClose={() => setKycCreateOpen(false)} onCreated={() => { setKycCreateOpen(false); void load(); }} />
        <ConsentLoadModal open={consentOpen} onClose={() => setConsentOpen(false)} customerId={consentCustomerId} setCustomerId={setConsentCustomerId} onLoad={loadConsents} />
        <RetentionCreateModal open={retentionOpen} onClose={() => setRetentionOpen(false)} onCreated={() => { setRetentionOpen(false); void load(); }} />
      </div>
    </PageTransition>
  );
}

function KycCreateModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [customerId, setCustomerId] = useState('');
  const [docType, setDocType] = useState('aadhaar');
  const [docNumber, setDocNumber] = useState('');
  const [busy, setBusy] = useState(false);
  const submit = async () => {
    if (!customerId || !docType) return;
    setBusy(true);
    try { await createKycVerification({ customer_id: Number(customerId), document_type: docType, document_number: docNumber.trim() || undefined, status: 'pending' }); toast('KYC submitted'); onCreated(); }
    catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
    finally { setBusy(false); }
  };
  return (
    <Modal open={open} onClose={onClose} title="New KYC verification" width="md"
      footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Submit</Button></>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Customer ID" type="number" required value={customerId} onChange={(e) => setCustomerId(e.target.value)} />
        <SelectField label="Document type" value={docType} onChange={(e) => setDocType(e.target.value)}
          options={[{ value: 'aadhaar', label: 'Aadhaar' }, { value: 'pan', label: 'PAN' }, { value: 'passport', label: 'Passport' }, { value: 'driving_license', label: 'Driving License' }]} />
        <TextField label="Document number" value={docNumber} onChange={(e) => setDocNumber(e.target.value)} />
      </form>
    </Modal>
  );
}

function ConsentLoadModal({ open, onClose, customerId, setCustomerId, onLoad }: { open: boolean; onClose: () => void; customerId: string; setCustomerId: (v: string) => void; onLoad: (id: number) => void }) {
  return (
    <Modal open={open} onClose={onClose} title="Load consents" width="sm"
      footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={() => { if (customerId) { onLoad(Number(customerId)); onClose(); } }} disabled={!customerId}>Load</Button></>}>
      <TextField label="Customer ID" type="number" required value={customerId} onChange={(e) => setCustomerId(e.target.value)} />
    </Modal>
  );
}

function RetentionCreateModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [entityType, setEntityType] = useState('kyc_document');
  const [retentionDays, setRetentionDays] = useState('2555');
  const [action, setAction] = useState('anonymize');
  const [busy, setBusy] = useState(false);
  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try { await createRetentionPolicy({ name: name.trim(), entity_type: entityType, retention_days: Number(retentionDays), action, is_active: true }); toast('Policy created'); onCreated(); }
    catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
    finally { setBusy(false); }
  };
  return (
    <Modal open={open} onClose={onClose} title="New retention policy" width="md"
      footer={<><Button variant="ghost" onClick={onClose}>Cancel</Button><Button onClick={submit} loading={busy}>Create</Button></>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Policy name" required value={name} onChange={(e) => setName(e.target.value)} placeholder="e.g. KYC docs retention" />
        <SelectField label="Entity type" value={entityType} onChange={(e) => setEntityType(e.target.value)}
          options={[{ value: 'kyc_document', label: 'KYC documents' }, { value: 'consent', label: 'Consent records' }, { value: 'audit_log', label: 'Audit logs' }, { value: 'invoice', label: 'Invoices' }]} />
        <TextField label="Retention (days)" type="number" value={retentionDays} onChange={(e) => setRetentionDays(e.target.value)} />
        <SelectField label="Action after retention" value={action} onChange={(e) => setAction(e.target.value)}
          options={[{ value: 'delete', label: 'Delete' }, { value: 'anonymize', label: 'Anonymize' }, { value: 'archive', label: 'Archive' }]} />
      </form>
    </Modal>
  );
}
