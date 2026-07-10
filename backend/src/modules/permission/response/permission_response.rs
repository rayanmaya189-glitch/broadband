use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionResponse {
    pub id: i64,
    pub name: String,
    pub method: String,
    pub api_url: String,
    pub guard: String,
    pub module: String,
    pub created_at: DateTime<Utc>,
}

impl PermissionResponse {
    pub fn from_model(m: crate::modules::permission::model::permission_entity::Model) -> Self {
        Self {
            id: m.id, name: m.name, method: m.method, api_url: m.api_url,
            guard: m.guard, module: m.module, created_at: m.created_at.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
