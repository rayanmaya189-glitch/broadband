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

#[derive(Debug, Clone, FromRow)]
pub struct CoveragePincode {
    pub id: i64,
    pub coverage_area_id: i64,
    pub pincode: String,
    pub city: String,
    pub district: Option<String>,
    pub state: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CoverageStats {
    pub total_areas: i64,
    pub active_areas: i64,
    pub total_pincodes: i64,
    pub total_customers: i64,
    pub fiber_available_areas: i64,
}
