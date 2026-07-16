use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "payment", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub link_id: String,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: sea_orm::prelude::Decimal,
    pub currency: String,
    pub gateway_id: String,
    pub gateway_order_id: Option<String>,
    pub payment_url: Option<String>,
    pub status: String, // pending, processing, completed, failed, expired
    pub idempotency_key: String,
    pub metadata: Option<serde_json::Value>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
