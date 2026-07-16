use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "monitoring_alerts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub device_id: i64,
    pub branch_id: i64,
    pub alert_rule_id: Option<i64>,
    pub alert_type: String,
    pub severity: String,
    pub status: String,
    pub title: String,
    pub message: String,
    pub metric_name: Option<String>,
    pub metric_value: Option<f64>,
    pub threshold_value: Option<f64>,
    pub acknowledged_by: Option<i64>,
    pub acknowledged_at: Option<chrono::DateTime<chrono::Utc>>,
    pub resolved_by: Option<i64>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub resolution_notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
