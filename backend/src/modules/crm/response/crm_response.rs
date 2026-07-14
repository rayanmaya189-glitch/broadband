use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InteractionResponse {
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub user_id: i64,
    pub interaction_type: String,
    pub subject: String,
    pub body: Option<String>,
    pub channel: String,
    pub duration_seconds: Option<i32>,
    pub sentiment: Option<String>,
    pub follow_up_date: Option<NaiveDate>,
    pub follow_up_done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NoteResponse {
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub priority: String,
    pub is_pinned: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TagResponse {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub category: String,
    pub usage_count: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SegmentResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub criteria: serde_json::Value,
    pub customer_count: i64,
    pub is_dynamic: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
