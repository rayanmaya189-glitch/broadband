import { describe, it, expect } from 'vitest';
import {
  roleHasModule,
  modulesForRole,
  ALL_MODULES,
  ROLE_LABELS,
  type ModuleKey,
} from '../permissions';

describe('ALL_MODULES', () => {
  it('contains all expected module keys', () => {
    expect(ALL_MODULES).toContain('dashboard');
    expect(ALL_MODULES).toContain('customers');
    expect(ALL_MODULES).toContain('plans');
    expect(ALL_MODULES).toContain('billing');
    expect(ALL_MODULES).toContain('tickets');
    expect(ALL_MODULES).toContain('network');
    expect(ALL_MODULES).toContain('devices');
    expect(ALL_MODULES).toContain('monitoring');
    expect(ALL_MODULES).toContain('bandwidth');
    expect(ALL_MODULES).toContain('scheduler');
    expect(ALL_MODULES).toContain('compliance');
    expect(ALL_MODULES).toContain('referrals');
    expect(ALL_MODULES).toContain('discovery');
    expect(ALL_MODULES).toContain('inventory');
    expect(ALL_MODULES).toContain('documents');
    expect(ALL_MODULES).toContain('gateway');
    expect(ALL_MODULES).toContain('profile');
  });

  it('has 28 modules', () => {
    expect(ALL_MODULES.length).toBe(28);
  });
});

describe('roleHasModule', () => {
  it('returns false for null role', () => {
    expect(roleHasModule(null, 'dashboard')).toBe(false);
  });

  it('returns false for undefined role', () => {
    expect(roleHasModule(undefined, 'dashboard')).toBe(false);
  });

  it('returns false for unknown role', () => {
    expect(roleHasModule('unknown_role' as any, 'dashboard')).toBe(false);
  });

  it('super_admin has access to all modules', () => {
    for (const mod of ALL_MODULES) {
      expect(roleHasModule('super_admin', mod)).toBe(true);
    }
  });

  it('isp_owner has access to all modules', () => {
    for (const mod of ALL_MODULES) {
      expect(roleHasModule('isp_owner', mod)).toBe(true);
    }
  });

  it('noc_engineer has network-related modules', () => {
    expect(roleHasModule('noc_engineer', 'network')).toBe(true);
    expect(roleHasModule('noc_engineer', 'devices')).toBe(true);
    expect(roleHasModule('noc_engineer', 'monitoring')).toBe(true);
    expect(roleHasModule('noc_engineer', 'bandwidth')).toBe(true);
  });

  it('noc_engineer does NOT have billing modules', () => {
    expect(roleHasModule('noc_engineer', 'billing')).toBe(false);
    expect(roleHasModule('noc_engineer', 'accounting')).toBe(false);
  });

  it('field_technician has limited modules', () => {
    expect(roleHasModule('field_technician', 'dashboard')).toBe(true);
    expect(roleHasModule('field_technician', 'customers')).toBe(true);
    expect(roleHasModule('field_technician', 'tickets')).toBe(true);
    expect(roleHasModule('field_technician', 'devices')).toBe(true);
    expect(roleHasModule('field_technician', 'installations')).toBe(true);
    // Should NOT have these
    expect(roleHasModule('field_technician', 'billing')).toBe(false);
    expect(roleHasModule('field_technician', 'accounting')).toBe(false);
    expect(roleHasModule('field_technician', 'roles')).toBe(false);
    expect(roleHasModule('field_technician', 'network')).toBe(false);
  });

  it('customer_support has limited modules', () => {
    expect(roleHasModule('customer_support', 'tickets')).toBe(true);
    expect(roleHasModule('customer_support', 'customers')).toBe(true);
    expect(roleHasModule('customer_support', 'subscriptions')).toBe(true);
    expect(roleHasModule('customer_support', 'billing')).toBe(false);
    expect(roleHasModule('customer_support', 'network')).toBe(false);
  });

  it('sales_agent has sales-related modules', () => {
    expect(roleHasModule('sales_agent', 'leads')).toBe(true);
    expect(roleHasModule('sales_agent', 'plans')).toBe(true);
    expect(roleHasModule('sales_agent', 'coverage')).toBe(true);
    expect(roleHasModule('sales_agent', 'referrals')).toBe(true);
    expect(roleHasModule('sales_agent', 'billing')).toBe(false);
  });

  it('finance_manager has finance modules', () => {
    expect(roleHasModule('finance_manager', 'billing')).toBe(true);
    expect(roleHasModule('finance_manager', 'accounting')).toBe(true);
    expect(roleHasModule('finance_manager', 'compliance')).toBe(true);
    expect(roleHasModule('finance_manager', 'gateway')).toBe(true);
  });

  it('billing_operator has limited billing modules', () => {
    expect(roleHasModule('billing_operator', 'billing')).toBe(true);
    expect(roleHasModule('billing_operator', 'subscriptions')).toBe(true);
    expect(roleHasModule('billing_operator', 'accounting')).toBe(false);
  });
});

describe('modulesForRole', () => {
  it('returns empty array for null', () => {
    expect(modulesForRole(null)).toEqual([]);
  });

  it('returns all modules for super_admin', () => {
    expect(modulesForRole('super_admin')).toEqual(ALL_MODULES);
  });

  it('returns all modules for isp_owner', () => {
    expect(modulesForRole('isp_owner')).toEqual(ALL_MODULES);
  });

  it('returns limited modules for field_technician', () => {
    const mods = modulesForRole('field_technician');
    expect(mods.length).toBeLessThan(ALL_MODULES.length);
    expect(mods).toContain('dashboard');
    expect(mods).toContain('tickets');
  });

  it('returns empty array for unknown role', () => {
    expect(modulesForRole('unknown')).toEqual([]);
  });
});

describe('ROLE_LABELS', () => {
  it('has labels for all known roles', () => {
    expect(ROLE_LABELS['super_admin']).toBe('Super Admin');
    expect(ROLE_LABELS['isp_owner']).toBe('ISP Owner');
    expect(ROLE_LABELS['network_admin']).toBe('Network Admin');
    expect(ROLE_LABELS['noc_engineer']).toBe('NOC Engineer');
    expect(ROLE_LABELS['field_technician']).toBe('Field Technician');
    expect(ROLE_LABELS['customer_support']).toBe('Customer Support');
    expect(ROLE_LABELS['sales_agent']).toBe('Sales Agent');
    expect(ROLE_LABELS['finance_manager']).toBe('Finance Manager');
    expect(ROLE_LABELS['billing_operator']).toBe('Billing Operator');
    expect(ROLE_LABELS['customer']).toBe('Customer');
  });
});
