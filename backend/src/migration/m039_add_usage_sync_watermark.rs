use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/030_add_usage_sync_watermark.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE network.pppoe_sessions DROP COLUMN IF EXISTS usage_synced_bytes_in;\n\
                 ALTER TABLE network.pppoe_sessions DROP COLUMN IF EXISTS usage_synced_bytes_out;",
            )
            .await?;
        Ok(())
    }
}
