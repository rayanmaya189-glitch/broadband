use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "installation", table_name = "installation_equipment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub installation_order_id: i64,
    pub equipment_type: String,
    pub model_name: Option<String>,
    pub serial_number: Option<String>,
    pub quantity: i32,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
