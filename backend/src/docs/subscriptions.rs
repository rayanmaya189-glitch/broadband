/// OpenAPI schemas and stub handlers for Subscription endpoints.
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubscriptionRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpgradeSubscriptionRequest {
    pub new_plan_id: i64,
    pub new_billing_period_months: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DowngradeSubscriptionRequest {
    pub new_plan_id: i64,
    pub new_billing_period_months: Option<i32>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct SubscriptionListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub status: Option<String>,
    pub customer_id: Option<i64>,
}

// ── Stub handler functions ───────────────────────────────────────────

/// List all subscriptions with optional filters
#[utoipa::path(
    get,
    path = "/api/v1/subscriptions",
    tag = "Subscriptions",
    params(SubscriptionListParams),
    responses(
        (status = 200, description = "List of subscriptions"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_subscriptions() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new subscription for a customer
#[utoipa::path(
    post,
    path = "/api/v1/subscriptions",
    tag = "Subscriptions",
    request_body = CreateSubscriptionRequest,
    responses(
        (status = 201, description = "Subscription created", body = SubscriptionResponse),
        (status = 400, description = "Invalid plan or customer"),
        (status = 409, description = "Customer already has an active subscription")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_subscription() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Cancel an active subscription
#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/cancel",
    tag = "Subscriptions",
    params(("id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription cancelled"),
        (status = 409, description = "Cannot cancel — subscription not in cancellable state")
    ),
    security(("bearer_auth" = []))
)]
pub async fn cancel_subscription() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Suspend an active subscription
#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/suspend",
    tag = "Subscriptions",
    params(("id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription suspended"),
        (status = 400, description = "Only active subscriptions can be suspended")
    ),
    security(("bearer_auth" = []))
)]
pub async fn suspend_subscription() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Reactivate a suspended subscription
#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/reactivate",
    tag = "Subscriptions",
    params(("id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription reactivated"),
        (status = 400, description = "Only suspended subscriptions can be reactivated")
    ),
    security(("bearer_auth" = []))
)]
pub async fn reactivate_subscription() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Upgrade a subscription to a higher-tier plan
#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/upgrade",
    tag = "Subscriptions",
    params(("id" = i64, Path, description = "Subscription ID")),
    request_body = UpgradeSubscriptionRequest,
    responses(
        (status = 200, description = "Subscription upgraded"),
        (status = 400, description = "New plan must be higher tier")
    ),
    security(("bearer_auth" = []))
)]
pub async fn upgrade_subscription() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Downgrade a subscription to a lower-tier plan
#[utoipa::path(
    post,
    path = "/api/v1/subscriptions/{id}/downgrade",
    tag = "Subscriptions",
    params(("id" = i64, Path, description = "Subscription ID")),
    request_body = DowngradeSubscriptionRequest,
    responses(
        (status = 200, description = "Subscription downgraded"),
        (status = 400, description = "New plan must be lower tier")
    ),
    security(("bearer_auth" = []))
)]
pub async fn downgrade_subscription() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
