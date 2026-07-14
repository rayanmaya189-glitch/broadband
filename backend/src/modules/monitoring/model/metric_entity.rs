use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "monitoring_metrics")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub metric_name: String,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub metric_type: String,
    pub value: f64,
    #[sea_orm(column_type = "Json", nullable)]
    pub tags: Option<serde_json::Value>,
    pub recorded_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
