use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type RoleDetailResponse = RoleResponse;

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
