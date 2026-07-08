use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateReferralProgramRequest {
    pub name: String,
    pub referrer_reward_type: String,
    pub referrer_reward_value: rust_decimal::Decimal,
    pub referee_reward_type: String,
    pub referee_reward_value: rust_decimal::Decimal,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReferralQuery { pub page: Option<i64>, pub per_page: Option<i64> }
