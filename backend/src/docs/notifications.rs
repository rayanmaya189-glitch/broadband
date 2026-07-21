/// OpenAPI schemas and stub handlers for Notifications endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct NotificationTemplateResponse {
    /// Template ID
    pub id: i64,
    /// Template name
    pub name: String,
    /// Channel (sms, email, push)
    pub channel: String,
    /// Whether template is active
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTemplateRequest {
    /// Template name
    pub name: String,
    /// Channel (sms, email, push)
    pub channel: String,
    /// Body template with placeholders
    pub body_template: String,
    /// Subject template (for email)
    #[serde(default)]
    pub subject_template: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTemplateRequest {
    /// Updated subject template
    #[serde(default)]
    pub subject: Option<String>,
    /// Updated body template
    #[serde(default)]
    pub body: Option<String>,
    /// Updated channel
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendNotificationRequest {
    /// Channel (sms, email, push)
    pub channel: String,
    /// Recipient type (customer, lead, user)
    pub recipient_type: String,
    /// Recipient entity ID
    pub recipient_id: i64,
    /// Recipient address (email/phone)
    pub recipient_address: String,
    /// Subject (for email)
    #[serde(default)]
    pub subject: Option<String>,
    /// Notification body
    pub body: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NotificationResponse {
    /// Notification ID
    pub id: i64,
    /// Channel used
    pub channel: String,
    /// Delivery status
    pub status: String,
    /// Recipient address
    pub recipient_address: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NotificationChannelResponse {
    /// Channel ID
    pub id: i64,
    /// Channel name
    pub name: String,
    /// Channel type (sms, email, push)
    pub channel_type: String,
    /// Whether channel is active
    pub is_active: bool,
    /// Channel configuration
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateChannelRequest {
    /// Whether channel is active
    #[serde(default)]
    pub is_active: Option<bool>,
    /// Updated channel config
    #[serde(default)]
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeliveryHistoryResponse {
    /// History entry ID
    pub id: i64,
    /// Associated notification ID
    pub notification_id: i64,
    /// Channel used
    pub channel: String,
    /// Delivery status
    pub status: String,
    /// Number of delivery attempts
    pub attempts: i32,
    /// Last error message if any
    pub last_error: Option<String>,
    /// Timestamp when sent
    pub sent_at: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeliveryHistoryParams {
    /// Page number
    #[serde(default)]
    pub page: Option<u64>,
    /// Items per page
    #[serde(default)]
    pub limit: Option<u64>,
    /// Filter by status
    #[serde(default)]
    pub status: Option<String>,
    /// Filter by channel
    #[serde(default)]
    pub channel: Option<String>,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all notification templates
#[utoipa::path(
    get,
    path = "/api/v1/notifications/templates",
    tag = "Notifications",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "Paginated list of templates"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_templates() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new notification template
#[utoipa::path(
    post,
    path = "/api/v1/notifications/templates",
    tag = "Notifications",
    request_body = CreateTemplateRequest,
    responses(
        (status = 201, description = "Template created", body = NotificationTemplateResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_template() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update an existing notification template
#[utoipa::path(
    put,
    path = "/api/v1/notifications/templates/{id}",
    tag = "Notifications",
    params(("id" = i64, Path, description = "Template ID")),
    request_body = UpdateTemplateRequest,
    responses(
        (status = 200, description = "Template updated", body = NotificationTemplateResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Template not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_template() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a notification template
#[utoipa::path(
    delete,
    path = "/api/v1/notifications/templates/{id}",
    tag = "Notifications",
    params(("id" = i64, Path, description = "Template ID")),
    responses(
        (status = 204, description = "Template deleted"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Template not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_template() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Send a notification
#[utoipa::path(
    post,
    path = "/api/v1/notifications/send",
    tag = "Notifications",
    request_body = SendNotificationRequest,
    responses(
        (status = 201, description = "Notification sent", body = NotificationResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn send_notification() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all sent notifications
#[utoipa::path(
    get,
    path = "/api/v1/notifications/list",
    tag = "Notifications",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "Paginated list of notifications"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_notifications() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Retry a specific failed notification
#[utoipa::path(
    post,
    path = "/api/v1/notifications/{id}/retry",
    tag = "Notifications",
    params(("id" = i64, Path, description = "Notification ID")),
    responses(
        (status = 200, description = "Notification retried", body = NotificationResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Notification not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn retry_notification() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Retry all failed notifications
#[utoipa::path(
    post,
    path = "/api/v1/notifications/retry",
    tag = "Notifications",
    responses(
        (status = 200, description = "Retry summary"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn retry_all_notifications() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all notification channels
#[utoipa::path(
    get,
    path = "/api/v1/notifications/channels",
    tag = "Notifications",
    responses(
        (status = 200, description = "List of notification channels"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_channels() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a notification channel configuration
#[utoipa::path(
    put,
    path = "/api/v1/notifications/channels/{id}",
    tag = "Notifications",
    params(("id" = i64, Path, description = "Channel ID")),
    request_body = UpdateChannelRequest,
    responses(
        (status = 200, description = "Channel updated", body = NotificationChannelResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Channel not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_channel() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List delivery history with filters
#[utoipa::path(
    get,
    path = "/api/v1/notifications/delivery-history",
    tag = "Notifications",
    params(
        ("page" = Option<u64>, Query, description = "Page number"),
        ("limit" = Option<u64>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("channel" = Option<String>, Query, description = "Filter by channel"),
    ),
    responses(
        (status = 200, description = "Paginated delivery history"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_delivery_history() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
