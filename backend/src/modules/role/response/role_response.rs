use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type RoleDetailResponse = RoleResponse;

impl RoleResponse {
    pub fn from_model(m: crate::modules::role::model::role_entity::Model) -> Self {
        Self {
            id: m.id, name: m.name, display_name: m.display_name, description: m.description,
            is_system: m.is_system, is_active: m.is_active,
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
