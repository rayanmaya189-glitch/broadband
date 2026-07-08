use crate::modules::permission::model::permission::Permission;
use crate::modules::permission::response::permission_response::PermissionResponse;

pub fn permission_to_response(perm: &Permission) -> PermissionResponse {
    PermissionResponse {
        id: perm.id,
        name: perm.name.clone(),
        description: perm.description.clone(),
        module: perm.module.clone(),
        created_at: perm.created_at,
    }
}
