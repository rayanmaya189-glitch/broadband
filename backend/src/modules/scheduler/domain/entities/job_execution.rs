use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Record of each scheduled job execution.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "job_executions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub job_definition_id: i64,
    /// Execution status: running, completed, failed, timed_out
    pub status: String,
    /// Input payload used for this execution (JSONB)
    pub input_payload: serde_json::Value,
    /// Output payload after execution (JSONB)
    pub output_payload: Option<serde_json::Value>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Duration in milliseconds
    pub duration_ms: Option<i64>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
