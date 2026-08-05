import { NavLink } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Activity,
  BookOpen,
  Building2,
  CheckCircle2,
  CreditCard,
  Gauge,
  LayoutDashboard,
  LifeBuoy,
  MapPin,
  Network,
  Repeat,
  Router,
  ScrollText,
  Shield,
  Target,
  UserCog,
  Users,
  Wrench,
  Bell,
} from 'lucide-react';
import { useAdminStore } from '../../adminStore';
import { modulesForRole, type ModuleKey } from '../../lib/permissions';
import { toLabel } from '../../lib/format';

interface NavItem {
  key: ModuleKey;
  to: string;
  label: string;
  icon: typeof LayoutDashboard;
}

interface NavGroup {
  label: string;
  items: NavItem[];
}

const ICONS = {
  dashboard: LayoutDashboard,
  customers: Users,
  plans: Gauge,
  subscriptions: Repeat,
  billing: CreditCard,
  tickets: LifeBuoy,
  leads: Target,
  network: Network,
  devices: Router,
  monitoring: Activity,
  installations: Wrench,
  approvals: CheckCircle2,
  notifications: Bell,
  users: UserCog,
  roles: Shield,
  branches: Building2,
  audit: ScrollText,
  accounting: BookOpen,
  coverage: MapPin,
} as const;

const GROUPING: { label: string; keys: ModuleKey[] }[] = [
  { label: 'Overview', keys: ['dashboard'] },
  {
    label: 'Operations',
    keys: ['customers', 'subscriptions', 'tickets', 'installations', 'approvals', 'leads'],
  },
  { label: 'Billing & Finance', keys: ['billing', 'accounting'] },
  { label: 'Network', keys: ['network', 'devices', 'monitoring'] },
  {
    label: 'Administration',
    keys: ['plans', 'users', 'roles', 'branches', 'notifications', 'audit', 'coverage'],
  },
];

export function Sidebar({ collapsed }: { collapsed: boolean }) {
  const role = useAdminStore((s) => s.role);
  const allowed = new Set(modulesForRole(role));

  const groups: NavGroup[] = GROUPING.map((g) => ({
    label: g.label,
    items: g.keys
      .filter((k) => allowed.has(k))
      .map((k) => ({
        key: k,
        to: `/admin/${k === 'dashboard' ? '' : k}`,
        label: toLabel(k),
        icon: ICONS[k as keyof typeof ICONS],
      })),
  })).filter((g) => g.items.length > 0);

  return (
    <aside
      className={`sticky top-0 flex h-screen shrink-0 flex-col border-r border-white/[0.06] bg-dark-950/90 backdrop-blur-xl transition-[width] duration-300 ${
        collapsed ? 'w-[72px]' : 'w-64'
      }`}
    >
      <div className={`flex items-center gap-2.5 px-4 py-5 ${collapsed ? 'justify-center px-0' : ''}`}>
        <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-primary-500 to-accent-500 font-bold text-white shadow-lg shadow-primary-900/40">
          AX
        </div>
        <AnimatePresence>
          {!collapsed && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="min-w-0"
            >
              <p className="truncate text-sm font-semibold text-white">AeroXe Admin</p>
              <p className="truncate text-[11px] text-dark-500">Broadband Operations</p>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      <nav className="flex-1 space-y-5 overflow-y-auto px-3 pb-6">
        {groups.map((group) => (
          <div key={group.label}>
            <AnimatePresence>
              {!collapsed && (
                <motion.p
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  className="mb-1.5 px-2 text-[10px] font-semibold uppercase tracking-widest text-dark-500"
                >
                  {group.label}
                </motion.p>
              )}
            </AnimatePresence>
            <ul className="space-y-0.5">
              {group.items.map((item) => (
                <SidebarLink key={item.key} item={item} collapsed={collapsed} />
              ))}
            </ul>
          </div>
        ))}
      </nav>

      <div className={`border-t border-white/[0.06] px-3 py-3 ${collapsed ? 'text-center' : ''}`}>
        <div className="flex items-center gap-2.5 rounded-xl px-2 py-2">
          <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-primary-500/40 bg-primary-500/15 text-xs font-semibold text-primary-300">
            {role?.slice(0, 2).toUpperCase() ?? '?'}
          </div>
          <AnimatePresence>
            {!collapsed && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="min-w-0"
              >
                <p className="truncate text-xs font-medium text-white">
                  {role ? toLabel(role) : 'Signed out'}
                </p>
                <p className="truncate text-[11px] text-dark-500">{role === 'super_admin' ? 'Platform-wide' : 'Scoped access'}</p>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>
    </aside>
  );
}

function SidebarLink({ item, collapsed }: { item: NavItem; collapsed: boolean }) {
  const Icon = item.icon;
  return (
    <li>
      <NavLink
        to={item.to}
        end={item.to === '/admin'}
        className={({ isActive }) =>
          `group flex items-center gap-3 rounded-xl px-2.5 py-2 text-sm transition-all duration-200 ${
            isActive
              ? 'bg-primary-500/15 font-medium text-primary-200 border border-primary-500/30 shadow-lg shadow-primary-950/30'
              : 'border border-transparent text-dark-400 hover:bg-white/[0.04] hover:text-white'
          } ${collapsed ? 'justify-center px-0' : ''}`
        }
      >
        <Icon className="h-4 w-4 shrink-0" />
        <AnimatePresence>
          {!collapsed && (
            <motion.span
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="truncate"
            >
              {item.label}
            </motion.span>
          )}
        </AnimatePresence>
        {collapsed && <span className="sr-only">{item.label}</span>}
      </NavLink>
    </li>
  );
}
