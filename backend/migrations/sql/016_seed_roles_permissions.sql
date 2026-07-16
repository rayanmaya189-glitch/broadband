-- AeroXe Backend Migration 016: Seed Roles and Permissions
-- Default RBAC data for the ISP platform

-- Seed default roles
INSERT INTO roles (name, slug, description, parent_role_id, is_system) VALUES
('Super Admin', 'super_admin', 'Platform-wide control', NULL, TRUE),
('ISP Owner', 'isp_owner', 'Business owner', 1, TRUE),
('Network Admin', 'network_admin', 'Network infrastructure management', 2, TRUE),
('NOC Engineer', 'noc_engineer', 'Network monitoring and operations', 3, TRUE),
('Field Technician', 'field_technician', 'Field operations and installations', NULL, TRUE),
('Customer Support', 'customer_support', 'Customer support handling', NULL, TRUE),
('Sales Agent', 'sales_agent', 'Lead management and sales', NULL, TRUE),
('Finance Manager', 'finance_manager', 'Financial oversight and approvals', 2, TRUE),
('Billing Operator', 'billing_operator', 'Billing operations', 8, TRUE),
('Customer', 'customer', 'End-user self-service portal', NULL, TRUE)
ON CONFLICT (slug) DO NOTHING;

-- Seed permissions
INSERT INTO permissions (name, module, resource, action, description) VALUES
-- Auth permissions
('auth.login', 'auth', 'session', 'login', 'User login'),
('auth.logout', 'auth', 'session', 'logout', 'User logout'),
('auth.register', 'auth', 'user', 'register', 'User registration'),
('auth.password.change', 'auth', 'password', 'change', 'Change password'),
('auth.password.reset.request', 'auth', 'password', 'reset_request', 'Request password reset'),
('auth.password.reset.confirm', 'auth', 'password', 'reset_confirm', 'Confirm password reset'),
('auth.2fa.enable', 'auth', '2fa', 'enable', 'Enable 2FA'),
('auth.2fa.disable', 'auth', '2fa', 'disable', 'Disable 2FA'),
('auth.sessions.view', 'auth', 'sessions', 'view', 'View active sessions'),
('auth.sessions.revoke', 'auth', 'sessions', 'revoke', 'Revoke session'),

-- User management permissions
('user.account.view', 'user', 'account', 'view', 'View user accounts'),
('user.account.create', 'user', 'account', 'create', 'Create user accounts'),
('user.account.update', 'user', 'account', 'update', 'Update user accounts'),
('user.account.delete', 'user', 'account', 'delete', 'Delete user accounts'),
('user.account.disable', 'user', 'account', 'disable', 'Disable user account'),
('user.account.enable', 'user', 'account', 'enable', 'Enable user account'),
('user.role.assign', 'user', 'role', 'assign', 'Assign role to user'),
('user.role.revoke', 'user', 'role', 'revoke', 'Revoke role from user'),

-- Branch permissions
('branch.view', 'branch', 'branch', 'view', 'View branches'),
('branch.create', 'branch', 'branch', 'create', 'Create branches'),
('branch.update', 'branch', 'branch', 'update', 'Update branches'),
('branch.manage_working_hours', 'branch', 'branch', 'manage_hours', 'Manage working hours'),
('branch.view_reports', 'branch', 'branch', 'view_reports', 'View branch reports'),
('branch.manage_staff', 'branch', 'branch', 'manage_staff', 'Manage branch staff'),

