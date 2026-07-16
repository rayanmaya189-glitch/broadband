use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Workflow instances represent running sagas / long-running processes.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "workflow", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Workflow type (e.g. "customer_activation", "subscription_upgrade", "payment_failure_recovery")
    pub workflow_type: String,
    /// Reference entity type (e.g. "customer", "subscription", "invoice")
    pub reference_type: String,
    /// Reference entity ID
    pub reference_id: i64,
    /// Current status: pending, running, completed, failed, compensation, cancelled
    pub status: String,
    /// Current step index
    pub current_step: i32,
    /// Total number of steps
    pub total_steps: i32,
    /// Input data for the workflow (JSONB)
    pub input_data: serde_json::Value,
    /// Output data after completion (JSONB)
    pub output_data: Option<serde_json::Value>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// User who initiated the workflow
    pub initiated_by: Option<i64>,
    /// Branch context
    pub branch_id: Option<i64>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
