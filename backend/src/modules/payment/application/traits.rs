use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type PaymentLinkModel = crate::modules::payment::domain::entities::payment_link::Model;
pub type GatewayConfigModel = crate::modules::payment::domain::entities::gateway_config::Model;
pub type WebhookLogModel = crate::modules::payment::domain::entities::webhook_log::Model;

#[async_trait]
pub trait PaymentServiceTrait: Send + Sync {
    async fn create_payment_link(
        &self,
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
    ) -> Result<PaymentLinkModel, AppError>;

    async fn process_successful_payment(
        &self,
        db: &DatabaseConnection,
        gateway_id: &str,
        gateway_transaction_id: &str,
        amount: sea_orm::prelude::Decimal,
        payment_method: Option<String>,
    ) -> Result<PaymentLinkModel, AppError>;

    async fn process_failed_payment(
        &self,
        db: &DatabaseConnection,
        gateway_id: &str,
        gateway_transaction_id: &str,
        error_reason: Option<String>,
    ) -> Result<PaymentLinkModel, AppError>;

    async fn record_manual_payment(
        &self,
        db: &DatabaseConnection,
        invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        amount: sea_orm::prelude::Decimal,
        payment_method: String,
        reference_number: Option<String>,
        notes: Option<String>,
        recorded_by: i64,
    ) -> Result<PaymentLinkModel, AppError>;

    async fn get_gateway_config(
        &self,
        db: &DatabaseConnection,
        gateway_id: &str,
    ) -> Result<GatewayConfigModel, AppError>;

    async fn list_gateways(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<GatewayConfigModel>, AppError>;

    async fn log_webhook(
        &self,
        db: &DatabaseConnection,
        gateway_id: &str,
        event_id: &str,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Result<bool, AppError>;

    async fn mark_webhook_processed(
        &self,
        db: &DatabaseConnection,
        gateway_id: &str,
        event_id: &str,
    ) -> Result<(), AppError>;
}
