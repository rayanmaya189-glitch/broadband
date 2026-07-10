//! SeaORM entity for the `speed_profiles` table.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "speed_profiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub plan_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    pub download_limit_kbps: i32,
    pub upload_limit_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: i32,
    pub priority_queue: i32,
    pub qos_marking: Option<String>,
    pub htb_parent_queue: Option<String>,
    pub fq_codel_enabled: bool,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub device_type: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
