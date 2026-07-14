use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRuleRequest { pub name: String, pub description: Option<String>, pub priority: Option<i32> }
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddTriggerRequest { pub trigger_type: String, pub config: serde_json::Value }
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddActionRequest { pub action_type: String, pub config: serde_json::Value, pub order_index: Option<i32> }
#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecutionQuery { pub rule_id: Option<i64>, pub page: Option<i64>, pub per_page: Option<i64> }
