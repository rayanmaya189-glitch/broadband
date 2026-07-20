use crate::modules::referral::application::services::ReferralService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ==================== Existing endpoints ====================

#[derive(Debug, Serialize)]
pub struct ReferralResponse {
    pub id: i64,
    pub referral_code: String,
    pub status: String,
    pub referee_phone: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReferralRequest {
    pub program_id: i64,
    pub referee_phone: String,
    pub referral_code: String,
}

pub async fn list_referrals(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (refs, total) = ReferralService::list_referrals(&state.db, p.page(), p.limit()).await?;
    let items: Vec<ReferralResponse> = refs
        .into_iter()
        .map(|r| ReferralResponse {
            id: r.id,
            referral_code: r.referral_code,
            status: r.status,
            referee_phone: r.referee_phone,
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

pub async fn create_referral(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateReferralRequest>,
) -> Result<(StatusCode, Json<ReferralResponse>), AppError> {
    require_permission(&user, "referral.create").map_err(|e| AppError::Forbidden(e.1))?;
    let r = ReferralService::create_referral(
        &state.db,
        req.program_id,
        user.user_id,
        req.referee_phone,
        req.referral_code,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "referral.created", "referral", r.id,
        serde_json::json!({"referral_id": r.id, "referral_code": r.referral_code, "status": r.status}), None,
        Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish referral.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(ReferralResponse {
            id: r.id,
            referral_code: r.referral_code,
            status: r.status,
            referee_phone: r.referee_phone,
        }),
    ))
}

#[derive(Debug, Serialize)]
pub struct WalletResponse {
    pub id: i64,
    pub customer_id: i64,
    pub balance: String,
    pub total_earned: String,
}

pub async fn get_wallet(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<WalletResponse>, AppError> {
    let w = ReferralService::get_or_create_wallet(&state.db, user.user_id).await?;
    Ok(Json(WalletResponse {
        id: w.id,
        customer_id: w.customer_id,
        balance: w.balance.to_string(),
        total_earned: w.total_earned.to_string(),
    }))
}

// ==================== Customer-facing endpoints ====================

pub async fn get_my_referral_code(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.view").map_err(|e| AppError::Forbidden(e.1))?;
    let stats = ReferralService::get_referral_stats(&state.db, user.user_id).await?;
    Ok(Json(serde_json::json!({
        "referral_code": stats.referral_code,
        "total_shared": stats.total_shared,
        "total_registered": stats.total_registered,
        "total_active": stats.total_active,
        "total_rewarded": stats.total_rewarded,
    })))
}

pub async fn list_my_referrals(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.view").map_err(|e| AppError::Forbidden(e.1))?;
    let refs = ReferralService::list_my_referrals(&state.db, user.user_id).await?;
    let items: Vec<serde_json::Value> = refs
        .into_iter()
        .map(|r| serde_json::json!({
            "id": r.id,
            "referral_code": r.referral_code,
            "status": r.status,
            "referee_phone": r.referee_phone,
            "shared_at": r.shared_at,
            "registered_at": r.registered_at,
            "activated_at": r.activated_at,
            "rewarded_at": r.rewarded_at,
            "referrer_reward_status": r.referrer_reward_status,
            "referrer_reward_amount": r.referrer_reward_amount,
        }))
        .collect();
    Ok(Json(serde_json::json!({"items": items})))
}

#[derive(Debug, Deserialize)]
pub struct ShareReferralRequest {
    pub referred_phone: String,
    pub channel: String,
}

pub async fn share_referral(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<ShareReferralRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "referral.create").map_err(|e| AppError::Forbidden(e.1))?;
    let referral_code = ReferralService::get_referral_code(&state.db, user.user_id).await?;
    let programs = ReferralService::list_programs(&state.db, 0, 1).await?;
    let program_id = programs
        .0
        .first()
        .map(|p| p.id)
        .unwrap_or(1);
    let r = ReferralService::create_referral(
        &state.db,
        program_id,
        user.user_id,
        req.referred_phone,
        referral_code,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "referral.shared", "referral", r.id,
        serde_json::json!({"referral_id": r.id, "channel": req.channel, "referee_phone": r.referee_phone}),
        None, Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish referral.shared event");
    }
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": r.id,
            "referral_code": r.referral_code,
            "status": r.status,
            "referee_phone": r.referee_phone,
            "channel": req.channel,
        })),
    ))
}

