import { useState, type FormEvent } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import { User, Mail, Phone, Lock, MapPin, ArrowRight } from 'lucide-react';
import { registerCustomer } from '../../api/customer';
import { useCustomerAuthStore } from '../../store/customerAuthStore';

export default function CustomerSignupPage() {
  const navigate = useNavigate();
  const [name, setName] = useState('');
  const [phone, setPhone] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [pincode, setPincode] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [touched, setTouched] = useState(false);

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    setTouched(true);
    if (!name.trim() || !phone.trim() || !password) return;
    if (password.length < 8) { setError('Password must be at least 8 characters.'); return; }
    setBusy(true);
    setError('');
    try {
      const res = await registerCustomer({
        name: name.trim(),
        phone: phone.trim(),
        email: email.trim() || undefined,
        password,
        pincode: pincode.trim() || undefined,
      });
      // Auto-login after registration
      if (res.access_token) {
        sessionStorage.setItem('customer_access_token', res.access_token);
        sessionStorage.setItem('customer_refresh_token', res.refresh_token);
        if (res.user) sessionStorage.setItem('customer_user', JSON.stringify(res.user));
        useCustomerAuthStore.getState().hydrate();
        navigate('/portal', { replace: true });
      } else {
        navigate('/portal/login', { replace: true });
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Registration failed');
    } finally {
      setBusy(false);
    }
  };

  const fields = [
    { label: 'Full name', icon: User, type: 'text', value: name, onChange: setName, placeholder: 'John Doe', required: true, key: 'name' },
    { label: 'Phone', icon: Phone, type: 'tel', value: phone, onChange: setPhone, placeholder: '+91 98765 43210', required: true, key: 'phone' },
    { label: 'Email (optional)', icon: Mail, type: 'email', value: email, onChange: setEmail, placeholder: 'you@example.com', required: false, key: 'email' },
    { label: 'Password', icon: Lock, type: 'password', value: password, onChange: setPassword, placeholder: 'Min 8 characters', required: true, key: 'password' },
    { label: 'Pincode (optional)', icon: MapPin, type: 'text', value: pincode, onChange: setPincode, placeholder: '425001', required: false, key: 'pincode' },
  ];

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
          <h1 className="text-xl font-semibold text-white">Create your account</h1>
          <p className="mt-1.5 text-sm text-dark-400">Join AeroXe Broadband and get connected.</p>
        </div>

        <form onSubmit={submit} className="space-y-4" noValidate>
          {fields.map((f) => (
            <div key={f.key}>
              <label className="mb-1.5 block text-xs font-medium text-dark-300">{f.label}</label>
              <div className="relative">
                <f.icon className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-dark-500" />
                <input
                  type={f.type} required={f.required} autoComplete={f.key}
                  placeholder={f.placeholder} value={f.value}
                  onChange={(e) => f.onChange(e.target.value)}
                  className="w-full rounded-xl border border-white/10 bg-dark-950/60 py-2.5 pl-10 pr-3.5 text-sm text-white placeholder-dark-500 focus:border-accent-500/50 focus:outline-none focus:ring-2 focus:ring-accent-500/30"
                />
              </div>
              {touched && f.required && !f.value.trim() && (
                <p className="mt-1 text-xs text-red-400">{f.label} is required.</p>
              )}
            </div>
          ))}

          <button type="submit" disabled={busy} className="flex w-full items-center justify-center gap-2 rounded-xl bg-primary-600 px-4 py-2.5 text-sm font-medium text-white transition-colors hover:bg-primary-500 disabled:opacity-50">
            {busy ? 'Creating account…' : 'Create account'} <ArrowRight className="h-4 w-4" />
          </button>

          {error && (
            <motion.p initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="rounded-lg border border-red-500/30 bg-red-500/10 px-3 py-2 text-xs text-red-300">
              {error}
            </motion.p>
          )}
        </form>

        <div className="mt-6 border-t border-white/[0.06] pt-4 text-center text-xs text-dark-500">
          <p>Already have an account? <Link to="/portal/login" className="text-accent-400 hover:text-accent-300">Sign in</Link></p>
        </div>
      </motion.div>
    </div>
  );
}
