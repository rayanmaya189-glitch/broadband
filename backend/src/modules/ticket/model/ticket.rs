use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Ticket {
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
}

#[derive(Debug, Clone, FromRow)]
pub struct TicketComment {
    pub id: i64,
    pub ticket_id: i64,
    pub user_id: Option<i64>,
    pub is_customer: bool,
    pub comment: String,
    pub is_internal: bool,
    pub attachments: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TicketEscalation {
    pub id: i64,
    pub ticket_id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub from_priority: Option<String>,
    pub to_priority: Option<String>,
    pub reason: String,
    pub escalated_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TicketStatusHistory {
    pub id: i64,
    pub ticket_id: i64,
    pub old_status: Option<String>,
    pub new_status: String,
    pub changed_by: i64,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
