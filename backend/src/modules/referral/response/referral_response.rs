use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReferralProgramResponse {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub referrer_reward_type: String,
    pub referrer_reward_value: Decimal,
    pub referee_reward_type: String,
    pub referee_reward_value: Decimal,
    pub max_referrals_per_customer: Option<i32>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReferralTrackingResponse {
    pub id: i64,
    pub program_id: i64,
    pub referrer_id: i64,
    pub referee_id: Option<i64>,
    pub referral_code: String,
    pub referee_phone: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReferralTrackingListResponse {
    pub referrals: Vec<ReferralTrackingResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReferralStatsResponse {
    pub total_referrals: i64,
    pub activated: i64,
    pub rewarded: i64,
    pub by_status: Vec<StatusCount>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── Wallet ─────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CustomerWalletResponse {
    pub id: i64,
    pub customer_id: i64,
    pub balance: Decimal,
    pub total_earned: Decimal,
    pub total_spent: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Alias for backward compatibility
pub type WalletResponse = CustomerWalletResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WalletTransactionResponse {
    pub id: i64,
    pub wallet_id: i64,
    pub transaction_type: String,
    pub amount: Decimal,
    pub balance_after: Decimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub description: Option<String>,
    pub performed_by: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WalletTransactionListResponse {
    pub transactions: Vec<WalletTransactionResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
