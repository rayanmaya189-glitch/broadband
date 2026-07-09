use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct BranchResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type BranchDetailResponse = BranchResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── Working Hours ──────────────────────────────────────────

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct WorkingHourResponse {
    pub id: i64,
    pub branch_id: i64,
    pub day_of_week: i32,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

// ── User-Branch Assignment ─────────────────────────────────

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct BranchUserResponse {
    pub user_id: i64,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub is_primary: bool,
    pub assigned_at: DateTime<Utc>,
}

// ── Branch Statistics ──────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct BranchStatsResponse {
    pub branch_id: i64,
    pub total_customers: i64,
    pub active_customers: i64,
    pub total_subscriptions: i64,
    pub active_subscriptions: i64,
}
