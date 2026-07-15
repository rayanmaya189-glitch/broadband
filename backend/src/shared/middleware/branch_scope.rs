use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;

use crate::shared::middleware::auth::UserContext;

/// Branch filter context for branch-scoped queries.
#[derive(Debug, Clone)]
pub struct BranchScope {
    pub branch_ids: Vec<i64>,
    pub is_company_wide: bool,
}

/// Company-wide roles that bypass branch filtering.
const COMPANY_WIDE_ROLES: &[&str] = &["super_admin", "isp_owner", "finance_manager"];

impl BranchScope {
    /// Create a new BranchScope from a UserContext
    pub fn from_user_context(user: &UserContext) -> Self {
        let is_company_wide =
            user.is_company_wide || COMPANY_WIDE_ROLES.contains(&user.role.as_str());

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
            // No filter needed
            return "1=1".to_string();
        }

        if self.branch_ids.is_empty() {
            // No branches assigned - deny all
            return "1=0".to_string();
        }

        let ids: Vec<String> = self.branch_ids.iter().map(|id| id.to_string()).collect();
        format!("branch_id IN ({})", ids.join(","))
    }
}

/// Tower middleware layer for branch scoping
#[derive(Clone)]
pub struct BranchScopeLayer;

impl<S> tower::Layer<S> for BranchScopeLayer {
    type Service = BranchScopeMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BranchScopeMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct BranchScopeMiddleware<S> {
    inner: S,
}

impl<S> tower::Service<axum::http::Request<axum::body::Body>> for BranchScopeMiddleware<S>
where
    S: tower::Service<axum::http::Request<axum::body::Body>, Response = axum::response::Response>
        + Send
        + 'static,
    S::Future: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: axum::http::Request<axum::body::Body>) -> Self::Future {
        let fut = self.inner.call(req);
        Box::pin(async move { fut.await })
    }
}

/// Extract branch scope from request parts for use in handlers
#[axum::async_trait]
impl<S> FromRequestParts<S> for BranchScope
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // This will be populated by the auth middleware
        // For now, return a default - in production, extract from extensions
        Ok(BranchScope {
            branch_ids: Vec::new(),
            is_company_wide: true,
        })
    }
}

/// Helper to create branch scope from UserContext (for use in handlers)
pub fn create_branch_scope(user: &UserContext) -> BranchScope {
    BranchScope::from_user_context(user)
}

/// Helper to inject branch scope into request extensions
pub fn inject_branch_scope(req: &mut axum::http::Request<axum::body::Body>, user: &UserContext) {
    let scope = BranchScope::from_user_context(user);
    req.extensions_mut().insert(scope);
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
