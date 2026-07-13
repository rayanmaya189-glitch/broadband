use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;
use crate::modules::referral::service::referral_service::ReferralService;

// ── Programs ────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/referrals/programs",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of referral programs", body = Vec<ReferralProgramResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_programs(State(state): State<SharedState>) -> Result<Json<Vec<ReferralProgramResponse>>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.list_programs().await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/referrals/programs",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    request_body = CreateReferralProgramRequest,
    responses(
        (status = 200, description = "Program created", body = ReferralProgramResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_program(State(state): State<SharedState>, Json(req): Json<CreateReferralProgramRequest>) -> Result<Json<ReferralProgramResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.create_program(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/referrals/programs/{id}",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Program ID")),
    request_body = UpdateReferralProgramRequest,
    responses(
        (status = 200, description = "Program updated", body = ReferralProgramResponse),
        (status = 404, description = "Program not found")
    )
)]
pub async fn update_program(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateReferralProgramRequest>) -> Result<Json<ReferralProgramResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.update_program(id, req).await?))
}

// ── Tracking ────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/referrals/tracking",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    request_body = ShareReferralRequest,
    responses(
        (status = 200, description = "Referral shared", body = ReferralTrackingResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn share_referral(State(state): State<SharedState>, user: UserContext, Json(req): Json<ShareReferralRequest>) -> Result<Json<ReferralTrackingResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.share_referral(user.user_id, req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/referrals/tracking",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of referrals"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_tracking(State(state): State<SharedState>, Query(query): Query<TrackingQuery>) -> Result<Json<ReferralTrackingListResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.list_tracking(query).await?))
}

// ── Stats ───────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/referrals/stats/{referrer_id}",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(("referrer_id" = i64, Path, description = "Referrer customer ID")),
    responses(
        (status = 200, description = "Referral statistics", body = ReferralStatsResponse),
        (status = 404, description = "Referrer not found")
    )
)]
pub async fn get_stats(State(state): State<SharedState>, Path(referrer_id): Path<i64>) -> Result<Json<ReferralStatsResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.get_stats(referrer_id).await?))
}

// ── Wallet ─────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/referrals/wallet/{customer_id}",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(("customer_id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "Wallet details", body = WalletResponse),
        (status = 404, description = "Wallet not found")
    )
)]
pub async fn get_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<WalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.get_wallet(customer_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/referrals/wallet/{customer_id}",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(("customer_id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "Wallet retrieved or created", body = WalletResponse),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn get_or_create_wallet(State(state): State<SharedState>, Path(customer_id): Path<i64>) -> Result<Json<WalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.get_or_create_wallet(customer_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/referrals/wallet/{customer_id}/credit",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(("customer_id" = i64, Path, description = "Customer ID")),
    request_body = WalletCreditRequest,
    responses(
        (status = 200, description = "Wallet credited", body = WalletTransactionResponse),
        (status = 404, description = "Customer not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn credit_wallet(State(state): State<SharedState>, user: UserContext, Path(customer_id): Path<i64>, Json(req): Json<WalletCreditRequest>) -> Result<Json<WalletTransactionResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.credit_wallet(customer_id, req, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/referrals/wallet/{customer_id}/debit",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(("customer_id" = i64, Path, description = "Customer ID")),
    request_body = WalletDebitRequest,
    responses(
        (status = 200, description = "Wallet debited", body = WalletTransactionResponse),
        (status = 404, description = "Customer not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn debit_wallet(State(state): State<SharedState>, user: UserContext, Path(customer_id): Path<i64>, Json(req): Json<WalletDebitRequest>) -> Result<Json<WalletTransactionResponse>, AppError> {
    req.validate()?;
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.debit_wallet(customer_id, req, user.user_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/referrals/wallet/{customer_id}/transactions",
    tag = "Referrals",
    security(("bearer_auth" = [])),
    params(("customer_id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "List of wallet transactions"),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn list_wallet_transactions(State(state): State<SharedState>, Path(customer_id): Path<i64>, Query(query): Query<TrackingQuery>) -> Result<Json<WalletTransactionListResponse>, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let svc = ReferralService::new(&state.db, &state.redis);
    Ok(Json(svc.list_wallet_transactions(customer_id, page, per_page).await?))
}
