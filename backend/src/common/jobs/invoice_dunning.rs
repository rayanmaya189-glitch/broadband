use std::time::Duration;

use sqlx::{FromRow, PgPool};
use tracing::{info, warn, error};

use crate::app::SharedState;

/// Default check interval: every 6 hours.
const DEFAULT_INTERVAL_SECS: u64 = 21600;

/// Dunning escalation thresholds (days overdue).
const DUNNING_STAGES: &[(i32, &str)] = &[
    (1, "first_reminder"),    // 1 day overdue → first payment reminder
    (7, "second_reminder"),   // 7 days overdue → second reminder
    (15, "final_notice"),     // 15 days overdue → final notice
    (30, "suspension_warning"), // 30 days overdue → suspension warning
];

#[derive(Debug, FromRow)]
struct OverdueInvoice {
    id: i64,
    invoice_number: String,
    customer_id: i64,
    branch_id: i64,
    subscription_id: i64,
    total_amount: rust_decimal::Decimal,
    #[allow(dead_code)] due_date: chrono::NaiveDate,
    days_overdue: i64,
    current_dunning_stage: Option<String>,
    #[allow(dead_code)] customer_name: String,
    #[allow(dead_code)] customer_phone: String,
}

/// Find overdue invoices that need dunning action.
async fn find_overdue_invoices(pool: &PgPool) -> Result<Vec<OverdueInvoice>, sqlx::Error> {
    sqlx::query_as::<_, OverdueInvoice>(
        "SELECT i.id, i.invoice_number, i.customer_id, i.branch_id, i.subscription_id,
                i.total_amount, i.due_date,
                EXTRACT(DAY FROM CURRENT_DATE - i.due_date)::bigint as days_overdue,
                i.notes as current_dunning_stage,
                c.first_name || COALESCE(' ' || c.last_name, '') as customer_name,
                c.phone as customer_phone
         FROM invoices i
         JOIN customers c ON c.id = i.customer_id AND c.deleted_at IS NULL
         WHERE i.status = 'pending'
           AND i.due_date < CURRENT_DATE
         ORDER BY i.due_date ASC
         LIMIT 200",
    )
    .fetch_all(pool)
    .await
}

/// Determine which dunning stage this invoice should be in.
fn determine_dunning_stage(days_overdue: i64) -> &'static str {
    let mut stage = "first_reminder";
    for &(threshold, name) in DUNNING_STAGES {
        if days_overdue >= threshold as i64 {
            stage = name;
        }
    }
    stage
}

