//! Migration runner — uses sqlx migrate! macro for PostgreSQL schema management.
//! See connection.rs for database pool setup.

use sqlx::PgPool;

/// Run pending database migrations.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    Ok(())
}
