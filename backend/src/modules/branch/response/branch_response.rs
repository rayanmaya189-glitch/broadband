use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WorkingHourResponse {
    pub id: i64,
    pub branch_id: i64,
    pub day_of_week: i32,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

// ── User-Branch Assignment ─────────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

// ── from_model implementations ─────────────────────────────

impl BranchResponse {
    pub fn from_model(m: crate::modules::branch::model::branch_entity::Model) -> Self {
        Self {
            id: m.id, name: m.name, code: m.code, address: m.address, city: m.city,
            state: m.state, pincode: m.pincode, phone: m.phone, email: m.email,
            is_active: m.is_active, timezone: m.timezone,
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }
    }
}

impl WorkingHourResponse {
    pub fn from_model(m: crate::modules::branch::model::branch_working_hour_entity::Model) -> Self {
        Self {
            id: m.id, branch_id: m.branch_id, day_of_week: m.day_of_week,
            open_time: m.open_time.map(|t| t.format("%H:%M").to_string()).unwrap_or_default(),
            close_time: m.close_time.map(|t| t.format("%H:%M").to_string()).unwrap_or_default(),
            is_closed: m.is_closed,
        }
    }
}

impl BranchUserResponse {
    pub fn from_model(m: crate::modules::branch::model::user_branch_entity::Model) -> Self {
        Self {
            user_id: m.user_id, user_name: None, user_email: None,
            is_primary: m.is_primary, assigned_at: m.created_at.into(),
        }
    }
}
