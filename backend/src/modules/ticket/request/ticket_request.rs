use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTicketRequest {
    pub branch_id: i64,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    pub category: String,
    pub subcategory: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: String,
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[serde(default = "default_source")]
    pub source: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTicketRequest {
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub resolution_notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignTicketRequest {
    pub assigned_to: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EscalateTicketRequest {
    pub escalated_to: i64,
    pub reason: String,
    pub new_priority: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResolveTicketRequest {
    #[validate(length(min = 1))]
    pub resolution_notes: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CloseTicketRequest {
    pub closure_notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReopenTicketRequest {
    #[validate(length(min = 1))]
    pub reason: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddCommentRequest {
    #[validate(length(min = 1))]
    pub comment: String,
    pub is_internal: Option<bool>,
    pub attachments: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TicketFeedbackRequest {
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_feedback: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TicketQuery {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub category: Option<String>,
    pub assigned_to: Option<i64>,
    pub branch_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

fn default_priority() -> String {
    "medium".to_string()
}

fn default_source() -> String {
    "customer".to_string()
}
