//! Background jobs for the AeroXe backend.
//!
//! All jobs use SeaORM for data operations. The only exception is `set_rls_bypass`
//! which uses minimal raw SQL for PostgreSQL session configuration (set_config).

use sea_orm::*;

pub mod data_cleanup;
pub mod invoice_dunning;
pub mod notification_dedup;
pub mod partition_manager;
pub mod sla_checker;
pub mod subscription_renewal_reminder;
pub mod wallet_expiry_cleanup;

/// Set RLS bypass context on a SeaORM connection for background jobs.
///
/// Note: This uses minimal raw SQL because PostgreSQL's `set_config()` function
/// has no SeaORM entity equivalent. This is session configuration, not data queries.
pub async fn set_rls_bypass(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "SELECT set_config('app.current_branch_id', '0', true)",
        vec![],
    )).await?;
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "SELECT set_config('app.is_company_wide', 'true', true)",
        vec![],
    )).await?;
    Ok(())
}
