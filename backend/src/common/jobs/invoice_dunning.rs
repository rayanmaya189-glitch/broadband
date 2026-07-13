//! Invoice dunning — processes overdue invoices through reminder stages.
//!
//! Converted from raw sqlx to SeaORM for consistency.

use std::time::Duration;

use sea_orm::*;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;
use crate::modules::billing::model::invoice_entity::{self, Entity as InvoiceEntity};
use crate::modules::customer::model::customer_entity::{self, Entity as CustomerEntity};
use crate::modules::subscription::model::subscription_entity::{self, Entity as SubscriptionEntity};
use crate::modules::notification::model::notification_entity;
use crate::modules::event::model::event_entity;

const DEFAULT_INTERVAL_SECS: u64 = 21600; // 6 hours

const DUNNING_STAGES: &[(i32, &str, &str, &str)] = &[
    (1, "first_reminder", "Payment Reminder", "Your invoice is overdue. Please pay soon."),
    (7, "second_reminder", "Second Payment Reminder", "Your invoice is significantly overdue. Pay immediately."),
    (15, "final_notice", "Final Payment Notice", "URGENT: Final notice before service suspension."),
    (30, "suspension_warning", "Service Suspension Warning", "CRITICAL: Service will be suspended within 7 days."),
];

fn determine_dunning_stage(days_overdue: i64) -> &'static str {
    let mut stage = "first_reminder";
    for &(threshold, name, _, _) in DUNNING_STAGES {
        if days_overdue >= threshold as i64 {
            stage = name;
        }
    }
    stage
}

/// Find overdue invoices (status = 'pending' and due_date < today).
async fn find_overdue_invoices(
    db: &DatabaseConnection,
) -> Result<Vec<(invoice_entity::Model, customer_entity::Model)>, DbErr> {
    let today = chrono::Utc::now().date_naive();

    let invoices = InvoiceEntity::find()
        .filter(invoice_entity::Column::Status.eq("pending"))
        .filter(invoice_entity::Column::DueDate.lt(today))
        .order_by_asc(invoice_entity::Column::DueDate)
        .limit(200)
        .all(db)
        .await?;

    let mut results = Vec::new();
    for invoice in invoices {
        let customer = match CustomerEntity::find_by_id(invoice.customer_id)
            .one(db)
            .await
        {
            Ok(Some(c)) => c,
            _ => continue,
        };
        results.push((invoice, customer));
    }

    Ok(results)
}

/// Process dunning for a single overdue invoice.
async fn process_dunning(
    db: &DatabaseConnection,
    invoice: &invoice_entity::Model,
    customer: &customer_entity::Model,
) -> Result<bool, DbErr> {
    let today = chrono::Utc::now().date_naive();
    let days_overdue = (today - invoice.due_date).num_days();
    let new_stage = determine_dunning_stage(days_overdue);

    // Determine notification content
    let (title, body, action) = DUNNING_STAGES
        .iter()
        .find(|&&(_, stage, _, _)| stage == new_stage)
        .map(|&(_, _, title, body_tpl)| {
            let body = format!(
                "{} Invoice {} for ₹{} is {} days overdue.",
                body_tpl,
                invoice.invoice_number,
                invoice.total_amount,
                days_overdue
            );
            let action = if new_stage == "suspension_warning" {
                "suspend"
            } else {
                "none"
            };
            (title.to_string(), body, action)
        })
        .unwrap_or_default();

    let customer_name = format!(
        "{}{}",
        customer.first_name,
        customer
            .last_name
            .as_ref()
            .map(|ln| format!(" {ln}"))
            .unwrap_or_default()
    );

    let notification_active = notification_entity::ActiveModel {
        customer_id: Set(Some(invoice.customer_id)),
        branch_id: Set(Some(invoice.branch_id)),
        r#type: Set("dunning".to_string()),
        channel: Set("in_app".to_string()),
        title: Set(Some(title)),
        body: Set(Some(body)),
        status: Set("queued".to_string()),
        ..Default::default()
    };
    notification_active.insert(db).await?;

    let event_active = event_entity::ActiveModel {
        event_type: Set("invoice.dunning".to_string()),
        aggregate_type: Set("invoice".to_string()),
        aggregate_id: Set(invoice.id),
        payload: Set(serde_json::json!({
            "invoice_id": invoice.id,
            "invoice_number": invoice.invoice_number,
            "customer_id": invoice.customer_id,
            "customer_name": customer_name,
            "dunning_stage": new_stage,
            "days_overdue": days_overdue,
            "total_amount": invoice.total_amount,
            "action": action,
        })),
        processed: Set(false),
        ..Default::default()
    };
    event_active.insert(db).await?;

    // Auto-suspend subscription if at suspension_warning stage
    if action == "suspend" {
        let sub_result = SubscriptionEntity::find()
            .filter(subscription_entity::Column::Id.eq(invoice.subscription_id))
            .filter(subscription_entity::Column::Status.eq("active"))
            .one(db)
            .await;

        if let Ok(Some(sub)) = sub_result {
            let mut sub_active: subscription_entity::ActiveModel = sub.into();
            sub_active.status = Set("suspended".to_string());
            sub_active.updated_at = Set(chrono::Utc::now().into());
            sub_active.update(db).await?;
            info!(
                invoice_id = invoice.id,
                "Subscription auto-suspended due to overdue invoice"
            );
        }
    }

    info!(
        invoice_id = invoice.id,
        invoice_number = %invoice.invoice_number,
        dunning_stage = new_stage,
        "Invoice dunning processed"
    );
    Ok(true)
}

pub async fn run_invoice_dunning(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("DUNNING_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Invoice dunning background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = super::set_rls_bypass(&state.db_seaorm).await {
                    warn!(error = %e, "Failed to set RLS bypass context");
                    continue;
                }

                match find_overdue_invoices(&state.db_seaorm).await {
                    Ok(invoices) if invoices.is_empty() => {}
                    Ok(invoices) => {
                        let count = invoices.len();
                        info!(count = count, "Found overdue invoices, processing dunning");
                        let mut processed = 0u64;
                        let mut skipped = 0u64;
                        let mut failed = 0u64;
                        for (invoice, customer) in &invoices {
                            match process_dunning(&state.db_seaorm, invoice, customer).await {
                                Ok(true) => processed += 1,
                                Ok(false) => skipped += 1,
                                Err(e) => {
                                    failed += 1;
                                    warn!(invoice_id = invoice.id, error = %e, "Failed to process dunning");
                                }
                            }
                        }
                        info!(total = count, processed = processed, skipped = skipped, failed = failed, "Dunning batch complete");
                    }
                    Err(e) => error!(error = %e, "Failed to query overdue invoices"),
                }
            }
            _ = token.cancelled() => {
                info!("Invoice dunning shutting down gracefully");
                break;
            }
        }
    }
}
