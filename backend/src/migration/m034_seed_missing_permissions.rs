use sea_orm_migration::prelude::*;

/// Seeds the 84 permission keys that are enforced by `require_permission` /
/// `has_permission` in the codebase but were never inserted by migrations 016
/// or 026. Without these rows, every non-company-wide role (network_admin,
/// noc_engineer, field_technician, customer_support, sales_agent,
/// billing_operator, customer) receives a hard 403 on the affected endpoints.
///
/// Idempotent: `ON CONFLICT (name) DO NOTHING` on permissions and
/// `ON CONFLICT (role_id, permission_id) DO NOTHING` on grants.
#[derive(DeriveMigrationName)]
pub struct Migration;

const PERMISSIONS_SQL: &str = r#"INSERT INTO security.permissions (name, module, resource, action, description) VALUES
('accounting.accounts.update', 'accounting', 'accounts', 'update', 'Update chart of accounts'),
('accounting.journal.void', 'accounting', 'journal', 'void', 'Void journal entries'),
('admin.data.manage', 'admin', 'data', 'manage', 'Manage platform data'),
('audit.event.export', 'audit', 'event', 'export', 'Export audit events'),
('audit.event.replay', 'audit', 'event', 'replay', 'Replay audit events'),
('audit.event.view', 'audit', 'event', 'view', 'View audit events'),
('audit.history.export', 'audit', 'history', 'export', 'Export audit history'),
('audit.history.view', 'audit', 'history', 'view', 'View audit history'),
('billing.invoice.auto_generate', 'billing', 'invoice', 'auto_generate', 'Auto-generate invoices'),
('billing.invoice.create', 'billing', 'invoice', 'create', 'Create invoices'),
('billing.payment.record', 'billing', 'payment', 'record', 'Record payments'),
('billing.tds.calculate', 'billing', 'tds', 'calculate', 'Calculate TDS'),
('billing.tds.return', 'billing', 'tds', 'return', 'File TDS returns'),
('branch.delete', 'branch', 'branch', 'delete', 'Delete branches'),
('compliance.consent.grant', 'compliance', 'consent', 'grant', 'Grant consent'),
('compliance.consent.revoke', 'compliance', 'consent', 'revoke', 'Revoke consent'),
('compliance.kyc.create', 'compliance', 'kyc', 'create', 'Submit KYC'),
('compliance.kyc.update', 'compliance', 'kyc', 'update', 'Update KYC'),
('compliance.kyc.view', 'compliance', 'kyc', 'view', 'View KYC'),
('compliance.retention.create', 'compliance', 'retention', 'create', 'Create retention policies'),
('compliance.retention.update', 'compliance', 'retention', 'update', 'Update retention policies'),
('compliance.retention.view', 'compliance', 'retention', 'view', 'View retention policies'),
('coverage.create', 'coverage', 'coverage', 'create', 'Create coverage areas'),
('coverage.view', 'coverage', 'coverage', 'view', 'View coverage areas'),
('customer.account.update_status', 'customer', 'account', 'update_status', 'Update customer account status'),
('device.port.update', 'device', 'port', 'update', 'Update device ports'),
('device.router.update_firmware', 'device', 'router', 'update_firmware', 'Update router firmware'),
('device.router.update_status', 'device', 'router', 'update_status', 'Update router status'),
('discovery.result.approve', 'discovery', 'result', 'approve', 'Approve discovery results'),
('discovery.result.view', 'discovery', 'result', 'view', 'View discovery results'),
('discovery.scan.create', 'discovery', 'scan', 'create', 'Create discovery scans'),
('discovery.scan.view', 'discovery', 'scan', 'view', 'View discovery scans'),
('document.delete', 'document', 'document', 'delete', 'Delete documents'),
('document.upload', 'document', 'document', 'upload', 'Upload documents'),
('document.view', 'document', 'document', 'view', 'View documents'),
('installation.order.cancel', 'installation', 'order', 'cancel', 'Cancel installation orders'),
('installation.order.complete', 'installation', 'order', 'complete', 'Complete installation orders'),
('installation.order.create', 'installation', 'order', 'create', 'Create installation orders'),
('installation.order.schedule', 'installation', 'order', 'schedule', 'Schedule installation orders'),
('installation.order.update', 'installation', 'order', 'update', 'Update installation orders'),
('installation.order.view', 'installation', 'order', 'view', 'View installation orders'),
('inventory.item.assign', 'inventory', 'item', 'assign', 'Assign inventory items'),
('inventory.item.create', 'inventory', 'item', 'create', 'Create inventory items'),
('inventory.item.view', 'inventory', 'item', 'view', 'View inventory items'),
('lead.assign', 'lead', 'lead', 'assign', 'Assign leads'),
('lead.convert', 'lead', 'lead', 'convert', 'Convert leads'),
('lead.create', 'lead', 'lead', 'create', 'Create leads'),
('lead.status.update', 'lead', 'status', 'update', 'Update lead status'),
('lead.update', 'lead', 'lead', 'update', 'Update leads'),
('lead.view', 'lead', 'lead', 'view', 'View leads'),
('monitoring.alert.acknowledge', 'monitoring', 'alert', 'acknowledge', 'Acknowledge monitoring alerts'),
('monitoring.alert.create', 'monitoring', 'alert', 'create', 'Create monitoring alerts'),
('monitoring.alert.resolve', 'monitoring', 'alert', 'resolve', 'Resolve monitoring alerts'),
('monitoring.alert.view', 'monitoring', 'alert', 'view', 'View monitoring alerts'),
('monitoring.metrics.view', 'monitoring', 'metrics', 'view', 'View monitoring metrics'),
('network.mac_binding.create', 'network', 'mac_binding', 'create', 'Create MAC bindings'),
('network.mac_binding.view', 'network', 'mac_binding', 'view', 'View MAC bindings'),
('notification.channel.update', 'notification', 'channel', 'update', 'Update notification channels'),
('notification.retry', 'notification', 'notification', 'retry', 'Retry notifications'),
('notification.template.delete', 'notification', 'template', 'delete', 'Delete notification templates'),
('payment.gateway.view', 'payment', 'gateway', 'view', 'View payment gateways'),
('payment.link.create', 'payment', 'link', 'create', 'Create payment links'),
('payment.manual.record', 'payment', 'manual', 'record', 'Record manual payments'),
('payment.retry', 'payment', 'payment', 'retry', 'Retry payments'),
('payment.wallet.pay', 'payment', 'wallet', 'pay', 'Pay from wallet'),
('plan.approve', 'plan', 'plan', 'approve', 'Approve plans'),
('plan.deactivate', 'plan', 'plan', 'deactivate', 'Deactivate plans'),
('plan.manage', 'plan', 'plan', 'manage', 'Manage plans'),
('plan.pricing.update', 'plan', 'pricing', 'update', 'Update plan pricing'),
('rbac.role.permission.assign', 'rbac', 'role_permission', 'assign', 'Assign permissions to roles'),
('rbac.role.permission.revoke', 'rbac', 'role_permission', 'revoke', 'Revoke permissions from roles'),
('referral.create', 'referral', 'referral', 'create', 'Create referrals'),
('referral.program.create', 'referral', 'program', 'create', 'Create referral programs'),
('referral.program.delete', 'referral', 'program', 'delete', 'Delete referral programs'),
('referral.program.update', 'referral', 'program', 'update', 'Update referral programs'),
('referral.program.view', 'referral', 'program', 'view', 'View referral programs'),
('referral.view', 'referral', 'referral', 'view', 'View referrals'),
('referral.wallet.adjust', 'referral', 'wallet', 'adjust', 'Adjust referral wallet balances'),
('referral.wallet.view', 'referral', 'wallet', 'view', 'View referral wallet balances'),
('subscription.manage', 'subscription', 'subscription', 'manage', 'Manage subscriptions'),
('ticket.comment', 'ticket', 'ticket', 'comment', 'Comment on tickets'),
('workflow.approval.create', 'workflow', 'approval', 'create', 'Create approval requests'),
('workflow.approval.read', 'workflow', 'approval', 'read', 'Read approval requests'),
('workflow.approval.review', 'workflow', 'approval', 'review', 'Review approval requests')
ON CONFLICT (name) DO NOTHING"#;

