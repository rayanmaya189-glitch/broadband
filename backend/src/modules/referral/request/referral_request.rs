use utoipa::ToSchema;
use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateReferralProgramRequest {
    pub name: String,
    pub referrer_reward_type: String,
    pub referrer_reward_value: rust_decimal::Decimal,
    pub referee_reward_type: String,
    pub referee_reward_value: rust_decimal::Decimal,
    pub max_referrals_per_customer: Option<i32>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateReferralProgramRequest {
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ShareReferralRequest {
    pub program_id: i64,
    pub referee_phone: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TrackingQuery {
    pub referrer_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ── Wallet ─────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WalletCreditRequest {
    pub amount: Decimal,
    pub transaction_type: String,  // referral_reward, refund, manual, adjustment
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WalletDebitRequest {
    pub amount: Decimal,
    pub transaction_type: String,  // invoice_payment, manual, expiry
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub description: Option<String>,
}
