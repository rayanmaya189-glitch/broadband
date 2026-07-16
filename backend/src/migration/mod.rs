//! AeroXe Backend - Database Migration Runner
//! 
//! SeaORM migrations for all 17 database schema changes.
//! Run with: `cargo run -- migrate` or `cargo run -- migrate --apply`

use sea_orm_migration::prelude::*;

mod m001_create_extensions;
mod m002_create_branches;
mod m003_create_rbac;
mod m004_create_customers;
mod m005_create_plans;
mod m006_create_subscriptions;
mod m007_create_billing;
mod m008_create_accounting;
mod m009_create_devices;
mod m010_create_network;
mod m011_create_tickets;
mod m012_create_notifications;
mod m013_create_audit;
mod m014_create_events;
mod m015_create_documents;
mod m016_seed_roles_permissions;
mod m017_seed_initial_plans;
mod m018_add_2fa_backup_codes;

pub struct Migrator;

/// Helper to execute raw SQL from a migration file
pub async fn exec_sql_file(manager: &SchemaManager<'_>, sql: &str) -> Result<(), DbErr> {
    let conn = manager.get_connection();
    for stmt in sql.split(';') {
        let stmt = stmt.trim();
        // Skip empty statements, comments, and DO $$ blocks (they handle their own execution)
        if !stmt.is_empty() && !stmt.starts_with("--") && stmt != "DO $$" {
            conn.execute_unprepared(stmt).await?;
        }
    }
    Ok(())
}

/// Helper to drop multiple tables in reverse dependency order
pub async fn drop_tables(manager: &SchemaManager<'_>, tables: Vec<&str>) -> Result<(), DbErr> {
    let conn = manager.get_connection();
    for table in tables {
        conn.execute_unprepared(&format!("DROP TABLE IF EXISTS {} CASCADE", table)).await?;
    }
    Ok(())
}

/// Helper to execute a single raw SQL statement
pub async fn exec_stmt_raw(manager: &SchemaManager<'_>, sql: &str) -> Result<(), DbErr> {
    manager.get_connection().execute_unprepared(sql).await?;
    Ok(())
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_create_extensions::Migration),
            Box::new(m002_create_branches::Migration),
            Box::new(m003_create_rbac::Migration),
            Box::new(m004_create_customers::Migration),
            Box::new(m005_create_plans::Migration),
            Box::new(m006_create_subscriptions::Migration),
            Box::new(m007_create_billing::Migration),
            Box::new(m008_create_accounting::Migration),
            Box::new(m009_create_devices::Migration),
            Box::new(m010_create_network::Migration),
            Box::new(m011_create_tickets::Migration),
            Box::new(m012_create_notifications::Migration),
            Box::new(m013_create_audit::Migration),
            Box::new(m014_create_events::Migration),
            Box::new(m015_create_documents::Migration),
            Box::new(m016_seed_roles_permissions::Migration),
            Box::new(m017_seed_initial_plans::Migration),
            Box::new(m018_add_2fa_backup_codes::Migration),
        ]
    }
}
