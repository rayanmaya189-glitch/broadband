use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "refunds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub refund_number: String,
    pub payment_id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: sea_orm::prelude::Decimal,
    pub reason: String,
    pub requested_by: Option<i64>,
    pub approved_by: Option<i64>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub review_notes: Option<String>,
    pub status: String,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
