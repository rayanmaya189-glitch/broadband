use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "workflow_step_instances")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub instance_id: i64,
    pub step_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub assigned_to: Option<i64>,
    pub decided_by: Option<i64>,
    pub decision: Option<String>,
    pub comments: Option<String>,
    pub started_at: DateTimeWithTimeZone,
    pub completed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
