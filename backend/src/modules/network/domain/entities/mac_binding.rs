use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mac_bindings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub mac_address: String,
    pub assigned_ip: String,
    pub vlan_id: Option<i64>,
    pub bound_at: chrono::DateTime<chrono::Utc>,
    pub bound_by: Option<i64>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
