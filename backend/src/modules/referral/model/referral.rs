use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct CustomerWallet {
    pub id: i64,
    pub customer_id: i64,
    pub balance: Decimal,
    pub total_earned: Decimal,
    pub total_spent: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct WalletTransaction {
    pub id: i64,
    pub wallet_id: i64,
    pub transaction_type: String,
    pub amount: Decimal,
    pub balance_after: Decimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub description: Option<String>,
    pub performed_by: Option<i64>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
