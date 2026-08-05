import { useEffect, useState } from 'react';
import { motion } from 'framer-motion';
import { KeyRound, Lock, Monitor, ShieldCheck, Smartphone, User } from 'lucide-react';
import { useAdminStore } from '../adminStore';
import {
  adminChangePassword,
  adminConfirm2fa,
  adminDisable2fa,
  adminListSessions,
  adminRevokeSession,
  adminSetup2fa,
} from '../../api/admin/auth';
import { PageHeader } from '../components/ui/PageHeader';
import { PageTransition } from '../components/ui/PageTransition';
import { Button } from '../components/ui/Button';
import { TextField } from '../components/ui/FormField';
import { StatusBadge } from '../components/ui/StatusBadge';
import { Spinner } from '../components/ui/Spinner';
import { formatDateTime, initials, timeAgo } from '../lib/format';
import { ROLE_LABELS } from '../lib/permissions';
import { toast } from '../lib/toast';

interface Session {
  id: string;
  user_id?: number;
  user_agent?: string;
  ip_address?: string;
  is_current?: boolean;
  created_at?: string;
  last_active_at?: string;
  status?: string;
}

function Section({ icon, title, children }: { icon: React.ReactNode; title: string; children: React.ReactNode }) {
  return (
    <motion.section
      initial={{ opacity: 0, y: 12 }}
      animate={{ opacity: 1, y: 0 }}
      className="glass-card rounded-2xl p-6"
    >
      <h2 className="mb-5 flex items-center gap-2 text-sm font-semibold text-white">
        {icon}
        {title}
      </h2>
      {children}
    </motion.section>
  );
}

