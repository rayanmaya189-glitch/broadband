use std::time::Duration;

use sqlx::{FromRow, PgPool};
use tracing::{info, warn, error};

use crate::app::SharedState;

/// Default check interval: daily at 06:00 UTC equivalent (every 24 hours).
const DEFAULT_INTERVAL_SECS: u64 = 86400;

/// How many days before expiry to send the reminder.
const REMINDER_DAYS_AHEAD: i32 = 7;

#[derive(Debug, FromRow)]
struct ExpiringSubscription {
    id: i64,
    customer_id: i64,
    #[allow(dead_code)] plan_id: i64,
    branch_id: i64,
    next_billing_date: Option<chrono::NaiveDate>,
    #[allow(dead_code)] customer_name: String,
    #[allow(dead_code)] customer_phone: String,
    plan_name: String,
    price_monthly: rust_decimal::Decimal,
}

/// Find active subscriptions where next_billing_date is within REMINDER_DAYS_AHEAD.
async fn find_expiring_subscriptions(pool: &PgPool) -> Result<Vec<ExpiringSubscription>, sqlx::Error> {
    sqlx::query_as::<_, ExpiringSubscription>(
        "SELECT s.id, s.customer_id, s.plan_id, s.branch_id, s.next_billing_date,
                c.first_name || COALESCE(' ' || c.last_name, '') as customer_name,
                c.phone as customer_phone,
                p.name as plan_name,
                p.price_monthly
         FROM subscriptions s
         JOIN customers c ON c.id = s.customer_id AND c.deleted_at IS NULL
         JOIN plans p ON p.id = s.plan_id
         WHERE s.status = 'active'
           AND s.auto_renew = true
           AND s.next_billing_date IS NOT NULL
           AND s.next_billing_date <= CURRENT_DATE + INTERVAL '7 days'
           AND s.next_billing_date >= CURRENT_DATE
         ORDER BY s.next_billing_date ASC
         LIMIT 200",
    )
    .fetch_all(pool)
    .await
}

/// Record a notification event for the renewal reminder.
/// Uses the notifications table if it exists, otherwise logs to events.
async fn record_renewal_reminder(
    pool: &PgPool,
    sub: &ExpiringSubscription,
) -> Result<(), sqlx::Error> {
    let message = format!(
        "Subscription renewal reminder: Your {} plan (₹{}/month) will renew on {}. Please ensure sufficient balance.",
        sub.plan_name,
        sub.price_monthly,
        sub.next_billing_date.map(|d| d.to_string()).unwrap_or_default()
    );

    // Insert into notifications table
    sqlx::query(
        "INSERT INTO notifications (customer_id, branch_id, type, channel, title, body, metadata)
         VALUES ($1, $2, 'renewal_reminder', 'in_app',
                 'Subscription Renewal Reminder',
                 $3,
                 $4::jsonb)
         ON CONFLICT DO NOTHING",
    )
    .bind(sub.customer_id)
    .bind(sub.branch_id)
    .bind(&message)
    .bind(serde_json::json!({
        "subscription_id": sub.id,
        "plan_name": sub.plan_name,
        "renewal_date": sub.next_billing_date,
        "amount": sub.price_monthly,
    }))
    .execute(pool)
    .await?;

    // Also record as an event for the event sourcing log
    sqlx::query(
        "INSERT INTO events (event_type, entity_type, entity_id, payload, branch_id)
         VALUES ('subscription.renewal_reminder', 'subscription', $1, $2::jsonb, $3)",
    )
    .bind(sub.id)
    .bind(serde_json::json!({
        "customer_id": sub.customer_id,
        "plan_name": sub.plan_name,
        "renewal_date": sub.next_billing_date,
        "amount": sub.price_monthly,
        "days_until_renewal": sub.next_billing_date.map(|d| (d - chrono::Utc::now().date_naive()).num_days()).unwrap_or(0),
    }))
    .bind(sub.branch_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Main subscription renewal reminder loop.
pub async fn run_subscription_renewal_reminder(state: SharedState) {
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
        interval.tick().await;

        match find_expiring_subscriptions(&state.db).await {
            Ok(subs) if subs.is_empty() => {
                // No expiring subscriptions
            }
            Ok(subs) => {
                let count = subs.len();
                info!(count = count, "Found expiring subscriptions, sending reminders");

                let mut sent = 0u64;
                let mut failed = 0u64;

                for sub in &subs {
                    match record_renewal_reminder(&state.db, sub).await {
                        Ok(()) => sent += 1,
                        Err(e) => {
                            failed += 1;
                            warn!(
                                subscription_id = sub.id,
                                customer_id = sub.customer_id,
                                error = %e,
                                "Failed to record renewal reminder"
                            );
                        }
                    }
                }

                info!(
                    total = count,
                    sent = sent,
                    failed = failed,
                    "Renewal reminder batch complete"
                );
            }
            Err(e) => {
                error!(error = %e, "Failed to query expiring subscriptions");
            }
        }
    }
}
