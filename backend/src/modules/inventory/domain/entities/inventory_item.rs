use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "inventory", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub branch_id: i64,
    pub item_type: String,
    pub device_model_id: Option<i64>,
    pub serial_number: Option<String>,
    pub barcode: Option<String>,
    pub purchase_date: Option<chrono::NaiveDate>,
    pub purchase_price: Option<sea_orm::prelude::Decimal>,
    pub warranty_expiry: Option<chrono::NaiveDate>,
    pub supplier: Option<String>,
    pub status: String,
    pub assigned_to: Option<i64>,
    pub assigned_to_branch_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
