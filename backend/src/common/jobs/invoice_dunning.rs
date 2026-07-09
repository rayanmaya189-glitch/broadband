use std::time::Duration;

use sqlx::FromRow;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;

const DEFAULT_INTERVAL_SECS: u64 = 21600;
const DUNNING_STAGES: &[(i32, &str)] = &[
    (1, "first_reminder"), (7, "second_reminder"),
    (15, "final_notice"), (30, "suspension_warning"),
];

#[derive(Debug, FromRow)]
struct OverdueInvoice {
    id: i64, invoice_number: String, customer_id: i64, branch_id: i64,
    subscription_id: i64, total_amount: rust_decimal::Decimal,
    #[allow(dead_code)] due_date: chrono::NaiveDate, days_overdue: i64,
    dunning_stage: Option<String>,
    #[allow(dead_code)] dunning_notified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[allow(dead_code)] customer_name: String, #[allow(dead_code)] customer_phone: String,
}

async fn find_overdue_invoices(conn: &mut sqlx::PgConnection) -> Result<Vec<OverdueInvoice>, sqlx::Error> {
    sqlx::query_as::<_, OverdueInvoice>(
        "SELECT i.id, i.invoice_number, i.customer_id, i.branch_id, i.subscription_id,
                i.total_amount, i.due_date,
                EXTRACT(DAY FROM CURRENT_DATE - i.due_date)::bigint as days_overdue,
                i.dunning_stage, i.dunning_notified_at,
                c.first_name || COALESCE(' ' || c.last_name, '') as customer_name, c.phone as customer_phone
         FROM invoices i
         JOIN customers c ON c.id = i.customer_id AND c.deleted_at IS NULL
         WHERE i.status = 'pending' AND i.due_date < CURRENT_DATE
         ORDER BY i.due_date ASC LIMIT 200",
    )    .fetch_all(&mut *conn).await
}

fn determine_dunning_stage(days_overdue: i64) -> &'static str {
    let mut stage = "first_reminder";
    for &(threshold, name) in DUNNING_STAGES {
        if days_overdue >= threshold as i64 { stage = name; }
    }
    stage
}

async fn process_dunning(conn: &mut sqlx::PgConnection, invoice: &OverdueInvoice) -> Result<(), sqlx::Error> {
    let new_stage = determine_dunning_stage(invoice.days_overdue);
    if invoice.dunning_stage.as_deref() == Some(new_stage) { return Ok(()); }

    sqlx::query("UPDATE invoices SET dunning_stage = $2, dunning_notified_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(invoice.id)    .bind(new_stage).execute(&mut *conn).await?;

    let (title, body, action) = match new_stage {
        "first_reminder" => ("Payment Reminder", format!("Invoice {} for ₹{} is {} days overdue.", invoice.invoice_number, invoice.total_amount, invoice.days_overdue), "none"),
        "second_reminder" => ("Second Payment Reminder", format!("Invoice {} for ₹{} is {} days overdue. Pay immediately.", invoice.invoice_number, invoice.total_amount, invoice.days_overdue), "none"),
        "final_notice" => ("Final Payment Notice", format!("URGENT: Invoice {} for ₹{} is {} days overdue. Final notice.", invoice.invoice_number, invoice.total_amount, invoice.days_overdue), "none"),
        "suspension_warning" => ("Service Suspension Warning", format!("CRITICAL: Invoice {} for ₹{} is {} days overdue. Service will be suspended in 7 days.", invoice.invoice_number, invoice.total_amount, invoice.days_overdue), "suspend"),
        _ => return Ok(()),
    };

    // Deduplication: skip if we already sent this customer a dunning notification today
    let dunning_key = format!("dunning_{}", new_stage);
    if super::notification_dedup::notification_exists_today(&mut *conn, invoice.customer_id, &dunning_key).await? {
        return Ok(());
    }

    sqlx::query("INSERT INTO notifications (customer_id, branch_id, type, channel, title, body, metadata) VALUES ($1, $2, 'dunning', 'in_app', $3, $4, $5::jsonb)")
        .bind(invoice.customer_id).bind(invoice.branch_id).bind(title).bind(&body)
        .bind(serde_json::json!({"invoice_id": invoice.id, "dunning_stage": new_stage, "action": action}))
        .execute(&mut *conn).await?;

    sqlx::query("INSERT INTO events (event_type, entity_type, entity_id, payload, branch_id) VALUES ('invoice.dunning', 'invoice', $1, $2::jsonb, $3)")
        .bind(invoice.id)
        .bind(serde_json::json!({"invoice_id": invoice.id, "dunning_stage": new_stage}))
        .bind(invoice.branch_id).execute(&mut *conn).await?;

    if action == "suspend" {
        sqlx::query("UPDATE subscriptions SET status = 'suspended', updated_at = NOW() WHERE id = $1 AND status = 'active'")
            .bind(invoice.subscription_id).execute(&mut *conn).await?;
        info!(invoice_id = invoice.id, "Subscription auto-suspended due to overdue invoice");
    }

    info!(invoice_id = invoice.id, invoice_number = %invoice.invoice_number, dunning_stage = new_stage, "Invoice dunning processed");
    Ok(())
}

pub async fn run_invoice_dunning(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("DUNNING_INTERVAL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Invoice dunning background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut tx = match super::rls_bypass::begin_bypass_transaction(&state.db).await {
                    Ok(t) => t, Err(e) => { warn!(error = %e, "Failed to begin RLS bypass transaction"); continue; }
                };
                match find_overdue_invoices(&mut tx).await {
                    Ok(invoices) if invoices.is_empty() => {}
                    Ok(invoices) => {
                        let count = invoices.len(); info!(count = count, "Found overdue invoices, processing dunning");
                        let mut processed = 0u64; let mut skipped = 0u64; let mut failed = 0u64;
                        for invoice in &invoices {
                            match process_dunning(&mut tx, invoice).await {
                                Ok(()) => {
                                    if invoice.dunning_stage.as_deref() == Some(determine_dunning_stage(invoice.days_overdue)) { skipped += 1; } else { processed += 1; }
                                }
                                Err(e) => { failed += 1; warn!(invoice_id = invoice.id, error = %e, "Failed to process dunning"); }
                            }
                        }
                        info!(total = count, processed = processed, skipped = skipped, failed = failed, "Dunning batch complete");
                    }
                    Err(e) => error!(error = %e, "Failed to query overdue invoices"),
                }
                if let Err(e) = tx.commit().await { error!(error = %e, "Failed to commit RLS bypass transaction"); }
            }
            _ = token.cancelled() => { info!("Invoice dunning shutting down gracefully"); break; }
        }
    }
}
