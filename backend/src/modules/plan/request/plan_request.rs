use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePlanRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: String,
    #[validate(length(min = 2, max = 50, message = "Code must be 2-50 characters"))]
    pub code: String,
    pub description: Option<String>,
    pub speed_down_mbps: i32,
    pub speed_up_mbps: i32,
    pub data_cap_gb: Option<i32>,
    pub price_monthly: rust_decimal::Decimal,
    pub price_quarterly: Option<rust_decimal::Decimal>,
    pub price_half_yearly: Option<rust_decimal::Decimal>,
    pub price_yearly: Option<rust_decimal::Decimal>,
    pub gst_percent: Option<rust_decimal::Decimal>,
    pub is_promotional: Option<bool>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub speed_down_mbps: Option<i32>,
    pub speed_up_mbps: Option<i32>,
    pub data_cap_gb: Option<i32>,
    pub price_monthly: Option<rust_decimal::Decimal>,
    pub price_quarterly: Option<rust_decimal::Decimal>,
    pub price_half_yearly: Option<rust_decimal::Decimal>,
    pub price_yearly: Option<rust_decimal::Decimal>,
    pub gst_percent: Option<rust_decimal::Decimal>,
    pub is_active: Option<bool>,
    pub is_promotional: Option<bool>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListPlansQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub is_active: Option<bool>,
    pub category: Option<String>,
}

// ── Speed Profile ──────────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSpeedProfileRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub download_limit_kbps: i32,
    pub upload_limit_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: Option<i32>,
    pub priority_queue: Option<i32>,
    pub qos_marking: Option<String>,
    pub htb_parent_queue: Option<String>,
    pub fq_codel_enabled: Option<bool>,
    pub device_type: Option<String>,
}

// ── Publish / Unpublish ───────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct PublishPlanRequest {
    pub reason: Option<String>,
}

// ── Plan Pricing ──────────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePlanPricingRequest {
    pub billing_period_months: i32,
    pub price: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListPlanPricingQuery {
    pub plan_id: Option<i64>,
}
