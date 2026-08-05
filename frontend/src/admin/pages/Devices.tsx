import { useEffect, useState } from 'react';
import { Router, Plus, Power, RotateCw } from 'lucide-react';
import {
  deviceAction,
  listDevices,
  listDeviceMetrics,
  registerDevice,
  updateDeviceStatus,
} from '../../api/admin/modules';
import type { DeviceMetricRow, NetworkDevice } from '../../types/admin';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { DataTable, type Column } from '../components/ui/DataTable';
import { Modal } from '../components/ui/Modal';
import { ConfirmDialog } from '../components/ui/ConfirmDialog';
import { TextField, SelectField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime } from '../lib/format';
import { toast } from '../lib/toast';

export function DevicesPage() {
  const [data, setData] = useState<NetworkDevice[]>([]);
  const [loading, setLoading] = useState(true);
  const [createOpen, setCreateOpen] = useState(false);
  const [detail, setDetail] = useState<NetworkDevice | null>(null);
  const [metrics, setMetrics] = useState<DeviceMetricRow[]>([]);
  const [detailLoading, setDetailLoading] = useState(false);
  const [confirm, setConfirm] = useState<{ device: NetworkDevice; action: 'restart' | 'shutdown' } | null>(null);
  const [acting, setActing] = useState<{ id: number; action: string } | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      setData(await listDevices());
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to load devices', 'error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const openDetail = async (device: NetworkDevice) => {
    setDetail(device);
    setDetailLoading(true);
    setMetrics([]);
    try {
      setMetrics(await listDeviceMetrics(device.id));
    } catch {
      setMetrics([]);
    } finally {
      setDetailLoading(false);
    }
  };

  const setStatus = async (device: NetworkDevice, status: string) => {
    setActing({ id: device.id, action: `status:${status}` });
    try {
      await updateDeviceStatus(device.id, status);
      toast(`Device marked ${status}`);
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to update status', 'error');
    } finally {
      setActing(null);
    }
  };

  const runPower = async (device: NetworkDevice, action: 'restart' | 'shutdown') => {
    setConfirm(null);
    setActing({ id: device.id, action });
    try {
      await deviceAction(device.id, action);
      toast(action === 'restart' ? 'Restart command sent' : 'Shutdown command sent', action === 'restart' ? 'info' : 'error');
      void load();
    } catch (err) {
      toast(err instanceof Error ? err.message : `Failed to ${action} device`, 'error');
    } finally {
      setActing(null);
    }
  };

  const columns: Column<NetworkDevice>[] = [
    { key: 'name', header: 'Device', render: (d) => (
      <div>
        <p className="font-medium text-white">{d.name}</p>
        <p className="text-xs text-dark-500">{d.device_type} Â· {d.model ?? 'â€”'}</p>
      </div>
    )},
    { key: 'ip_address', header: 'IP address', render: (d) => <span className="font-mono text-xs text-dark-300">{d.ip_address ?? 'â€”'}</span> },
    { key: 'mac_address', header: 'MAC', render: (d) => <span className="font-mono text-[11px] text-dark-500">{d.mac_address ?? 'â€”'}</span> },
    { key: 'branch_id', header: 'Branch', render: (d) => <span className="text-dark-400">{d.branch_id ? `#${d.branch_id}` : 'â€”'}</span> },
    { key: 'status', header: 'Status', render: (d) => <StatusBadge status={d.status} pulse={d.status === 'online'} /> },
    { key: 'firmware_version', header: 'Firmware', render: (d) => <span className="text-xs text-dark-500">{d.firmware_version ?? 'â€”'}</span> },
  ];

  return (
    <PageTransition>
      <div className="space-y-5">
        <PageHeader
          title="Devices"
          subtitle="Network devices, status and control."
          icon={<Router className="h-5 w-5" />}
          actions={<Button onClick={() => setCreateOpen(true)}><Plus className="h-4 w-4" /> Register device</Button>}
        />

        <DataTable
          columns={columns}
          data={data}
          rowKey={(d) => d.id}
          loading={loading}
          onRowClick={openDetail}
          emptyTitle="No devices"
          emptyDescription="Registered network devices will appear here."
        />

        <RegisterDeviceModal open={createOpen} onClose={() => setCreateOpen(false)} onCreated={() => { setCreateOpen(false); void load(); }} />

        <Modal open={detail !== null} onClose={() => setDetail(null)} title="Device details" subtitle={detail?.name} width="lg">
          {detail ? (
            <div className="space-y-5">
              <div className="flex flex-wrap items-center gap-2">
                <StatusBadge status={detail.status} pulse={detail.status === 'online'} />
                <select
                  value={detail.status}
                  onChange={(e) => void setStatus(detail, e.target.value)}
                  className="rounded-lg border border-white/10 bg-dark-950/70 px-2 py-1.5 text-xs text-dark-300 focus:border-accent-500/50 focus:outline-none"
                >
                  {['online', 'offline', 'maintenance', 'disabled'].map((s) => (
                    <option key={s} value={s} className="bg-dark-900 text-white">{s}</option>
                  ))}
                </select>
                <div className="ml-auto flex gap-2">
                  <Button variant="secondary" size="sm" loading={acting?.id === detail.id && acting.action === 'restart'}
                    onClick={() => setConfirm({ device: detail, action: 'restart' })}>
                    <RotateCw className="h-4 w-4" /> Restart
                  </Button>
                  <Button variant="danger" size="sm" loading={acting?.id === detail.id && acting.action === 'shutdown'}
                    onClick={() => setConfirm({ device: detail, action: 'shutdown' })}>
                    <Power className="h-4 w-4" /> Shutdown
                  </Button>
                </div>
              </div>

              <div className="grid gap-3 rounded-xl border border-white/[0.06] bg-dark-950/40 p-4 text-sm text-dark-300 sm:grid-cols-2">
                <div>Type: <span className="text-dark-200">{detail.device_type}</span></div>
                <div>IP: <span className="font-mono text-dark-200">{detail.ip_address ?? 'â€”'}</span></div>
                <div>MAC: <span className="font-mono text-dark-200">{detail.mac_address ?? 'â€”'}</span></div>
                <div>Firmware: <span className="text-dark-200">{detail.firmware_version ?? 'â€”'}</span></div>
                <div>Model: <span className="text-dark-200">{detail.model ?? 'â€”'}</span></div>
                <div>Branch: <span className="text-dark-200">{detail.branch_id ? `#${detail.branch_id}` : 'â€”'}</span></div>
              </div>

              <div>
                <h3 className="mb-2 text-sm font-semibold text-white">Recent metrics</h3>
                {detailLoading ? (
                  <Spinner />
                ) : metrics.length === 0 ? (
                  <p className="text-sm text-dark-500">No metrics recorded yet.</p>
                ) : (
                  <div className="grid gap-2 sm:grid-cols-3">
                    {metrics.slice(0, 6).map((m) => (
                      <div key={m.id} className="rounded-xl border border-white/[0.06] bg-white/[0.02] px-4 py-3 text-sm">
                        <p className="text-xs text-dark-500">{formatDateTime(m.recorded_at)}</p>
                        <p className="mt-1 text-dark-200">
                          CPU <span className="tabular-nums text-white">{m.cpu_load_percent ?? 'â€”'}%</span>
                        </p>
                        <p className="text-dark-200">
                          Mem <span className="tabular-nums text-white">{m.memory_used_percent ?? 'â€”'}%</span>
                        </p>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          ) : (
            <Spinner />
          )}
        </Modal>

        <ConfirmDialog
          open={confirm !== null}
          onClose={() => setConfirm(null)}
          onConfirm={() => {
            if (confirm) void runPower(confirm.device, confirm.action);
          }}
          title={confirm?.action === 'restart' ? 'Restart device?' : 'Shut down device?'}
          description={
            confirm?.action === 'restart'
              ? `A restart command will be sent to "${confirm?.device.name ?? 'device'}". This may briefly interrupt service.`
              : `A shutdown command will be sent to "${confirm?.device.name ?? 'device'}". Service will be interrupted until the device is powered on.`
          }
          confirmLabel={confirm?.action === 'restart' ? 'Restart' : 'Shut down'}
          danger={confirm?.action === 'shutdown'}
        />
      </div>
    </PageTransition>
  );
}

function RegisterDeviceModal({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: () => void }) {
  const [name, setName] = useState('');
  const [deviceType, setDeviceType] = useState('router');
  const [ipAddress, setIpAddress] = useState('');
  const [macAddress, setMacAddress] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async () => {
    if (!name.trim()) return;
    setBusy(true);
    try {
      await registerDevice({
        name: name.trim(),
        device_type: deviceType,
        ip_address: ipAddress.trim() || undefined,
        mac_address: macAddress.trim() || undefined,
      });
      toast('Device registered');
      onCreated();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to register device', 'error');
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Register device" subtitle="Add a network device to inventory." width="md"
      footer={<>
        <Button variant="ghost" onClick={onClose}>Cancel</Button>
        <Button onClick={submit} loading={busy}>Register</Button>
      </>}>
      <form onSubmit={(e) => { e.preventDefault(); void submit(); }} className="space-y-4">
        <TextField label="Device name" required value={name} onChange={(e) => setName(e.target.value)} />
        <SelectField label="Device type" value={deviceType} onChange={(e) => setDeviceType(e.target.value)}
          options={[
            { value: 'router', label: 'Router' },
            { value: 'switch', label: 'Switch' },
            { value: 'olt', label: 'OLT' },
            { value: 'ont', label: 'ONT' },
            { value: 'access_point', label: 'Access point' },
            { value: 'firewall', label: 'Firewall' },
            { value: 'other', label: 'Other' },
          ]}
        />
        <TextField label="IP address" value={ipAddress} onChange={(e) => setIpAddress(e.target.value)} />
        <TextField label="MAC address" value={macAddress} onChange={(e) => setMacAddress(e.target.value)} />
      </form>
    </Modal>
  );
}
