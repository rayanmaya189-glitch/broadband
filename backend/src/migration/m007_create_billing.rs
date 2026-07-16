use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/007_create_billing.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(manager, vec![
            "discounts_history",
            "refunds_history",
            "invoices_history",
            "payment_reminders",
            "discounts",
            "refunds",
            "payments",
            "invoice_line_items",
            "invoices",
        ]).await
    }
}
