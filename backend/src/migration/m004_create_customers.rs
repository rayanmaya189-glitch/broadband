use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/004_create_customers.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(manager, vec![
            "lead_activities",
            "leads",
            "coverage_pincode_map",
            "coverage_areas",
            "installation_orders",
            "addresses",
            "kyc_documents",
            "customers_history",
            "customers",
            "customer_profiles",
        ]).await
    }
}
