use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "installation", table_name = "installation_photo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub installation_order_id: i64,
    pub storage_key: String,
    pub storage_bucket: String,
    pub photo_type: String,
    pub uploaded_by: Option<i64>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
