use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
#[derive(ToSchema)]
pub struct PlanResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub speed_down_mbps: i32,
    pub speed_up_mbps: i32,
    pub data_cap_gb: Option<i32>,
    pub price_monthly: rust_decimal::Decimal,
    pub price_quarterly: Option<rust_decimal::Decimal>,
    pub price_half_yearly: Option<rust_decimal::Decimal>,
    pub price_yearly: Option<rust_decimal::Decimal>,
    pub gst_percent: rust_decimal::Decimal,
    pub is_active: bool,
    pub is_promotional: bool,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type PlanDetailResponse = PlanResponse;

#[derive(Debug, Serialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
