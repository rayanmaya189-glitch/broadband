use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/024_make_payment_invoice_nullable.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Column nullability is not restored; use `migrate fresh` to rebuild.
        Ok(())
    }
}
