use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InstallationResponse {
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
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub customer_name: Option<String>,
    pub technician_name: Option<String>,
}

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
