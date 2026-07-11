use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;

/// Roles that are considered "admin/staff" with access to admin API.
const ADMIN_ROLES: &[&str] = &[
    "super_admin",
    "isp_owner",
    "network_admin",
    "noc_engineer",
    "admin",
    "finance_manager",
    "support_agent",
    "field_technician",
    "sales_agent",
];

/// Axum middleware: verifies that the authenticated user has an admin/staff role.
///
/// Must be placed AFTER `jwt_middleware` so that `UserContext` is available
/// in the request extensions.
pub async fn admin_role_guard(req: Request, next: Next) -> Result<Response, AppError> {
    let user = req
        .extensions()
        .get::<UserContext>()
        .cloned()
        .ok_or(AppError::Unauthorized)?;

    if !ADMIN_ROLES.contains(&user.role.as_str()) {
        tracing::warn!(role = %user.role, user_id = user.user_id, "Non-admin user attempted to access admin API");
        return Err(AppError::Forbidden("Admin access required".into()));
    }

    Ok(next.run(req).await)
}
