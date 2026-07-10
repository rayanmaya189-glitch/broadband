//! SeaORM entity for the `plans` table.

use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "plans")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(50))", unique)]
    pub code: String,
    pub description: Option<String>,
    pub speed_down_mbps: i32,
    pub speed_up_mbps: i32,
    pub data_cap_gb: Option<i32>,
    #[sea_orm(column_precision = 12, column_scale = 2)]
    pub price_monthly: Decimal,
    #[sea_orm(column_precision = 12, column_scale = 2, nullable)]
    pub price_quarterly: Option<Decimal>,
    #[sea_orm(column_precision = 12, column_scale = 2, nullable)]
    pub price_half_yearly: Option<Decimal>,
    #[sea_orm(column_precision = 12, column_scale = 2, nullable)]
    pub price_yearly: Option<Decimal>,
    #[sea_orm(column_precision = 5, column_scale = 2)]
    pub gst_percent: Decimal,
    pub is_active: bool,
    pub is_promotional: bool,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub category: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
