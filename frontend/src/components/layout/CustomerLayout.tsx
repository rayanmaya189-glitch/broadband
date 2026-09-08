import { useState } from 'react';
import { Link, Outlet, useLocation, useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import { Menu, X, User, LogOut, LayoutDashboard, CreditCard, LifeBuoy, Gauge } from 'lucide-react';
import { useCustomerAuthStore } from '../../store/customerAuthStore';
import { SITE_CONFIG } from '../../config/site';

const NAV_LINKS = [
  { label: 'Plans', href: '/plans', icon: Gauge },
  { label: 'Dashboard', href: '/portal', icon: LayoutDashboard, auth: true },
  { label: 'Invoices', href: '/portal/invoices', icon: CreditCard, auth: true },
  { label: 'Support', href: '/portal/tickets', icon: LifeBuoy, auth: true },
];

export default function CustomerLayout() {
  const [mobileOpen, setMobileOpen] = useState(false);
  const { isAuthenticated, user, logout } = useCustomerAuthStore();
  const location = useLocation();
  const navigate = useNavigate();

  const visibleLinks = NAV_LINKS.filter((l) => !l.auth || isAuthenticated);

  const handleLogout = () => {
    logout();
    navigate('/');
  };

  return (
    <div className="min-h-screen bg-dark-950 text-white">
      {/* Top navbar */}
      <nav className="sticky top-0 z-50 border-b border-white/[0.06] bg-dark-950/80 backdrop-blur-xl">
        <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6">
          {/* Logo */}
          <Link to="/" className="flex items-center gap-2.5">
            <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 text-sm font-bold text-white shadow-lg shadow-primary-900/40">
              AX
            </div>
            <div className="hidden sm:block">
              <p className="text-sm font-semibold text-white">AeroXe</p>
              <p className="text-[11px] text-dark-500">Broadband</p>
            </div>
          </Link>

          {/* Desktop nav */}
          <div className="hidden items-center gap-1 md:flex">
            {visibleLinks.map((link) => {
              const active = location.pathname === link.href || (link.href !== '/' && location.pathname.startsWith(link.href));
              return (
                <Link
                  key={link.href}
                  to={link.href}
                  className={`rounded-lg px-3 py-2 text-sm font-medium transition-colors ${
                    active ? 'bg-primary-500/15 text-primary-200' : 'text-dark-400 hover:text-white'
                  }`}
                >
                  {link.label}
                </Link>
              );
            })}
          </div>

          {/* Auth buttons */}
          <div className="flex items-center gap-2">
            {isAuthenticated ? (
              <div className="flex items-center gap-2">
                <span className="hidden text-xs text-dark-400 sm:block">{user?.name}</span>
                <button onClick={handleLogout} className="flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs text-dark-400 transition-colors hover:text-white">
                  <LogOut className="h-3.5 w-3.5" /> Sign out
                </button>
              </div>
            ) : (
              <>
                <Link to="/portal/login" className="rounded-lg px-3 py-1.5 text-sm text-dark-400 transition-colors hover:text-white">
                  Sign in
                </Link>
                <Link to="/portal/signup" className="rounded-lg bg-primary-600 px-4 py-1.5 text-sm font-medium text-white transition-colors hover:bg-primary-500">
                  Get Started
                </Link>
              </>
            )}

            {/* Mobile menu toggle */}
            <button onClick={() => setMobileOpen(!mobileOpen)} className="ml-1 rounded-lg p-2 text-dark-400 hover:text-white md:hidden">
              {mobileOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
            </button>
          </div>
        </div>

        {/* Mobile nav */}
        <AnimatePresence>
          {mobileOpen && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: 'auto', opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              className="overflow-hidden border-t border-white/[0.06] md:hidden"
            >
              <div className="space-y-1 px-4 py-3">
                {visibleLinks.map((link) => (
                  <Link
                    key={link.href}
                    to={link.href}
                    onClick={() => setMobileOpen(false)}
                    className="block rounded-lg px-3 py-2 text-sm text-dark-400 hover:text-white"
                  >
                    {link.label}
                  </Link>
                ))}
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </nav>

      {/* Page content */}
      <main className="mx-auto max-w-7xl px-4 py-8 sm:px-6 sm:py-12">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="border-t border-white/[0.06] py-8 text-center text-xs text-dark-500">
        <p>© {new Date().getFullYear()} {SITE_CONFIG.company.name}. All rights reserved.</p>
        <div className="mt-2 flex justify-center gap-4">
          <Link to="/privacy" className="hover:text-dark-300">Privacy</Link>
          <Link to="/terms" className="hover:text-dark-300">Terms</Link>
          <Link to="/support" className="hover:text-dark-300">Support</Link>
        </div>
      </footer>
    </div>
  );
}
