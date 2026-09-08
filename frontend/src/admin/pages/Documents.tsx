import { useEffect, useState, useRef } from 'react';
import { FolderOpen, Upload, Trash2, Download, Eye } from 'lucide-react';
import { listDocuments, deleteDocument, getDocumentDownloadUrl } from '../../api/admin/modules';
import type { DocumentRecord } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

function formatSize(bytes: number | undefined): string {
  if (!bytes) return '—';
  if (bytes >= 1048576) return `${(bytes / 1048576).toFixed(1)} MB`;
  if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${bytes} B`;
}

export function DocumentsPage() {
  const [data, setData] = useState<DocumentRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [deleteTarget, setDeleteTarget] = useState<DocumentRecord | null>(null);
  const [deleting, setDeleting] = useState(false);
  const [previewTarget, setPreviewTarget] = useState<DocumentRecord | null>(null);

  const load = async () => {
    setLoading(true);
    try { setData(await listDocuments()); } catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
    finally { setLoading(false); }
  };
  useEffect(() => { void load(); }, []);

  const handleDelete = async () => {
    if (!deleteTarget) return;
    setDeleting(true);
    try { await deleteDocument(deleteTarget.id); toast('Document deleted'); setDeleteTarget(null); void load(); }
    catch (err) { toast(err instanceof Error ? err.message : 'Failed', 'error'); }
    finally { setDeleting(false); }
  };

  const handleDownload = async (doc: DocumentRecord) => {
    try {
      const { download_url } = await getDocumentDownloadUrl(doc.id);
      window.open(download_url, '_blank');
    } catch (err) { toast(err instanceof Error ? err.message : 'Failed to get download URL', 'error'); }
  };

  const columns: Column<DocumentRecord>[] = [
    { key: 'name', header: 'Document', render: (d) => <div><p className="font-medium text-white">{d.name}</p>{d.file_name && <p className="text-xs text-dark-500">{d.file_name}</p>}</div> },
    { key: 'content_type', header: 'Type', render: (d) => <StatusBadge status={d.content_type?.split('/').pop() ?? 'file'} /> },
    { key: 'size_bytes', header: 'Size', render: (d) => <span className="tabular-nums text-dark-300">{formatSize(d.size_bytes)}</span> },
    { key: 'entity_type', header: 'Entity', render: (d) => <span className="text-dark-300">{d.entity_type ? `${d.entity_type} #${d.entity_id ?? ''}` : '—'}</span> },
    { key: 'uploaded_by_name', header: 'Uploaded by', render: (d) => <span className="text-dark-300">{d.uploaded_by_name ?? '—'}</span> },
    { key: 'created_at', header: 'Date', render: (d) => <span className="text-xs text-dark-500">{formatDateTime(d.created_at)}</span> },
    { key: 'actions', header: '', align: 'right', render: (d) => (
      <div className="flex gap-1" onClick={(e) => e.stopPropagation()}>
        <Button variant="ghost" size="sm" onClick={() => void handleDownload(d)} title="Download"><Download className="h-4 w-4" /></Button>
        <Button variant="ghost" size="sm" onClick={() => setDeleteTarget(d)} title="Delete"><Trash2 className="h-3.5 w-3.5 text-red-400" /></Button>
      </div>
    )},
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader title="Documents" subtitle="File storage and document management." icon={<FolderOpen className="h-5 w-5" />} />
        {loading ? <Spinner /> : (
          <DataTable columns={columns} data={data} rowKey={(d) => d.id}
            onRowClick={(d) => setPreviewTarget(d)}
            emptyTitle="No documents" emptyDescription="Uploaded documents will appear here." />
        )}
        <ConfirmDialog open={deleteTarget !== null} onClose={() => setDeleteTarget(null)} onConfirm={handleDelete} loading={deleting}
          title="Delete document?" description={`Permanently delete "${deleteTarget?.name}".`} confirmLabel="Delete" danger />
        <Modal open={previewTarget !== null} onClose={() => setPreviewTarget(null)} title={previewTarget?.name ?? 'Document'} width="lg">
          {previewTarget && (
            <div className="space-y-3">
              <div className="grid gap-3 text-sm sm:grid-cols-3">
                <div><p className="text-xs text-dark-500">File</p><p className="text-dark-200">{previewTarget.file_name ?? '—'}</p></div>
                <div><p className="text-xs text-dark-500">Type</p><p className="text-dark-200">{previewTarget.content_type ?? '—'}</p></div>
                <div><p className="text-xs text-dark-500">Size</p><p className="text-dark-200">{formatSize(previewTarget.size_bytes)}</p></div>
              </div>
              {previewTarget.entity_type && <p className="text-xs text-dark-500">Attached to: {previewTarget.entity_type} #{previewTarget.entity_id}</p>}
              <Button onClick={() => void handleDownload(previewTarget)}><Download className="h-4 w-4" /> Download</Button>
            </div>
          )}
        </Modal>
      </div>
    </PageTransition>
  );
}
