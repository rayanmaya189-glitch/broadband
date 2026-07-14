use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskRequest { pub name: String, pub task_type: String, pub config: Option<serde_json::Value>, pub schedule_type: String, pub schedule_value: String }
#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecutionQuery { pub task_id: Option<i64>, pub page: Option<i64>, pub per_page: Option<i64> }
