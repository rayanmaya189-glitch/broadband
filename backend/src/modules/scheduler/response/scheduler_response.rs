use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TaskResponse { pub id: i64, pub name: String, pub task_type: String, pub config: Option<serde_json::Value>, pub schedule_type: String, pub schedule_value: String, pub next_run_at: Option<DateTime<Utc>>, pub last_run_at: Option<DateTime<Utc>>, pub is_active: bool, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExecutionResponse { pub id: i64, pub task_id: i64, pub status: String, pub result: Option<serde_json::Value>, pub error_message: Option<String>, pub started_at: DateTime<Utc>, pub completed_at: Option<DateTime<Utc>>, pub duration_ms: Option<i64> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse { pub message: String }