-- Customer permissions
('customer.account.view', 'customer', 'account', 'view', 'View customer accounts'),
('customer.account.create', 'customer', 'account', 'create', 'Create customer accounts'),
('customer.account.update', 'customer', 'account', 'update', 'Update customer accounts'),
('customer.account.delete', 'customer', 'account', 'delete', 'Delete customer accounts'),
('customer.account.suspend', 'customer', 'account', 'suspend', 'Suspend customer'),
('customer.account.reactivate', 'customer', 'account', 'reactivate', 'Reactivate customer'),
('customer.subscription.view', 'customer', 'subscription', 'view', 'View customer subscriptions'),
('customer.subscription.create', 'customer', 'subscription', 'create', 'Create subscriptions'),
('customer.subscription.upgrade', 'customer', 'subscription', 'upgrade', 'Upgrade subscription'),
('customer.subscription.downgrade', 'customer', 'subscription', 'downgrade', 'Downgrade subscription'),
('customer.profile.view', 'customer', 'profile', 'view', 'View customer profile'),
('customer.profile.update', 'customer', 'profile', 'update', 'Update customer profile'),
('customer.profile.verify_kyc', 'customer', 'profile', 'verify_kyc', 'Verify KYC'),
('customer.address.view', 'customer', 'address', 'view', 'View addresses'),
('customer.address.create', 'customer', 'address', 'create', 'Create addresses'),
('customer.address.update', 'customer', 'address', 'update', 'Update addresses'),
('customer.address.delete', 'customer', 'address', 'delete', 'Delete addresses'),

-- Plan permissions
('plan.view', 'plan', 'plan', 'view', 'View plans'),
('plan.create', 'plan', 'plan', 'create', 'Create plans'),
('plan.update', 'plan', 'plan', 'update', 'Update plans'),
('plan.delete', 'plan', 'plan', 'delete', 'Delete plans'),
('plan.publish', 'plan', 'plan', 'publish', 'Publish plans'),
('plan.unpublish', 'plan', 'plan', 'unpublish', 'Unpublish plans'),
('plan.clone', 'plan', 'plan', 'clone', 'Clone plans'),
('plan.speed_profile.view', 'plan', 'speed_profile', 'view', 'View speed profiles'),
('plan.speed_profile.create', 'plan', 'speed_profile', 'create', 'Create speed profiles'),
('plan.speed_profile.update', 'plan', 'speed_profile', 'update', 'Update speed profiles'),
('plan.speed_profile.delete', 'plan', 'speed_profile', 'delete', 'Delete speed profiles'),

-- Subscription permissions
('subscription.view', 'subscription', 'subscription', 'view', 'View subscriptions'),
('subscription.create', 'subscription', 'subscription', 'create', 'Create subscriptions'),
('subscription.update', 'subscription', 'subscription', 'update', 'Update subscriptions'),
('subscription.upgrade', 'subscription', 'subscription', 'upgrade', 'Upgrade subscriptions'),
('subscription.downgrade', 'subscription', 'subscription', 'downgrade', 'Downgrade subscriptions'),
('subscription.cancel', 'subscription', 'subscription', 'cancel', 'Cancel subscriptions'),
('subscription.suspend', 'subscription', 'subscription', 'suspend', 'Suspend subscriptions'),
('subscription.reactivate', 'subscription', 'subscription', 'reactivate', 'Reactivate subscriptions'),

-- Billing permissions
('billing.invoice.view', 'billing', 'invoice', 'view', 'View invoices'),
('billing.invoice.generate', 'billing', 'invoice', 'generate', 'Generate invoices'),
('billing.invoice.send', 'billing', 'invoice', 'send', 'Send invoices'),
('billing.invoice.void', 'billing', 'invoice', 'void', 'Void invoices'),
('billing.invoice.refund', 'billing', 'invoice', 'refund', 'Refund invoices'),
('billing.payment.view', 'billing', 'payment', 'view', 'View payments'),
('billing.payment.process', 'billing', 'payment', 'process', 'Process payments'),
('billing.payment.refund', 'billing', 'payment', 'refund', 'Refund payments'),
('billing.discount.view', 'billing', 'discount', 'view', 'View discounts'),
('billing.discount.create', 'billing', 'discount', 'create', 'Create discounts'),
('billing.discount.update', 'billing', 'discount', 'update', 'Update discounts'),
('billing.discount.delete', 'billing', 'discount', 'delete', 'Delete discounts'),
('billing.tax.view', 'billing', 'tax', 'view', 'View tax config'),
('billing.tax.configure', 'billing', 'tax', 'configure', 'Configure tax'),
('billing.dunning.view', 'billing', 'dunning', 'view', 'View dunning config'),
('billing.dunning.configure', 'billing', 'dunning', 'configure', 'Configure dunning'),

