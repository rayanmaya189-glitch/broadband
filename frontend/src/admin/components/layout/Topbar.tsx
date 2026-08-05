import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { AnimatePresence, motion } from 'framer-motion';
import {
  ChevronsLeft,
  ChevronsRight,
  LogOut,
  Menu,
  PanelLeftClose,
  PanelLeftOpen,
  User,
  X,
} from 'lucide-react';
import { useAdminStore } from '../../adminStore';
import { ROLE_LABELS } from '../../lib/permissions';
import { initials } from '../../lib/format';
import { toast } from '../../lib/toast';
import { Sidebar } from './Sidebar';

interface TopbarProps {
  collapsed: boolean;
  onToggleCollapse: () => void;
  onOpenMobile: () => void;
}

export function Topbar({ collapsed, onToggleCollapse, onOpenMobile }: TopbarProps) {
  const user = useAdminStore((s) => s.user);
  const role = useAdminStore((s) => s.role);
  const logout = useAdminStore((s) => s.logout);
  const navigate = useNavigate();
  const [menuOpen, setMenuOpen] = useState(false);
  const [loggingOut, setLoggingOut] = useState(false);

  const handleLogout = async () => {
    setLoggingOut(true);
    await logout();
    setLoggingOut(false);
    toast('Signed out', 'info');
    navigate('/admin/login', { replace: true });
  };

  const name = user?.name ?? 'Administrator';

  return (
    <header className="sticky top-0 z-40 flex h-14 items-center justify-between gap-3 border-b border-white/[0.06] bg-dark-950/80 px-4 backdrop-blur-xl">
      <div className="flex items-center gap-2">
        <button
          onClick={onOpenMobile}
          className="flex h-8 w-8 items-center justify-center rounded-lg text-dark-300 hover:bg-white/5 md:hidden"
          aria-label="Open menu"
        >
          <Menu className="h-4 w-4" />
        </button>
        <button
          onClick={onToggleCollapse}
          className="hidden h-8 w-8 items-center justify-center rounded-lg text-dark-400 transition-colors hover:bg-white/5 hover:text-white md:flex"
          aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
        >
          {collapsed ? <PanelLeftOpen className="h-4 w-4" /> : <PanelLeftClose className="h-4 w-4" />}
        </button>
        <span className="text-sm text-dark-500">AeroXe ISP Operations</span>
      </div>

      <div className="relative">
        <button
          onClick={() => setMenuOpen((v) => !v)}
          className="flex items-center gap-2.5 rounded-xl border border-white/10 bg-white/[0.03] py-1.5 pl-1.5 pr-3 transition-colors hover:bg-white/[0.06]"
        >
          <span className="flex h-7 w-7 items-center justify-center rounded-lg bg-gradient-to-br from-primary-500 to-accent-500 text-[11px] font-bold text-white">
            {initials(name)}
          </span>
          <span className="hidden text-left sm:block">
            <span className="block max-w-[140px] truncate text-xs font-medium text-white">{name}</span>
            <span className="block text-[10px] text-dark-500">{ROLE_LABELS[role ?? ''] ?? role}</span>
          </span>
        </button>

        <AnimatePresence>
          {menuOpen && (
            <>
              <div className="fixed inset-0 z-10" onClick={() => setMenuOpen(false)} />
              <motion.div
                initial={{ opacity: 0, y: 6, scale: 0.98 }}
                animate={{ opacity: 1, y: 0, scale: 1 }}
                exit={{ opacity: 0, y: 6, scale: 0.98 }}
                transition={{ duration: 0.15 }}
                className="absolute right-0 z-20 mt-2 w-52 overflow-hidden rounded-xl border border-white/10 bg-dark-900 shadow-2xl"
              >
                <div className="border-b border-white/10 px-4 py-3">
                  <p className="text-xs font-semibold text-white">{name}</p>
                  <p className="mt-0.5 truncate text-[11px] text-dark-500">{user?.email}</p>
                </div>
                <Link
                  to="/admin/profile"
                  onClick={() => setMenuOpen(false)}
                  className="flex items-center gap-2.5 px-4 py-2.5 text-sm text-dark-300 transition-colors hover:bg-white/5 hover:text-white"
                >
                  <User className="h-4 w-4" /> My profile
                </Link>
                <button
                  onClick={handleLogout}
                  disabled={loggingOut}
                  className="flex w-full items-center gap-2.5 px-4 py-2.5 text-sm text-red-300 transition-colors hover:bg-red-500/10 disabled:opacity-50"
                >
                  <LogOut className="h-4 w-4" /> {loggingOut ? 'Signing out…' : 'Sign out'}
                </button>
              </motion.div>
            </>
          )}
        </AnimatePresence>
      </div>
    </header>
  );
}

export function MobileSidebar({ open, onClose }: { open: boolean; onClose: () => void }) {
  return (
    <AnimatePresence>
      {open && (
        <div className="fixed inset-0 z-50 md:hidden">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="absolute inset-0 bg-dark-950/80 backdrop-blur-sm"
          />
          <motion.div
            initial={{ x: -280 }}
            animate={{ x: 0 }}
            exit={{ x: -280 }}
            transition={{ type: 'spring', stiffness: 300, damping: 32 }}
            className="absolute inset-y-0 left-0 w-64 overflow-y-auto border-r border-white/10 bg-dark-900"
          >
            <div className="flex items-center justify-between px-4 py-4">
              <p className="text-sm font-semibold text-white">AeroXe Admin</p>
              <button
                onClick={onClose}
                className="flex h-8 w-8 items-center justify-center rounded-lg text-dark-400 hover:bg-white/5"
                aria-label="Close menu"
              >
                <X className="h-4 w-4" />
              </button>
            </div>
            <Sidebar collapsed={false} />
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
}
