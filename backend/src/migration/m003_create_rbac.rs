use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/003_create_rbac.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(manager, vec![
            "approval_requests",
            "approval_workflows",
            "permission_group_permissions",
            "permission_groups",
            "user_roles",
            "role_permissions",
            "permissions",
            "roles",
            "user_sessions",
            "users",
        ]).await
    }
}
