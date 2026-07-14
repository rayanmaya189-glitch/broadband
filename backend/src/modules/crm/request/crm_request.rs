use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInteractionRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    #[validate(length(min = 1, max = 30))]
    pub interaction_type: String,
    #[validate(length(min = 1))]
    pub subject: String,
    pub body: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub channel: String,
    pub duration_seconds: Option<i32>,
    pub sentiment: Option<String>,
    pub follow_up_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    #[validate(length(min = 1, max = 20))]
    pub priority: String,
    pub is_pinned: Option<bool>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTagRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub color: Option<String>,
    #[validate(length(min = 1, max = 30))]
    pub category: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSegmentRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub criteria: serde_json::Value,
    pub is_dynamic: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InteractionQuery {
    pub customer_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub interaction_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TagQuery {
    pub branch_id: Option<i64>,
    pub category: Option<String>,
}
