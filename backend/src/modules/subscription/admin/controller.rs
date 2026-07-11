use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;
use crate::modules::subscription::service::subscription_service::SubscriptionService;

/// List all subscriptions (admin: full access).
pub async fn list_subscriptions(
    State(state): State<SharedState>,
    Query(query): Query<ListSubscriptionsQuery>,
) -> Result<Json<PaginatedResponse<SubscriptionResponse>>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.list_subscriptions(&query).await?))
}

/// Get subscription by ID (admin: any subscription).
pub async fn get_subscription(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.get_subscription(id).await?))
}

/// Create a new subscription (admin).
pub async fn create_subscription(
    State(state): State<SharedState>,
    Json(req): Json<CreateSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.create_subscription(&req).await?))
}

/// Suspend a subscription (admin).
pub async fn suspend_subscription(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<SubscriptionActionRequest>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.suspend_subscription(id, req.reason.as_deref()).await?))
}

/// Reactivate a suspended subscription (admin).
pub async fn reactivate_subscription(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.reactivate_subscription(id).await?))
}

/// Cancel a subscription (admin).
pub async fn cancel_subscription(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<SubscriptionActionRequest>,
) -> Result<Json<crate::modules::subscription::response::subscription_response::MessageResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.cancel_subscription(id, req.reason.as_deref()).await?))
}

/// Upgrade subscription plan (admin).
pub async fn upgrade_subscription(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpgradeDowngradeRequest>,
) -> Result<Json<UpgradeDowngradeResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.upgrade_subscription(id, &req).await?))
}

/// Downgrade subscription plan (admin).
pub async fn downgrade_subscription(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpgradeDowngradeRequest>,
) -> Result<Json<UpgradeDowngradeResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    Ok(Json(svc.downgrade_subscription(id, &req).await?))
}

/// View subscription change history (admin).
pub async fn get_subscription_history(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<SubscriptionHistoryEntry>>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    let query = SubscriptionHistoryQuery { pagination: Default::default() };
    Ok(Json(svc.get_history(id, &query).await?))
}
