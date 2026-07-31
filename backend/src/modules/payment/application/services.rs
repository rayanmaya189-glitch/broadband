use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::{debug, info, warn};

use crate::modules::payment::domain::entities::{gateway_config, payment_link, webhook_log};
use crate::shared::errors::AppError;

pub struct PaymentService;

impl PaymentService {
    /// Create a payment link for an invoice
    pub async fn create_payment_link(
        db: &DatabaseConnection,
        invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        amount: sea_orm::prelude::Decimal,
        currency: String,
        gateway_id: String,
        idempotency_key: String,
        metadata: Option<serde_json::Value>,
        expires_in_hours: i64,
    ) -> Result<payment_link::Model, AppError> {
        // Check idempotency
        let existing = payment_link::Entity::find()
            .filter(payment_link::Column::IdempotencyKey.eq(&idempotency_key))
            .one(db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to check idempotency: {}", e))
            })?;

        if let Some(link) = existing {
            debug!(link_id = %link.link_id, "Returning existing payment link (idempotent)");
            return Ok(link);
        }

        let link_id = crate::shared::utils::uuid_v7::new_v7_string().to_string();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(expires_in_hours);

        let model = payment_link::ActiveModel {
            link_id: Set(link_id.clone()),
            invoice_id: Set(invoice_id),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            amount: Set(amount),
            currency: Set(currency),
            gateway_id: Set(gateway_id.clone()),
            gateway_order_id: Set(None),
            payment_url: Set(None),
            status: Set("pending".to_string()),
            idempotency_key: Set(idempotency_key),
            metadata: Set(metadata),
            expires_at: Set(Some(expires_at)),
            paid_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let link = model.insert(db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to create payment link: {}", e))
        })?;

