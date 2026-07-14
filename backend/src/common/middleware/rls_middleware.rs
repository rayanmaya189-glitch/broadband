use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;

/// Company-wide roles that bypass branch scoping.
const COMPANY_WIDE_ROLES: &[&str] = &["super_admin", "isp_owner", "finance_manager"];

/// Tower middleware: set PostgreSQL session variables for Row-Level Security.
///
/// Before each request, this middleware sets:
///   - `app.current_branch_id` → the user's branch ID
///   - `app.is_company_wide`   → true if the user bypasses branch filtering
///
/// These variables are read by the `fn_branch_scope()` trigger function
/// in the database to enforce RLS policies on branch-scoped tables.
///
/// Requires `DatabaseConnection` to be in request extensions (set by `inject_pool_middleware`).
pub async fn rls_middleware(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let user = req
        .extensions()
        .get::<UserContext>()
        .cloned()
        .ok_or(AppError::Unauthorized)?;

    let pool = req
        .extensions()
        .get::<DatabaseConnection>()
        .cloned()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("DatabaseConnection not found in request extensions")))?;

    let is_company_wide =
        user.is_company_wide || COMPANY_WIDE_ROLES.contains(&user.role.as_str());

    // Set PostgreSQL session variables for RLS using SeaORM with parameterized queries
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT set_config('app.is_company_wide', $1, true)",
        vec![is_company_wide.to_string().into()],
    );
    pool.execute(stmt).await?;

    if let Some(branch_id) = user.branch_id {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT set_config('app.current_branch_id', $1, true)",
            vec![branch_id.into()],
        );
        pool.execute(stmt).await?;
    }

    Ok(next.run(req).await)
}
