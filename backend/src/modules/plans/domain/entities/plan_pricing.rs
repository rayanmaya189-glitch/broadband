use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "plan_pricing")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
    pub price: sea_orm::prelude::Decimal,
    pub savings: Option<sea_orm::prelude::Decimal>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
