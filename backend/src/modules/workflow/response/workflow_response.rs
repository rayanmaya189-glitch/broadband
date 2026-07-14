use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DefinitionResponse { pub id: i64, pub name: String, pub description: Option<String>, pub entity_type: String, pub is_active: bool, pub version: i32, pub created_at: DateTime<Utc>, pub updated_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InstanceResponse { pub id: i64, pub definition_id: i64, pub entity_id: i64, pub status: String, pub current_step_index: i32, pub started_by: i64, pub started_at: DateTime<Utc>, pub completed_at: Option<DateTime<Utc>>, pub notes: Option<String> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse { pub message: String }
