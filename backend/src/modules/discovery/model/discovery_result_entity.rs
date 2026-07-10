use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "discovery_results")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub scan_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub discovered_ip: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub reviewed_by: Option<i64>,
    pub reviewed_at: Option<DateTimeWithTimeZone>,
    pub rejection_reason: Option<String>,
    pub discovered_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
