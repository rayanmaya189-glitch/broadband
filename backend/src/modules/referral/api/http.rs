use crate::modules::referral::application::services::ReferralService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use axum::extract::State;
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
) -> Result<Json<Vec<ReferralResponse>>, AppError> {
    require_permission(&user, "referral.view").map_err(|e| AppError::Forbidden(e.1))?;
    let refs = ReferralService::list_referrals(&state.db).await?;
    Ok(Json(
        refs.into_iter()
            .map(|r| ReferralResponse {
                id: r.id,
                referral_code: r.referral_code,
                status: r.status,
                referee_phone: r.referee_phone,
            })
            .collect(),
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
