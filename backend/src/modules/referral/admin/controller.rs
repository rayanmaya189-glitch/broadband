use axum::extract::{Json, Path, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;
use crate::modules::referral::service::referral_service::ReferralService;

/// List all referral programs (admin).
pub async fn list_programs(
    State(state): State<SharedState>,
) -> Result<Json<Vec<ReferralProgramResponse>>, AppError> {
    let svc = ReferralService::new(&state.db);
    Ok(Json(svc.list_programs().await?))
}

/// Create a referral program (admin).
pub async fn create_program(
    State(state): State<SharedState>,
    Json(req): Json<CreateReferralProgramRequest>,
) -> Result<Json<ReferralProgramResponse>, AppError> {
    let svc = ReferralService::new(&state.db);
    Ok(Json(svc.create_program(req).await?))
}

/// Update a referral program (admin).
pub async fn update_program(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateReferralProgramRequest>,
) -> Result<Json<ReferralProgramResponse>, AppError> {
    let svc = ReferralService::new(&state.db);
    Ok(Json(svc.update_program(id, req).await?))
}

/// Get referral stats for a referrer (admin).
pub async fn get_referral_stats(
    State(state): State<SharedState>,
    Path(referrer_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = ReferralService::new(&state.db);
    Ok(Json(svc.get_stats(referrer_id).await?))
}
