use axum::middleware;
use axum::Router;

use crate::api::admin::middleware::admin_role_guard;
use crate::api::customer::middleware::customer_role_guard;
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

/// Wrap a router with JWT auth + admin role guard middleware.
///
/// Applies `admin_role_guard` AFTER `jwt_middleware` (so UserContext is available).
/// In axum, the last `.layer()` added runs first, so:
///   - `jwt_middleware` is added second → runs first
///   - `admin_role_guard` is added first → runs second (after JWT)
///
/// Only users with admin/staff roles (super_admin, isp_owner, network_admin, etc.)
/// will be allowed through. Non-admin users receive a 403 Forbidden response.
///
/// Usage:
/// ```ignore
/// pub fn admin_routes() -> Router<SharedState> {
///     rls_setup::admin_scoped(
///         Router::new()
///             .route("/", get(controller::list))
///     )
/// }
/// ```
pub fn admin_scoped(router: Router<SharedState>) -> Router<SharedState> {
    router
        .layer(middleware::from_fn(admin_role_guard))
        .layer(middleware::from_fn(jwt_middleware))
}

/// Wrap a router with JWT auth + admin role guard + RLS middleware.
///
/// Combines `admin_scoped` and `branch_scoped` — applies JWT, admin role guard,
/// AND PostgreSQL RLS session variables in that order. Use for admin routes that
/// access branch-scoped data (e.g., subscriptions, tickets, billing).
///
/// In axum, the last `.layer()` added runs first, so:
///   - `jwt_middleware` is added third → runs first
///   - `admin_role_guard` is added second → runs second
///   - `rls_middleware` is added first → runs third (after JWT + role check)
///
/// Only admin/staff users are allowed through, and their branch context is
/// injected into the database session for RLS policy enforcement.
///
/// Usage:
/// ```ignore
/// pub fn subscription_routes() -> Router<SharedState> {
///     rls_setup::admin_branch_scoped(
///         Router::new()
///             .route("/", get(controller::list))
///     )
/// }
/// ```
pub fn admin_branch_scoped(router: Router<SharedState>) -> Router<SharedState> {
    router
        .layer(middleware::from_fn(rls_middleware))
        .layer(middleware::from_fn(admin_role_guard))
        .layer(middleware::from_fn(jwt_middleware))
}

/// Wrap a router with JWT auth + customer role guard middleware.
///
/// Applies `customer_role_guard` AFTER `jwt_middleware` (so UserContext is available).
/// In axum, the last `.layer()` added runs first, so:
///   - `jwt_middleware` is added second → runs first
///   - `customer_role_guard` is added first → runs second (after JWT)
///
/// Only users with the "customer" role will be allowed through.
/// Staff/admin users receive a 403 Forbidden response.
///
/// Usage:
/// ```ignore
/// pub fn customer_routes() -> Router<SharedState> {
///     rls_setup::customer_scoped(
///         Router::new()
///             .route("/", get(controller::list))
///     )
/// }
/// ```
pub fn customer_scoped(router: Router<SharedState>) -> Router<SharedState> {
    router
        .layer(middleware::from_fn(customer_role_guard))
        .layer(middleware::from_fn(jwt_middleware))
}
