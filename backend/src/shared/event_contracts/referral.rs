use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralCreatedV1 {
    pub referral_id: i64,
    pub referrer_id: i64,
    pub referee_phone: String,
    pub program_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralRewardedV1 {
    pub referral_id: i64,
    pub referrer_reward: Decimal,
    pub referee_reward: Decimal,
}