/// Process dunning for a single overdue invoice.
async fn process_dunning(
    pool: &PgPool,
    invoice: &OverdueInvoice,
) -> Result<(), sqlx::Error> {
    let new_stage = determine_dunning_stage(invoice.days_overdue);

    // Skip if already at this stage (avoid duplicate notifications)
    if invoice.current_dunning_stage.as_deref() == Some(new_stage) {
        return Ok(());
    }

    // Update the invoice notes with current dunning stage
    sqlx::query(
        "UPDATE invoices SET notes = $2, updated_at = NOW() WHERE id = $1",
    )
    .bind(invoice.id)
    .bind(new_stage)
    .execute(pool)
    .await?;

    // Determine escalation action based on stage
    let (title, body, action) = match new_stage {
        "first_reminder" => (
            "Payment Reminder",
            format!(
                "Your invoice {} for ₹{} is {} days overdue. Please make payment to avoid service interruption.",
                invoice.invoice_number, invoice.total_amount, invoice.days_overdue
            ),
            "none",
        ),
        "second_reminder" => (
            "Second Payment Reminder",
            format!(
                "Your invoice {} for ₹{} is {} days overdue. This is a second reminder. Please pay immediately.",
                invoice.invoice_number, invoice.total_amount, invoice.days_overdue
            ),
            "none",
        ),
        "final_notice" => (
            "Final Payment Notice",
            format!(
                "URGENT: Your invoice {} for ₹{} is {} days overdue. This is your final notice before service suspension.",
                invoice.invoice_number, invoice.total_amount, invoice.days_overdue
            ),
            "none",
        ),
        "suspension_warning" => (
            "Service Suspension Warning",
            format!(
                "CRITICAL: Your invoice {} for ₹{} is {} days overdue. Your service will be suspended if payment is not received within 7 days.",
                invoice.invoice_number, invoice.total_amount, invoice.days_overdue
            ),
            "suspend",
        ),
        _ => return Ok(()),
    };

    // Record notification
    sqlx::query(
        "INSERT INTO notifications (customer_id, branch_id, type, channel, title, body, metadata)
         VALUES ($1, $2, 'dunning', 'in_app', $3, $4, $5::jsonb)
         ON CONFLICT DO NOTHING",
    )
    .bind(invoice.customer_id)
    .bind(invoice.branch_id)
    .bind(title)
    .bind(&body)
    .bind(serde_json::json!({
        "invoice_id": invoice.id,
        "invoice_number": invoice.invoice_number,
        "amount": invoice.total_amount,
        "days_overdue": invoice.days_overdue,
        "dunning_stage": new_stage,
        "action": action,
    }))
    .execute(pool)
    .await?;

    // Record event
    sqlx::query(
        "INSERT INTO events (event_type, entity_type, entity_id, payload, branch_id)
         VALUES ('invoice.dunning', 'invoice', $1, $2::jsonb, $3)",
    )
    .bind(invoice.id)
    .bind(serde_json::json!({
        "customer_id": invoice.customer_id,
        "invoice_number": invoice.invoice_number,
        "amount": invoice.total_amount,
        "days_overdue": invoice.days_overdue,
        "dunning_stage": new_stage,
        "action": action,
    }))
    .bind(invoice.branch_id)
    .execute(pool)
    .await?;

    // If suspension warning, auto-suspend the subscription
    if action == "suspend" {
        sqlx::query(
            "UPDATE subscriptions SET status = 'suspended', updated_at = NOW()
             WHERE id = $1 AND status = 'active'",
        )
        .bind(invoice.subscription_id)
        .execute(pool)
        .await?;

        info!(
            invoice_id = invoice.id,
            subscription_id = invoice.subscription_id,
            customer_id = invoice.customer_id,
            "Subscription auto-suspended due to overdue invoice"
        );
    }

    info!(
        invoice_id = invoice.id,
        invoice_number = %invoice.invoice_number,
        customer_id = invoice.customer_id,
        days_overdue = invoice.days_overdue,
        dunning_stage = new_stage,
        action = action,
        "Invoice dunning processed"
    );

    Ok(())
}

/// Main dunning loop.
pub async fn run_invoice_dunning(state: SharedState) {
    let interval_secs = std::env::var("DUNNING_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);

    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

    info!(
        interval_secs = interval_secs,
        "Invoice dunning background job started"
    );

    loop {
        interval.tick().await;

        match find_overdue_invoices(&state.db).await {
            Ok(invoices) if invoices.is_empty() => {
                // No overdue invoices
            }
            Ok(invoices) => {
                let count = invoices.len();
                info!(count = count, "Found overdue invoices, processing dunning");

                let mut processed = 0u64;
                let mut skipped = 0u64;
                let mut failed = 0u64;

                for invoice in &invoices {
                    match process_dunning(&state.db, invoice).await {
                        Ok(()) => {
                            if invoice.current_dunning_stage.as_deref() == Some(determine_dunning_stage(invoice.days_overdue)) {
                                skipped += 1;
                            } else {
                                processed += 1;
                            }
                        }
                        Err(e) => {
                            failed += 1;
                            warn!(
                                invoice_id = invoice.id,
                                invoice_number = %invoice.invoice_number,
                                error = %e,
                                "Failed to process dunning"
                            );
                        }
                    }
                }

                info!(
                    total = count,
                    processed = processed,
                    skipped = skipped,
                    failed = failed,
                    "Dunning batch complete"
                );
            }
            Err(e) => {
                error!(error = %e, "Failed to query overdue invoices");
            }
        }
    }
}
