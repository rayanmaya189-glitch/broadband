use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Individual steps within a workflow instance.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workflow_steps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub workflow_instance_id: i64,
    /// Step name (e.g. "verify_kyc", "create_billing_account", "provision_network")
    pub step_name: String,
    /// Step order (0-based)
    pub step_order: i32,
    /// Target module for this step (e.g. "compliance", "billing", "network")
    pub target_module: String,
    /// Action to perform (e.g. "verify_kyc", "create_account")
    pub action: String,
    /// Input payload for this step (JSONB)
    pub input_payload: serde_json::Value,
    /// Output payload after step completion (JSONB)
    pub output_payload: Option<serde_json::Value>,
    /// Status: pending, running, completed, failed, skipped
    pub status: String,
    /// Error message if step failed
    pub error_message: Option<String>,
    /// Number of retry attempts
    pub retry_count: i32,
    /// Maximum retries allowed
    pub max_retries: i32,
    /// Compensation action name (for rollback)
    pub compensation_action: Option<String>,
    /// Whether compensation has been executed
    pub compensation_executed: bool,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
