use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RuleResponse { pub id: i64, pub name: String, pub description: Option<String>, pub priority: i32, pub is_active: bool, pub created_at: DateTime<Utc>, pub updated_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExecutionResponse { pub id: i64, pub rule_id: i64, pub status: String, pub trigger_data: Option<serde_json::Value>, pub result: Option<serde_json::Value>, pub error_message: Option<String>, pub started_at: DateTime<Utc>, pub completed_at: Option<DateTime<Utc>> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse { pub message: String }
