use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;
use crate::modules::subscription::service::subscription_service::SubscriptionService;

#[utoipa::path(
    get,
    path = "/api/v1/subscriptions",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("customer_id" = Option<i64>, Query, description = "Filter by customer"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of subscriptions"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_subscriptions(State(state): State<SharedState>, Query(query): Query<ListSubscriptionsQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<SubscriptionResponse>>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.list_subscriptions(&query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/subscriptions",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    request_body = CreateSubscriptionRequest,
    responses(
        (status = 200, description = "Subscription created", body = SubscriptionResponse),
        (status = 404, description = "Customer or plan not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_subscription(State(state): State<SharedState>, Json(req): Json<CreateSubscriptionRequest>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.create_subscription(&req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/subscriptions/{id}",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription details", body = SubscriptionResponse),
        (status = 404, description = "Subscription not found")
    )
)]
pub async fn get_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.get_subscription(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/suspend",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    request_body = SubscriptionActionRequest,
    responses(
        (status = 200, description = "Subscription suspended", body = SubscriptionResponse),
        (status = 404, description = "Subscription not found"),
        (status = 422, description = "Invalid action")
    )
)]
pub async fn suspend_subscription(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<SubscriptionActionRequest>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.suspend_subscription(id, req.reason.as_deref()).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/reactivate",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription reactivated", body = SubscriptionResponse),
        (status = 404, description = "Subscription not found")
    )
)]
pub async fn reactivate_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.reactivate_subscription(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/cancel",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    request_body = SubscriptionActionRequest,
    responses(
        (status = 200, description = "Subscription cancelled"),
        (status = 404, description = "Subscription not found")
    )
)]
pub async fn cancel_subscription(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<SubscriptionActionRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.cancel_subscription(id, req.reason.as_deref()).await?))
}

// ── Upgrade / Downgrade ─────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/upgrade",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    request_body = UpgradeDowngradeRequest,
    responses(
        (status = 200, description = "Subscription upgraded", body = UpgradeDowngradeResponse),
        (status = 404, description = "Subscription not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn upgrade_subscription(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpgradeDowngradeRequest>) -> Result<Json<UpgradeDowngradeResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.upgrade_subscription(id, &req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/downgrade",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    request_body = UpgradeDowngradeRequest,
    responses(
        (status = 200, description = "Subscription downgraded", body = UpgradeDowngradeResponse),
        (status = 404, description = "Subscription not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn downgrade_subscription(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpgradeDowngradeRequest>) -> Result<Json<UpgradeDowngradeResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.downgrade_subscription(id, &req).await?))
}

// ── History ─────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/subscriptions/{id}/history",
    tag = "Subscriptions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription history", body = Vec<SubscriptionHistoryEntry>),
        (status = 404, description = "Subscription not found")
    )
)]
pub async fn get_subscription_history(State(state): State<SharedState>, Path(id): Path<i64>, Query(query): Query<SubscriptionHistoryQuery>) -> Result<Json<Vec<SubscriptionHistoryEntry>>, AppError> {
    let svc = SubscriptionService::new(&state.db, &state.redis);
    Ok(Json(svc.get_history(id, &query).await?))
}
