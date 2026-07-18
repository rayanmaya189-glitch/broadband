use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/016_seed_roles_permissions.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(manager, "DELETE FROM role_permissions WHERE role_id IN (SELECT id FROM roles WHERE is_system = TRUE)").await?;
        crate::migration::exec_stmt_raw(manager, "DELETE FROM permissions").await?;
        crate::migration::exec_stmt_raw(manager, "DELETE FROM roles WHERE is_system = TRUE")
            .await?;
        Ok(())
    }
}
