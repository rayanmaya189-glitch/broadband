use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bandwidth_applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub profile_id: i64,
    pub subscription_id: i64,
    pub device_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub applied_at: Option<DateTimeWithTimeZone>,
    pub failed_reason: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
