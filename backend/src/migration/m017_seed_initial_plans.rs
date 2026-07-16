use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/017_seed_initial_plans.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(manager, "DELETE FROM notification_channels WHERE channel IN ('email', 'sms', 'whatsapp', 'push')").await?;
        crate::migration::exec_stmt_raw(manager, "DELETE FROM approval_workflows").await?;
        crate::migration::exec_stmt_raw(manager, "DELETE FROM branches WHERE slug IN ('jalgaon-main', 'bhusawal', 'mumbai', 'navi-mumbai')").await?;
        crate::migration::exec_stmt_raw(manager, "DELETE FROM speed_profiles").await?;
        crate::migration::exec_stmt_raw(manager, "DELETE FROM plan_pricing").await?;
        crate::migration::exec_stmt_raw(manager, "DELETE FROM plans WHERE slug IN ('basic-50', 'standard-100', 'premium-150', 'pro-200', 'ultimate-300', 'business-500')").await?;
        Ok(())
    }
}
