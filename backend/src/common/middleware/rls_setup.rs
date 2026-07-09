use axum::middleware;
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::common::middleware::rls_middleware::rls_middleware;

/// Wrap a router with JWT auth + RLS middleware.
///
/// Applies `rls_middleware` AFTER `jwt_middleware` (so UserContext is available).
/// In axum, the last `.layer()` added runs first, so:
///   - `jwt_middleware` is added second → runs first
///   - `rls_middleware` is added first → runs second (after JWT)
///
/// Usage:
/// ```ignore
/// pub fn customers_routes() -> Router<SharedState> {
///     rls_setup::branch_scoped(
///         Router::new()
///             .route("/", get(list_customers).post(create_customer))
///     )
/// }
/// ```
pub fn branch_scoped(router: Router<SharedState>) -> Router<SharedState> {
    router
        .layer(middleware::from_fn(rls_middleware))
        .layer(middleware::from_fn(jwt_middleware))
}
