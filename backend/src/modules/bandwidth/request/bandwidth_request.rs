use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBandwidthProfileRequest {
    pub name: String,
    pub description: Option<String>,
    pub plan_id: Option<i64>,
    pub download_kbps: i32,
    pub upload_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: Option<i32>,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBandwidthProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub download_kbps: Option<i32>,
    pub upload_kbps: Option<i32>,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub is_active: Option<bool>,
}
