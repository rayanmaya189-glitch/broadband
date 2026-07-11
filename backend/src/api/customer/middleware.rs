use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;

/// Roles that are considered "customer" with access to customer self-service API.
const CUSTOMER_ROLES: &[&str] = &[
    "customer",
];

/// Axum middleware: verifies that the authenticated user has a customer role.
///
/// Must be placed AFTER `jwt_middleware` so that `UserContext` is available
/// in the request extensions.
pub async fn customer_role_guard(req: Request, next: Next) -> Result<Response, AppError> {
    let user = req
        .extensions()
        .get::<UserContext>()
        .cloned()
        .ok_or(AppError::Unauthorized)?;

    if !CUSTOMER_ROLES.contains(&user.role.as_str()) {
        tracing::warn!(role = %user.role, user_id = user.user_id, "Non-customer user attempted to access customer API");
        return Err(AppError::Forbidden("Customer access required".into()));
    }

    Ok(next.run(req).await)
}
