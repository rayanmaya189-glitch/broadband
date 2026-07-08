use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ReferralProgram {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub referrer_reward_type: String,
    pub referrer_reward_value: Decimal,
    pub referee_reward_type: String,
    pub referee_reward_value: Decimal,
    pub max_referrals_per_customer: Option<i32>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ReferralTracking {
    pub id: i64,
    pub program_id: i64,
    pub referrer_id: i64,
    pub referee_id: Option<i64>,
    pub referral_code: String,
    pub referee_phone: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
