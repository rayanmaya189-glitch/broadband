use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::{error, info};

use crate::infrastructure::messaging::outbox;

/// Background worker for billing operations:
/// - Detect overdue invoices and update statuses
/// - Send dunning reminders (email/SMS)
/// - Suspend subscriptions after grace period
pub struct BillingWorker {
    db: DatabaseConnection,
}

impl BillingWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Run the full billing worker cycle.
    pub async fn run_cycle(&self) -> anyhow::Result<()> {
        info!("Billing worker: starting cycle");
        self.check_overdue_invoices().await?;
        self.send_dunning_reminders().await?;
        self.suspend_overdue_subscriptions().await?;
        info!("Billing worker: cycle complete");
        Ok(())
    }

    /// Check for overdue invoices and update their status from 'pending'/'sent' to 'overdue'.
    pub async fn check_overdue_invoices(&self) -> anyhow::Result<()> {
        info!("Billing worker: checking overdue invoices");

        use crate::modules::billing::domain::entities::invoice;

        let today = chrono::Utc::now().date_naive();

        // Find invoices that are past due date but not yet marked overdue
        let overdue_invoices = invoice::Entity::find()
            .filter(invoice::Column::DueDate.lt(today))
            .filter(invoice::Column::Status.is_in(vec!["pending", "sent"]))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query overdue invoices: {}", e))?;

        let count = overdue_invoices.len();
        if count == 0 {
            info!("Billing worker: no overdue invoices found");
            return Ok(());
        }

        info!(count = count, "Billing worker: found overdue invoices");

        for inv in &overdue_invoices {
            let mut active: invoice::ActiveModel = inv.clone().into();
            active.status = Set("overdue".to_string());
            active.updated_at = Set(chrono::Utc::now());

            if let Err(e) = active.update(&self.db).await {
                error!(
                    invoice_id = inv.id,
                    error = %e,
                    "Failed to mark invoice as overdue"
                );
                continue;
            }

            // Publish event to outbox
            let payload = serde_json::json!({
                "invoice_id": inv.id,
                "invoice_number": inv.invoice_number,
                "customer_id": inv.customer_id,
                "total_amount": inv.total_amount,
                "due_date": inv.due_date,
            });

            if let Err(e) = outbox::insert_outbox_event(
                &self.db,
                "invoice.overdue",
                "invoice",
                inv.id,
                payload,
                None,
                None,
                Some(inv.branch_id),
            )
            .await
            {
                error!(
                    invoice_id = inv.id,
                    error = %e,
                    "Failed to publish invoice.overdue event"
                );
            }
        }

        info!(count = count, "Billing worker: marked invoices as overdue");
        Ok(())
    }

    /// Send dunning reminders for overdue invoices.
    pub async fn send_dunning_reminders(&self) -> anyhow::Result<()> {
        info!("Billing worker: sending dunning reminders");

        use crate::modules::billing::domain::entities::invoice;
        use crate::modules::billing::domain::entities::payment_reminder;

        let today = chrono::Utc::now().date_naive();

        // Find overdue invoices that haven't been reminded today
        let overdue_invoices = invoice::Entity::find()
            .filter(invoice::Column::DueDate.lt(today))
            .filter(invoice::Column::Status.is_in(vec!["overdue"]))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query overdue invoices: {}", e))?;

        let mut reminders_sent = 0;

        for inv in &overdue_invoices {
            // Check if we already sent a reminder today
            let today_start = today.and_hms_opt(0, 0, 0).unwrap();
            let already_reminded = payment_reminder::Entity::find()
                .filter(payment_reminder::Column::InvoiceId.eq(inv.id))
                .filter(payment_reminder::Column::SentAt.gte(today_start))
                .one(&self.db)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to check reminders: {}", e))?;

            if already_reminded.is_some() {
                continue;
            }

            // Determine reminder type based on days overdue
            let days_overdue = (today - inv.due_date).num_days();
            let reminder_type = if days_overdue <= 3 {
                "first_reminder"
            } else if days_overdue <= 7 {
                "second_reminder"
            } else if days_overdue <= 14 {
                "final_warning"
            } else {
                "suspension_warning"
            };

            // Record the reminder (actual sending via notification worker)
            let reminder = payment_reminder::ActiveModel {
                invoice_id: Set(inv.id),
                reminder_type: Set(reminder_type.to_string()),
                channel: Set("email".to_string()),
                sent_at: Set(chrono::Utc::now()),
                status: Set("sent".to_string()),
                ..Default::default()
            };

            if let Err(e) = reminder.insert(&self.db).await {
                error!(
                    invoice_id = inv.id,
                    error = %e,
                    "Failed to record dunning reminder"
                );
                continue;
            }

            // Publish notification event
            let payload = serde_json::json!({
                "invoice_id": inv.id,
                "invoice_number": inv.invoice_number,
                "customer_id": inv.customer_id,
                "reminder_type": reminder_type,
                "days_overdue": days_overdue,
                "total_amount": inv.total_amount,
            });

            if let Err(e) = outbox::insert_outbox_event(
                &self.db,
                "notification.dunning_reminder",
                "invoice",
                inv.id,
                payload,
                None,
                None,
                Some(inv.branch_id),
            )
            .await
            {
                error!(
                    invoice_id = inv.id,
                    error = %e,
                    "Failed to publish dunning reminder event"
                );
            }

            reminders_sent += 1;
        }

        info!(
            count = reminders_sent,
            "Billing worker: dunning reminders sent"
        );
        Ok(())
    }

    /// Suspend subscriptions for customers with invoices overdue beyond grace period.
    pub async fn suspend_overdue_subscriptions(&self) -> anyhow::Result<()> {
        info!("Billing worker: checking subscriptions for suspension");

        use crate::modules::billing::domain::entities::invoice;
        use crate::modules::subscription::domain::entities::subscription;

        let today = chrono::Utc::now().date_naive();
        let grace_period_days = 30; // Configurable

        // Find invoices overdue beyond grace period
        let cutoff_date = today - chrono::Duration::days(grace_period_days);

        let overdue_invoices = invoice::Entity::find()
            .filter(invoice::Column::DueDate.lt(cutoff_date))
            .filter(invoice::Column::Status.is_in(vec!["overdue"]))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query overdue invoices: {}", e))?;

        let mut suspended = 0;

        for inv in &overdue_invoices {
            // Find active subscription for this customer
            let sub = subscription::Entity::find()
                .filter(subscription::Column::CustomerId.eq(inv.customer_id))
                .filter(subscription::Column::Status.eq("active"))
                .one(&self.db)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to query subscription: {}", e))?;

            let Some(sub) = sub else {
                continue;
            };

            // Suspend the subscription
            let mut active: subscription::ActiveModel = sub.clone().into();
            active.status = Set("suspended".to_string());
            active.updated_at = Set(chrono::Utc::now());

            if let Err(e) = active.update(&self.db).await {
                error!(
                    subscription_id = sub.id,
                    error = %e,
                    "Failed to suspend subscription"
                );
                continue;
            }

            // Publish event
            let payload = serde_json::json!({
                "subscription_id": sub.id,
                "customer_id": inv.customer_id,
                "invoice_id": inv.id,
                "reason": "overdue_payment",
                "days_overdue": (today - inv.due_date).num_days(),
            });

            if let Err(e) = outbox::insert_outbox_event(
                &self.db,
                "subscription.suspended",
                "subscription",
                sub.id,
                payload,
                None,
                None,
                Some(sub.branch_id),
            )
            .await
            {
                error!(
                    subscription_id = sub.id,
                    error = %e,
                    "Failed to publish subscription.suspended event"
                );
            }

            suspended += 1;
        }

        info!(count = suspended, "Billing worker: subscriptions suspended");
        Ok(())
    }
}
