use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "addresses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    pub address_type: String,
    pub line1: String,
    pub line2: Option<String>,
    pub area: Option<String>,
    pub city: String,
    pub state: String,
    pub pincode: String,
    pub country: String,
    pub latitude: Option<sea_orm::prelude::Decimal>,
    pub longitude: Option<sea_orm::prelude::Decimal>,
    pub landmark: Option<String>,
    pub is_primary: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
