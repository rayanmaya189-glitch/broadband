use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;

use crate::app::SharedState;

/// Tower middleware: inject PgPool into request extensions.
///
/// This must run as an outer layer so that downstream middleware
/// (like rls_middleware) can access the pool from extensions.
pub async fn inject_pool_middleware(
    State(state): State<SharedState>,
    mut req: Request,
    next: Next,
) -> Response {
    req.extensions_mut().insert(state.db.clone());
    next.run(req).await
}
