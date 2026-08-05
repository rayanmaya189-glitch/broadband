import type { AdminRole } from '../../types/admin';

/** Module keys used for navigation and access gating. */
export type ModuleKey =
  | 'dashboard'
  | 'customers'
  | 'plans'
  | 'subscriptions'
  | 'billing'
  | 'tickets'
  | 'leads'
  | 'network'
  | 'devices'
  | 'monitoring'
  | 'installations'
  | 'approvals'
  | 'notifications'
  | 'users'
  | 'roles'
  | 'branches'
  | 'audit'
  | 'accounting'
  | 'coverage'
  | 'profile';

export const ALL_MODULES: ModuleKey[] = [
  'dashboard',
  'customers',
  'plans',
  'subscriptions',
  'billing',
  'tickets',
  'leads',
  'network',
  'devices',
  'monitoring',
  'installations',
  'approvals',
  'notifications',
  'users',
  'roles',
  'branches',
  'audit',
  'accounting',
  'coverage',
  'profile',
];

/** Backend roles → modules they may open. Super admin / ISP owner get everything. */
const ROLE_MODULES: Record<string, ModuleKey[]> = {
  super_admin: ALL_MODULES,
  isp_owner: ALL_MODULES,
  network_admin: [
    'dashboard',
    'customers',
    'plans',
    'subscriptions',
    'billing',
    'tickets',
    'network',
    'devices',
    'monitoring',
    'installations',
    'approvals',
    'notifications',
    'branches',
    'audit',
  ],
  noc_engineer: [
    'dashboard',
    'customers',
    'tickets',
    'network',
    'devices',
    'monitoring',
    'installations',
    'notifications',
    'audit',
  ],
  field_technician: ['dashboard', 'customers', 'tickets', 'devices', 'installations'],
  customer_support: [
    'dashboard',
    'customers',
    'subscriptions',
    'tickets',
    'installations',
    'notifications',
    'profile',
  ],
  sales_agent: ['dashboard', 'customers', 'leads', 'plans', 'coverage', 'profile'],
  finance_manager: [
    'dashboard',
    'customers',
    'subscriptions',
    'billing',
    'approvals',
    'accounting',
    'branches',
    'audit',
    'notifications',
  ],
  billing_operator: ['dashboard', 'customers', 'subscriptions', 'billing', 'profile'],
  customer: ['dashboard', 'profile'],
};

export function roleHasModule(role: AdminRole | null | undefined, module: ModuleKey): boolean {
  if (!role) return false;
  return (ROLE_MODULES[role] ?? []).includes(module);
}

export function modulesForRole(role: AdminRole | null | undefined): ModuleKey[] {
  if (!role) return [];
  return ROLE_MODULES[role] ?? [];
}

export const ROLE_LABELS: Record<string, string> = {
  super_admin: 'Super Admin',
  isp_owner: 'ISP Owner',
  network_admin: 'Network Admin',
  noc_engineer: 'NOC Engineer',
  field_technician: 'Field Technician',
  customer_support: 'Customer Support',
  sales_agent: 'Sales Agent',
  finance_manager: 'Finance Manager',
  billing_operator: 'Billing Operator',
  customer: 'Customer',
};
