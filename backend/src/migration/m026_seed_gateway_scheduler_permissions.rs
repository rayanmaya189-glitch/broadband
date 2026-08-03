use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(
            manager,
            "INSERT INTO permissions (name, module, resource, action, description) VALUES
('gateway.ratelimit.view', 'gateway', 'ratelimit', 'view', 'View rate limit rules'),
('gateway.apikey.view', 'gateway', 'apikey', 'view', 'View API keys'),
('gateway.log.view', 'gateway', 'log', 'view', 'View gateway request logs and stats'),
('scheduler.job.view', 'scheduler', 'job', 'view', 'View scheduled jobs')
ON CONFLICT (name) DO NOTHING",
        )
        .await?;

        crate::migration::exec_stmt_raw(
            manager,
            "INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.is_system = TRUE
  AND r.slug IN ('super_admin', 'isp_owner')
  AND p.name IN ('gateway.ratelimit.view', 'gateway.ratelimit.create', 'gateway.ratelimit.delete',
                 'gateway.apikey.view', 'gateway.apikey.create', 'gateway.apikey.revoke',
                 'gateway.log.view',
                 'scheduler.job.view', 'scheduler.job.create', 'scheduler.job.update',
                 'scheduler.job.delete', 'scheduler.job.trigger')
ON CONFLICT (role_id, permission_id) DO NOTHING",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(
            manager,
            "DELETE FROM role_permissions WHERE permission_id IN (SELECT id FROM permissions WHERE name IN ('gateway.ratelimit.view', 'gateway.apikey.view', 'gateway.log.view', 'scheduler.job.view'))",
        )
        .await?;
        crate::migration::exec_stmt_raw(
            manager,
            "DELETE FROM permissions WHERE name IN ('gateway.ratelimit.view', 'gateway.apikey.view', 'gateway.log.view', 'scheduler.job.view')",
        )
        .await?;
        Ok(())
    }
}
