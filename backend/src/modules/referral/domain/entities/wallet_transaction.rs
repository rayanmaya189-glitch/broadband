use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "referral", table_name = "wallet_transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub wallet_id: i64,
    pub transaction_type: String,
    pub amount: sea_orm::prelude::Decimal,
    pub reference_id: Option<i64>,
    pub reference_type: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
