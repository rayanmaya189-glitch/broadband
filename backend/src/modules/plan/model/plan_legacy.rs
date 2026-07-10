//! Legacy Plan model with `FromRow` for backward compatibility.
//! Used by modules that still reference the old Plan struct via sqlx.
//! Will be removed once all modules are converted to SeaORM.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;

/// Legacy row type mapping to the `plans` table — kept for sqlx-based callers.
#[derive(Debug, Clone, FromRow)]
pub struct PlanLegacy {
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
