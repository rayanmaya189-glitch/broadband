//! Partition manager — creates monthly partitions for high-volume tables.
//!
//! Note: DDL operations (CREATE TABLE) cannot be done via SeaORM entities.
//! This is the only file that uses Statement for DDL - all data operations use SeaORM.

use std::time::Duration;

use chrono::Datelike;
use sea_orm::*;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::app::SharedState;

const DEFAULT_INTERVAL_SECS: u64 = 86400; // 24 hours

const PARTITIONED_TABLES: &[&str] = &[
    "audit_logs",
    "device_logs",
    "notifications",
    "events",
    "bandwidth_usage",
    "customer_sessions",
];

/// Create monthly partitions for all partitioned tables.
/// Note: DDL operations require raw SQL as SeaORM doesn't support CREATE TABLE.
async fn create_monthly_partitions(db: &DatabaseConnection) -> Result<u32, DbErr> {
    let now = chrono::Utc::now();
    let next_month = now
        .date_naive()
        .with_day(1)
        .unwrap()
        .checked_add_months(chrono::Months::new(1))
        .unwrap();
    let next_next_month = next_month
        .checked_add_months(chrono::Months::new(1))
        .unwrap();

    let suffix = next_month.format("%Y_%m").to_string();
    let start = next_month.format("%Y-%m-01").to_string();
    let end = next_next_month.format("%Y-%m-01").to_string();

    let mut created = 0u32;

    for table in PARTITIONED_TABLES {
        let partition_name = format!("{}_{}", table, suffix);

        // DDL operations require raw SQL - SeaORM doesn't support CREATE TABLE
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            partition_name, table, start, end
        );

        let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, &sql, vec![]);

        match db.execute(stmt).await {
            Ok(_) => {
                info!(
                    table = table,
                    partition = %partition_name,
                    "Created monthly partition"
                );
                created += 1;
            }
            Err(e) => {
                warn!(
                    table = table,
                    error = %e,
                    "Could not create partition (table may not be partitioned yet)"
                );
            }
        }
    }

    Ok(created)
}

pub async fn run_partition_manager(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("PARTITION_MANAGER_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Partition manager background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = super::set_rls_bypass(&state.db).await {
                    warn!(error = %e, "Failed to set RLS bypass context");
                    continue;
                }

                match create_monthly_partitions(&state.db).await {
                    Ok(created) if created > 0 => {
                        info!(created = created, "Monthly partitions created successfully");
                    }
                    Ok(_) => {}
                    Err(e) => tracing::error!(error = %e, "Failed to create monthly partitions"),
                }
            }
            _ = token.cancelled() => {
                info!("Partition manager shutting down gracefully");
                break;
            }
        }
    }
}
