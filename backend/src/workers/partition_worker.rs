/// Partition Management Jobs per §32 docs.
/// Auto-creates monthly partitions for history, audit, notification, and events tables.
/// Runs as a scheduled background job (via scheduler module).
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use chrono::{Utc, Datelike, NaiveDate};
use tracing::{debug, info, warn};

/// Tables that require monthly partitioning per §32 docs.
const PARTITIONED_TABLES: &[&str] = &[
    "customers_history",
    "subscriptions_history",
    "plans_history",
    "invoices_history",
    "refunds_history",
    "journal_entries_history",
    "manual_payments_history",
    "network_devices_history",
    "payment_gateways_history",
    "discounts_history",
    "approval_requests_history",
    "bandwidth_profiles_history",
    "audit_logs",
    "notifications",
    "events",
];

/// Create monthly partitions for all partitioned tables.
/// Should be called monthly (e.g., by scheduler on the 1st of each month).
/// Best-effort: skips tables that don't exist yet.
pub async fn create_monthly_partitions(db: &DatabaseConnection) -> Result<(), anyhow::Error> {
    let now = Utc::now().date_naive();
    let next_month = NaiveDate::from_ymd_opt(
        if now.month() == 12 { now.year() + 1 } else { now.year() },
        if now.month() == 12 { 1 } else { now.month() + 1 },
        1,
    ).unwrap();

    let next_next_month = NaiveDate::from_ymd_opt(
        if next_month.month() == 12 { next_month.year() + 1 } else { next_month.year() },
        if next_month.month() == 12 { 1 } else { next_month.month() + 1 },
        1,
    ).unwrap();

    let partition_name_suffix = format!("{:04}_{:02}", next_month.year(), next_month.month());
    let partition_start = next_month.format("%Y-%m-01").to_string();
    let partition_end = next_next_month.format("%Y-%m-01").to_string();

    let mut created = 0u32;
    let mut skipped = 0u32;

    for table in PARTITIONED_TABLES {
        let partition_name = format!("{}_{}", table, partition_name_suffix);
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            partition_name, table, partition_start, partition_end
        );

        match db.execute(Statement::from_string(
            db.get_database_backend(),
            query,
        )).await {
            Ok(_) => {
                info!(partition = %partition_name, "Created partition");
                created += 1;
            }
            Err(e) => {
                // Best-effort: if the parent table doesn't exist, just skip
                let err_str = e.to_string();
                if err_str.contains("does not exist") || err_str.contains("relation") {
                    warn!(
                        table = table,
                        "Table does not exist yet, skipping partition creation"
                    );
                    skipped += 1;
                } else {
                    warn!(
                        table = table,
                        error = %e,
                        "Failed to create partition"
                    );
                }
            }
        }
    }

    info!(
        created = created,
        skipped = skipped,
        total = PARTITIONED_TABLES.len(),
        "Partition creation cycle complete"
    );

    Ok(())
}

/// Run data cleanup based on retention policies per §30 Appendix C.
/// Best-effort: skips tables that don't exist yet.
pub async fn run_cleanup(db: &DatabaseConnection) -> Result<u64, anyhow::Error> {
    let mut total_deleted: u64 = 0;

    // Cleanup queries - best effort, skip missing tables
    let cleanup_queries: Vec<(&str, &str)> = vec![
        ("otp_codes", "DELETE FROM otp_codes WHERE expires_at < NOW()"),
        ("user_sessions", "DELETE FROM user_sessions WHERE expires_at < NOW()"),
        ("refresh_tokens", "DELETE FROM refresh_tokens WHERE expires_at < NOW()"),
        ("device_metrics", "DELETE FROM device_metrics WHERE recorded_at < NOW() - INTERVAL '90 days'"),
        ("device_logs", "DELETE FROM device_logs WHERE recorded_at < NOW() - INTERVAL '30 days'"),
        ("notifications", "DELETE FROM notifications WHERE created_at < NOW() - INTERVAL '90 days'"),
        ("outbox_events", "DELETE FROM outbox_events WHERE published = true AND created_at < NOW() - INTERVAL '24 hours'"),
    ];

    for (table_name, query) in cleanup_queries {
        match db.execute(Statement::from_string(
            db.get_database_backend(),
            query.to_string(),
        )).await {
            Ok(result) => {
                let affected = result.rows_affected();
                if affected > 0 {
                    info!(table = table_name, rows_deleted = affected, "Cleaned up table");
                }
                total_deleted += affected;
            }
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("does not exist") || err_str.contains("relation") {
                    debug!(table = table_name, "Table does not exist yet, skipping cleanup");
                } else {
                    warn!(table = table_name, error = %e, "Failed to clean up table");
                }
            }
        }
    }

    // History cleanup per retention policies
    let retention_policies: Vec<(&str, i64)> = vec![
        ("customers_history", 2555),
        ("subscriptions_history", 2555),
        ("plans_history", 2555),
        ("invoices_history", 2555),
        ("refunds_history", 2555),
        ("journal_entries_history", 2555),
        ("manual_payments_history", 2555),
        ("network_devices_history", 1095),
        ("payment_gateways_history", 1095),
        ("discounts_history", 1095),
        ("approval_requests_history", 1095),
        ("bandwidth_profiles_history", 730),
    ];

    for (table, retention_days) in retention_policies {
        let query = format!(
            "DELETE FROM {} WHERE created_at < NOW() - INTERVAL '{} days'",
            table, retention_days
        );
        if let Ok(result) = db.execute(Statement::from_string(
            db.get_database_backend(),
            query,
        )).await {
            let affected = result.rows_affected();
            if affected > 0 {
                info!(table = table, rows_deleted = affected, "Cleaned up history");
            }
            total_deleted += affected;
        }
        // Skip silently if table doesn't exist
    }

    info!(total_deleted = total_deleted, "Data cleanup completed");
    Ok(total_deleted)
}

