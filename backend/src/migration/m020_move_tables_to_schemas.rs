use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/019_move_tables_to_schemas.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Irreversible: moving tables between schemas is not undone. Roll back
        // by running `migrate fresh` (drops module schemas + rebuilds).
        Ok(())
    }
}
