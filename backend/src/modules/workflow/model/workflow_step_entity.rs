use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "workflow_steps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub definition_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub step_type: String,
    pub step_order: i32,
    pub required_role: Option<String>,
    #[sea_orm(column_type = "Json", nullable)]
    pub config: Option<serde_json::Value>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
