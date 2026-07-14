use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "automation_executions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub rule_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub trigger_data: Option<serde_json::Value>,
    #[sea_orm(column_type = "Json", nullable)]
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub started_at: DateTimeWithTimeZone,
    pub completed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
