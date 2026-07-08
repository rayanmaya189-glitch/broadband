use crate::modules::role::model::role::Role;
use crate::modules::role::response::role_response::RoleResponse;

pub fn role_to_response(role: &Role) -> RoleResponse {
    RoleResponse {
        id: role.id,
        name: role.name.clone(),
        display_name: role.display_name.clone(),
        description: role.description.clone(),
        is_system: role.is_system,
        is_active: role.is_active,
        created_at: role.created_at,
        updated_at: role.updated_at,
    }
}
