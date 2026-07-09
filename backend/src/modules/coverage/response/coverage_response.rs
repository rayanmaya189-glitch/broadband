use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CoverageAreaResponse {
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub area_type: String,
    pub is_active: bool,
    pub fiber_available: bool,
    pub estimated_installation_days: Option<i32>,
    pub current_customers: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AvailabilityCheckResponse {
    pub available: bool,
    pub area_name: Option<String>,
    pub estimated_days: Option<i32>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CoveragePincodeResponse {
    pub id: i64,
    pub pincode: String,
    pub city: String,
    pub district: Option<String>,
    pub state: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CoverageStatsResponse {
    pub total_areas: i64,
    pub active_areas: i64,
    pub total_pincodes: i64,
    pub total_customers: i64,
    pub fiber_available_areas: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
