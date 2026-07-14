use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "crm_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(7))", nullable)]
    pub color: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub category: String,
    pub usage_count: i64,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
