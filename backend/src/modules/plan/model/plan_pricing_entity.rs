//! SeaORM entity for the `plan_pricing` table.

use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "plan_pricing")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
    #[sea_orm(column_precision = 12, column_scale = 2)]
    pub price: Decimal,
    #[sea_orm(column_precision = 12, column_scale = 2, nullable)]
    pub savings_amount: Option<Decimal>,
    #[sea_orm(column_precision = 5, column_scale = 2, nullable)]
    pub savings_percent: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
