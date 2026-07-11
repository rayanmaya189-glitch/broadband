use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InstallationOrderResponse {
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: Option<i64>,
    pub assigned_technician_id: Option<i64>,
    pub status: String,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_time_slot: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub installation_type: String,
    pub equipment_issued: Option<serde_json::Value>,
    pub fiber_drop_length_meters: Option<i32>,
    pub onu_power_dbm: Option<f64>,
    pub notes: Option<String>,
    pub photos: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Alias for backward compatibility
pub type InstallationResponse = InstallationOrderResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InstallationListResponse {
    pub installations: Vec<InstallationResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
