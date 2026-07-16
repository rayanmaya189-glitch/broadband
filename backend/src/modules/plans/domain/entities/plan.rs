use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "plans", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub speed_label: String,
    pub download_mbps: i32,
    pub upload_mbps: i32,
    pub burst_mbps: Option<i32>,
    pub data_quota: Option<String>,
    pub fair_usage_policy: Option<serde_json::Value>,
    pub qos_priority: Option<String>,
    pub sla_uptime_percent: Option<sea_orm::prelude::Decimal>,
    pub is_popular: bool,
    pub is_business: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_by: Option<i64>,
    pub review_status: Option<String>,
    pub approved_by: Option<i64>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
