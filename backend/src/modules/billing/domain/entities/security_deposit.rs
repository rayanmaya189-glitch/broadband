use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "security_deposits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(index)]
    pub customer_id: i64,
    #[sea_orm(index)]
    pub branch_id: i64,
    pub amount: sea_orm::prelude::Decimal,
    pub deposit_type: String,
    pub status: String,
    pub collected_at: chrono::DateTime<chrono::Utc>,
    pub refunded_at: Option<chrono::DateTime<chrono::Utc>>,
    pub refund_amount: Option<sea_orm::prelude::Decimal>,
    pub refund_reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
