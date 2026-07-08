use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
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

#[derive(Debug, Serialize)]
pub struct MessageResponse { pub message: String }
