use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ip_pools")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    pub dns_primary: Option<String>,
    pub dns_secondary: Option<String>,
    pub vlan_id: Option<i64>,
    pub pool_type: String,
    pub allocated_count: i32,
    pub total_count: i32,
    pub status: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
