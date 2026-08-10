use sea_orm_migration::prelude::*;

/// Grants read/view permission keys to operational roles that legitimately
/// need them now that read handlers enforce permission checks (added after
/// m034).
///
/// `subscription.view` is required by `GET /api/v1/subscriptions`,
/// `GET /api/v1/subscriptions/:id` and `GET /api/v1/subscriptions/:id/history`.
/// Those endpoints are used by support, billing and sales staff, but the key
/// was previously only granted to isp_owner / noc_engineer / super_admin.
///
/// Idempotent: `ON CONFLICT (role_id, permission_id) DO NOTHING`.
#[derive(DeriveMigrationName)]
pub struct Migration;

const GRANTS_SQL: &str = r#"INSERT INTO security.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM security.roles r, security.permissions p
WHERE r.is_system = TRUE
  AND r.slug IN ('customer_support', 'billing_operator', 'sales_agent', 'finance_manager')
  AND p.name IN ('subscription.view')
ON CONFLICT (role_id, permission_id) DO NOTHING"#;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(manager, GRANTS_SQL).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(
            manager,
            r#"DELETE FROM security.role_permissions
WHERE permission_id IN (
    SELECT id FROM security.permissions WHERE name IN ('subscription.view')
)
  AND role_id IN (
    SELECT id FROM security.roles
    WHERE is_system = TRUE AND slug IN ('customer_support', 'billing_operator', 'sales_agent', 'finance_manager')
  )"#,
        )
        .await
    }
}