-- Network permissions
('network.vlan.view', 'network', 'vlan', 'view', 'View VLANs'),
('network.vlan.create', 'network', 'vlan', 'create', 'Create VLANs'),
('network.vlan.update', 'network', 'vlan', 'update', 'Update VLANs'),
('network.vlan.delete', 'network', 'vlan', 'delete', 'Delete VLANs'),
('network.ippool.view', 'network', 'ippool', 'view', 'View IP pools'),
('network.ippool.create', 'network', 'ippool', 'create', 'Create IP pools'),
('network.ippool.update', 'network', 'ippool', 'update', 'Update IP pools'),
('network.ippool.allocate', 'network', 'ippool', 'allocate', 'Allocate IPs'),
('network.ippool.release', 'network', 'ippool', 'release', 'Release IPs'),
('network.pppoe.view', 'network', 'pppoe', 'view', 'View PPPoE sessions'),
('network.pppoe.create', 'network', 'pppoe', 'create', 'Create PPPoE sessions'),
('network.pppoe.terminate', 'network', 'pppoe', 'terminate', 'Terminate PPPoE sessions'),

-- Device permissions
('device.router.view', 'device', 'router', 'view', 'View routers'),
('device.router.register', 'device', 'router', 'register', 'Register routers'),
('device.router.configure', 'device', 'router', 'configure', 'Configure routers'),
('device.router.restart', 'device', 'router', 'restart', 'Restart routers'),
('device.router.shutdown', 'device', 'router', 'shutdown', 'Shutdown routers'),
('device.olt.view', 'device', 'olt', 'view', 'View OLTs'),
('device.olt.register', 'device', 'olt', 'register', 'Register OLTs'),
('device.olt.configure', 'device', 'olt', 'configure', 'Configure OLTs'),
('device.ont.view', 'device', 'ont', 'view', 'View ONTs'),
('device.ont.register', 'device', 'ont', 'register', 'Register ONTs'),
('device.ont.provision', 'device', 'ont', 'provision', 'Provision ONTs'),

-- Bandwidth permissions
('bandwidth.profile.view', 'bandwidth', 'profile', 'view', 'View bandwidth profiles'),
('bandwidth.profile.create', 'bandwidth', 'profile', 'create', 'Create bandwidth profiles'),
('bandwidth.profile.update', 'bandwidth', 'profile', 'update', 'Update bandwidth profiles'),
('bandwidth.profile.delete', 'bandwidth', 'profile', 'delete', 'Delete bandwidth profiles'),
('bandwidth.profile.apply', 'bandwidth', 'profile', 'apply', 'Apply bandwidth profiles'),

-- Ticket permissions
('ticket.view', 'ticket', 'ticket', 'view', 'View tickets'),
('ticket.create', 'ticket', 'ticket', 'create', 'Create tickets'),
('ticket.update', 'ticket', 'ticket', 'update', 'Update tickets'),
('ticket.assign', 'ticket', 'ticket', 'assign', 'Assign tickets'),
('ticket.resolve', 'ticket', 'ticket', 'resolve', 'Resolve tickets'),
('ticket.close', 'ticket', 'ticket', 'close', 'Close tickets'),
('ticket.escalate', 'ticket', 'ticket', 'escalate', 'Escalate tickets'),
('ticket.reopen', 'ticket', 'ticket', 'reopen', 'Reopen tickets'),

-- Notification permissions
('notification.template.view', 'notification', 'template', 'view', 'View notification templates'),
('notification.template.create', 'notification', 'template', 'create', 'Create notification templates'),
('notification.template.update', 'notification', 'template', 'update', 'Update notification templates'),
('notification.send', 'notification', 'notification', 'send', 'Send notifications'),

-- Audit permissions
('audit.log.view', 'audit', 'log', 'view', 'View audit logs'),
('audit.log.export', 'audit', 'log', 'export', 'Export audit logs'),

-- Accounting permissions
('accounting.journal.view', 'accounting', 'journal', 'view', 'View journal entries'),
('accounting.journal.create', 'accounting', 'journal', 'create', 'Create journal entries'),
('accounting.journal.post', 'accounting', 'journal', 'post', 'Post journal entries'),
('accounting.accounts.view', 'accounting', 'accounts', 'view', 'View chart of accounts'),
('accounting.accounts.create', 'accounting', 'accounts', 'create', 'Create accounts'),
('accounting.trial_balance.view', 'accounting', 'trial_balance', 'view', 'View trial balance'),
('accounting.statements.view', 'accounting', 'statements', 'view', 'View financial statements'),
('accounting.gst.view', 'accounting', 'gst', 'view', 'View GST data'),
('accounting.gst.file', 'accounting', 'gst', 'file', 'File GST returns'),

