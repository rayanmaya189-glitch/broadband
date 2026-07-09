use utoipa::ToSchema;
use chrono::NaiveDate;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSubscriptionRequest {
    pub customer_id: i64,
    pub plan_id: i64,
    pub branch_id: i64,
    pub billing_period_months: Option<i32>,
    pub start_date: NaiveDate,
    pub auto_renew: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListSubscriptionsQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub status: Option<String>,
    pub customer_id: Option<i64>,
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubscriptionActionRequest {
    pub reason: Option<String>,
}

// ── Upgrade / Downgrade ─────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpgradeDowngradeRequest {
    pub new_plan_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubscriptionHistoryQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
}
