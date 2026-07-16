use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "branches", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub slug: String,
    #[sea_orm(unique)]
    pub code: String,
    pub city: String,
    pub state: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub latitude: Option<sea_orm::prelude::Decimal>,
    pub longitude: Option<sea_orm::prelude::Decimal>,
    pub timezone: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
