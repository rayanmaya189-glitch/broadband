//! SeaORM-based service for the PaymentGateway domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::payment_gateway::repository::payment_gateway_repository::PaymentGatewayRepository;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;

pub struct PaymentGatewayService<'a> {
    repo: PaymentGatewayRepository<'a>,
}

impl<'a> PaymentGatewayService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: PaymentGatewayRepository::new(db) }
    }

    pub async fn list_gateways(&self) -> Result<Vec<GatewayConfigResponse>, AppError> {
        let gateways = self.repo.list_gateways().await?;
        Ok(gateways.into_iter().map(|g| GatewayConfigResponse {
            id: g.id, gateway_id: g.gateway_id, name: g.name, is_primary: g.is_primary,
            is_active: g.is_active, supported_methods: Some(g.supported_methods), currency: Some(g.currency),
            created_at: g.created_at.into(), updated_at: g.updated_at.into(),
        }).collect())
    }

    pub async fn create_gateway(&self, req: CreateGatewayRequest) -> Result<GatewayConfigResponse, AppError> {
        let is_primary = req.is_primary.unwrap_or(false);
        let g = self.repo.create_gateway(&req.gateway_id, &req.name, is_primary).await?;
        Ok(GatewayConfigResponse {
            id: g.id, gateway_id: g.gateway_id, name: g.name, is_primary: g.is_primary,
            is_active: g.is_active, supported_methods: Some(g.supported_methods), currency: Some(g.currency),
            created_at: g.created_at.into(), updated_at: g.updated_at.into(),
        })
    }

    pub async fn list_transactions(&self, gateway_id: Option<&str>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<PaymentTransactionResponse>, i64), AppError> {
        let (txns, total) = self.repo.list_transactions(gateway_id, status, page, per_page).await?;
        let responses = txns.into_iter().map(|t| PaymentTransactionResponse {
            id: t.id, gateway_id: t.gateway_id, invoice_id: t.invoice_id, customer_id: t.customer_id,
            amount: t.amount, currency: t.currency, payment_method: t.payment_method,
            gateway_transaction_id: t.gateway_transaction_id,
            status: t.status, failure_reason: t.failure_reason, created_at: t.created_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn create_transaction(&self, req: CreateTransactionRequest) -> Result<PaymentTransactionResponse, AppError> {
        let t = self.repo.create_transaction(&req.gateway_id, req.invoice_id, Some(req.customer_id), req.amount, &req.payment_method, req.description.as_deref()).await?;
        Ok(PaymentTransactionResponse {
            id: t.id, gateway_id: t.gateway_id, invoice_id: t.invoice_id, customer_id: t.customer_id,
            amount: t.amount, currency: t.currency, payment_method: t.payment_method,
            gateway_transaction_id: t.gateway_transaction_id,
            status: t.status, failure_reason: t.failure_reason, created_at: t.created_at.into(),
        })
    }

    pub async fn update_gateway(&self, id: i64, req: crate::modules::payment_gateway::request::payment_gateway_request::UpdateGatewayRequest) -> Result<GatewayConfigResponse, AppError> {
        let g = self.repo.update_gateway(id, req.name.as_deref(), req.is_primary, req.is_active).await?;
        Ok(GatewayConfigResponse {
            id: g.id, gateway_id: g.gateway_id, name: g.name, is_primary: g.is_primary,
            is_active: g.is_active, supported_methods: Some(g.supported_methods), currency: Some(g.currency),
            created_at: g.created_at.into(), updated_at: g.updated_at.into(),
        })
    }

    pub async fn get_transaction(&self, id: i64) -> Result<PaymentTransactionResponse, AppError> {
        let t = self.repo.get_transaction(id).await?.ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
        Ok(PaymentTransactionResponse {
            id: t.id, gateway_id: t.gateway_id, invoice_id: t.invoice_id, customer_id: t.customer_id,
            amount: t.amount, currency: t.currency, payment_method: t.payment_method,
            gateway_transaction_id: t.gateway_transaction_id,
            status: t.status, failure_reason: t.failure_reason, created_at: t.created_at.into(),
        })
    }

    pub async fn process_webhook(&self, webhook: crate::modules::payment_gateway::request::payment_gateway_request::WebhookPayload) -> Result<crate::modules::payment_gateway::response::payment_gateway_response::WebhookProcessResponse, AppError> {
        self.repo.log_webhook(&webhook.gateway_id, &webhook.event_type, webhook.payload, true, None).await?;
        Ok(crate::modules::payment_gateway::response::payment_gateway_response::WebhookProcessResponse {
            status: "processed".into(), message: "Webhook logged".into(), transaction_id: None,
        })
    }
}
