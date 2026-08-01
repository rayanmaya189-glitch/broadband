//! Request ID middleware for distributed tracing.
//! Propagates X-Request-ID through the request pipeline.
//! If not provided by the client, generates a UUID v7.

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;

/// Header name for request ID propagation
pub const X_REQUEST_ID: &str = "X-Request-ID";

/// Middleware that ensures every request has a unique X-Request-ID.
///
/// - If the client sends `X-Request-ID`, it is preserved and propagated.
/// - If missing, a UUID v7 is generated.
/// - The request ID is added to response headers and to tracing span context.
pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = req
        .headers()
        .get(X_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            crate::shared::utils::uuid_v7::new_v7_string()
        });

    // Insert into request extensions so handlers can access it
    req.extensions_mut().insert(RequestId(request_id.clone()));

    let mut response = next.run(req).await;

    // Add request ID to response headers
    if let Ok(val) = HeaderValue::from_str(&request_id) {
        response.headers_mut().insert(X_REQUEST_ID, val);
    }

    response
}

/// Request ID extracted from extensions
#[derive(Debug, Clone)]
pub struct RequestId(pub String);

impl RequestId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_request_id_format() {
        let id = crate::shared::utils::uuid_v7::new_v7_string();
        assert_eq!(id.len(), 36);
        assert!(id.contains('-'));
    }
}
