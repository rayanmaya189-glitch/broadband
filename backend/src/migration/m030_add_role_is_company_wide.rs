use sea_orm_migration::prelude::*;

/// Add the missing `is_company_wide` column to `security.roles`.
///
/// The SeaORM `role` entity declares `is_company_wide` and the identity /
/// branch-scope / ABAC modules read it at runtime, but no earlier migration
/// ever created the column. This backfills it and marks the seeded system
/// roles that must bypass branch filtering.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(
            manager,
            "ALTER TABLE security.roles
                ADD COLUMN IF NOT EXISTS is_company_wide BOOLEAN NOT NULL DEFAULT FALSE",
        )
        .await?;
        crate::migration::exec_stmt_raw(
            manager,
            "UPDATE security.roles
                SET is_company_wide = TRUE
              WHERE slug IN ('super_admin', 'isp_owner', 'finance_manager')",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::exec_stmt_raw(
            manager,
            "ALTER TABLE security.roles DROP COLUMN IF EXISTS is_company_wide",
        )
        .await?;
        Ok(())
    }
}
