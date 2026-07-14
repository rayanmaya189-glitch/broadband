use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "monitoring_health_checks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub service_name: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub response_time_ms: Option<i32>,
    pub error_message: Option<String>,
    pub checked_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
