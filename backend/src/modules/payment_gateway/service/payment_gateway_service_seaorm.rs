//! SeaORM-based service for the PaymentGateway domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::payment_gateway::repository::payment_gateway_repository_seaorm::PaymentGatewayRepositorySeaorm;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;

pub struct PaymentGatewayServiceSeaorm<'a> {
    repo: PaymentGatewayRepositorySeaorm<'a>,
}

impl<'a> PaymentGatewayServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: PaymentGatewayRepositorySeaorm::new(db) }
    }

    pub async fn list_gateways(&self) -> Result<Vec<GatewayConfigResponse>, AppError> {
        let gateways = self.repo.list_gateways().await?;
        Ok(gateways.into_iter().map(|g| GatewayConfigResponse {
            id: g.id, gateway_id: g.gateway_id, name: g.name, is_primary: g.is_primary,
            is_active: g.is_active, supported_methods: g.supported_methods, currency: g.currency,
            created_at: g.created_at.into(), updated_at: g.updated_at.into(),
        }).collect())
    }

    pub async fn create_gateway(&self, req: CreateGatewayRequest) -> Result<GatewayConfigResponse, AppError> {
        let g = self.repo.create_gateway(&req.gateway_id, &req.name, req.is_primary).await?;
        Ok(GatewayConfigResponse {
            id: g.id, gateway_id: g.gateway_id, name: g.name, is_primary: g.is_primary,
            is_active: g.is_active, supported_methods: g.supported_methods, currency: g.currency,
            created_at: g.created_at.into(), updated_at: g.updated_at.into(),
        })
    }

    pub async fn list_transactions(&self, gateway_id: Option<&str>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<PaymentTransactionResponse>, i64), AppError> {
        let (txns, total) = self.repo.list_transactions(gateway_id, status, page, per_page).await?;
        let responses = txns.into_iter().map(|t| PaymentTransactionResponse {
            id: t.id, gateway_id: t.gateway_id, invoice_id: t.invoice_id, customer_id: t.customer_id,
            amount: t.amount, currency: t.currency, payment_method: t.payment_method,
            status: t.status, failure_reason: t.failure_reason, created_at: t.created_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn create_transaction(&self, req: CreateTransactionRequest) -> Result<PaymentTransactionResponse, AppError> {
        let t = self.repo.create_transaction(&req.gateway_id, req.invoice_id, req.customer_id, req.amount, &req.payment_method, req.idempotency_key.as_deref()).await?;
        Ok(PaymentTransactionResponse {
            id: t.id, gateway_id: t.gateway_id, invoice_id: t.invoice_id, customer_id: t.customer_id,
            amount: t.amount, currency: t.currency, payment_method: t.payment_method,
            status: t.status, failure_reason: t.failure_reason, created_at: t.created_at.into(),
        })
    }
}
