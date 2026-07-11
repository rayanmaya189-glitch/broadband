use axum::extract::{Json, Path, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::common::utils::helpers::PaginationParams;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;
use crate::modules::subscription::service::subscription_service::SubscriptionService;

/// Get current customer's subscriptions.
pub async fn get_my_subscriptions(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<Vec<SubscriptionResponse>>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    let query = ListSubscriptionsQuery {
        pagination: PaginationParams { page: 1, limit: 100, sort_by: None, sort_order: None, search: None },
        status: None,
        customer_id: Some(user.user_id),
        branch_id: None,
    };
    let paginated = svc.list_subscriptions(&query).await?;
    Ok(Json(paginated.data))
}

/// Get specific subscription (customer: only own).
pub async fn get_subscription(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    let sub = svc.get_subscription(id).await?;
    if sub.customer_id != user.user_id {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(sub))
}

/// Upgrade own subscription.
pub async fn upgrade_subscription(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpgradeDowngradeRequest>,
) -> Result<Json<UpgradeDowngradeResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    let sub = svc.get_subscription(id).await?;
    if sub.customer_id != user.user_id {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(svc.upgrade_subscription(id, &req).await?))
}

/// Downgrade own subscription.
pub async fn downgrade_subscription(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpgradeDowngradeRequest>,
) -> Result<Json<UpgradeDowngradeResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    let sub = svc.get_subscription(id).await?;
    if sub.customer_id != user.user_id {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(svc.downgrade_subscription(id, &req).await?))
}

/// View own subscription history.
pub async fn get_subscription_history(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<SubscriptionHistoryEntry>>, AppError> {
    let svc = SubscriptionService::new(&state.db_seaorm);
    let sub = svc.get_subscription(id).await?;
    if sub.customer_id != user.user_id {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    let query = SubscriptionHistoryQuery { pagination: Default::default() };
    Ok(Json(svc.get_history(id, &query).await?))
}
