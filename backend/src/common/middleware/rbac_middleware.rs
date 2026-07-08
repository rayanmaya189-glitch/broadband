use crate::common::middleware::auth_middleware::UserContext;

/// Check if a user has a specific permission (supports wildcards).
pub fn has_permission(user: &UserContext, permission: &str) -> bool {
    if user.is_company_wide && user.role == "super_admin" {
        return true;
    }

    user.permissions
        .iter()
        .any(|p| p == permission || matches_wildcard(p, permission))
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

/// Require a specific permission or return an error.
pub fn require_permission(
    user: &UserContext,
    permission: &str,
) -> Result<(), crate::common::errors::app_error::AppError> {
    if has_permission(user, permission) {
        Ok(())
    } else {
        Err(crate::common::errors::app_error::AppError::Forbidden(
            format!("Missing permission: {permission}"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wildcard_permission() {
        assert!(matches_wildcard("device.*.view", "device.router.view"));
        assert!(matches_wildcard("device.*.view", "device.olt.view"));
        assert!(!matches_wildcard("device.*.view", "device.router.restart"));
    }

    #[test]
    fn test_exact_permission() {
        assert!(matches_wildcard("auth.login", "auth.login"));
        assert!(!matches_wildcard("auth.login", "auth.logout"));
    }
}
