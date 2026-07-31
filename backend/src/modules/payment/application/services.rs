use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::{debug, info, warn};

use crate::modules::payment::domain::entities::{gateway_config, payment_link, webhook_log};
use crate::modules::payment::infrastructure::gateway_adapter::{
    GatewayAdapter, PayuAdapter, RazorpayAdapter, StripeAdapter,
};
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

        // Validate the gateway is active before creating the order.
        gateway_config::Entity::find()
            .filter(gateway_config::Column::GatewayId.eq(&gateway_id))
            .filter(gateway_config::Column::IsActive.eq(true))
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load gateway config: {}", e)))?
            .ok_or_else(|| AppError::Validation(format!("Payment gateway '{}' is not active", gateway_id)))?;

        // Create the real gateway order so the webhook can match by order ID.
        let receipt = format!("{}-{}", invoice_id, idempotency_key);
        let mut gw_meta = serde_json::json!({
            "customer_id": customer_id,
            "branch_id": branch_id,
            "invoice_id": invoice_id,
        });
        if let (serde_json::Value::Object(ref mut map), Some(serde_json::Value::Object(ref src))) =
            (&mut gw_meta, metadata.as_ref())
        {
            for (k, v) in src {
                map.insert(k.clone(), v.clone());
            }
        }

        let gateway_response = match gateway_id.as_str() {
            "razorpay" => {
                let adapter = RazorpayAdapter::from_env();
                if adapter.key_id.is_empty() || adapter.key_secret.is_empty() {
                    return Err(AppError::External(
                        "Razorpay is not configured (set RAZORPAY_KEY_ID / RAZORPAY_KEY_SECRET)".into(),
                    ));
                }
                adapter
                    .create_payment_link(amount, &currency, &receipt, gw_meta)
                    .await?
            }
            "payu" => {
                let adapter = PayuAdapter::from_env();
                if adapter.merchant_key.is_empty() || adapter.merchant_salt.is_empty() {
                    return Err(AppError::External(
                        "PayU is not configured (set PAYU_MERCHANT_KEY / PAYU_MERCHANT_SALT)".into(),
                    ));
                }
                adapter
                    .create_payment_link(amount, &currency, &receipt, gw_meta)
                    .await?
            }
            "stripe" => {
                let adapter = StripeAdapter::from_env();
                if !adapter.is_configured() {
                    return Err(AppError::External(
                        "Stripe is not configured (set STRIPE_SECRET_KEY)".into(),
                    ));
                }
                adapter
                    .create_payment_link(amount, &currency, &receipt, gw_meta)
                    .await?
            }
            other => {
                return Err(AppError::Validation(format!(
                    "Unsupported payment gateway '{}'",
                    other
                )));
            }
        };

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
            gateway_order_id: Set(Some(gateway_response.order_id)),
            payment_url: Set(Some(gateway_response.payment_url)),
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

    /// Process a successful payment from gateway webhook.
    ///
    /// Reconciles the full payment chain atomically: creates the authoritative
    /// billing payment record, marks the payment link completed, settles the
    /// invoice (paid/partial), credits any excess to the customer wallet, and
    /// emits a `payment.completed` outbox event for downstream reconciliation.
    pub async fn process_successful_payment(
        db: &DatabaseConnection,
        gateway_id: &str,
        gateway_transaction_id: &str,
        order_id: Option<&str>,
        amount: sea_orm::prelude::Decimal,
        payment_method: Option<String>,
    ) -> Result<payment_link::Model, AppError> {
        use crate::infrastructure::messaging::outbox;
        use crate::modules::billing::domain::entities::payment as payment_entity;
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

        // Resolve the payment link: prefer the webhook order id, then fall back to
        // the transaction id (PayU/Stripe send the stored id as the transaction id,
        // Razorpay sends the payment id but carries the order id separately).
        let by_order = if let Some(oid) = order_id.filter(|o| !o.is_empty()) {
            payment_link::Entity::find()
                .filter(payment_link::Column::GatewayOrderId.eq(oid))
                .one(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find payment link: {}", e)))?
        } else {
            None
        };
        let link = match by_order {
            Some(l) => l,
            None => payment_link::Entity::find()
                .filter(payment_link::Column::GatewayOrderId.eq(gateway_transaction_id))
                .one(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find payment link: {}", e)))?
                .ok_or_else(|| AppError::NotFound("Payment link not found".to_string()))?,
        };

        // Idempotency: skip only when the link is completed AND a matching payment
        // record already exists. This also heals links completed by older code that
        // never reconciled the invoice.
        let existing_payment = payment_entity::Entity::find()
            .filter(payment_entity::Column::GatewayTransactionId.eq(gateway_transaction_id))
            .one(&txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to check payment: {}", e)))?;

        if link.status == "completed" && existing_payment.is_some() {
            debug!(link_id = %link.link_id, "Payment already processed (idempotent)");
            return Ok(link);
        }

        // 1. Create the authoritative billing payment record.
        let payment_number = crate::shared::utils::business_number::new_business_number("PAY");
        let method = payment_method.unwrap_or_else(|| "online".to_string());
        let payment_model = payment_entity::ActiveModel {
            payment_number: Set(payment_number.clone()),
            invoice_id: Set(link.invoice_id),
            customer_id: Set(link.customer_id),
            branch_id: Set(link.branch_id),
            amount: Set(amount),
            currency: Set(link.currency.clone()),
            payment_method: Set(method.clone()),
            payment_gateway: Set(Some(gateway_id.to_string())),
            gateway_transaction_id: Set(Some(gateway_transaction_id.to_string())),
            status: Set("completed".to_string()),
            processed_at: Set(Some(now)),
            created_at: Set(now),
            ..Default::default()
        };
        let payment = payment_model
            .insert(&txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to record payment: {}", e)))?;

        // 2. Mark the payment link completed.
        let mut active: payment_link::ActiveModel = link.clone().into();
        active.status = Set("completed".to_string());
        active.paid_at = Set(Some(now));
        active.updated_at = Set(now);
        let updated = active
            .update(&txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to update payment link: {}", e)))?;

        // 3. Settle the invoice and credit any excess to the wallet.
        let total_paid = payment_entity::Entity::find()
            .filter(payment_entity::Column::InvoiceId.eq(link.invoice_id))
            .filter(payment_entity::Column::Status.eq("completed"))
            .all(&txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load invoice payments: {}", e)))?
            .into_iter()
            .fold(zero, |acc, p| acc + p.amount);

        Self::settle_invoice(&txn, link.invoice_id, link.customer_id, total_paid, &method).await?;

        let mut wallet_credit = zero;
        if let Some(inv) =
            crate::modules::billing::domain::entities::invoice::Entity::find_by_id(link.invoice_id)
                .one(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load invoice: {}", e)))?
        {
            if total_paid > inv.total_amount {
                wallet_credit = total_paid - inv.total_amount;
            }
        }
        if wallet_credit > zero {
            Self::credit_wallet(
                &txn,
                link.customer_id,
                wallet_credit,
                "overpayment",
                payment.id,
                None,
            )
            .await?;
        }

        // 4. Publish the payment.completed event for downstream reconciliation.
        outbox::insert_outbox_event(
            &txn,
            "payment.completed",
            "payment",
            payment.id,
            serde_json::json!({
                "payment_id": payment.id,
                "payment_number": payment_number,
                "invoice_id": link.invoice_id,
                "customer_id": link.customer_id,
                "amount": amount.to_string(),
                "method": method,
                "gateway": gateway_id,
                "manual": false,
                "wallet_credit": wallet_credit.to_string(),
            }),
            None,
            None,
            Some(link.branch_id),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to queue payment event: {}", e)))?;

        txn.commit()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to commit transaction: {}", e)))?;

        info!(
            link_id = %updated.link_id,
            payment_id = payment.id,
            invoice_id = link.invoice_id,
            amount = %amount,
            gateway = %gateway_id,
            transaction_id = %gateway_transaction_id,
            wallet_credit = %wallet_credit,
            "Payment completed successfully"
        );
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

        // 3. Settle the invoice and determine any wallet credit.
        let mut invoice_status: Option<String> = None;
        let mut wallet_credit = zero;

        if let Some(inv_id) = invoice_id {
            // Sum all completed payments for this invoice (including this one).
            let total_paid = payment_entity::Entity::find()
                .filter(payment_entity::Column::InvoiceId.eq(inv_id))
                .filter(payment_entity::Column::Status.eq("completed"))
                .all(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load invoice payments: {}", e)))?
                .into_iter()
                .fold(zero, |acc, p| acc + p.amount);

            invoice_status = Self::settle_invoice(&txn, inv_id, customer_id, total_paid, &payment_method).await?;

            if let Some(inv) = invoice_entity::Entity::find_by_id(inv_id)
                .one(&txn)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load invoice: {}", e)))?
            {
                if total_paid > inv.total_amount {
                    wallet_credit = total_paid - inv.total_amount;
                }
            }
        } else {
            // Pure wallet topup — full amount goes to the wallet.
            wallet_credit = amount;
        }

        // 4. Credit the customer wallet with any topup/excess amount.
        let mut wallet_balance = zero;
        let mut wallet_id: Option<i64> = None;
        if wallet_credit > zero {
            let tx_type = if invoice_id.is_some() {
                "overpayment"
            } else {
                "manual_topup"
            };
            let desc = notes.clone().unwrap_or_else(|| tx_type.to_string());
            let (balance, wid) =
                Self::credit_wallet(&txn, customer_id, wallet_credit, tx_type, payment.id, Some(&desc))
                    .await?;
            wallet_balance = balance;
            wallet_id = Some(wid);
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

    /// Settle an invoice based on total paid, updating status to paid/partial.
    /// Validates invoice ownership and state. Returns the new status.
    async fn settle_invoice(
        txn: &sea_orm::DatabaseTransaction,
        invoice_id: i64,
        customer_id: i64,
        total_paid: sea_orm::prelude::Decimal,
        payment_method: &str,
    ) -> Result<Option<String>, AppError> {
        use crate::modules::billing::domain::entities::invoice as invoice_entity;
        use sea_orm::IntoActiveModel;

        let now = chrono::Utc::now();
        let zero = sea_orm::prelude::Decimal::ZERO;

        let inv = invoice_entity::Entity::find_by_id(invoice_id)
            .one(txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load invoice: {}", e)))?
            .ok_or_else(|| AppError::NotFound(format!("Invoice {} not found", invoice_id)))?;

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

        let remaining = inv.total_amount - total_paid;
        let status = if remaining <= zero { "paid" } else { "partial" };
        let mut active = inv.into_active_model();
        active.status = Set(status.to_string());
        if status == "paid" {
            active.paid_at = Set(Some(now));
        }
        active.payment_method = Set(Some(payment_method.to_string()));
        active.updated_at = Set(now);
        active
            .update(txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to update invoice: {}", e)))?;

        Ok(Some(status.to_string()))
    }

    /// Credit a customer wallet and record the wallet transaction.
    /// Creates the wallet if it does not exist. Returns (new_balance, wallet_id).
    async fn credit_wallet(
        txn: &sea_orm::DatabaseTransaction,
        customer_id: i64,
        amount: sea_orm::prelude::Decimal,
        tx_type: &str,
        reference_id: i64,
        description: Option<&str>,
    ) -> Result<(sea_orm::prelude::Decimal, i64), AppError> {
        use crate::modules::referral::domain::entities::{customer_wallet, wallet_transaction};
        use sea_orm::IntoActiveModel;

        let now = chrono::Utc::now();
        let zero = sea_orm::prelude::Decimal::ZERO;

        let wallet = match customer_wallet::Entity::find()
            .filter(customer_wallet::Column::CustomerId.eq(customer_id))
            .one(txn)
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
            .insert(txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create wallet: {}", e)))?,
        };

        let old_balance = wallet.balance;
        let old_earned = wallet.total_earned;
        let mut wallet_active = wallet.clone().into_active_model();
        wallet_active.balance = Set(old_balance + amount);
        wallet_active.total_earned = Set(old_earned + amount);
        wallet_active.updated_at = Set(now);
        wallet_active
            .update(txn)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to credit wallet: {}", e)))?;

        wallet_transaction::ActiveModel {
            wallet_id: Set(wallet.id),
            transaction_type: Set(tx_type.to_string()),
            amount: Set(amount),
            reference_id: Set(Some(reference_id)),
            reference_type: Set(Some("payment".to_string())),
            description: Set(description.map(|s| s.to_string())),
            created_at: Set(now),
            ..Default::default()
        }
        .insert(txn)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to record wallet transaction: {}", e)))?;

        Ok((old_balance + amount, wallet.id))
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
