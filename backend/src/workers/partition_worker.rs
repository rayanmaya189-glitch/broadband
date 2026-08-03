use chrono::{Datelike, NaiveDate, Utc};
/// Partition Management Jobs per §32 docs.
/// Auto-creates monthly partitions for history, audit, notification, and events tables.
/// Runs as a scheduled background job (via scheduler module).
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use tracing::{debug, info, warn};

/// Tables that actually use RANGE partitioning and require monthly partitions.
/// Schema-qualified. History/audit tables without PARTITION BY RANGE are plain
/// tables and are pruned via `run_cleanup` instead.
const PARTITIONED_TABLES: &[&str] = &[
    "events",
    "audit.audit_logs",
    "notification.notifications",
    "notification.notification_history",
    "device.device_metrics",
    "device.device_logs",
    "document.document_access_logs",
    "network.customer_sessions",
];

/// Create monthly partitions for all partitioned tables.
/// Should be called monthly (e.g., by scheduler on the 1st of each month).
/// Best-effort: skips tables that don't exist yet.
pub async fn create_monthly_partitions(db: &DatabaseConnection) -> Result<(), anyhow::Error> {
    let now = Utc::now().date_naive();
    let next_month = NaiveDate::from_ymd_opt(
        if now.month() == 12 {
            now.year() + 1
        } else {
            now.year()
        },
        if now.month() == 12 {
            1
        } else {
            now.month() + 1
        },
        1,
    )
    .ok_or_else(|| anyhow::anyhow!("Failed to calculate next month date"))?;

    let next_next_month = NaiveDate::from_ymd_opt(
        if next_month.month() == 12 {
            next_month.year() + 1
        } else {
            next_month.year()
        },
        if next_month.month() == 12 {
            1
        } else {
            next_month.month() + 1
        },
        1,
    )
    .ok_or_else(|| anyhow::anyhow!("Failed to calculate next-next month date"))?;

    let partition_name_suffix = format!("{:04}_{:02}", next_month.year(), next_month.month());
    let partition_start = next_month.format("%Y-%m-01").to_string();
    let partition_end = next_next_month.format("%Y-%m-01").to_string();

    let mut created = 0u32;
    let mut skipped = 0u32;

    for table in PARTITIONED_TABLES {
        // Partition name derived from the qualified table name (dots → underscores).
        let partition_name = format!("{}_{}", table.replace('.', "_"), partition_name_suffix);
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            partition_name, table, partition_start, partition_end
        );

        match db
            .execute(Statement::from_string(db.get_database_backend(), query))
            .await
        {
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

    // Cleanup queries - schema-qualified, columns verified against migrations.
    let cleanup_queries: Vec<(&str, &str)> = vec![
        ("identity.otp_codes", "DELETE FROM identity.otp_codes WHERE expires_at < NOW()"),
        ("identity.user_sessions", "DELETE FROM identity.user_sessions WHERE expires_at < NOW()"),
        ("device.device_metrics", "DELETE FROM device.device_metrics WHERE recorded_at < NOW() - INTERVAL '90 days'"),
        ("device.device_logs", "DELETE FROM device.device_logs WHERE created_at < NOW() - INTERVAL '30 days'"),
        ("notification.notifications", "DELETE FROM notification.notifications WHERE created_at < NOW() - INTERVAL '90 days'"),
        ("outbox_events", "DELETE FROM outbox_events WHERE published = true AND created_at < NOW() - INTERVAL '24 hours'"),
    ];

    for (table_name, query) in cleanup_queries {
        match db
            .execute(Statement::from_string(
                db.get_database_backend(),
                query.to_string(),
            ))
            .await
        {
            Ok(result) => {
                let affected = result.rows_affected();
                if affected > 0 {
                    info!(
                        table = table_name,
                        rows_deleted = affected,
                        "Cleaned up table"
                    );
                }
                total_deleted += affected;
            }
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("does not exist") || err_str.contains("relation") {
                    debug!(
                        table = table_name,
                        "Table does not exist yet, skipping cleanup"
                    );
                } else {
                    warn!(table = table_name, error = %e, "Failed to clean up table");
                }
            }
        }
    }

    // History cleanup per retention policies.
    // Schema-qualified; history tables timestamp column is `performed_at`.
    let retention_policies: Vec<(&str, i64)> = vec![
        ("customer.customers_history", 2555),
        ("subscription.subscriptions_history", 2555),
        ("plans.plans_history", 2555),
        ("billing.invoices_history", 2555),
        ("billing.refunds_history", 2555),
        ("device.network_devices_history", 1095),
        ("billing.discounts_history", 1095),
        ("plans.bandwidth_profiles_history", 730),
    ];

    for (table, retention_days) in retention_policies {
        let query = format!(
            "DELETE FROM {} WHERE performed_at < NOW() - INTERVAL '{} days'",
            table, retention_days
        );
        if let Ok(result) = db
            .execute(Statement::from_string(db.get_database_backend(), query))
            .await
        {
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
