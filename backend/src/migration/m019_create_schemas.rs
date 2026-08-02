use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const MODULE_SCHEMAS: [&str; 28] = [
    "identity",
    "customer",
    "subscription",
    "billing",
    "payment",
    "network",
    "device",
    "bandwidth",
    "branches",
    "plans",
    "audit",
    "compliance",
    "ticket",
    "notification",
    "coverage",
    "discovery",
    "document",
    "inventory",
    "installation",
    "lead",
    "referral",
    "gateway",
    "security",
    "workflow",
    "accounting",
    "scheduler",
    "monitoring",
    "integrations",
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/018_create_schemas.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for schema in MODULE_SCHEMAS {
            let stmt = format!("DROP SCHEMA IF EXISTS {schema} CASCADE");
            crate::migration::exec_stmt_raw(manager, &stmt).await?;
        }
        Ok(())
    }
}
