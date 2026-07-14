use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "monitoring_alert_rules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub service_name: String,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub metric_name: String,
    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub operator: String,
    pub threshold: f64,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub severity: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
