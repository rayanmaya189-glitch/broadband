use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;

/// Tower middleware: log every request as an audit entry.
pub async fn audit_middleware(req: Request, next: Next) -> Result<Response, AppError> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let user = req.extensions().get::<UserContext>().cloned();
    let start = std::time::Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status = response.status().as_u16();

    tracing::info!(
        method = %method,
        path = %uri,
        status = status,
        user_id = user.as_ref().map(|u| u.user_id),
        elapsed_ms = elapsed.as_millis() as u64,
        "Request completed"
    );

    if status >= 400 {
        tracing::warn!(
            method = %method,
            path = %uri,
            status = status,
            user_id = user.as_ref().map(|u| u.user_id),
            "Request failed"
        );
    }

    Ok(response)
}
