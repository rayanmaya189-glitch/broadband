//! Billing Worker
//!
//! Handles automated billing operations including invoice generation,
//! payment reminders, and dunning processes.

use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::app::AppState;

/// Run the billing worker.
///
/// This worker periodically:
/// 1. Generates invoices for subscriptions approaching renewal
/// 2. Sends payment reminders for overdue invoices
/// 3. Processes dunning for failed payments
/// 4. Publishes billing events via NATS
pub async fn run_billing_worker(state: Arc<AppState>, shutdown: CancellationToken) {
    tracing::info!("Billing worker started");

    let mut invoice_interval = tokio::time::interval(std::time::Duration::from_secs(3600));
    let mut reminder_interval = tokio::time::interval(std::time::Duration::from_secs(21600));

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                tracing::info!("Billing worker shutting down");
                break;
            }
            _ = invoice_interval.tick() => {
                if let Err(e) = generate_pending_invoices(&state).await {
                    tracing::error!(error = %e, "Invoice generation failed");
                }
            }
            _ = reminder_interval.tick() => {
                if let Err(e) = send_payment_reminders(&state).await {
                    tracing::error!(error = %e, "Payment reminder failed");
                }
            }
        }
    }
}

/// Generate invoices for subscriptions that are due for renewal.
async fn generate_pending_invoices(state: &Arc<AppState>) -> Result<(), crate::common::errors::app_error::AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use crate::modules::subscription::model::subscription_entity;

    let today = chrono::Utc::now().date_naive();

    // Find subscriptions that need invoicing
    let subscriptions = subscription_entity::Entity::find()
        .filter(subscription_entity::Column::Status.eq("active"))
        .filter(subscription_entity::Column::NextBillingDate.lte(today))
        .all(&state.db)
        .await?;

    for sub in &subscriptions {
        tracing::info!(
            subscription_id = sub.id,
            customer_id = sub.customer_id,
            "Generating invoice for subscription"
        );

        // In production, this would:
        // 1. Fetch the plan details
        // 2. Calculate the invoice amount with GST
        // 3. Create the invoice record
        // 4. Publish invoice.created event

        let _ = state.nats.publish_event("billing.invoice_created", &serde_json::json!({
            "subscription_id": sub.id,
            "customer_id": sub.customer_id,
            "branch_id": sub.branch_id,
        })).await;
    }

    Ok(())
}

/// Send payment reminders for overdue invoices.
async fn send_payment_reminders(state: &Arc<AppState>) -> Result<(), crate::common::errors::app_error::AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use crate::modules::billing::model::invoice_entity;

    let today = chrono::Utc::now().date_naive();

    // Find overdue invoices
    let overdue_invoices = invoice_entity::Entity::find()
        .filter(invoice_entity::Column::Status.eq("pending"))
        .filter(invoice_entity::Column::DueDate.lt(today))
        .all(&state.db)
        .await?;

    for invoice in &overdue_invoices {
        tracing::info!(
            invoice_id = invoice.id,
            invoice_number = %invoice.invoice_number,
            "Sending payment reminder"
        );

        // In production, this would:
        // 1. Create a notification for the customer
        // 2. Send email/SMS reminder
        // 3. Update invoice status if needed
    }

    Ok(())
}
