use crate::modules::user::model::user::{User};
use crate::modules::user::response::user_response::{AuthUserResponse, UserResponse};

/// Map a User database row to a UserResponse.
pub fn user_to_response(user: &User, role_name: Option<&str>) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email.clone(),
        name: user.name.clone(),
        phone: user.phone.clone(),
        avatar_url: user.avatar_url.clone(),
        role_id: user.role_id,
        role_name: role_name.map(String::from),
        branch_id: user.branch_id,
        is_company_wide: user.is_company_wide,
        is_active: user.is_active,
        is_locked: user.is_locked,
        two_factor_enabled: user.two_factor_enabled,
        last_login_at: user.last_login_at,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

/// Map a User database row to an AuthUserResponse.
pub fn user_to_auth_response(user: &User, role: &str) -> AuthUserResponse {
    AuthUserResponse {
        id: user.id,
        email: user.email.clone(),
        name: user.name.clone(),
        phone: user.phone.clone(),
        avatar_url: user.avatar_url.clone(),
        role: role.to_string(),
        branch_id: user.branch_id,
        is_company_wide: user.is_company_wide,
    }
}
