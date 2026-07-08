use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::{BranchFilter, UserContext};

/// Company-wide roles that bypass branch scoping.
const COMPANY_WIDE_ROLES: &[&str] = &["super_admin", "isp_owner", "finance_manager"];

/// Tower middleware: inject `BranchFilter` based on the user's assigned branches.
pub async fn branch_scope_middleware(mut req: Request, next: Next) -> Result<Response, AppError> {
    let user = req
        .extensions()
        .get::<UserContext>()
        .cloned()
        .ok_or(AppError::Unauthorized)?;

    let is_company_wide =
        user.is_company_wide || COMPANY_WIDE_ROLES.contains(&user.role.as_str());

    let filter = BranchFilter {
        branch_ids: if is_company_wide {
            vec![]
        } else if let Some(branch_id) = user.branch_id {
            vec![branch_id]
        } else {
            vec![]
        },
        is_company_wide,
    };

    req.extensions_mut().insert(filter);

    Ok(next.run(req).await)
}
