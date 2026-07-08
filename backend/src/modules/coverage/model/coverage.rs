use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CoverageArea {
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub area_type: String,
    pub pincodes: Option<Vec<String>>,
    pub is_active: bool,
    pub max_customers: Option<i32>,
    pub current_customers: Option<i32>,
    pub fiber_available: bool,
    pub estimated_installation_days: Option<i32>,
    pub created_at: DateTime<Utc>,
}