const SUPER_ADMIN_OWNER_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE
  AND r.slug IN ('super_admin', 'isp_owner')
  AND p.name IN ('accounting.accounts.update','accounting.journal.void','admin.data.manage','audit.event.export','audit.event.replay','audit.event.view','audit.history.export','audit.history.view','billing.invoice.auto_generate','billing.invoice.create','billing.payment.record','billing.tds.calculate','billing.tds.return','branch.delete','compliance.consent.grant','compliance.consent.revoke','compliance.kyc.create','compliance.kyc.update','compliance.kyc.view','compliance.retention.create','compliance.retention.update','compliance.retention.view','coverage.create','coverage.view','customer.account.update_status','device.port.update','device.router.update_firmware','device.router.update_status','discovery.result.approve','discovery.result.view','discovery.scan.create','discovery.scan.view','document.delete','document.upload','document.view','installation.order.cancel','installation.order.complete','installation.order.create','installation.order.schedule','installation.order.update','installation.order.view','inventory.item.assign','inventory.item.create','inventory.item.view','lead.assign','lead.convert','lead.create','lead.status.update','lead.update','lead.view','monitoring.alert.acknowledge','monitoring.alert.create','monitoring.alert.resolve','monitoring.alert.view','monitoring.metrics.view','network.mac_binding.create','network.mac_binding.view','notification.channel.update','notification.retry','notification.template.delete','payment.gateway.view','payment.link.create','payment.manual.record','payment.retry','payment.wallet.pay','plan.approve','plan.deactivate','plan.manage','plan.pricing.update','rbac.role.permission.assign','rbac.role.permission.revoke','referral.create','referral.program.create','referral.program.delete','referral.program.update','referral.program.view','referral.view','referral.wallet.adjust','referral.wallet.view','subscription.manage','ticket.comment','workflow.approval.create','workflow.approval.read','workflow.approval.review')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const FINANCE_MANAGER_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'finance_manager'
  AND p.name IN ('accounting.accounts.update','accounting.journal.void','audit.event.export','audit.event.view','audit.history.export','audit.history.view','billing.invoice.auto_generate','billing.invoice.create','billing.payment.record','billing.tds.calculate','billing.tds.return','compliance.kyc.view','compliance.retention.create','compliance.retention.update','compliance.retention.view','payment.gateway.view','payment.link.create','payment.manual.record','payment.retry','plan.approve','plan.pricing.update','referral.wallet.adjust','referral.wallet.view','subscription.manage','workflow.approval.create','workflow.approval.read','workflow.approval.review')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const NETWORK_ADMIN_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'network_admin'
  AND p.name IN ('audit.event.view','audit.history.view','coverage.create','coverage.view','device.port.update','device.router.update_firmware','device.router.update_status','discovery.result.approve','discovery.result.view','discovery.scan.create','discovery.scan.view','document.upload','document.view','installation.order.schedule','installation.order.view','inventory.item.assign','inventory.item.create','inventory.item.view','monitoring.alert.acknowledge','monitoring.alert.create','monitoring.alert.resolve','monitoring.alert.view','monitoring.metrics.view','network.mac_binding.create','network.mac_binding.view','notification.channel.update','notification.template.delete','plan.approve','plan.deactivate','plan.manage','ticket.comment','workflow.approval.create','workflow.approval.read')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const NOC_ENGINEER_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'noc_engineer'
  AND p.name IN ('coverage.view','device.port.update','device.router.update_status','discovery.result.view','discovery.scan.create','discovery.scan.view','document.upload','document.view','installation.order.view','inventory.item.assign','inventory.item.view','monitoring.alert.acknowledge','monitoring.alert.create','monitoring.alert.resolve','monitoring.alert.view','monitoring.metrics.view','network.mac_binding.create','network.mac_binding.view','ticket.comment')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const FIELD_TECHNICIAN_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'field_technician'
  AND p.name IN ('coverage.view','device.port.update','device.router.update_firmware','device.router.update_status','discovery.result.view','discovery.scan.create','discovery.scan.view','document.upload','document.view','installation.order.cancel','installation.order.complete','installation.order.create','installation.order.schedule','installation.order.update','installation.order.view','inventory.item.assign','inventory.item.view','network.mac_binding.view','ticket.comment')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const CUSTOMER_SUPPORT_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'customer_support'
  AND p.name IN ('compliance.consent.grant','compliance.consent.revoke','compliance.kyc.create','compliance.kyc.update','compliance.kyc.view','coverage.view','customer.account.update_status','document.upload','document.view','installation.order.cancel','installation.order.create','installation.order.schedule','installation.order.update','installation.order.view','lead.assign','lead.convert','lead.create','lead.status.update','lead.update','lead.view','monitoring.alert.view','notification.retry','payment.link.create','referral.program.view','referral.view','subscription.manage','ticket.comment','workflow.approval.create','workflow.approval.read')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const SALES_AGENT_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'sales_agent'
  AND p.name IN ('compliance.consent.grant','compliance.consent.revoke','compliance.kyc.create','coverage.view','document.upload','document.view','installation.order.cancel','installation.order.create','installation.order.schedule','installation.order.update','installation.order.view','lead.assign','lead.convert','lead.create','lead.status.update','lead.update','lead.view','payment.link.create','referral.create','referral.program.view','referral.view','referral.wallet.view','ticket.comment','workflow.approval.create','workflow.approval.read')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const BILLING_OPERATOR_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'billing_operator'
  AND p.name IN ('billing.invoice.auto_generate','billing.invoice.create','billing.payment.record','billing.tds.calculate','billing.tds.return','customer.account.update_status','document.upload','document.view','notification.retry','payment.gateway.view','payment.link.create','payment.manual.record','payment.retry','subscription.manage','ticket.comment','workflow.approval.read')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

