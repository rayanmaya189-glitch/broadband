use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDefinitionRequest { pub name: String, pub description: Option<String>, pub entity_type: String }
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddStepRequest { pub name: String, pub step_type: String, pub step_order: i32, pub required_role: Option<String>, pub config: Option<serde_json::Value> }
#[derive(Debug, Deserialize, ToSchema)]
pub struct StartInstanceRequest { pub definition_id: i64, pub entity_id: i64 }
#[derive(Debug, Deserialize, ToSchema)]
pub struct StepDecisionRequest { pub comments: Option<String> }
#[derive(Debug, Deserialize, ToSchema)]
pub struct InstanceQuery { pub branch_id: Option<i64>, pub status: Option<String>, pub page: Option<i64>, pub per_page: Option<i64> }
