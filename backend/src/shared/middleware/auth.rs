use axum::http::request::Parts;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::identity::application::services::IdentityService;
use crate::shared::app_state::AppState;
use crate::shared::utils::jwt_keys::StandardClaims;

/// User context extracted from JWT token + Redis permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserContext {
    pub user_id: i64,
    pub email: String,
    pub role: String,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub permissions: Vec<String>,
}

#[axum::async_trait]
impl axum::extract::FromRequestParts<Arc<AppState>> for UserContext {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing authorization header".to_string(),
                )
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid authorization header format".to_string(),
            )
        })?;

        let claims: StandardClaims = state
            .jwt_keys
            .verify(token)
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;

        let user_id = claims.sub.parse::<i64>().unwrap_or(0);
        let email = claims.email;
        let role = claims.role;
        let branch_id = claims.branch_id;
        let is_company_wide = claims.is_company_wide;

        // Fetch permissions from Redis (not from JWT) - prevents token leak exposure
        let mut redis = state.redis.clone();
        let permissions = IdentityService::get_permissions_from_redis(&mut redis, user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Permissions service unavailable".to_string(),
                )
            })?;

        Ok(UserContext {
            user_id,
            email,
            role,
            branch_id,
            is_company_wide,
            permissions,
        })
    }
}

/// Branch filter context for branch-scoped queries.
#[derive(Debug, Clone)]
pub struct BranchFilter {
    pub branch_ids: Vec<i64>,
    pub is_company_wide: bool,
}

/// Check if user has a specific permission.
pub fn has_permission(user: &UserContext, permission: &str) -> bool {
    if user.is_company_wide {
        return true;
    }
    if user.permissions.contains(&permission.to_string()) {
        return true;
    }
    for user_perm in &user.permissions {
        if user_perm.contains('*') {
            let pattern_parts: Vec<&str> = user_perm.split('.').collect();
            let perm_parts: Vec<&str> = permission.split('.').collect();
            if pattern_parts.len() == perm_parts.len() {
                let matches = pattern_parts
                    .iter()
                    .zip(perm_parts.iter())
                    .all(|(p, q)| p == &"*" || p == q);
                if matches {
                    return true;
                }
            }
        }
    }
    false
}

/// Require a specific permission.
pub fn require_permission(
    user: &UserContext,
    permission: &str,
) -> Result<(), (StatusCode, String)> {
    if has_permission(user, permission) {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            format!("Permission '{}' required", permission),
        ))
    }
}

/// Company-wide roles that bypass branch filtering.
pub const COMPANY_WIDE_ROLES: &[&str] = &["super_admin", "isp_owner", "finance_manager"];

/// Check if a role is company-wide.
pub fn is_company_wide_role(role: &str) -> bool {
    COMPANY_WIDE_ROLES.contains(&role)
}
