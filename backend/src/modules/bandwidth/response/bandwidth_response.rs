use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BandwidthProfileResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub plan_id: Option<i64>,
    pub download_kbps: i32,
    pub upload_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: Option<i32>,
    pub priority: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BandwidthProfileListResponse {
    pub profiles: Vec<BandwidthProfileResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
