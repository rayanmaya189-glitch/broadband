use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub payment_number: String,
    #[sea_orm(index)]
    pub invoice_id: Option<i64>,
    #[sea_orm(index)]
    pub customer_id: i64,
    #[sea_orm(index)]
    pub branch_id: i64,
    pub amount: sea_orm::prelude::Decimal,
    pub currency: String,
    pub payment_method: String,
    pub payment_gateway: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub status: String,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
