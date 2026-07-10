use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "vlans")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    pub vlan_id: i32,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub name: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub vlan_type: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
