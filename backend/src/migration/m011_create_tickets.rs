use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/011_create_tickets.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(
            manager,
            vec![
                "tickets_history",
                "ticket_status_history",
                "ticket_attachments",
                "ticket_escalations",
                "ticket_comments",
                "tickets",
            ],
        )
        .await
    }
}
