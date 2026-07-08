use axum::extract::{Json, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;
use crate::modules::payment_gateway::service::payment_gateway_service::PaymentGatewayService;

pub async fn list_gateways(State(state): State<SharedState>) -> Result<Json<Vec<GatewayConfigResponse>>, AppError> {
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.list_gateways().await?))
}

pub async fn create_gateway(State(state): State<SharedState>, Json(req): Json<CreateGatewayConfigRequest>) -> Result<Json<GatewayConfigResponse>, AppError> {
    req.validate()?;
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.create_gateway(req).await?))
}

pub async fn create_payment_link(State(state): State<SharedState>, Json(req): Json<CreatePaymentLinkRequest>) -> Result<Json<PaymentLinkResponse>, AppError> {
    req.validate()?;
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.create_payment_link(req).await?))
}
