use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "customer_wallets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    pub balance: sea_orm::prelude::Decimal,
    pub total_earned: sea_orm::prelude::Decimal,
    pub total_used: sea_orm::prelude::Decimal,
    pub currency: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
