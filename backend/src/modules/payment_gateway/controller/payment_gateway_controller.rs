//! SeaORM-based controller for the PaymentGateway domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;
use crate::modules::payment_gateway::service::payment_gateway_service::PaymentGatewayService;

pub async fn list_gateways(State(state): State<SharedState>) -> Result<Json<Vec<GatewayConfigResponse>>, AppError> {
    let svc = PaymentGatewayService::new(&state.db_seaorm);
    Ok(Json(svc.list_gateways().await?))
}

pub async fn create_gateway(State(state): State<SharedState>, Json(req): Json<CreateGatewayRequest>) -> Result<Json<GatewayConfigResponse>, AppError> {
    req.validate()?;
    let svc = PaymentGatewayService::new(&state.db_seaorm);
    Ok(Json(svc.create_gateway(req).await?))
}

pub async fn list_transactions(State(state): State<SharedState>, Query(q): Query<TransactionQuery>) -> Result<Json<Vec<PaymentTransactionResponse>>, AppError> {
    let svc = PaymentGatewayService::new(&state.db_seaorm);
    let (txns, _) = svc.list_transactions(q.gateway_id.as_deref(), q.status.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(txns))
}

pub async fn create_transaction(State(state): State<SharedState>, Json(req): Json<CreateTransactionRequest>) -> Result<Json<PaymentTransactionResponse>, AppError> {
    req.validate()?;
    let svc = PaymentGatewayService::new(&state.db_seaorm);
    Ok(Json(svc.create_transaction(req).await?))
}

pub async fn get_transaction(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<PaymentTransactionResponse>, AppError> {
    let svc = PaymentGatewayService::new(&state.db_seaorm);
    Ok(Json(svc.get_transaction(id).await?))
}

pub async fn update_gateway(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateGatewayRequest>) -> Result<Json<GatewayConfigResponse>, AppError> {
    let svc = PaymentGatewayService::new(&state.db_seaorm);
    Ok(Json(svc.update_gateway(id, req).await?))
}

pub async fn process_webhook(State(state): State<SharedState>, Json(req): Json<WebhookPayload>) -> Result<Json<WebhookProcessResponse>, AppError> {
    let svc = PaymentGatewayService::new(&state.db_seaorm);
    Ok(Json(svc.process_webhook(req).await?))
}
