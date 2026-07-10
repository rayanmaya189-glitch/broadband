//! SeaORM entity for the `network_devices` table.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "network_devices")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    pub device_model_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub serial_number: String,
    #[sea_orm(column_type = "String(StringLen::N(45))")]
    pub management_ip: String,
    pub management_port: Option<i32>,
    pub firmware_version: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub health_score: Option<i32>,
    pub location_city: Option<String>,
    pub location_area: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
