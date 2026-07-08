use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TicketResponse {
    pub id: i64,
    pub ticket_number: String,
    pub branch_id: i64,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    pub created_by: i64,
    pub assigned_to: Option<i64>,
    pub escalated_to: Option<i64>,
    pub category: String,
    pub subcategory: Option<String>,
    pub priority: String,
    pub status: String,
    pub subject: String,
    pub description: String,
    pub source: String,
    pub resolution_notes: Option<String>,
    pub sla_response_at: Option<DateTime<Utc>>,
    pub sla_resolution_at: Option<DateTime<Utc>>,
    pub first_response_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub reopen_count: i32,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_feedback: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(default)]
    pub creator_name: Option<String>,
    #[sqlx(default)]
    pub assignee_name: Option<String>,
    #[sqlx(default)]
    pub branch_name: Option<String>,
    #[sqlx(default)]
    pub customer_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketListResponse {
    pub tickets: Vec<TicketResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketCommentResponse {
    pub id: i64,
    pub ticket_id: i64,
    pub user_id: Option<i64>,
    pub is_customer: bool,
    pub comment: String,
    pub is_internal: bool,
    pub attachments: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub user_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketDashboardResponse {
    pub total_open: i64,
    pub total_in_progress: i64,
    pub total_resolved_today: i64,
    pub total_overdue: i64,
    pub by_priority: Vec<PriorityCount>,
    pub by_category: Vec<CategoryCount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityCount {
    pub priority: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}
