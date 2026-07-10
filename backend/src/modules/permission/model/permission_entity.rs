//! SeaORM entity for the `permissions` table.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub method: String,
    #[sea_orm(column_type = "String(StringLen::N(500))")]
    pub api_url: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub guard: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub module: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
