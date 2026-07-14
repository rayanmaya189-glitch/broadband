use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "report_schedules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: Option<i64>,
    pub user_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub report_type: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub parameters: Option<serde_json::Value>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub frequency: String,
    pub next_run_at: Option<DateTimeWithTimeZone>,
    pub last_run_at: Option<DateTimeWithTimeZone>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
