use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "monitoring_alerts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub rule_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub service_name: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub severity: String,
    #[sea_orm(column_type = "Text")]
    pub message: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub acknowledged_by: Option<i64>,
    pub acknowledged_at: Option<DateTimeWithTimeZone>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
