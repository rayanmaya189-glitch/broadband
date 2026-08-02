use rust_decimal_macros::dec;
use sea_orm::sea_query::{LockBehavior, LockType};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
    TransactionTrait,
};
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
        self.sync_usage_from_sessions().await?;
        self.renew_due_subscriptions().await?;
        self.check_overdue_invoices().await?;
        self.apply_late_fees_with_gst().await?;
        self.send_dunning_reminders().await?;
        self.suspend_overdue_subscriptions().await?;
        self.recognize_deferred_revenue().await?;
        self.validate_rcm_entries().await?;
        info!("Billing worker: cycle complete");
        Ok(())
    }

    /// Check for overdue invoices and update their status from 'pending'/'sent' to 'overdue'.
    pub async fn check_overdue_invoices(&self) -> anyhow::Result<()> {
        info!("Billing worker: checking overdue invoices");

        use crate::modules::billing::domain::entities::invoice;

        let txn = self.db.begin().await?;
        let today = chrono::Utc::now().date_naive();

        // Find invoices that are past due date but not yet marked overdue
        let overdue_invoices = invoice::Entity::find()
            .filter(invoice::Column::DueDate.lt(today))
            .filter(invoice::Column::Status.is_in(vec!["pending", "sent"]))
            .lock_with_behavior(LockType::Update, LockBehavior::SkipLocked)
            .all(&txn)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query overdue invoices: {}", e))?;

        let count = overdue_invoices.len();
        if count == 0 {
            info!("Billing worker: no overdue invoices found");
            txn.commit().await?;
            return Ok(());
        }

        info!(count = count, "Billing worker: found overdue invoices");

        for inv in &overdue_invoices {
            let mut active: invoice::ActiveModel = inv.clone().into();
            active.status = Set("overdue".to_string());
            active.updated_at = Set(chrono::Utc::now());

            if let Err(e) = active.update(&txn).await {
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
                &txn,
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

        txn.commit().await?;
        info!(count = count, "Billing worker: marked invoices as overdue");
        Ok(())
    }

    /// Apply late fees (2% of invoice) with GST on overdue invoices.
    /// Late fees are applied once per invoice, after 7 days overdue.
    /// GST on late fee follows same intra/inter state logic as the main invoice.
    pub async fn apply_late_fees_with_gst(&self) -> anyhow::Result<()> {
        info!("Billing worker: applying late fees with GST");

        use crate::modules::billing::domain::entities::invoice;
        use crate::modules::billing::domain::rules::tax_service;

        let txn = self.db.begin().await?;
        let today = chrono::Utc::now().date_naive();
        let late_fee_threshold_days = 7;
        let late_fee_rate = dec!(0.02); // 2% of invoice subtotal
        let late_fee_cap_rate = dec!(0.10); // Max 10% of invoice subtotal

        // Find overdue invoices that haven't had late fees applied yet
        let overdue_invoices = invoice::Entity::find()
            .filter(
                invoice::Column::DueDate
                    .lt(today - chrono::Duration::days(late_fee_threshold_days)),
            )
            .filter(invoice::Column::Status.is_in(vec!["overdue"]))
            .lock_with_behavior(LockType::Update, LockBehavior::SkipLocked)
            .all(&txn)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query overdue invoices: {}", e))?;

        let mut fees_applied = 0;

        for inv in &overdue_invoices {
            // Skip if late fees already applied
            if inv.late_fee_subtotal > rust_decimal::Decimal::ZERO {
                continue;
            }

            let late_fee_base = (inv.subtotal * late_fee_rate).round_dp(2);
            let max_fee = (inv.subtotal * late_fee_cap_rate).round_dp(2);
            let late_fee_base = std::cmp::min(late_fee_base, max_fee);

            // Determine intra/inter state for GST on late fee
            let is_intra_state = inv.place_of_supply_state.to_lowercase() == "maharashtra";
            let late_fee_gst =
                tax_service::calculate_late_fee_with_gst(late_fee_base, is_intra_state);
            let total_late_fee = late_fee_gst.late_fee_subtotal + late_fee_gst.gst.total_tax;

            let mut active: invoice::ActiveModel = inv.clone().into();
            active.late_fee_subtotal = Set(late_fee_gst.late_fee_subtotal);
            active.late_fee_gst = Set(late_fee_gst.gst.total_tax);
            active.total_amount =
                Set(inv.subtotal - inv.discount_amount + inv.tax_amount + total_late_fee);
            active.updated_at = Set(chrono::Utc::now());

            if let Err(e) = active.update(&txn).await {
                error!(
                    invoice_id = inv.id,
                    error = %e,
                    "Failed to apply late fee"
                );
                continue;
            }

            fees_applied += 1;
            info!(
                invoice_id = inv.id,
                late_fee = %late_fee_gst.late_fee_subtotal,
                gst = %late_fee_gst.gst.total_tax,
                "Applied late fee with GST"
            );
        }

        txn.commit().await?;
        info!(count = fees_applied, "Billing worker: late fees applied");
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

        // Honor the dunning policy configured in the API (suspension_day).
        let grace_period_days = crate::shared::config::dunning::SUSPENSION_DAY;

        // Find invoices overdue beyond grace period
        let cutoff_date = today - chrono::Duration::days(grace_period_days as i64);

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

    /// Sync PPPoE session usage data into subscription records for usage-based billing.
    /// Queries active PPPoE sessions and updates subscription bytes_in/bytes_out/duration.
    pub async fn sync_usage_from_sessions(&self) -> anyhow::Result<()> {
        use crate::modules::network::domain::entities::pppoe_session;
        use crate::modules::subscription::domain::entities::subscription;

        let active_sessions = pppoe_session::Entity::find()
            .filter(pppoe_session::Column::Status.eq("active"))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query active sessions: {}", e))?;

        let mut synced = 0;
        for session in &active_sessions {
            let sub = subscription::Entity::find()
                .filter(subscription::Column::Id.eq(session.subscription_id))
                .filter(subscription::Column::Status.eq("active"))
                .one(&self.db)
                .await;
            let Ok(Some(sub)) = sub else { continue };

            let bytes_used =
                Some(sub.bytes_used.unwrap_or(0) + session.bytes_in + session.bytes_out);
            let mut active: subscription::ActiveModel = sub.into();
            active.bytes_used = Set(bytes_used);
            active.last_session_duration = Set(Some(session.session_duration_seconds));
            active.last_session_at = Set(Some(chrono::Utc::now()));
            active.updated_at = Set(chrono::Utc::now());
            if let Err(e) = active.update(&self.db).await {
                error!(session_id = session.id, error = %e, "Failed to sync usage to subscription");
                continue;
            }
            synced += 1;
        }

        info!(
            count = synced,
            "Billing worker: usage synced from PPPoE sessions"
        );
        Ok(())
    }

    /// Recognize deferred revenue per Ind AS 115.
    /// Each month, recognize `monthly_recognition_amount` until fully recognized.
    pub async fn recognize_deferred_revenue(&self) -> anyhow::Result<()> {
        use crate::modules::billing::domain::entities::deferred_revenue;

        let txn = self.db.begin().await?;
        let active_entries = deferred_revenue::Entity::find()
            .filter(deferred_revenue::Column::Status.eq("active"))
            .lock_with_behavior(LockType::Update, LockBehavior::SkipLocked)
            .all(&txn)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query deferred revenue entries: {}", e))?;

        let mut recognized_count = 0;

        for entry in &active_entries {
            if entry.deferred_amount <= rust_decimal::Decimal::ZERO {
                continue;
            }

            let new_recognized = entry.recognized_amount + entry.monthly_recognition_amount;
            let new_deferred = entry.total_amount - new_recognized;
            let (final_recognized, final_deferred, new_status) =
                if new_deferred <= rust_decimal::Decimal::ZERO {
                    (
                        entry.total_amount,
                        rust_decimal::Decimal::ZERO,
                        "fully_recognized".to_string(),
                    )
                } else {
                    (new_recognized, new_deferred, "active".to_string())
                };

            let mut active: deferred_revenue::ActiveModel = entry.clone().into();
            active.recognized_amount = Set(final_recognized);
            active.deferred_amount = Set(final_deferred);
            active.status = Set(new_status);
            active.updated_at = Set(chrono::Utc::now());

            if let Err(e) = active.update(&txn).await {
                error!(entry_id = entry.id, error = %e, "Failed to recognize deferred revenue");
                continue;
            }
            recognized_count += 1;
        }

        txn.commit().await?;
        info!(
            count = recognized_count,
            "Billing worker: deferred revenue recognized"
        );
        Ok(())
    }

    /// Validate and auto-claim RCM entries for eligible vendor invoices.
    /// Reverse Charge Mechanism: when vendor is unregistered under GST, the recipient
    /// must self-accrue and pay GST, then claim ITC.
    pub async fn validate_rcm_entries(&self) -> anyhow::Result<()> {
        use crate::modules::billing::domain::entities::rcm_entry;

        let txn = self.db.begin().await?;
        let pending_entries = rcm_entry::Entity::find()
            .filter(rcm_entry::Column::ItcClaimed.eq(false))
            .lock_with_behavior(LockType::Update, LockBehavior::SkipLocked)
            .all(&txn)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query RCM entries: {}", e))?;

        let mut claimed = 0;
        for entry in &pending_entries {
            // Validate: if vendor GSTIN is present and valid format, auto-claim ITC
            let can_claim = match &entry.vendor_gstin {
                Some(gstin) => gstin.len() == 15 && !gstin.is_empty(),
                None => true, // Unregistered vendor — RCM applies, auto-claim
            };

            if can_claim && entry.total_gst > rust_decimal::Decimal::ZERO {
                let mut active: rcm_entry::ActiveModel = entry.clone().into();
                active.itc_claimed = Set(true);
                if let Err(e) = active.update(&txn).await {
                    error!(entry_id = entry.id, error = %e, "Failed to claim ITC");
                    continue;
                }
                claimed += 1;
            }
        }

        txn.commit().await?;
        info!(count = claimed, "Billing worker: RCM ITC claims processed");
        Ok(())
    }

    /// Renew active auto-renew subscriptions whose `next_billing_date` has arrived.
    /// Renewals happen before overdue/suspension checks so freshly-billed customers
    /// are not immediately treated as overdue.
    pub async fn renew_due_subscriptions(&self) -> anyhow::Result<()> {
        use crate::modules::subscription::application::services::SubscriptionService;

        info!("Billing worker: checking due subscription renewals");
        let renewed = SubscriptionService::renew_due_subscriptions(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Renewal sweep failed: {}", e))?;
        info!(
            renewed = renewed,
            "Billing worker: subscription renewals processed"
        );
        Ok(())
    }
}
