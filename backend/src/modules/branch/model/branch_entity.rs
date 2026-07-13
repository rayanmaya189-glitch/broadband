use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "branches")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub code: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gstin: Option<String>,
    pub is_active: bool,
    pub timezone: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::branch_working_hour_entity::Entity")]
    WorkingHours,

    #[sea_orm(has_many = "super::user_branch_entity::Entity")]
    UserBranches,
}

impl Related<super::branch_working_hour_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkingHours.def()
    }
}

impl Related<super::user_branch_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBranches.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