pub async fn get_referral_stats(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.view").map_err(|e| AppError::Forbidden(e.1))?;
    let stats = ReferralService::get_referral_stats(&state.db, user.user_id).await?;
    Ok(Json(serde_json::json!({
        "referral_code": stats.referral_code,
        "total_shared": stats.total_shared,
        "total_registered": stats.total_registered,
        "total_active": stats.total_active,
        "total_rewarded": stats.total_rewarded,
        "total_reward_amount": stats.total_reward_amount,
        "wallet_balance": stats.wallet_balance,
        "wallet_total_earned": stats.wallet_total_earned,
    })))
}

// ==================== Admin-facing endpoints ====================

#[derive(Debug, Deserialize)]
pub struct CreateProgramRequest {
    pub name: String,
    pub reward_type: String,
    pub reward_value: Decimal,
    pub max_referrals_per_user: Option<i32>,
    pub valid_from: chrono::NaiveDate,
    pub valid_until: chrono::NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProgramRequest {
    pub name: Option<String>,
    pub reward_type: Option<String>,
    pub reward_value: Option<Decimal>,
    pub max_referrals_per_user: Option<Option<i32>>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_until: Option<chrono::NaiveDate>,
}

pub async fn list_programs(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.program.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (programs, total) = ReferralService::list_programs(&state.db, p.page(), p.limit()).await?;
    let items: Vec<serde_json::Value> = programs
        .into_iter()
        .map(|p| serde_json::json!({
            "id": p.id,
            "name": p.name,
            "reward_type": p.reward_type,
            "reward_value": p.reward_value,
            "max_referrals_per_user": p.max_referrals_per_user,
            "valid_from": p.valid_from,
            "valid_until": p.valid_until,
            "is_active": p.is_active,
            "created_at": p.created_at,
            "updated_at": p.updated_at,
        }))
        .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
}

pub async fn create_program(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateProgramRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "referral.program.create").map_err(|e| AppError::Forbidden(e.1))?;
    let p = ReferralService::create_program(
        &state.db,
        req.name,
        req.reward_type,
        req.reward_value,
        req.max_referrals_per_user,
        req.valid_from,
        req.valid_until,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": p.id,
            "name": p.name,
            "reward_type": p.reward_type,
            "reward_value": p.reward_value,
            "max_referrals_per_user": p.max_referrals_per_user,
            "valid_from": p.valid_from,
            "valid_until": p.valid_until,
            "is_active": p.is_active,
        })),
    ))
}

pub async fn update_program(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateProgramRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.program.update").map_err(|e| AppError::Forbidden(e.1))?;
    let p = ReferralService::update_program(
        &state.db,
        id,
        req.name,
        req.reward_type,
        req.reward_value,
        req.max_referrals_per_user,
        req.valid_from,
        req.valid_until,
    )
    .await?;
    Ok(Json(serde_json::json!({
        "id": p.id,
        "name": p.name,
        "reward_type": p.reward_type,
        "reward_value": p.reward_value,
        "max_referrals_per_user": p.max_referrals_per_user,
        "valid_from": p.valid_from,
        "valid_until": p.valid_until,
        "is_active": p.is_active,
    })))
}

pub async fn delete_program(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "referral.program.delete").map_err(|e| AppError::Forbidden(e.1))?;
    ReferralService::delete_program(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_analytics(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.view").map_err(|e| AppError::Forbidden(e.1))?;
    let a = ReferralService::get_analytics(&state.db).await?;
    Ok(Json(serde_json::json!({
        "total_referrals": a.total_referrals,
        "total_shared": a.total_shared,
        "total_active": a.total_active,
        "total_rewarded": a.total_rewarded,
        "conversion_rate": a.conversion_rate,
        "total_rewards_paid": a.total_rewards_paid,
    })))
}

pub async fn list_wallets(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "referral.wallet.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (wallets, total) = ReferralService::list_wallets(&state.db, p.page(), p.limit()).await?;
    let items: Vec<serde_json::Value> = wallets
        .into_iter()
        .map(|w| serde_json::json!({
            "id": w.id,
            "customer_id": w.customer_id,
            "balance": w.balance,
            "total_earned": w.total_earned,
            "total_used": w.total_used,
            "currency": w.currency,
            "created_at": w.created_at,
        }))
        .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
}

#[derive(Debug, Deserialize)]
pub struct AdjustWalletRequest {
    pub amount: Decimal,
    pub reason: String,
}

pub async fn adjust_wallet(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AdjustWalletRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "referral.wallet.adjust").map_err(|e| AppError::Forbidden(e.1))?;
    let tx = ReferralService::adjust_wallet(
        &state.db,
        id,
        req.amount,
        req.reason,
        user.user_id,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": tx.id,
            "wallet_id": tx.wallet_id,
            "transaction_type": tx.transaction_type,
            "amount": tx.amount,
            "description": tx.description,
            "created_at": tx.created_at,
        })),
    ))
}
