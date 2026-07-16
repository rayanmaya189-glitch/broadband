use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::{debug, info, warn};
use uuid::Uuid;

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

        let link_id = Uuid::new_v4().to_string();
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

    /// Record a manual payment (cash, bank transfer, etc.)
    pub async fn record_manual_payment(
        db: &DatabaseConnection,
        invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        amount: sea_orm::prelude::Decimal,
        payment_method: String,
        reference_number: Option<String>,
        notes: Option<String>,
        recorded_by: i64,
    ) -> Result<payment_link::Model, AppError> {
        let link_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let metadata = serde_json::json!({
            "type": "manual",
            "payment_method": payment_method,
            "reference_number": reference_number,
            "notes": notes,
            "recorded_by": recorded_by,
        });

        let model = payment_link::ActiveModel {
            link_id: Set(link_id.clone()),
            invoice_id: Set(invoice_id),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            amount: Set(amount),
            currency: Set("INR".to_string()),
            gateway_id: Set("manual".to_string()),
            gateway_order_id: Set(None),
            payment_url: Set(None),
            status: Set("completed".to_string()),
            idempotency_key: Set(Uuid::new_v4().to_string()),
            metadata: Set(Some(metadata)),
            expires_at: Set(None),
            paid_at: Set(Some(now)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let link = model.insert(db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to record manual payment: {}", e))
        })?;

        info!(link_id = %link_id, invoice_id = invoice_id, amount = %amount, method = %payment_method, "Manual payment recorded");
        Ok(link)
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

