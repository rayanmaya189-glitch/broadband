use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "discovery", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub scan_id: i64,
    pub discovered_ip: String,
    pub discovered_mac: Option<String>,
    pub sys_name: Option<String>,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
    pub management_protocol: Option<String>,
    pub matched_model_id: Option<i64>,
    pub matched_device_id: Option<i64>,
    pub status: String,
    pub reviewed_by: Option<i64>,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
