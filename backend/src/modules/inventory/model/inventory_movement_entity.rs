use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_movements")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub item_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub movement_type: String,
    pub from_branch_id: Option<i64>,
    pub to_branch_id: Option<i64>,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub performed_by: i64,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
