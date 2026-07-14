//! Migration runner — uses sea-orm-cli for PostgreSQL schema management.
//! See connection.rs for database pool setup.

use sea_orm::DatabaseConnection;

/// Run pending database migrations.
/// Migrations are managed via sea-orm-cli: `sea-orm-cli migrate up`
/// This function is a placeholder for runtime migration support.
pub async fn run_migrations(_pool: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    tracing::info!("Migrations should be run via: sea-orm-cli migrate up");
    Ok(())
}
