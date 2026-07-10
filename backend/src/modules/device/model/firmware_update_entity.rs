//! SeaORM entity for the `firmware_updates` table.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "firmware_updates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub device_id: i64,
    pub from_version: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub to_version: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub initiated_by: Option<i64>,
    pub started_at: Option<DateTimeWithTimeZone>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub failure_reason: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
