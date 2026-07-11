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

pub async fn get_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<CustomerWalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.get_wallet(customer_id).await?))
}
