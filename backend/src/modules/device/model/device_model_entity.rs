//! SeaORM entity for the `device_models` table.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "device_models")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub vendor: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub model: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub device_type: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub management_protocol: String,
    pub default_port: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
