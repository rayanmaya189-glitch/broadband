//! SeaORM entity for the `device_ports` table.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "device_ports")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub device_id: i64,
    pub port_number: i32,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed_mbps: Option<i32>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub connected_device_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
