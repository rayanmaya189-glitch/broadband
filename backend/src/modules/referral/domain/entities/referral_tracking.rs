use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "referral_tracking")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub program_id: i64,
    pub referrer_id: i64,
    pub referee_id: Option<i64>,
    pub referral_code: String,
    pub referee_phone: String,
    pub status: String,
    pub referrer_reward_status: Option<String>,
    pub referrer_reward_amount: Option<sea_orm::prelude::Decimal>,
    pub referee_reward_status: Option<String>,
    pub referee_reward_amount: Option<sea_orm::prelude::Decimal>,
    pub shared_at: chrono::DateTime<chrono::Utc>,
    pub registered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub activated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub rewarded_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
