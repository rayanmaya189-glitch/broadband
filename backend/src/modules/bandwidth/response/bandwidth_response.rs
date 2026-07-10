use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
pub struct BandwidthApplicationResponse {
    pub id: i64,
    pub profile_id: i64,
    pub subscription_id: i64,
    pub device_id: i64,
    pub status: String,
    pub applied_at: Option<DateTime<Utc>>,
    pub failed_reason: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BandwidthUsageResponse {
    pub subscription_id: i64,
    pub total_download_bytes: i64,
    pub total_upload_bytes: i64,
    pub records: Vec<BandwidthUsageRecord>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BandwidthUsageRecord {
    pub id: i64,
    pub download_bytes: i64,
    pub upload_bytes: i64,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
