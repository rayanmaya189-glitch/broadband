use crate::modules::referral::application::services::ReferralService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    let items: Vec<ReferralResponse> = refs.into_iter()
            .map(|r| ReferralResponse {
                id: r.id,
                referral_code: r.referral_code,
                status: r.status,
                referee_phone: r.referee_phone,
            })
            .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
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
