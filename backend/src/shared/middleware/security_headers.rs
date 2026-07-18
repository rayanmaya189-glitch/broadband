use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Middleware that adds security headers to all responses.
///
/// Headers implemented per OWASP recommendations:
/// - Strict-Transport-Security (HSTS)
/// - X-Content-Type-Options
/// - X-Frame-Options
/// - X-XSS-Protection
/// - Content-Security-Policy
/// - Referrer-Policy
/// - Permissions-Policy
/// - Cache-Control for API responses
pub async fn security_headers_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();

    // HSTS - Force HTTPS for 1 year including subdomains
    headers.insert(
        "strict-transport-security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );

    // Prevent MIME type sniffing
    headers.insert("x-content-type-options", "nosniff".parse().unwrap());

    // Prevent clickjacking
    headers.insert("x-frame-options", "DENY".parse().unwrap());

    // XSS Protection (legacy but still useful for older browsers)
    headers.insert("x-xss-protection", "1; mode=block".parse().unwrap());

    // Content Security Policy - restrict resource loading
    headers.insert(
        "content-security-policy",
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'".parse().unwrap(),
    );

    // Control referrer information
    headers.insert(
        "referrer-policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );

    // Restrict browser features
    headers.insert(
        "permissions-policy",
        "camera=(), microphone=(), geolocation=(), payment=()"
            .parse()
            .unwrap(),
    );

    // No caching for API responses (prevent sensitive data caching)
    headers.insert(
        "cache-control",
        "no-store, no-cache, must-revalidate, private"
            .parse()
            .unwrap(),
    );

    // Prevent search engines from indexing API responses
    headers.insert("x-robots-tag", "noindex, nofollow".parse().unwrap());

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    async fn dummy_handler() -> &'static str {
        "ok"
    }

    #[tokio::test]
    async fn test_security_headers_present() {
        let app = Router::new()
            .route("/test", get(dummy_handler))
            .layer(axum::middleware::from_fn(security_headers_middleware));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let headers = response.headers();

        assert!(headers.contains_key("strict-transport-security"));
        assert!(headers.contains_key("x-content-type-options"));
        assert!(headers.contains_key("x-frame-options"));
        assert!(headers.contains_key("x-xss-protection"));
        assert!(headers.contains_key("content-security-policy"));
        assert!(headers.contains_key("referrer-policy"));
        assert!(headers.contains_key("permissions-policy"));
        assert!(headers.contains_key("cache-control"));
        assert!(headers.contains_key("x-robots-tag"));

        assert_eq!(headers.get("x-content-type-options").unwrap(), "nosniff");
        assert_eq!(headers.get("x-frame-options").unwrap(), "DENY");
    }
}
