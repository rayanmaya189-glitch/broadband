import { useState, type FormEvent } from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import { ArrowLeft, ArrowRight, Lock, Mail, ShieldCheck } from 'lucide-react';
import { useAdminStore } from '../adminStore';
import { TextField } from '../components/ui/FormField';
import { Button } from '../components/ui/Button';
import { ROLE_LABELS } from '../lib/permissions';
import ToastContainer from '../../components/ui/Toast';

export function AdminLoginPage() {
  const login = useAdminStore((s) => s.login);
  const login2fa = useAdminStore((s) => s.login2fa);
  const requiresTwoFactor = useAdminStore((s) => s.requiresTwoFactor);
  const loginError = useAdminStore((s) => s.loginError);
  const navigate = useNavigate();
  const location = useLocation();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [code, setCode] = useState('');
  const [busy, setBusy] = useState(false);
  const [touched, setTouched] = useState(false);

  const from = (location.state as { from?: string } | null)?.from ?? '/admin';

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    setTouched(true);
    if (requiresTwoFactor) {
      if (!code.trim()) return;
      setBusy(true);
      try {
        await login2fa(code.trim());
        navigate(from, { replace: true });
      } catch {
        /* error shown via store */
      } finally {
        setBusy(false);
      }
      return;
    }
    if (!email.trim() || !password) return;
    setBusy(true);
    try {
      const result = await login(email.trim(), password);
      if (result !== '2fa') navigate(from, { replace: true });
    } catch {
      /* error shown via store */
    } finally {
      setBusy(false);
    }
  };

  const stepLabel = requiresTwoFactor ? 'Two-factor verification' : 'Staff sign in';
  const stepHint = requiresTwoFactor
    ? 'Enter the 6-digit code from your authenticator app or one-time password channel.'
    : 'Access the AeroXe operations console with your staff account.';

  return (
    <div className="relative flex min-h-screen items-center justify-center overflow-hidden bg-dark-950 p-4">
      <div className="bg-grid pointer-events-none absolute inset-0 opacity-40" />
      <div className="pointer-events-none absolute -top-32 left-1/2 h-96 w-[600px] -translate-x-1/2 rounded-full bg-primary-600/20 blur-[120px]" />
      <div className="pointer-events-none absolute bottom-0 right-0 h-72 w-72 rounded-full bg-accent-500/10 blur-[100px]" />

      <div className="relative grid w-full max-w-4xl gap-0 overflow-hidden rounded-3xl border border-white/[0.08] bg-white/[0.02] shadow-2xl backdrop-blur-xl lg:grid-cols-2">
        {/* Brand panel */}
        <motion.div
          initial={{ opacity: 0, x: -16 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.4 }}
          className="hidden flex-col justify-between bg-gradient-to-br from-primary-900/60 via-dark-900 to-dark-950 p-10 lg:flex"
        >
          <div>
            <div className="flex items-center gap-3">
              <div className="flex h-11 w-11 items-center justify-center rounded-2xl bg-gradient-to-br from-primary-500 to-accent-500 text-lg font-bold text-white shadow-xl shadow-primary-900/50">
                AX
              </div>
              <div>
                <p className="text-lg font-semibold text-white">AeroXe</p>
                <p className="text-xs text-dark-400">ISP Operations Console</p>
              </div>
            </div>
            <h1 className="mt-10 text-2xl font-semibold leading-snug text-white">
              Run your broadband network from one command center.
            </h1>
            <p className="mt-3 text-sm leading-relaxed text-dark-400">
              Manage customers, plans, billing, network devices, tickets and staff access â€” with role-based
              controls for every team.
            </p>
            <div className="mt-8 space-y-2.5 text-sm text-dark-300">
              {['Role-based access for every team', 'Live network & device status', 'Invoicing, payments & accounting'].map(
                (f) => (
                  <div key={f} className="flex items-center gap-2">
                    <ShieldCheck className="h-4 w-4 text-accent-400" />
                    {f}
                  </div>
                )
              )}
            </div>
          </div>
          <p className="text-[11px] text-dark-500">
            Secure console Â· 2FA supported Â· Session managed by AeroXe
          </p>
        </motion.div>

        {/* Form panel */}
        <motion.div
          initial={{ opacity: 0, y: 12 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.4, delay: 0.1 }}
          className="p-8 md:p-10"
        >
          <div className="mb-8 flex items-center gap-3 lg:hidden">
            <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 text-sm font-bold text-white">
              AX
            </div>
            <span className="font-semibold text-white">AeroXe Admin</span>
          </div>

          <div className="mb-6">
            <h2 className="text-xl font-semibold text-white">{stepLabel}</h2>
            <p className="mt-1.5 text-sm text-dark-400">{stepHint}</p>
          </div>

          <form onSubmit={submit} className="space-y-4" noValidate>
            {requiresTwoFactor ? (
              <>
                <TextField
                  label="Verification code"
                  required
                  autoFocus
                  inputMode="numeric"
                  autoComplete="one-time-code"
                  placeholder="6-digit code"
                  value={code}
                  onChange={(e) => setCode(e.target.value.replace(/[^0-9]/g, '').slice(0, 6))}
                  error={touched && !code ? 'Enter your verification code.' : undefined}
                />
                <Button type="submit" className="w-full" loading={busy}>
                  Verify and continue <ArrowRight className="h-4 w-4" />
                </Button>
                <button
                  type="button"
                  onClick={() => useAdminStore.setState({ requiresTwoFactor: false, pendingToken: null, loginError: null })}
                  className="flex items-center gap-1 text-xs text-dark-400 transition-colors hover:text-white"
                >
                  <ArrowLeft className="h-3.5 w-3.5" /> Back to sign in
                </button>
              </>
            ) : (
              <>
                <TextField
                  label="Email"
                  required
                  type="email"
                  autoComplete="email"
                  placeholder="you@aeroxe.isp"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  error={touched && !email.trim() ? 'Enter your email address.' : undefined}
                />
                <TextField
                  label="Password"
                  required
                  type="password"
                  autoComplete="current-password"
                  placeholder="â€¢â€¢â€¢â€¢â€¢â€¢â€¢â€¢"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  error={touched && !password ? 'Enter your password.' : undefined}
                />
                <Button type="submit" className="w-full" loading={busy}>
                  <Lock className="h-4 w-4" /> Sign in
                </Button>
              </>
            )}

            {loginError && (
              <motion.p
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="rounded-lg border border-red-500/30 bg-red-500/10 px-3 py-2 text-xs text-red-300"
              >
                {loginError}
              </motion.p>
            )}

            <div className="flex items-center justify-between pt-2 text-xs">
              <Link to="/" className="text-dark-400 transition-colors hover:text-accent-300">
                â† Back to website
              </Link>
              <span className="text-dark-500">{requiresTwoFactor ? '' : 'Staff access only'}</span>
            </div>
          </form>

          <div className="mt-8 border-t border-white/[0.06] pt-4">
            <p className="text-[11px] leading-relaxed text-dark-500">
              Roles: {Object.values(ROLE_LABELS).join(' Â· ')}
            </p>
          </div>
        </motion.div>
      </div>
      <ToastContainer />
    </div>
  );
}
