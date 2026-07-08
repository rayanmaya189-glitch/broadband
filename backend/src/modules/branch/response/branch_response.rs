use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
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

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
