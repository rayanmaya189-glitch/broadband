use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

impl PlanResponse {
    pub fn from_model(m: crate::modules::plan::model::plan_entity::Model) -> Self {
        Self {
            id: m.id, name: m.name, code: m.code, description: m.description,
            speed_down_mbps: m.speed_down_mbps, speed_up_mbps: m.speed_up_mbps,
            data_cap_gb: m.data_cap_gb, price_monthly: m.price_monthly,
            price_quarterly: m.price_quarterly, price_half_yearly: m.price_half_yearly,
            price_yearly: m.price_yearly, gst_percent: m.gst_percent,
            is_active: m.is_active, is_promotional: m.is_promotional, category: m.category,
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }
    }
}

impl SpeedProfileResponse {
    pub fn from_model(m: crate::modules::plan::model::speed_profile_entity::Model) -> Self {
        Self {
            id: m.id, plan_id: m.plan_id, name: m.name,
            download_limit_kbps: m.download_limit_kbps, upload_limit_kbps: m.upload_limit_kbps,
            burst_download_kbps: m.burst_download_kbps, burst_upload_kbps: m.burst_upload_kbps,
            burst_duration_seconds: m.burst_duration_seconds, priority_queue: m.priority_queue,
            qos_marking: m.qos_marking, fq_codel_enabled: m.fq_codel_enabled,
            device_type: m.device_type,
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }
    }
}

impl PlanPricingResponse {
    pub fn from_model(m: crate::modules::plan::model::plan_pricing_entity::Model) -> Self {
        Self {
            id: m.id, plan_id: m.plan_id, billing_period_months: m.billing_period_months,
            price: m.price, savings_amount: m.savings_amount, savings_percent: m.savings_percent,
            is_active: m.is_active, created_at: m.created_at.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── Speed Profile ─────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
