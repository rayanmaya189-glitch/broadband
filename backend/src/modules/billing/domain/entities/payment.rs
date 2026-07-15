use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub payment_number: String,
    pub invoice_id: i64,
    pub customer_id: i64,
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
