use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::subscription::application::services::SubscriptionService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::shared::primitives::PaginationParams;

#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub id: i64,
    pub customer_id: i64,
    pub plan_id: i64,
    pub status: String,
    pub billing_period_months: i32,
    pub start_date: String,
    pub next_billing_date: Option<String>,
    pub auto_renew: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
}

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
    let resp: Vec<SubscriptionResponse> = items
        .into_iter()
        .map(|s| SubscriptionResponse {
            id: s.id,
            customer_id: s.customer_id,
            plan_id: s.plan_id,
            status: s.status,
            billing_period_months: s.billing_period_months,
            start_date: s.start_date.to_string(),
            next_billing_date: s.next_billing_date.map(|d| d.to_string()),
            auto_renew: s.auto_renew,
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": resp, "total": total, "page": p.page()}),
    ))
}

pub async fn create_subscription(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<CreateSubscriptionRequest>,
) -> Result<(StatusCode, Json<SubscriptionResponse>), AppError> {
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

    Ok((
        StatusCode::CREATED,
        Json(SubscriptionResponse {
            id: sub.id,
            customer_id: sub.customer_id,
            plan_id: sub.plan_id,
            status: sub.status,
            billing_period_months: sub.billing_period_months,
            start_date: sub.start_date.to_string(),
            next_billing_date: sub.next_billing_date.map(|d| d.to_string()),
            auto_renew: sub.auto_renew,
        }),
    ))
}

pub async fn cancel_subscription(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    SubscriptionService::cancel_subscription(&state.db, id, "").await?;

    // Publish event to outbox
    let payload = serde_json::json!({
        "subscription_id": id,
        "action": "cancelled",
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "subscription.cancelled",
        "subscription",
        id,
        payload,
        None,
        None,
        None,
    ).await {
        tracing::error!(subscription_id = id, error = %e, "Failed to publish subscription.cancelled event");
    }

    Ok(StatusCode::OK)
}

pub async fn suspend_subscription(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    SubscriptionService::suspend_subscription(&state.db, id, "").await?;

    // Publish event to outbox
    let payload = serde_json::json!({
        "subscription_id": id,
        "action": "suspended",
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "subscription.suspended",
        "subscription",
        id,
        payload,
        None,
        None,
        None,
    ).await {
        tracing::error!(subscription_id = id, error = %e, "Failed to publish subscription.suspended event");
    }

    Ok(StatusCode::OK)
}
