use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gateway_configs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub gateway_id: String, // razorpay, payu, instamojo, ccavenue
    pub name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub credentials: serde_json::Value, // encrypted key_id, key_secret
    pub webhook_secret: Option<String>,
    pub fee_percentage: sea_orm::prelude::Decimal,
    pub fee_fixed: sea_orm::prelude::Decimal,
    pub gst_on_fee: sea_orm::prelude::Decimal,
    pub supported_methods: serde_json::Value, // ["upi", "card", "netbanking"]
    pub currency: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
