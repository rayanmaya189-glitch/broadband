use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::extract::FromRequestParts;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::shared::middleware::auth::UserContext;
use crate::shared::utils::jwt_keys::{JwtKeyPair, StandardClaims};
use std::sync::OnceLock;

/// Global JWT key pair reference for the branch scope middleware.
/// Initialized once at startup via `init_jwt_keys_global`.
static JWT_KEYS: OnceLock<JwtKeyPair> = OnceLock::new();

/// Initialize the global JWT key pair. Call once at startup.
pub fn init_jwt_keys_global(keys: JwtKeyPair) {
    let _ = JWT_KEYS.set(keys);
}

/// Get the global JWT key pair reference.
fn get_jwt_keys() -> Option<&'static JwtKeyPair> {
    JWT_KEYS.get()
}

/// Company-wide roles that bypass branch filtering.
const COMPANY_WIDE_ROLES: &[&str] = &["super_admin", "isp_owner", "finance_manager"];

/// Branch filter context for branch-scoped queries.
#[derive(Debug, Clone)]
pub struct BranchScope {
    pub branch_ids: Vec<i64>,
    pub is_company_wide: bool,
}

impl BranchScope {
    /// Create a new BranchScope from a UserContext
    pub fn from_user_context(user: &UserContext) -> Self {
        let is_company_wide = user.is_company_wide
            || COMPANY_WIDE_ROLES.contains(&user.role.as_str());

        let branch_ids = if is_company_wide {
            Vec::new() // Company-wide users don't need branch filtering
        } else {
            user.branch_id.map(|id| vec![id]).unwrap_or_default()
        };

        Self {
            branch_ids,
            is_company_wide,
        }
    }

    /// Check if a given branch_id is accessible by this scope
    pub fn can_access_branch(&self, branch_id: i64) -> bool {
        self.is_company_wide || self.branch_ids.contains(&branch_id)
    }

    /// Get SQL WHERE clause for branch filtering
    pub fn to_sql_filter(&self) -> String {
        if self.is_company_wide {
            return "1=1".to_string();
        }

        if self.branch_ids.is_empty() {
            return "1=0".to_string();
        }

        let ids: Vec<String> = self.branch_ids.iter().map(|id| id.to_string()).collect();
        format!("branch_id IN ({})", ids.join(", "))
    }
}

/// Extract UserContext from JWT in Authorization header (without Redis permission lookup).
/// This is used by the branch scope middleware to populate extensions before handlers run.
fn extract_user_context_from_headers(headers: &axum::http::HeaderMap) -> Option<UserContext> {
    let auth_header = headers
        .get("Authorization")?
        .to_str()
        .ok()?;

    let token = auth_header.strip_prefix("Bearer ")?;

    let jwt_keys = get_jwt_keys()?;
    let claims: StandardClaims = jwt_keys.verify(token).ok()?;

    let user_id = claims.sub.parse::<i64>().unwrap_or(0);

    // Note: permissions are NOT fetched here (no Redis call in middleware).
    // Full permissions are loaded by the auth extractor per-handler.
    Some(UserContext {
        user_id,
        email: claims.email,
        role: claims.role,
        branch_id: claims.branch_id,
        is_company_wide: claims.is_company_wide,
        permissions: Vec::new(), // Placeholder - full permissions loaded per-handler
    })
}

/// Middleware that extracts JWT from Authorization header, creates UserContext,
/// and injects both UserContext and BranchScope into request extensions.
///
/// This runs as a global layer so that:
/// 1. `UserContext` is available in extensions for any handler/middleware that needs it
/// 2. `BranchScope` is available in extensions for branch-scoped queries
///
/// NOTE: The full UserContext with permissions is still loaded per-handler by the
/// auth extractor (which calls Redis). This middleware provides a lightweight
/// version for branch scoping purposes only.
pub async fn branch_scope_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Try to extract UserContext from JWT via Authorization header
    if let Some(user) = extract_user_context_from_headers(request.headers()) {
        // Create BranchScope from UserContext
        let scope = BranchScope::from_user_context(&user);

        // Inject both into request extensions
        request.extensions_mut().insert(user);
        request.extensions_mut().insert(scope);
    }
    // If no JWT or invalid token, proceed without scope (public routes work fine)

    Ok(next.run(request).await)
}

/// Extract branch scope from request parts for use in handlers
#[axum::async_trait]
impl<S> FromRequestParts<S> for BranchScope
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract from extensions (injected by branch_scope_middleware)
        // If not present, create a company-wide default
        Ok(parts
            .extensions
            .get::<BranchScope>()
            .cloned()
            .unwrap_or_else(|| BranchScope {
                branch_ids: Vec::new(),
                is_company_wide: true,
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_scope_company_wide() {
        let user = UserContext {
            user_id: 1,
            email: "admin@aeroxe.com".to_string(),
            role: "super_admin".to_string(),
            branch_id: None,
            is_company_wide: true,
            permissions: Vec::new(),
        };

        let scope = BranchScope::from_user_context(&user);
        assert!(scope.is_company_wide);
        assert!(scope.can_access_branch(1));
        assert!(scope.can_access_branch(999));
    }

    #[test]
    fn test_branch_scope_branch_user() {
        let user = UserContext {
            user_id: 2,
            email: "noc@aeroxe.com".to_string(),
            role: "noc_engineer".to_string(),
            branch_id: Some(1),
            is_company_wide: false,
            permissions: Vec::new(),
        };

        let scope = BranchScope::from_user_context(&user);
        assert!(!scope.is_company_wide);
        assert!(scope.can_access_branch(1));
        assert!(!scope.can_access_branch(2));
    }

    #[test]
    fn test_sql_filter_company_wide() {
        let scope = BranchScope {
            branch_ids: Vec::new(),
            is_company_wide: true,
        };
        assert_eq!(scope.to_sql_filter(), "1=1");
    }

    #[test]
    fn test_sql_filter_branch_user() {
        let scope = BranchScope {
            branch_ids: vec![1, 2],
            is_company_wide: false,
        };
        assert_eq!(scope.to_sql_filter(), "branch_id IN (1, 2)");
    }
}
