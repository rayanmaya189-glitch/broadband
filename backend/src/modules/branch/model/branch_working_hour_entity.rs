use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "branch_working_hours")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub branch_id: i64,
    pub day_of_week: i32,
    pub open_time: Option<Time>,
    pub close_time: Option<Time>,
    pub is_closed: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::branch_entity::Entity",
        from = "Column::BranchId",
        to = "super::branch_entity::Column::Id"
    )]
    Branch,
}

impl Related<super::branch_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Branch.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
