use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::subscription::application::services::SubscriptionService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub id: i64,
    pub customer_id: i64,
    pub plan_id: i64,
    pub status: String,
    pub billing_period_months: i32,
    pub start_date: String,
    pub end_date: Option<String>,
    pub next_billing_date: Option<String>,
    pub auto_renew: bool,
    pub review_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpgradeSubscriptionRequest {
    pub new_plan_id: i64,
    pub new_billing_period_months: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct DowngradeSubscriptionRequest {
    pub new_plan_id: i64,
    pub new_billing_period_months: Option<i32>,
}

fn to_response(s: crate::modules::subscription::domain::entities::subscription::Model) -> SubscriptionResponse {
    SubscriptionResponse {
        id: s.id,
        customer_id: s.customer_id,
        plan_id: s.plan_id,
        status: s.status,
        billing_period_months: s.billing_period_months,
        start_date: s.start_date.to_string(),
        end_date: s.end_date.map(|d| d.to_string()),
        next_billing_date: s.next_billing_date.map(|d| d.to_string()),
        auto_renew: s.auto_renew,
        review_status: s.review_status,
    }
}

/// GET /api/v1/subscriptions
pub async fn list_subscriptions(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (items, total) =
        SubscriptionService::list_subscriptions(&state.db, bid, p.page(), p.limit()).await?;
    let resp: Vec<SubscriptionResponse> = items.into_iter().map(to_response).collect();
    Ok(Json(
        serde_json::json!({ "items": resp, "total": total, "page": p.page() }),
    ))
}

/// POST /api/v1/subscriptions
pub async fn create_subscription(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateSubscriptionRequest>,
) -> Result<(StatusCode, Json<SubscriptionResponse>), AppError> {
    require_permission(&user, "subscription.create").map_err(|e| AppError::Forbidden(e.1))?;
    let sub = SubscriptionService::create_subscription(
        &state.db,
        req.customer_id,
        req.branch_id,
        req.plan_id,
        req.billing_period_months,
    )
    .await?;

    // Publish event to outbox
    let payload = serde_json::json!({
        "subscription_id": sub.id,
        "customer_id": sub.customer_id,
        "plan_id": sub.plan_id,
        "branch_id": sub.branch_id,
        "status": sub.status,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "subscription.created",
        "subscription",
        sub.id,
        payload,
        None,
        None,
        Some(sub.branch_id),
    ).await {
        tracing::error!(subscription_id = sub.id, error = %e, "Failed to publish subscription.created event");
    }

    Ok((StatusCode::CREATED, Json(to_response(sub))))
}

/// POST /api/v1/subscriptions/:id/cancel
pub async fn cancel_subscription(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "subscription.cancel").map_err(|e| AppError::Forbidden(e.1))?;
    SubscriptionService::cancel_subscription(&state.db, id, "").await?;

    let payload = serde_json::json!({ "subscription_id": id, "action": "cancelled" });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "subscription.cancelled", "subscription", id, payload, None, None, None,
    ).await {
        tracing::error!(subscription_id = id, error = %e, "Failed to publish subscription.cancelled event");
    }

    Ok(StatusCode::OK)
}

/// POST /api/v1/subscriptions/:id/suspend
pub async fn suspend_subscription(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "subscription.suspend").map_err(|e| AppError::Forbidden(e.1))?;
    SubscriptionService::suspend_subscription(&state.db, id, "").await?;

    let payload = serde_json::json!({ "subscription_id": id, "action": "suspended" });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "subscription.suspended", "subscription", id, payload, None, None, None,
    ).await {
        tracing::error!(subscription_id = id, error = %e, "Failed to publish subscription.suspended event");
    }

    Ok(StatusCode::OK)
}

/// POST /api/v1/subscriptions/:id/reactivate
pub async fn reactivate_subscription(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    require_permission(&user, "subscription.reactivate").map_err(|e| AppError::Forbidden(e.1))?;
    let sub = SubscriptionService::reactivate_subscription(&state.db, id).await?;

    let payload = serde_json::json!({
        "subscription_id": sub.id,
        "customer_id": sub.customer_id,
        "action": "reactivated",
        "status": sub.status,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "subscription.reactivated", "subscription", sub.id, payload, None, None, None,
    ).await {
        tracing::error!(subscription_id = sub.id, error = %e, "Failed to publish subscription.reactivated event");
    }

    Ok(Json(to_response(sub)))
}

/// POST /api/v1/subscriptions/:id/upgrade
pub async fn upgrade_subscription(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpgradeSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    require_permission(&user, "subscription.upgrade").map_err(|e| AppError::Forbidden(e.1))?;
    let sub = SubscriptionService::upgrade_subscription(
        &state.db, id, req.new_plan_id, req.new_billing_period_months,
    ).await?;

    let payload = serde_json::json!({
        "subscription_id": sub.id,
        "customer_id": sub.customer_id,
        "action": "upgraded",
        "new_plan_id": sub.plan_id,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "subscription.upgraded", "subscription", sub.id, payload, None, None, None,
    ).await {
        tracing::error!(subscription_id = sub.id, error = %e, "Failed to publish subscription.upgraded event");
    }

    Ok(Json(to_response(sub)))
}

/// POST /api/v1/subscriptions/:id/downgrade
pub async fn downgrade_subscription(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<DowngradeSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    require_permission(&user, "subscription.downgrade").map_err(|e| AppError::Forbidden(e.1))?;
    let sub = SubscriptionService::downgrade_subscription(
        &state.db, id, req.new_plan_id, req.new_billing_period_months,
    ).await?;

    let payload = serde_json::json!({
        "subscription_id": sub.id,
        "customer_id": sub.customer_id,
        "action": "downgrade_pending",
        "new_plan_id": sub.plan_id,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "subscription.downgrade.pending", "subscription", sub.id, payload, None, None, None,
    ).await {
        tracing::error!(subscription_id = sub.id, error = %e, "Failed to publish subscription.downgrade.pending event");
    }

    Ok(Json(to_response(sub)))
}
