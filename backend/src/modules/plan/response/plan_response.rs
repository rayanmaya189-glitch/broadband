use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PlanResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub speed_down_mbps: i32,
    pub speed_up_mbps: i32,
    pub data_cap_gb: Option<i32>,
    pub price_monthly: Decimal,
    pub price_quarterly: Option<Decimal>,
    pub price_half_yearly: Option<Decimal>,
    pub price_yearly: Option<Decimal>,
    pub gst_percent: Decimal,
    pub is_active: bool,
    pub is_promotional: bool,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type PlanDetailResponse = PlanResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── Speed Profile ─────────────────────────────────────────

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct SpeedProfileResponse {
    pub id: i64,
    pub plan_id: i64,
    pub name: String,
    pub download_limit_kbps: i32,
    pub upload_limit_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: i32,
    pub priority_queue: i32,
    pub qos_marking: Option<String>,
    pub fq_codel_enabled: bool,
    pub device_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Plan Pricing ──────────────────────────────────────────

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PlanPricingResponse {
    pub id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
    pub price: Decimal,
    pub savings_amount: Option<Decimal>,
    pub savings_percent: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

// ── Plan Clone ────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanCloneResponse {
    pub original_plan_id: i64,
    pub new_plan_id: i64,
    pub new_plan_code: String,
    pub message: String,
}
