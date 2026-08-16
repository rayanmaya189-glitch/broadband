use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/029_add_idempotency_constraints.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DROP INDEX IF EXISTS billing.uq_payments_gateway_transaction_id;
            DROP INDEX IF EXISTS billing.uq_invoices_subscription_period;
            DROP INDEX IF EXISTS payment.uq_webhook_logs_gateway_event;
        ";
        for stmt in crate::migration::split_sql_statements(sql) {
            manager.get_connection().execute_unprepared(&stmt).await?;
        }
        Ok(())
    }
}
