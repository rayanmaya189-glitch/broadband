use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "deferred_revenue")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(index)]
    pub subscription_id: i64,
    #[sea_orm(index)]
    pub customer_id: i64,
    #[sea_orm(index)]
    pub branch_id: i64,
    pub total_amount: sea_orm::prelude::Decimal,
    pub recognized_amount: sea_orm::prelude::Decimal,
    pub deferred_amount: sea_orm::prelude::Decimal,
    pub recognition_period_start: chrono::NaiveDate,
    pub recognition_period_end: chrono::NaiveDate,
    pub monthly_recognition_amount: sea_orm::prelude::Decimal,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
