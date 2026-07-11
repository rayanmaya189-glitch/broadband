use axum::extract::{Json, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::referral::response::referral_response::*;
use crate::modules::referral::service::referral_service::ReferralService;

/// Get my referral stats.
pub async fn get_my_stats(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.get_stats(user.user_id).await?))
}

/// Get my referral wallet.
pub async fn get_my_wallet(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<CustomerWalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.get_wallet(user.user_id).await?))
}

/// Get my referral transactions.
pub async fn get_my_transactions(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<Vec<WalletTransactionResponse>>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    let (txns, _total) = svc.list_wallet_transactions(user.user_id).await?;
    Ok(Json(txns))
}

/// Get my referral code and links.
pub async fn get_my_referral_info(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<CustomerWalletResponse>, AppError> {
    let svc = ReferralService::new(&state.db_seaorm);
    Ok(Json(svc.get_wallet(user.user_id).await?))
}
