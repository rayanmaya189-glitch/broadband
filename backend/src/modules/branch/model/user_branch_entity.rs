use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_branches")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub branch_id: i64,
    pub is_primary: bool,
    pub created_at: DateTimeWithTimeZone,
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
