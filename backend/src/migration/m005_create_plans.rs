use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/005_create_plans.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(manager, vec![
            "bandwidth_applications",
            "bandwidth_profiles_history",
            "bandwidth_profiles",
            "plan_service_packages",
            "service_packages",
            "speed_profiles",
            "plan_pricing",
            "plans_history",
            "plans",
        ]).await
    }
}
