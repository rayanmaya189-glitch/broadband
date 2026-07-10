use utoipa::ToSchema;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionResponse {
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub plan_id: i64,
    pub status: String,
    pub billing_period_months: i32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub next_billing_date: Option<NaiveDate>,
    pub auto_renew: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type SubscriptionDetailResponse = SubscriptionResponse;

impl SubscriptionResponse {
    pub fn from_model(m: crate::modules::subscription::model::subscription_entity::Model) -> Self {
        Self {
            id: m.id, customer_id: m.customer_id, branch_id: m.branch_id, plan_id: m.plan_id,
            status: m.status, billing_period_months: m.billing_period_months,
            start_date: m.start_date, end_date: m.end_date,
            next_billing_date: m.next_billing_date, auto_renew: m.auto_renew,
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── Upgrade / Downgrade ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProRataAdjustment {
    pub old_plan_id: i64,
    pub new_plan_id: i64,
    pub old_plan_price: Decimal,
    pub new_plan_price: Decimal,
    pub old_plan_credit: Decimal,
    pub new_plan_charge: Decimal,
    pub adjustment: Decimal,
    pub remaining_days: i32,
    pub billing_period_days: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpgradeDowngradeResponse {
    pub subscription: SubscriptionResponse,
    pub pro_rata: ProRataAdjustment,
    pub message: String,
}

// ── Subscription History ────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionHistoryEntry {
    pub id: i64,
    pub subscription_id: i64,
    pub action: String,
    pub old_data: Option<serde_json::Value>,
    pub new_data: Option<serde_json::Value>,
    pub performed_by: Option<i64>,
    pub performed_at: DateTime<Utc>,
    pub reason: Option<String>,
}
