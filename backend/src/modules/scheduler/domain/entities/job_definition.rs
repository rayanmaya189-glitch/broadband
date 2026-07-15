use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Scheduled job definitions (cron-like recurring tasks).
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "job_definitions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Unique job name (e.g. "generate_monthly_invoices", "check_sla_breaches")
    pub name: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Job type: cron, interval, one_time
    pub job_type: String,
    /// Cron expression (e.g. "0 0 1 * *") or interval in seconds
    pub schedule: String,
    /// Target module to trigger (e.g. "billing", "monitoring")
    pub target_module: String,
    /// Action to trigger (e.g. "generate_invoices", "check_health")
    pub action: String,
    /// Payload to send with the trigger (JSONB)
    pub payload: serde_json::Value,
    /// Whether the job is currently enabled
    pub is_active: bool,
    /// Maximum execution time in seconds
    pub timeout_seconds: Option<i32>,
    /// Next scheduled run
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Last execution datetime
    pub last_run_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Last execution status
    pub last_run_status: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
