use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "traffic_policies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    pub priority: i32,
    #[sea_orm(column_type = "Json")]
    pub criteria: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub action: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
