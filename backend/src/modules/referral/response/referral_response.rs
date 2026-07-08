use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct ReferralProgramResponse { pub id: i64, pub name: String, pub status: String, pub referrer_reward_value: Decimal, pub referee_reward_value: Decimal, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct ReferralTrackingResponse { pub id: i64, pub referral_code: String, pub status: String, pub referee_phone: String, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse { pub message: String }
