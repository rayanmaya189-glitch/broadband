//! SeaORM-based controller for the Referral domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;
use crate::modules::referral::service::referral_service::ReferralService;

pub async fn list_programs(State(state): State<SharedState>) -> Result<Json<Vec<ReferralProgramResponse>>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.list_programs().await?))
}

pub async fn create_program(State(state): State<SharedState>, Json(req): Json<CreateReferralProgramRequest>) -> Result<Json<ReferralProgramResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.create_program(req).await?))
}

pub async fn update_program(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateReferralProgramRequest>) -> Result<Json<ReferralProgramResponse>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.update_program(id, req).await?))
}

pub async fn list_tracking(State(state): State<SharedState>, Query(q): Query<TrackingQuery>) -> Result<Json<Vec<ReferralTrackingResponse>>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    let (tracking, _) = svc.list_tracking(q.referrer_id, q.status.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(tracking))
}

pub async fn share_referral(State(state): State<SharedState>, Json(req): Json<ShareReferralRequest>) -> Result<Json<ReferralTrackingResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.share_referral(req).await?))
}

pub async fn get_stats(State(state): State<SharedState>, Path(referrer_id): Path<i64>) -> Result<Json<serde_json::Value>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.get_stats(referrer_id).await?))
}

pub async fn get_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<CustomerWalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.get_wallet(customer_id).await?))
}

pub async fn get_or_create_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<CustomerWalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.get_or_create_wallet(customer_id).await?))
}

pub async fn credit_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>, Json(req): Json<WalletCreditRequest>) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.credit_wallet(customer_id, req).await?))
}

pub async fn debit_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>, Json(req): Json<WalletDebitRequest>) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.debit_wallet(customer_id, req).await?))
}

pub async fn list_wallet_transactions(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<Vec<WalletTransactionResponse>>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    let (txns, _) = svc.list_wallet_transactions(customer_id).await?;
    Ok(Json(txns))
}
