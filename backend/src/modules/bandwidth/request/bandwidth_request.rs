use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBandwidthProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub download_kbps: Option<i32>,
    pub upload_kbps: Option<i32>,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ApplyProfileRequest {
    pub subscription_id: i64,
    pub device_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ApplicationQuery {
    pub profile_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UsageQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
