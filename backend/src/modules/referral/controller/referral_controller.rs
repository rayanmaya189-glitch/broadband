use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;
use crate::modules::referral::service::referral_service::ReferralService;

// ── Programs ────────────────────────────────────────────────

pub async fn list_programs(State(state): State<SharedState>) -> Result<Json<Vec<ReferralProgramResponse>>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.list_programs().await?))
}

pub async fn create_program(State(state): State<SharedState>, Json(req): Json<CreateReferralProgramRequest>) -> Result<Json<ReferralProgramResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.create_program(req).await?))
}

pub async fn update_program(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateReferralProgramRequest>) -> Result<Json<ReferralProgramResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.update_program(id, req).await?))
}

// ── Tracking ────────────────────────────────────────────────

pub async fn share_referral(State(state): State<SharedState>, user: UserContext, Json(req): Json<ShareReferralRequest>) -> Result<Json<ReferralTrackingResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.share_referral(user.user_id, req).await?))
}

pub async fn list_tracking(State(state): State<SharedState>, Query(query): Query<TrackingQuery>) -> Result<Json<ReferralTrackingListResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.list_tracking(query).await?))
}

// ── Stats ───────────────────────────────────────────────────

pub async fn get_stats(State(state): State<SharedState>, Path(referrer_id): Path<i64>) -> Result<Json<ReferralStatsResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.get_stats(referrer_id).await?))
}

// ── Wallet ─────────────────────────────────────────────────

pub async fn get_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<WalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.get_wallet(customer_id).await?))
}

pub async fn get_or_create_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<WalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.get_or_create_wallet(customer_id).await?))
}

pub async fn credit_wallet(State(state): State<SharedState>, user: UserContext, Path(customer_id): Path<i64>, Json(req): Json<WalletCreditRequest>) -> Result<Json<WalletTransactionResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.credit_wallet(customer_id, req, user.user_id).await?))
}

pub async fn debit_wallet(State(state): State<SharedState>, user: UserContext, Path(customer_id): Path<i64>, Json(req): Json<WalletDebitRequest>) -> Result<Json<WalletTransactionResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.debit_wallet(customer_id, req, user.user_id).await?))
}

pub async fn list_wallet_transactions(State(state): State<SharedState>, Path(customer_id): Path<i64>, Query(query): Query<TrackingQuery>) -> Result<Json<WalletTransactionListResponse>, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.list_wallet_transactions(customer_id, page, per_page).await?))
}
