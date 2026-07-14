use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "scheduled_tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub task_type: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub config: Option<serde_json::Value>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub schedule_type: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub schedule_value: String,
    pub next_run_at: Option<DateTimeWithTimeZone>,
    pub last_run_at: Option<DateTimeWithTimeZone>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
