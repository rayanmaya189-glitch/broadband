use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Row type mapping to the `plans` table.
#[derive(Debug, Clone, FromRow)]
pub struct Plan {
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

/// Speed profile for bandwidth management (Mikrotik/TikSP integration).
#[derive(Debug, Clone, FromRow)]
pub struct SpeedProfile {
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
    pub htb_parent_queue: Option<String>,
    pub fq_codel_enabled: bool,
    pub device_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Plan pricing for multi-period billing.
#[derive(Debug, Clone, FromRow)]
pub struct PlanPricing {
    pub id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
    pub price: rust_decimal::Decimal,
    pub savings_amount: Option<rust_decimal::Decimal>,
    pub savings_percent: Option<rust_decimal::Decimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
