use sea_orm_migration::prelude::*;

/// Removes permission rows that are seeded but never enforced by any handler.
///
/// These 71 keys have no `require_permission` / `has_permission` call anywhere
/// in `src/`, and the features they describe either (a) exist under a different
/// permission name (aliases), (b) are intentionally public/self-service, or
/// (c) do not exist as endpoints at all. Keeping them in `security.permissions`
/// makes the catalog advertise protections that do not exist, which is
/// misleading for RBAC auditing and role grants.
///
/// Idempotent: deletes only exact name matches, skipping names not present.
#[derive(DeriveMigrationName)]
pub struct Migration;

const PERMISSION_NAMES: &str = r#"'accounting.gst.view',
'audit.log.export',
'audit.log.view',
'auth.2fa.disable',
'auth.2fa.enable',
'auth.login',
'auth.logout',
'auth.password.change',
'auth.password.reset.confirm',
'auth.password.reset.request',
'auth.register',
'auth.sessions.revoke',
'auth.sessions.view',
'bandwidth.profile.apply',
'billing.discount.delete',
'billing.discount.update',
'billing.dunning.configure',
'billing.invoice.generate',
'billing.payment.process',
'billing.payment.refund',
'billing.tax.configure',
'branch.manage_staff',
'branch.manage_working_hours',
'branch.view_reports',
'customer.account.reactivate',
'customer.account.suspend',
'customer.address.delete',
'customer.address.update',
'customer.address.view',
'customer.profile.update',
'customer.profile.verify_kyc',
'customer.profile.view',
'customer.subscription.create',
'customer.subscription.downgrade',
'customer.subscription.upgrade',
'customer.subscription.view',
'device.olt.configure',
'device.olt.register',
'device.olt.view',
'device.ont.provision',
'device.ont.register',
'device.ont.view',
'network.ippool.allocate',
'network.ippool.release',
'network.ippool.update',
'network.vlan.update',
'plan.clone',
'plan.delete',
'plan.publish',
'plan.speed_profile.create',
'plan.speed_profile.delete',
'plan.speed_profile.update',
'plan.speed_profile.view',
'plan.unpublish',
'plan.update',
'plan.view',
'rbac.approval.approve',
'rbac.approval.reject',
'rbac.approval.view',
'rbac.permission.grant',
'rbac.permission.revoke',
'rbac.temporary.grant',
'subscription.update',
'ticket.reopen',
'user.account.create',
'user.account.delete',
'user.account.disable',
'user.account.enable',
'user.account.update',
'user.role.assign',
'user.role.revoke'"#;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Drop role grants referencing the dead permissions (both system and
        //    custom roles, so the FK clean-up is complete).
        let grants_sql = format!(
            r#"DELETE FROM security.role_permissions
WHERE permission_id IN (SELECT id FROM security.permissions WHERE name IN ({names}))"#,
            names = PERMISSION_NAMES
        );
        crate::migration::exec_stmt_raw(manager, &grants_sql).await?;

        // 2. Drop any permission-group memberships referencing them.
        let group_sql = format!(
            r#"DELETE FROM identity.permission_group_permissions
WHERE permission_id IN (SELECT id FROM security.permissions WHERE name IN ({names}))"#,
            names = PERMISSION_NAMES
        );
        crate::migration::exec_stmt_raw(manager, &group_sql).await?;

        // 3. Drop the permission rows themselves.
        let perms_sql = format!(
            r#"DELETE FROM security.permissions WHERE name IN ({names})"#,
            names = PERMISSION_NAMES
        );
        crate::migration::exec_stmt_raw(manager, &perms_sql).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Irreversible by design: the removed rows were never enforced, so a
        // `migrate fresh` re-seeds them from migrations 016 / 026 / 034.
        Ok(())
    }
}