-- Security permissions
('rbac.role.view', 'rbac', 'role', 'view', 'View roles'),
('rbac.role.create', 'rbac', 'role', 'create', 'Create roles'),
('rbac.role.update', 'rbac', 'role', 'update', 'Update roles'),
('rbac.role.delete', 'rbac', 'role', 'delete', 'Delete roles'),
('rbac.permission.view', 'rbac', 'permission', 'view', 'View permissions'),
('rbac.permission.grant', 'rbac', 'permission', 'grant', 'Grant permissions'),
('rbac.permission.revoke', 'rbac', 'permission', 'revoke', 'Revoke permissions'),
('rbac.user.role.assign', 'rbac', 'user_role', 'assign', 'Assign user roles'),
('rbac.user.role.revoke', 'rbac', 'user_role', 'revoke', 'Revoke user roles'),
('rbac.temporary.grant', 'rbac', 'temporary', 'grant', 'Grant temporary permissions'),
('rbac.approval.view', 'rbac', 'approval', 'view', 'View approval requests'),
('rbac.approval.approve', 'rbac', 'approval', 'approve', 'Approve requests'),
('rbac.approval.reject', 'rbac', 'approval', 'reject', 'Reject requests')
ON CONFLICT (name) DO NOTHING;

-- Assign permissions to roles (system roles get comprehensive permissions)
-- Super Admin gets all permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'super_admin' AND r.is_system = TRUE
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- ISP Owner gets most permissions except super_admin-specific ones
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'isp_owner' AND r.is_system = TRUE
  AND p.name NOT LIKE 'rbac.role.delete%'
  AND p.name NOT LIKE 'rbac.role.create%'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Network Admin permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'network_admin' AND r.is_system = TRUE
  AND (p.module IN ('network', 'device', 'bandwidth', 'discovery')
       OR p.name LIKE 'device.%'
       OR p.name LIKE 'network.%'
       OR p.name LIKE 'bandwidth.%')
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- NOC Engineer permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'noc_engineer' AND r.is_system = TRUE
  AND (p.action IN ('view', 'restart', 'configure')
       OR p.name LIKE '%.view'
       OR p.name LIKE 'device.router.restart'
       OR p.name LIKE 'device.ont.provision')
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Customer Support permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'customer_support' AND r.is_system = TRUE
  AND (p.module IN ('ticket', 'customer')
       OR p.name LIKE 'ticket.%'
       OR p.name LIKE 'customer.account.%')
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Sales Agent permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'sales_agent' AND r.is_system = TRUE
  AND (p.name LIKE 'lead.%'
       OR p.name LIKE 'customer.account.create'
       OR p.name LIKE 'customer.account.view'
       OR p.name LIKE 'subscription.create')
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Finance Manager permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'finance_manager' AND r.is_system = TRUE
  AND (p.module IN ('billing', 'accounting')
       OR p.name LIKE 'billing.%'
       OR p.name LIKE 'accounting.%'
       OR p.name LIKE 'rbac.approval.%')
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Billing Operator permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'billing_operator' AND r.is_system = TRUE
  AND (p.name LIKE 'billing.invoice.%'
       OR p.name LIKE 'billing.payment.%'
       OR p.name LIKE 'billing.discount.view')
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Customer self-service permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.slug = 'customer' AND r.is_system = TRUE
  AND (p.name IN ('auth.login', 'auth.logout', 'auth.password.change',
                   'customer.profile.view', 'customer.profile.update',
                   'customer.subscription.view', 'customer.address.view',
                   'customer.address.create', 'customer.address.update',
                   'ticket.view', 'ticket.create', 'ticket.reopen',
                   'notification.template.view'))
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Assign super_admin role to the first admin user (user_id = 1)
-- This will be applied after the admin user is created
-- INSERT INTO user_roles (user_id, role_id, is_active) VALUES (1, 1, TRUE);
