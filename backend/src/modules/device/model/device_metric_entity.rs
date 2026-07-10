//! SeaORM entity for the `device_metrics` table.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "device_metrics")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub device_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub metric_name: String,
    pub metric_value: f64,
    pub unit: Option<String>,
    pub recorded_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
