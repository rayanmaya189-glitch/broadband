//! RBAC permission-checking middleware and helpers.

use axum::http::Request;
use axum::response::Response;

use crate::error::AppError;
use crate::shared::types::UserContext;

/// Check if a user has a specific permission (supports wildcards).
pub fn has_permission(user: &UserContext, permission: &str) -> bool {
    if user.is_company_wide && user.role == "super_admin" {
        return true;
    }

    user.permissions.iter().any(|p| {
        p == permission || matches_wildcard(p, permission)
    })
}

/// Check if a permission pattern (with `*` wildcards) matches a target.
fn matches_wildcard(pattern: &str, target: &str) -> bool {
    let pattern_parts: Vec<&str> = pattern.split('.').collect();
    let target_parts: Vec<&str> = target.split('.').collect();

    if pattern_parts.len() != target_parts.len() {
        return false;
    }

    pattern_parts
        .iter()
        .zip(target_parts.iter())
        .all(|(p, t)| *p == "*" || *p == *t)
}

/// Require a specific permission or return 403.
pub fn require_permission(user: &UserContext, permission: &str) -> Result<(), AppError> {
    if has_permission(user, permission) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "Missing permission: {permission}"
        )))
    }
}

/// Tower middleware that enforces a required permission on every request.
///
/// Usage:
/// ```rust
/// Router::new()
///     .route("/admin/devices", get(list_devices))
///     .layer(RequirePermission::new("device.router.view"))
/// ```
#[derive(Clone)]
pub struct RequirePermission {
    permission: String,
}

impl RequirePermission {
    pub fn new(permission: impl Into<String>) -> Self {
        Self {
            permission: permission.into(),
        }
    }
}

impl<S> tower::Layer<S> for RequirePermission {
    type Service = RequirePermissionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequirePermissionService {
            inner,
            permission: self.permission.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RequirePermissionService<S> {
    inner: S,
    permission: String,
}

impl<S> tower::Service<Request<axum::body::Body>> for RequirePermissionService<S>
where
    S: tower::Service<Request<axum::body::Body>, Response = Response> + Send + Sync + Clone + 'static,
    S::Future: Send,
{
    type Response = Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<axum::body::Body>) -> Self::Future {
        let perm = self.permission.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let user = req.extensions().get::<UserContext>().cloned();

            match user {
                Some(user) => {
                    if require_permission(&user, &perm).is_err() {
                        let response = axum::http::StatusCode::FORBIDDEN.into_response();
                        return Ok(response);
                    }
                    Ok(inner.call(req).await?)
                }
                None => {
                    let response = axum::http::StatusCode::UNAUTHORIZED.into_response();
                    Ok(response)
                }
            }
        })
    }
}

// We need this for the layer to work
use axum::response::IntoResponse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wildcard_permission() {
        assert!(matches_wildcard("device.*.view", "device.router.view"));
        assert!(matches_wildcard("device.*.view", "device.olt.view"));
        assert!(!matches_wildcard("device.*.view", "device.router.restart"));
        assert!(!matches_wildcard("device.*.view", "billing.invoice.view"));
    }

    #[test]
    fn test_exact_permission() {
        assert!(matches_wildcard("auth.login", "auth.login"));
        assert!(!matches_wildcard("auth.login", "auth.logout"));
    }
}
