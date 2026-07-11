use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub channel: String,
    pub subject_template: Option<String>,
    pub body_template: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub channel: Option<String>,
    pub subject_template: Option<String>,
    pub body_template: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendNotificationRequest {
    pub channel: String,
    pub recipient_id: i64,
    pub address: String,
    pub subject: Option<String>,
    pub body: String,
}

// Type alias for backward compatibility
pub type CreateNotificationTemplateRequest = CreateTemplateRequest;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpsertChannelRequest {
    pub channel: String,
    pub provider: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NotificationQuery {
    pub channel: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct HistoryQuery {
    pub notification_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
