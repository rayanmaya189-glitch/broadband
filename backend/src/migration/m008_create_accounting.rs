use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/008_create_accounting.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(
            manager,
            vec![
                "gst_returns",
                "trial_balances",
                "journal_entry_lines",
                "journal_entries",
                "chart_of_accounts",
            ],
        )
        .await
    }
}
