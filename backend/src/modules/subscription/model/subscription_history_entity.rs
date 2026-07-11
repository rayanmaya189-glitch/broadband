use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Subscription history entry — tracks plan changes, status changes, etc.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "subscription_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub subscription_id: i64,
    pub change_type: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::subscription_entity::Entity",
        from = "Column::SubscriptionId",
        to = "super::subscription_entity::Column::Id"
    )]
    Subscription,
}

impl Related<super::subscription_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscription.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
