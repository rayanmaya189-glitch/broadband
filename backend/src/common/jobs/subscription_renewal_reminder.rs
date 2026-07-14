//! Subscription renewal reminder — sends notifications for subscriptions expiring within 7 days.
//!
//! Converted from raw sqlx to SeaORM for consistency.

use std::time::Duration;

use sea_orm::*;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;
use crate::modules::subscription::model::subscription_entity::{self, Entity as SubscriptionEntity};
use crate::modules::customer::model::customer_entity::{self, Entity as CustomerEntity};
use crate::modules::plan::model::plan_entity::{self, Entity as PlanEntity};
use crate::modules::notification::model::notification_entity;
use crate::modules::event::model::event_entity;

const DEFAULT_INTERVAL_SECS: u64 = 86400;
const REMINDER_DAYS_AHEAD: i32 = 7;

/// Find subscriptions that are active, auto-renew, and expiring within the reminder window.
async fn find_expiring_subscriptions(
    db: &DatabaseConnection,
) -> Result<Vec<(subscription_entity::Model, customer_entity::Model, plan_entity::Model)>, DbErr> {
    let today = chrono::Utc::now().date_naive();
    let deadline = today + chrono::Duration::days(REMINDER_DAYS_AHEAD as i64);

    let subscriptions = SubscriptionEntity::find()
        .filter(subscription_entity::Column::Status.eq("active"))
        .filter(subscription_entity::Column::AutoRenew.eq(true))
        .filter(subscription_entity::Column::NextBillingDate.is_not_null())
        .filter(subscription_entity::Column::NextBillingDate.lte(deadline))
        .filter(subscription_entity::Column::NextBillingDate.gte(today))
        .order_by_asc(subscription_entity::Column::NextBillingDate)
        .limit(200)
        .all(db)
        .await?;

    let mut results = Vec::new();
    for sub in subscriptions {
        let customer = match CustomerEntity::find_by_id(sub.customer_id)
            .one(db)
            .await
        {
            Ok(Some(c)) => c,
            _ => continue,
        };

        let plan = match PlanEntity::find_by_id(sub.plan_id).one(db).await {
            Ok(Some(p)) => p,
            _ => continue,
        };

        results.push((sub, customer, plan));
    }

    Ok(results)
}

/// Record a renewal reminder notification for a subscription.
async fn record_renewal_reminder(
    db: &DatabaseConnection,
    sub: &subscription_entity::Model,
    customer: &customer_entity::Model,
    plan: &plan_entity::Model,
) -> Result<(), DbErr> {
    let customer_name = format!(
        "{}{}",
        customer.first_name,
        customer
            .last_name
            .as_ref()
            .map(|ln| format!(" {ln}"))
            .unwrap_or_default()
    );

    let message = format!(
        "Renewal reminder: Your {} plan (₹{}/month) renews on {}. Ensure sufficient balance.",
        plan.name,
        plan.price_monthly,
        sub.next_billing_date
            .map(|d| d.to_string())
            .unwrap_or_default()
    );

    let notification_active = notification_entity::ActiveModel {
        customer_id: Set(Some(sub.customer_id)),
        branch_id: Set(Some(sub.branch_id)),
        r#type: Set("renewal_reminder".to_string()),
        channel: Set("in_app".to_string()),
        title: Set(Some("Subscription Renewal Reminder".to_string())),
        body: Set(Some(message)),
        status: Set("queued".to_string()),
        ..Default::default()
    };
    notification_active.insert(db).await?;

    let event_active = event_entity::ActiveModel {
        event_type: Set("subscription.renewal_reminder".to_string()),
        aggregate_type: Set("subscription".to_string()),
        aggregate_id: Set(sub.id),
        payload: Set(serde_json::json!({
            "customer_id": sub.customer_id,
            "customer_name": customer_name,
            "plan_name": plan.name,
            "renewal_date": sub.next_billing_date,
            "amount": plan.price_monthly,
        })),
        processed: Set(false),
        ..Default::default()
    };
    event_active.insert(db).await?;

    Ok(())
}

pub async fn run_subscription_renewal_reminder(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("RENEWAL_REMINDER_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(
        interval_secs = interval_secs,
        reminder_days = REMINDER_DAYS_AHEAD,
        "Subscription renewal reminder job started"
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = super::set_rls_bypass(&state.db).await {
                    warn!(error = %e, "Failed to set RLS bypass context");
                    continue;
                }

                match find_expiring_subscriptions(&state.db).await {
                    Ok(subs) if subs.is_empty() => {}
                    Ok(subs) => {
                        let count = subs.len();
                        info!(count = count, "Found expiring subscriptions, sending reminders");
                        let mut sent = 0u64;
                        let mut failed = 0u64;
                        for (sub, customer, plan) in &subs {
                            match record_renewal_reminder(&state.db, sub, customer, plan).await {
                                Ok(()) => sent += 1,
                                Err(e) => {
                                    failed += 1;
                                    warn!(subscription_id = sub.id, error = %e, "Failed to record renewal reminder");
                                }
                            }
                        }
                        info!(total = count, sent = sent, failed = failed, "Renewal reminder batch complete");
                    }
                    Err(e) => error!(error = %e, "Failed to query expiring subscriptions"),
                }
            }
            _ = token.cancelled() => {
                info!("Subscription renewal reminder shutting down gracefully");
                break;
            }
        }
    }
}