        info!(link_id = %link_id, invoice_id = invoice_id, amount = %amount, gateway = %gateway_id, "Created payment link");
        Ok(link)
    }

    /// Process a successful payment from gateway webhook
    pub async fn process_successful_payment(
        db: &DatabaseConnection,
        gateway_id: &str,
        gateway_transaction_id: &str,
        _amount: sea_orm::prelude::Decimal,
        _payment_method: Option<String>,
    ) -> Result<payment_link::Model, AppError> {
        // Find the payment link by gateway order ID or transaction ID
        let link = payment_link::Entity::find()
            .filter(payment_link::Column::GatewayOrderId.eq(gateway_transaction_id))
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find payment link: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Payment link not found".to_string()))?;

        if link.status == "completed" {
            debug!(link_id = %link.link_id, "Payment already processed (idempotent)");
            return Ok(link);
        }

        let now = Utc::now();
        let mut active: payment_link::ActiveModel = link.into();
        active.status = Set("completed".to_string());
        active.paid_at = Set(Some(now));
        active.updated_at = Set(now);

        let updated = active.update(db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to update payment link: {}", e))
        })?;

        info!(link_id = %updated.link_id, gateway = %gateway_id, transaction_id = %gateway_transaction_id, "Payment completed successfully");
        Ok(updated)
    }

    /// Process a failed payment from gateway webhook
    pub async fn process_failed_payment(
        db: &DatabaseConnection,
        gateway_id: &str,
        gateway_transaction_id: &str,
        error_reason: Option<String>,
    ) -> Result<payment_link::Model, AppError> {
        let link = payment_link::Entity::find()
            .filter(payment_link::Column::GatewayOrderId.eq(gateway_transaction_id))
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find payment link: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Payment link not found".to_string()))?;

        let now = Utc::now();
        let mut active: payment_link::ActiveModel = link.into();
        active.status = Set("failed".to_string());
        active.updated_at = Set(now);
        if let Some(reason) = error_reason {
            // Get current metadata or default to empty object
            let mut meta = serde_json::json!({});
            if let sea_orm::ActiveValue::Set(Some(ref v)) = active.metadata {
                meta = v.clone();
            }
            if let serde_json::Value::Object(ref mut map) = meta {
                map.insert(
                    "failure_reason".to_string(),
                    serde_json::Value::String(reason),
                );
            }
            active.metadata = Set(Some(meta));
        }

        let updated = active.update(db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to update payment link: {}", e))
        })?;

        warn!(link_id = %updated.link_id, gateway = %gateway_id, "Payment failed");
        Ok(updated)
    }

    /// Record a manual payment (cash, bank transfer, UPI at branch).
    ///
    /// Handles two business flows atomically:
    /// - Invoice settlement: `invoice_id` is provided → creates the billing payment
    ///   record, settles the invoice (paid/partial), and any excess is credited to
    ///   the customer's wallet.
    /// - Pure wallet topup: `invoice_id` is `None` → the full amount is credited to
    ///   the customer's wallet.
    ///
    /// Emits a `payment.completed` outbox event so downstream subscribers
    /// (notifications, usage, etc.) reconcile automatically.
    #[allow(clippy::too_many_arguments)]
    pub async fn record_manual_payment(
        db: &DatabaseConnection,
        invoice_id: Option<i64>,
        customer_id: i64,
        branch_id: i64,
        amount: sea_orm::prelude::Decimal,
        payment_method: String,
        reference_number: Option<String>,
        notes: Option<String>,
        recorded_by: i64,
    ) -> Result<ManualPaymentResult, AppError> {
        use crate::infrastructure::messaging::outbox;
        use crate::modules::billing::domain::entities::{invoice as invoice_entity, payment as payment_entity};
        use crate::modules::referral::domain::entities::{customer_wallet, wallet_transaction};
        use sea_orm::IntoActiveModel;
        use sea_orm::TransactionTrait;

        if amount <= sea_orm::prelude::Decimal::ZERO {
            return Err(AppError::Validation("Payment amount must be positive".into()));
        }

        let txn = db
            .begin()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to begin transaction: {}", e)))?;
        let now = Utc::now();
        let zero = sea_orm::prelude::Decimal::ZERO;

        // 1. Create the authoritative billing payment record.
        let payment_number = crate::shared::utils::business_number::new_business_number("PAY");
        let payment_model = payment_entity::ActiveModel {
            payment_number: Set(payment_number.clone()),
            invoice_id: Set(invoice_id.unwrap_or(0)),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            amount: Set(amount),
            currency: Set("INR".to_string()),
            payment_method: Set(payment_method.clone()),
            payment_gateway: Set(Some("manual".to_string())),
            gateway_transaction_id: Set(reference_number.clone()),
            status: Set("completed".to_string()),
            processed_at: Set(Some(now)),
            created_at: Set(now),
            ..Default::default()
        };
        let payment = payment_model
            .insert(&txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to record manual payment: {}", e)))?;

        // 2. Keep a payment_link record for traceability.
        let link_id = crate::shared::utils::uuid_v7::new_v7_string().to_string();
        let metadata = serde_json::json!({
            "type": "manual",
            "payment_method": payment_method,
            "reference_number": reference_number,
            "notes": notes,
            "recorded_by": recorded_by,
            "payment_id": payment.id,
        });
        let link_model = payment_link::ActiveModel {
            link_id: Set(link_id.clone()),
            invoice_id: Set(invoice_id.unwrap_or(0)),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            amount: Set(amount),
            currency: Set("INR".to_string()),
            gateway_id: Set("manual".to_string()),
            gateway_order_id: Set(None),
            payment_url: Set(None),
            status: Set("completed".to_string()),
            idempotency_key: Set(crate::shared::utils::uuid_v7::new_v7_string().to_string()),
            metadata: Set(Some(metadata)),
            expires_at: Set(None),
            paid_at: Set(Some(now)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let link = link_model
            .insert(&txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to record payment link: {}", e)))?;

        // 3. Settle the invoice when one is referenced.
        let mut invoice_status: Option<String> = None;
        let mut wallet_credit = zero;

        if let Some(inv_id) = invoice_id {
            let inv = invoice_entity::Entity::find_by_id(inv_id)
                .one(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load invoice: {}", e)))?
                .ok_or_else(|| AppError::NotFound(format!("Invoice {} not found", inv_id)))?;

            if inv.customer_id != customer_id {
                return Err(AppError::Validation("Invoice does not belong to this customer".into()));
            }
            if inv.status == "paid" {
                return Err(AppError::Validation("Invoice is already paid".into()));
            }
            if inv.status == "voided" || inv.status == "cancelled" {
                return Err(AppError::Validation(format!(
                    "Cannot pay a {} invoice",
                    inv.status
                )));
            }

            // Sum all completed payments for this invoice (including this one).
            let total_paid = payment_entity::Entity::find()
                .filter(payment_entity::Column::InvoiceId.eq(inv_id))
                .filter(payment_entity::Column::Status.eq("completed"))
                .all(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load invoice payments: {}", e)))?
                .into_iter()
                .fold(zero, |acc, p| acc + p.amount);

            let remaining = inv.total_amount - total_paid;
            let mut active = inv.into_active_model();
            if remaining <= zero {
                active.status = Set("paid".to_string());
                active.paid_at = Set(Some(now));
                invoice_status = Some("paid".to_string());
            } else {
                active.status = Set("partial".to_string());
                invoice_status = Some("partial".to_string());
            }
            active.payment_method = Set(Some(payment_method.clone()));
            active.updated_at = Set(now);
            active
                .update(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to update invoice: {}", e)))?;

            if remaining < zero {
                wallet_credit = -remaining;
            }
        } else {
            // Pure wallet topup — full amount goes to the wallet.
            wallet_credit = amount;
        }

        // 4. Credit the customer wallet with any topup/excess amount.
        let mut wallet_balance = zero;
        let mut wallet_id: Option<i64> = None;
        if wallet_credit > zero {
            let wallet = match customer_wallet::Entity::find()
                .filter(customer_wallet::Column::CustomerId.eq(customer_id))
                .one(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load wallet: {}", e)))?
            {
                Some(w) => w,
                None => customer_wallet::ActiveModel {
                    customer_id: Set(customer_id),
                    balance: Set(zero),
                    total_earned: Set(zero),
                    total_used: Set(zero),
                    currency: Set("INR".to_string()),
                    created_at: Set(now),
                    updated_at: Set(now),
                    ..Default::default()
                }
                .insert(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create wallet: {}", e)))?,
            };

            let old_balance = wallet.balance;
            let old_earned = wallet.total_earned;
            let mut wallet_active = wallet.clone().into_active_model();
            wallet_active.balance = Set(old_balance + wallet_credit);
            wallet_active.total_earned = Set(old_earned + wallet_credit);
            wallet_active.updated_at = Set(now);
            wallet_active
                .update(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to credit wallet: {}", e)))?;

            wallet_balance = old_balance + wallet_credit;
            wallet_id = Some(wallet.id);

            let tx_type = if invoice_id.is_some() {
                "overpayment"
            } else {
                "manual_topup"
            };
            wallet_transaction::ActiveModel {
                wallet_id: Set(wallet.id),
                transaction_type: Set(tx_type.to_string()),
                amount: Set(wallet_credit),
                reference_id: Set(Some(payment.id)),
                reference_type: Set(Some("payment".to_string())),
                description: Set(Some(
                    notes.clone().unwrap_or_else(|| tx_type.to_string()),
                )),
                created_at: Set(now),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to record wallet transaction: {}", e)))?;
        }

        // 5. Publish the payment.completed event for downstream reconciliation.
        outbox::insert_outbox_event(
            &txn,
            "payment.completed",
            "payment",
            payment.id,
            serde_json::json!({
                "payment_id": payment.id,
                "payment_number": payment_number,
                "invoice_id": invoice_id,
                "customer_id": customer_id,
                "amount": amount.to_string(),
                "method": payment_method,
                "manual": true,
                "wallet_credit": wallet_credit.to_string(),
            }),
            None,
            Some(recorded_by),
            Some(branch_id),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to queue payment event: {}", e)))?;

        txn.commit()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to commit transaction: {}", e)))?;

        info!(
            link_id = %link_id,
            payment_id = payment.id,
            invoice_id = invoice_id,
            amount = %amount,
            method = %payment_method,
            wallet_credit = %wallet_credit,
            "Manual payment recorded"
        );
        Ok(ManualPaymentResult {
            payment_id: payment.id,
            payment_number,
            link_id: link.link_id,
            invoice_status,
            wallet_credited: wallet_credit > zero,
            wallet_credit_amount: wallet_credit,
            wallet_balance,
            wallet_id,
        })
    }

    /// Log webhook for idempotency tracking
    pub async fn log_webhook(
        db: &DatabaseConnection,
        gateway_id: &str,
        event_id: &str,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Result<bool, AppError> {
        // Check if already processed
        let existing = webhook_log::Entity::find()
            .filter(webhook_log::Column::EventId.eq(event_id))
            .filter(webhook_log::Column::GatewayId.eq(gateway_id))
            .one(db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to check webhook log: {}", e))
            })?;

        if existing.is_some() {
            debug!(event_id = %event_id, "Webhook already processed (idempotent)");
            return Ok(true); // Already processed
        }

        let now = Utc::now();
        let model = webhook_log::ActiveModel {
            gateway_id: Set(gateway_id.to_string()),
            event_id: Set(event_id.to_string()),
            event_type: Set(event_type.to_string()),
            payload: Set(payload),
            status: Set("received".to_string()),
            error_message: Set(None),
            processed_at: Set(None),
            created_at: Set(now),
            ..Default::default()
        };

        model
            .insert(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to log webhook: {}", e)))?;

        Ok(false) // New webhook
    }

    /// Mark webhook as processed
    pub async fn mark_webhook_processed(
        db: &DatabaseConnection,
        gateway_id: &str,
        event_id: &str,
    ) -> Result<(), AppError> {
        use sea_orm::IntoActiveModel;
        if let Some(log) = webhook_log::Entity::find()
            .filter(webhook_log::Column::EventId.eq(event_id))
            .filter(webhook_log::Column::GatewayId.eq(gateway_id))
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find webhook log: {}", e)))?
        {
            let mut active = log.into_active_model();
            active.status = Set("processed".to_string());
            active.processed_at = Set(Some(Utc::now()));
            active.update(db).await.map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to update webhook log: {}", e))
            })?;
        }
        Ok(())
    }

    /// Get gateway configuration
    pub async fn get_gateway_config(
        db: &DatabaseConnection,
        gateway_id: &str,
    ) -> Result<gateway_config::Model, AppError> {
        gateway_config::Entity::find()
            .filter(gateway_config::Column::GatewayId.eq(gateway_id))
            .filter(gateway_config::Column::IsActive.eq(true))
            .one(db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to get gateway config: {}", e))
            })?
            .ok_or_else(|| AppError::NotFound("Gateway not found".to_string()))
    }

    /// List all active gateways
    pub async fn list_gateways(
        db: &DatabaseConnection,
    ) -> Result<Vec<gateway_config::Model>, AppError> {
        gateway_config::Entity::find()
            .filter(gateway_config::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to list gateways: {}", e)))
    }
}

/// Result of a recorded manual payment, used to build the API response.
#[derive(Debug, Clone)]
pub struct ManualPaymentResult {
    pub payment_id: i64,
    pub payment_number: String,
    pub link_id: String,
    pub invoice_status: Option<String>,
    pub wallet_credited: bool,
    pub wallet_credit_amount: sea_orm::prelude::Decimal,
    pub wallet_balance: sea_orm::prelude::Decimal,
    pub wallet_id: Option<i64>,
}
