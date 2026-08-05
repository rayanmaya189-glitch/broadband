import { useEffect, useState } from 'react';
import { CheckCircle2, ClipboardCheck, XCircle } from 'lucide-react';
import { approvalAction, listPendingApprovals } from '../../api/admin/modules';
import type { ApprovalRequest } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { TextArea } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, toLabel } from '../lib/format';
import { toast } from '../lib/toast';

export function ApprovalsPage() {
  const [data, setData] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [review, setReview] = useState<{ request: ApprovalRequest; decision: 'approve' | 'reject' } | null>(null);
  const [reason, setReason] = useState('');
  const [busy, setBusy] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listPendingApprovals());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load approvals', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const decide = async () => {
    if (!review) return;
    setBusy(true);
    try {
      await approvalAction(review.request.id, review.decision, reason.trim() || undefined);
      toast(review.decision === 'approve' ? 'Request approved' : 'Request rejected');
      setReview(null);
      setReason('');
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Action failed', 'error');
    } finally {
      setBusy(false);
    }
  };

  const columns: Column<ApprovalRequest>[] = [
    { key: 'operation', header: 'Operation', render: (a) => (
      <div>
        <p className="font-medium text-white">{toLabel(a.operation)}</p>
        <p className="text-xs text-dark-500">{toLabel(a.entity_type)} Â· {a.entity_id ?? 'â€”'}</p>
      </div>
    )},
    { key: 'requested_by_name', header: 'Requested by', render: (a) => <span className="text-dark-300">{a.requested_by_name ?? a.requested_by ?? 'â€”'}</span> },
    { key: 'reason', header: 'Reason', render: (a) => <span className="max-w-md truncate text-dark-400">{a.reason ?? 'â€”'}</span> },
    { key: 'created_at', header: 'Requested', render: (a) => <span className="text-xs text-dark-500">{formatDateTime(a.created_at)}</span> },
    { key: 'status', header: 'Status', render: (a) => <StatusBadge status={a.status} pulse={a.status === 'pending'} /> },
    {
      key: 'actions',
      header: 'Actions',
      align: 'right',
      render: (a) => (
        <div className="flex items-center justify-end gap-1" onClick={(e) => e.stopPropagation()}>
          <Button variant="success" size="sm" onClick={() => setReview({ request: a, decision: 'approve' })}>
            <CheckCircle2 className="h-4 w-4" /> Approve
          </Button>
          <Button variant="danger" size="sm" onClick={() => setReview({ request: a, decision: 'reject' })}>
            <XCircle className="h-4 w-4" /> Reject
          </Button>
        </div>
      ),
    },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Approvals"
          subtitle="Pending workflow approvals awaiting your decision."
          icon={<ClipboardCheck className="h-5 w-5" />}
        />

        {loading ? (
          <Spinner />
        ) : (
          <DataTable
            columns={columns}
            data={data}
            rowKey={(a) => a.id}
            emptyTitle="No pending approvals"
            emptyDescription="Requests needing review will appear here."
          />
        )}

        <Modal
          open={review !== null}
          onClose={() => setReview(null)}
          title={review?.decision === 'approve' ? 'Approve request' : 'Reject request'}
          subtitle={review ? toLabel(review.request.operation) : undefined}
          width="sm"
          footer={<>
            <Button variant="ghost" onClick={() => setReview(null)}>Cancel</Button>
            <Button
              variant={review?.decision === 'approve' ? 'success' : 'danger'}
              onClick={decide}
              loading={busy}
            >
              {review?.decision === 'approve' ? 'Approve' : 'Reject'}
            </Button>
          </>}
        >
          <TextArea
            label="Reason (optional)"
            value={reason}
            onChange={(e) => setReason(e.target.value)}
            placeholder="Add a note for the requesterâ€¦"
          />
        </Modal>
      </div>
    </PageTransition>
  );
}