export function ProfilePage() {
  const user = useAdminStore((s) => s.user);
  const role = useAdminStore((s) => s.role);

  const [sessions, setSessions] = useState<Session[] | null>(null);
  const [loading2fa, setLoading2fa] = useState(false);
  const [totpSetup, setTotpSetup] = useState<{ secret: string; backupCodes: string[] } | null>(null);
  const [totpCode, setTotpCode] = useState('');
  const [twoFactorEnabled, setTwoFactorEnabled] = useState<boolean | null>(null);

  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [pwBusy, setPwBusy] = useState(false);

  const [disableCode, setDisableCode] = useState('');
  const [disableBusy, setDisableBusy] = useState(false);

  const loadSessions = async () => {
    try {
      setSessions((await adminListSessions()) as Session[]);
    } catch {
      setSessions([]);
    }
  };

  useEffect(() => {
    void loadSessions();
  }, []);

  const changePassword = async (e: React.FormEvent) => {
    e.preventDefault();
    if (newPassword !== confirmPassword) {
      toast('New passwords do not match', 'error');
      return;
    }
    if (newPassword.length < 8) {
      toast('Password must be at least 8 characters', 'error');
      return;
    }
    setPwBusy(true);
    try {
      await adminChangePassword({ current_password: currentPassword, new_password: newPassword });
      toast('Password updated');
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to change password', 'error');
    } finally {
      setPwBusy(false);
    }
  };

  const setup2fa = async () => {
    setLoading2fa(true);
    try {
      const res = await adminSetup2fa();
      setTotpSetup({ secret: res.secret_base32, backupCodes: res.backup_codes });
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to start 2FA setup';
      if (message.toLowerCase().includes('already enabled')) {
        setTwoFactorEnabled(true);
        toast('Two-factor authentication is already enabled', 'info');
      } else {
        toast(message, 'error');
      }
    } finally {
      setLoading2fa(false);
    }
  };

  const confirm2fa = async () => {
    if (!totpCode.trim()) return;
    try {
      await adminConfirm2fa(totpCode.trim());
      setTwoFactorEnabled(true);
      setTotpSetup(null);
      setTotpCode('');
      toast('Two-factor authentication enabled');
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Invalid verification code', 'error');
    }
  };

  const disable2fa = async () => {
    if (!disableCode.trim()) return;
    setDisableBusy(true);
    try {
      await adminDisable2fa(disableCode.trim());
      setTwoFactorEnabled(false);
      setDisableCode('');
      toast('Two-factor authentication disabled', 'info');
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Invalid verification code', 'error');
    } finally {
      setDisableBusy(false);
    }
  };

  const revokeSession = async (id: string) => {
    try {
      await adminRevokeSession(id);
      toast('Session revoked');
      void loadSessions();
    } catch (err) {
      toast(err instanceof Error ? err.message : 'Failed to revoke session', 'error');
    }
  };

  return (
    <PageTransition>
      <div className="space-y-6">
        <PageHeader
          title="My profile"
          subtitle="Manage your account security and active sessions."
          icon={<User className="h-5 w-5" />}
        />

        {/* Identity */}
        <Section icon={<User className="h-4 w-4 text-primary-400" />} title="Account">
          <div className="flex flex-col gap-4 sm:flex-row sm:items-center">
            <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-gradient-to-br from-primary-500 to-accent-500 text-lg font-bold text-white">
              {initials(user?.name)}
            </div>
            <div>
              <p className="font-medium text-white">{user?.name}</p>
              <p className="text-sm text-dark-400">{user?.email}</p>
            </div>
            <div className="sm:ml-auto">
              <StatusBadge status={user?.status} />
              <span className="ml-2 text-xs text-dark-500">{ROLE_LABELS[role ?? ''] ?? role}</span>
            </div>
          </div>
          {user && (
            <div className="mt-4 grid gap-3 border-t border-white/5 pt-4 text-sm text-dark-300 sm:grid-cols-3">
              <div>
                <p className="text-xs text-dark-500">Phone</p>
                <p className="mt-0.5">{user.phone || 'â€”'}</p>
              </div>
              <div>
                <p className="text-xs text-dark-500">Branch</p>
                <p className="mt-0.5">{user.branch_id ? `Branch #${user.branch_id}` : 'Platform-wide'}</p>
              </div>
              <div>
                <p className="text-xs text-dark-500">Last login</p>
                <p className="mt-0.5">{formatDateTime(user.last_login_at)}</p>
              </div>
            </div>
          )}
        </Section>

        <div className="grid gap-6 lg:grid-cols-2">
          {/* Change password */}
          <Section icon={<Lock className="h-4 w-4 text-primary-400" />} title="Change password">
            <form onSubmit={changePassword} className="space-y-4">
              <TextField
                label="Current password"
                type="password"
                required
                value={currentPassword}
                onChange={(e) => setCurrentPassword(e.target.value)}
              />
              <TextField
                label="New password"
                type="password"
                required
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
                hint="At least 8 characters."
              />
              <TextField
                label="Confirm new password"
                type="password"
                required
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
              />
              <Button type="submit" loading={pwBusy}>
                Update password
              </Button>
            </form>
          </Section>

          {/* Two-factor */}
          <Section icon={<ShieldCheck className="h-4 w-4 text-primary-400" />} title="Two-factor authentication">
            {totpSetup ? (
              <div className="space-y-4">
                <p className="text-sm text-dark-300">
                  Add this secret to your authenticator app (Google Authenticator, Aegis, etc.), then
                  enter the 6-digit code to confirm.
                </p>
                <div className="rounded-xl border border-white/10 bg-dark-950/60 p-4 text-center">
                  <p className="font-mono text-sm tracking-widest text-accent-300">{totpSetup.secret}</p>
                </div>
                {totpSetup.backupCodes.length > 0 && (
                  <div>
                    <p className="mb-1.5 text-xs text-dark-500">Save these backup codes â€” they are shown once:</p>
                    <div className="flex flex-wrap gap-1.5">
                      {totpSetup.backupCodes.map((code) => (
                        <span key={code} className="rounded-md border border-white/10 bg-white/[0.03] px-2 py-1 font-mono text-[11px] text-dark-300">
                          {code}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
                <div className="flex items-end gap-2">
                  <TextField
                    label="Confirmation code"
                    inputMode="numeric"
                    placeholder="6-digit code"
                    value={totpCode}
                    onChange={(e) => setTotpCode(e.target.value.replace(/[^0-9]/g, '').slice(0, 6))}
                  />
                  <Button onClick={confirm2fa}>Enable</Button>
                </div>
              </div>
            ) : twoFactorEnabled === true ? (
              <div className="space-y-4">
                <div className="flex items-center gap-2 text-sm text-dark-300">
                  <StatusBadge status="enabled" /> Two-factor authentication is active.
                </div>
                <div className="flex items-end gap-2">
                  <TextField
                    label="Current 6-digit code"
                    inputMode="numeric"
                    placeholder="â€¢â€¢â€¢â€¢â€¢â€¢"
                    value={disableCode}
                    onChange={(e) => setDisableCode(e.target.value.replace(/[^0-9]/g, '').slice(0, 6))}
                  />
                  <Button variant="danger" onClick={disable2fa} loading={disableBusy} disabled={!disableCode.trim()}>
                    Disable
                  </Button>
                </div>
              </div>
            ) : (
              <div className="space-y-4">
                <p className="text-sm text-dark-300">
                  Enable two-factor authentication to protect your admin session with a time-based code
                  from your authenticator app.
                </p>
                <Button variant="accent" onClick={setup2fa} loading={loading2fa}>
                  <Smartphone className="h-4 w-4" /> Set up 2FA
                </Button>
              </div>
            )}
          </Section>
        </div>

        {/* Sessions */}
        <Section icon={<Monitor className="h-4 w-4 text-primary-400" />} title="Active sessions">
          {sessions === null ? (
            <Spinner />
          ) : sessions.length === 0 ? (
            <p className="text-sm text-dark-400">No active sessions found.</p>
          ) : (
            <div className="space-y-2">
              {sessions.map((s) => (
                <div
                  key={s.id}
                  className="flex flex-wrap items-center gap-3 rounded-xl border border-white/[0.06] bg-dark-950/40 px-4 py-3"
                >
                  <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg border border-white/10 bg-white/[0.03] text-dark-400">
                    <Monitor className="h-4 w-4" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <p className="truncate text-sm text-white">{s.user_agent ?? 'Unknown device'}</p>
                    <p className="text-xs text-dark-500">
                      {s.ip_address ?? 'IP unknown'} Â· Active {timeAgo(s.last_active_at ?? s.created_at)}
                    </p>
                  </div>
                  {s.is_current && <StatusBadge status="current" />}
                  {!s.is_current && (
                    <Button variant="ghost" size="sm" onClick={() => revokeSession(s.id)}>
                      Revoke
                    </Button>
                  )}
                </div>
              ))}
            </div>
          )}
        </Section>

        <div className="flex items-center gap-2 text-xs text-dark-500">
          <KeyRound className="h-3.5 w-3.5" />
          Session keys are managed server-side. Use the profile menu to sign out.
        </div>
      </div>
    </PageTransition>
  );
}
