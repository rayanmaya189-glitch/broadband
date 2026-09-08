import { useState, type FormEvent } from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Lock, Mail, ArrowRight, ArrowLeft, ShieldCheck } from 'lucide-react';
import { useCustomerAuthStore } from '../../store/customerAuthStore';

export default function CustomerLoginPage() {
  const login = useCustomerAuthStore((s) => s.login);
  const login2fa = useCustomerAuthStore((s) => s.login2fa);
  const requiresTwoFactor = useCustomerAuthStore((s) => s.requiresTwoFactor);
  const loginError = useCustomerAuthStore((s) => s.loginError);
  const navigate = useNavigate();
  const location = useLocation();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [code, setCode] = useState('');
  const [busy, setBusy] = useState(false);
  const [touched, setTouched] = useState(false);

  const from = (location.state as { from?: string } | null)?.from ?? '/portal';

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    setTouched(true);
    if (requiresTwoFactor) {
      if (!code.trim()) return;
      setBusy(true);
      try { await login2fa(code.trim()); navigate(from, { replace: true }); } catch { /* error shown via store */ }
      finally { setBusy(false); }
      return;
    }
    if (!email.trim() || !password) return;
    setBusy(true);
    try {
      const result = await login(email.trim(), password);
      if (result !== '2fa') navigate(from, { replace: true });
    } catch { /* error shown via store */ }
    finally { setBusy(false); }
  };

  return (
    <div className="flex min-h-[80vh] items-center justify-center">
      <motion.div
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        className="w-full max-w-md rounded-2xl border border-white/[0.08] bg-white/[0.02] p-8 backdrop-blur-xl"
      >
        <div className="mb-6 text-center">
          <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-gradient-to-br from-primary-500 to-accent-500 text-lg font-bold text-white">
            AX
          </div>
          <h1 className="text-xl font-semibold text-white">
            {requiresTwoFactor ? 'Two-factor verification' : 'Sign in to your account'}
          </h1>
          <p className="mt-1.5 text-sm text-dark-400">
            {requiresTwoFactor ? 'Enter the 6-digit code from your authenticator app.' : 'Access your subscription, invoices and support.'}
          </p>
        </div>

        <form onSubmit={submit} className="space-y-4" noValidate>
          {requiresTwoFactor ? (
            <>
              <div>
                <label className="mb-1.5 block text-xs font-medium text-dark-300">Verification code</label>
                <input
                  autoFocus inputMode="numeric" autoComplete="one-time-code" placeholder="6-digit code"
                  value={code} onChange={(e) => setCode(e.target.value.replace(/[^0-9]/g, '').slice(0, 6))}
                  className="w-full rounded-xl border border-white/10 bg-dark-950/60 px-3.5 py-2.5 text-sm text-white placeholder-dark-500 focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30"
                />
              </div>
              <button type="submit" disabled={busy} className="flex w-full items-center justify-center gap-2 rounded-xl bg-primary-600 px-4 py-2.5 text-sm font-medium text-white transition-colors hover:bg-primary-500 disabled:opacity-50">
                {busy ? 'Verifying…' : 'Verify and continue'} <ArrowRight className="h-4 w-4" />
              </button>
              <button type="button" onClick={() => useCustomerAuthStore.setState({ requiresTwoFactor: false, pendingToken: null, loginError: null })} className="flex items-center gap-1 text-xs text-dark-400 hover:text-white">
                <ArrowLeft className="h-3.5 w-3.5" /> Back to sign in
              </button>
            </>
          ) : (
            <>
              <div>
                <label className="mb-1.5 block text-xs font-medium text-dark-300">Email</label>
                <div className="relative">
                  <Mail className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-dark-500" />
                  <input
                    type="email" required autoComplete="email" placeholder="you@example.com"
                    value={email} onChange={(e) => setEmail(e.target.value)}
                    className="w-full rounded-xl border border-white/10 bg-dark-950/60 py-2.5 pl-10 pr-3.5 text-sm text-white placeholder-dark-500 focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30"
                  />
                </div>
                {touched && !email.trim() && <p className="mt-1 text-xs text-red-400">Email is required.</p>}
              </div>
              <div>
                <label className="mb-1.5 block text-xs font-medium text-dark-300">Password</label>
                <div className="relative">
                  <Lock className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-dark-500" />
                  <input
                    type="password" required autoComplete="current-password" placeholder="••••••••"
                    value={password} onChange={(e) => setPassword(e.target.value)}
                    className="w-full rounded-xl border border-white/10 bg-dark-950/60 py-2.5 pl-10 pr-3.5 text-sm text-white placeholder-dark-500 focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30"
                  />
                </div>
                {touched && !password && <p className="mt-1 text-xs text-red-400">Password is required.</p>}
              </div>
              <button type="submit" disabled={busy} className="flex w-full items-center justify-center gap-2 rounded-xl bg-primary-600 px-4 py-2.5 text-sm font-medium text-white transition-colors hover:bg-primary-500 disabled:opacity-50">
                <Lock className="h-4 w-4" /> {busy ? 'Signing in…' : 'Sign in'}
              </button>
            </>
          )}

          {loginError && (
            <motion.p initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="rounded-lg border border-red-500/30 bg-red-500/10 px-3 py-2 text-xs text-red-300">
              {loginError}
            </motion.p>
          )}
        </form>

        <div className="mt-6 border-t border-white/[0.06] pt-4 text-center text-xs text-dark-500">
          <p>Don&apos;t have an account? <Link to="/portal/signup" className="text-accent-400 hover:text-accent-300">Create one</Link></p>
        </div>
      </motion.div>
    </div>
  );
}
