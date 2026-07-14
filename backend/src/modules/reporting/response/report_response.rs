use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReportResponse {
    pub id: i64,
    pub branch_id: Option<i64>,
    pub user_id: i64,
    pub report_type: String,
    pub name: String,
    pub parameters: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub status: String,
    pub file_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleResponse {
    pub id: i64,
    pub branch_id: Option<i64>,
    pub user_id: i64,
    pub report_type: String,
    pub name: String,
    pub parameters: Option<serde_json::Value>,
    pub frequency: String,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