const CUSTOMER_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE AND r.slug = 'customer'
  AND p.name IN ('compliance.consent.grant','compliance.consent.revoke','compliance.kyc.create','compliance.kyc.update','compliance.kyc.view','coverage.view','document.upload','document.view','installation.order.view','lead.create','payment.wallet.pay','referral.create','referral.program.view','referral.view','referral.wallet.view','ticket.comment','workflow.approval.read')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(manager, PERMISSIONS_SQL).await?;
        crate::migration::exec_stmt_raw(manager, SUPER_ADMIN_OWNER_SQL).await?;
        crate::migration::exec_stmt_raw(manager, FINANCE_MANAGER_SQL).await?;
        crate::migration::exec_stmt_raw(manager, NETWORK_ADMIN_SQL).await?;
        crate::migration::exec_stmt_raw(manager, NOC_ENGINEER_SQL).await?;
        crate::migration::exec_stmt_raw(manager, FIELD_TECHNICIAN_SQL).await?;
        crate::migration::exec_stmt_raw(manager, CUSTOMER_SUPPORT_SQL).await?;
        crate::migration::exec_stmt_raw(manager, SALES_AGENT_SQL).await?;
        crate::migration::exec_stmt_raw(manager, BILLING_OPERATOR_SQL).await?;
        crate::migration::exec_stmt_raw(manager, CUSTOMER_SQL).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(
            manager,
            r#"DELETE FROM security.role_permissions
WHERE permission_id IN (
    SELECT id FROM security.permissions WHERE name IN ('accounting.accounts.update','accounting.journal.void','admin.data.manage','audit.event.export','audit.event.replay','audit.event.view','audit.history.export','audit.history.view','billing.invoice.auto_generate','billing.invoice.create','billing.payment.record','billing.tds.calculate','billing.tds.return','branch.delete','compliance.consent.grant','compliance.consent.revoke','compliance.kyc.create','compliance.kyc.update','compliance.kyc.view','compliance.retention.create','compliance.retention.update','compliance.retention.view','coverage.create','coverage.view','customer.account.update_status','device.port.update','device.router.update_firmware','device.router.update_status','discovery.result.approve','discovery.result.view','discovery.scan.create','discovery.scan.view','document.delete','document.upload','document.view','installation.order.cancel','installation.order.complete','installation.order.create','installation.order.schedule','installation.order.update','installation.order.view','inventory.item.assign','inventory.item.create','inventory.item.view','lead.assign','lead.convert','lead.create','lead.status.update','lead.update','lead.view','monitoring.alert.acknowledge','monitoring.alert.create','monitoring.alert.resolve','monitoring.alert.view','monitoring.metrics.view','network.mac_binding.create','network.mac_binding.view','notification.channel.update','notification.retry','notification.template.delete','payment.gateway.view','payment.link.create','payment.manual.record','payment.retry','payment.wallet.pay','plan.approve','plan.deactivate','plan.manage','plan.pricing.update','rbac.role.permission.assign','rbac.role.permission.revoke','referral.create','referral.program.create','referral.program.delete','referral.program.update','referral.program.view','referral.view','referral.wallet.adjust','referral.wallet.view','subscription.manage','ticket.comment','workflow.approval.create','workflow.approval.read','workflow.approval.review')
)"#,
        )
        .await?;
        crate::migration::exec_stmt_raw(
            manager,
            r#"DELETE FROM security.permissions WHERE name IN ('accounting.accounts.update','accounting.journal.void','admin.data.manage','audit.event.export','audit.event.replay','audit.event.view','audit.history.export','audit.history.view','billing.invoice.auto_generate','billing.invoice.create','billing.payment.record','billing.tds.calculate','billing.tds.return','branch.delete','compliance.consent.grant','compliance.consent.revoke','compliance.kyc.create','compliance.kyc.update','compliance.kyc.view','compliance.retention.create','compliance.retention.update','compliance.retention.view','coverage.create','coverage.view','customer.account.update_status','device.port.update','device.router.update_firmware','device.router.update_status','discovery.result.approve','discovery.result.view','discovery.scan.create','discovery.scan.view','document.delete','document.upload','document.view','installation.order.cancel','installation.order.complete','installation.order.create','installation.order.schedule','installation.order.update','installation.order.view','inventory.item.assign','inventory.item.create','inventory.item.view','lead.assign','lead.convert','lead.create','lead.status.update','lead.update','lead.view','monitoring.alert.acknowledge','monitoring.alert.create','monitoring.alert.resolve','monitoring.alert.view','monitoring.metrics.view','network.mac_binding.create','network.mac_binding.view','notification.channel.update','notification.retry','notification.template.delete','payment.gateway.view','payment.link.create','payment.manual.record','payment.retry','payment.wallet.pay','plan.approve','plan.deactivate','plan.manage','plan.pricing.update','rbac.role.permission.assign','rbac.role.permission.revoke','referral.create','referral.program.create','referral.program.delete','referral.program.update','referral.program.view','referral.view','referral.wallet.adjust','referral.wallet.view','subscription.manage','ticket.comment','workflow.approval.create','workflow.approval.read','workflow.approval.review')"#,
        )
        .await?;
        Ok(())
    }
}
