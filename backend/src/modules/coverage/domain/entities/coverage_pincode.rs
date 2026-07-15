use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "coverage_pincode_map")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub coverage_area_id: i64,
    pub pincode: String,
    pub city: String,
    pub district: Option<String>,
    pub state: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
