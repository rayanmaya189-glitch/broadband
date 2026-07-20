use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "discounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    #[sea_orm(column_name = "type")]
    pub discount_type: String,
    pub value: sea_orm::prelude::Decimal,
    pub applicable_plan_ids: Option<serde_json::Value>,
    pub applicable_billing_periods: Option<Vec<i32>>,
    pub max_uses: Option<i32>,
    pub current_uses: Option<i32>,
    pub valid_from: chrono::NaiveDate,
    pub valid_until: chrono::NaiveDate,
    pub is_active: bool,
    pub created_by: Option<i64>,
    pub reviewed_by: Option<i64>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub review_status: Option<String>,
    pub approved_by: Option<i64>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
