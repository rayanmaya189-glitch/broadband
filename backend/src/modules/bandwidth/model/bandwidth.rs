use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct BandwidthProfile {
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
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct BandwidthApplication {
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

#[derive(Debug, Clone, FromRow)]
pub struct BandwidthUsage {
    pub id: i64,
    pub subscription_id: i64,
    pub download_bytes: i64,
    pub upload_bytes: i64,
    pub recorded_at: DateTime<Utc>,
}
