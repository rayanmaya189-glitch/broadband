//! SeaORM-based repository for the PaymentGateway domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::payment_gateway::model::payment_gateway_entity::{self, Model as GatewayConfigModel};
use crate::modules::payment_gateway::model::payment_transaction_entity::{self, Model as PaymentTransactionModel};
use crate::modules::payment_gateway::model::payment_link_entity::{self, Model as PaymentLinkModel};
use crate::modules::payment_gateway::model::webhook_log_entity::{self, Model as WebhookLogModel};

pub struct PaymentGatewayRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PaymentGatewayRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    // ── Gateway Config ────────────────────────────────────

    pub async fn list_gateways(&self) -> Result<Vec<GatewayConfigModel>, AppError> {
        Ok(payment_gateway_entity::Entity::find().order_by_desc(payment_gateway_entity::Column::IsPrimary).all(self.db).await?)
    }

    pub async fn create_gateway(&self, gateway_id: &str, name: &str, is_primary: bool) -> Result<GatewayConfigModel, AppError> {
        let now = chrono::Utc::now();
        let active = payment_gateway_entity::ActiveModel {
            gateway_id: Set(gateway_id.to_owned()),
            name: Set(name.to_owned()),
            is_primary: Set(is_primary),
            is_active: Set(true),
            currency: Set("INR".to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_gateway(&self, id: i64, name: Option<&str>, is_primary: Option<bool>, is_active: Option<bool>) -> Result<GatewayConfigModel, AppError> {
        let existing = payment_gateway_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Gateway not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = is_primary { active.is_primary = Set(v); }
        if let Some(v) = is_active { active.is_active = Set(v); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    // ── Payment Transactions ──────────────────────────────

    pub async fn create_transaction(&self, gateway_id: &str, invoice_id: Option<i64>, customer_id: Option<i64>, amount: rust_decimal::Decimal, payment_method: &str, idempotency_key: Option<&str>) -> Result<PaymentTransactionModel, AppError> {
        let now = chrono::Utc::now();
        let active = payment_transaction_entity::ActiveModel {
            gateway_id: Set(gateway_id.to_owned()),
            invoice_id: Set(invoice_id),
            customer_id: Set(customer_id),
            amount: Set(amount),
            currency: Set("INR".to_owned()),
            payment_method: Set(payment_method.to_owned()),
            idempotency_key: Set(idempotency_key.map(|s| s.to_owned())),
            status: Set("pending".to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn find_by_idempotency(&self, key: &str) -> Result<Option<PaymentTransactionModel>, AppError> {
        Ok(payment_transaction_entity::Entity::find()
            .filter(payment_transaction_entity::Column::IdempotencyKey.eq(key))
            .one(self.db).await?)
    }

    pub async fn update_transaction_status(&self, id: i64, status: &str, gateway_txn_id: Option<&str>, failure_reason: Option<&str>) -> Result<PaymentTransactionModel, AppError> {
        let existing = payment_transaction_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        if let Some(v) = gateway_txn_id { active.gateway_transaction_id = Set(Some(v.to_owned())); }
        if let Some(v) = failure_reason { active.failure_reason = Set(Some(v.to_owned())); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn get_transaction(&self, id: i64) -> Result<Option<PaymentTransactionModel>, AppError> {
        Ok(payment_transaction_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn list_transactions(&self, gateway_id: Option<&str>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<PaymentTransactionModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = payment_transaction_entity::Entity::find();
        if let Some(gid) = gateway_id { select = select.filter(payment_transaction_entity::Column::GatewayId.eq(gid)); }
        if let Some(s) = status { select = select.filter(payment_transaction_entity::Column::Status.eq(s)); }
        let total = select.clone().count(self.db).await?;
        let txns = select.order_by_desc(payment_transaction_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((txns, total as i64))
    }

    // ── Payment Links ─────────────────────────────────────

    pub async fn create_payment_link(&self, transaction_id: i64, payment_url: &str, expires_at: chrono::DateTime<chrono::Utc>) -> Result<PaymentLinkModel, AppError> {
        let now = chrono::Utc::now();
        let active = payment_link_entity::ActiveModel {
            transaction_id: Set(transaction_id),
            payment_url: Set(payment_url.to_owned()),
            expires_at: Set(expires_at.into()),
            is_used: Set(false),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    // ── Webhook Logs ──────────────────────────────────────

    pub async fn log_webhook(&self, gateway_id: &str, event_type: &str, payload: serde_json::Value, processed: bool, error_message: Option<&str>) -> Result<WebhookLogModel, AppError> {
        let now = chrono::Utc::now();
        let active = webhook_log_entity::ActiveModel {
            gateway_id: Set(gateway_id.to_owned()),
            event_type: Set(event_type.to_owned()),
            payload: Set(payload),
            processed: Set(processed),
            error_message: Set(error_message.map(|s| s.to_owned())),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
}
