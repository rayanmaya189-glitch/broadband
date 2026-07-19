use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::net::IpAddr;
use tracing::warn;

/// Hostnames that are always blocked (cloud metadata, etc.)
const BLOCKED_HOSTNAMES: &[&str] = &[
    "localhost",
    "127.0.0.1",
    "0.0.0.0",
    "169.254.169.254", // AWS/GCP/Azure metadata
    "metadata.google.internal",
    "metadata.azure.com",
    "[::1]",
    "0:0:0:0:0:0:0:1",
];

/// SSRF protection middleware.
///
/// Inspects the request body for URLs and validates them against the blocklist.
/// Only applies to POST, PUT, PATCH requests with JSON bodies.
pub async fn ssrf_protection_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    // Only check write methods with potential URL payloads
    if !matches!(
        method,
        axum::http::Method::POST | axum::http::Method::PUT | axum::http::Method::PATCH
    ) {
        return Ok(next.run(request).await);
    }

    // Skip webhook endpoints (these receive external URLs legitimately)
    if path.contains("/webhook/") || path.contains("/payments/webhook") {
        return Ok(next.run(request).await);
    }

    // Only inspect JSON request bodies for performance
    let content_type = request
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !content_type.contains("application/json") {
        return Ok(next.run(request).await);
    }

    // Extract the body to check for URLs
    let (parts, body) = request.into_parts();
    let body_bytes = match axum::body::to_bytes(body, 10 * 1024 * 1024).await {
        Ok(b) => b,
        // Body parse failed — let the request through (next layer will handle malformed body)
        Err(_) => {
            let req = axum::http::Request::from_parts(parts, axum::body::Body::empty());
            return Ok(next.run(req).await);
        }
    };

    // Check body for suspicious URLs
    if let Ok(body_str) = std::str::from_utf8(&body_bytes) {
        if let Some(blocked) = check_for_ssrf_urls(body_str) {
            warn!(
                path = %path,
                blocked_url = %blocked,
                "SSRF attempt blocked"
            );
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Reconstruct the request
    let request = axum::http::Request::from_parts(parts, axum::body::Body::from(body_bytes));
    Ok(next.run(request).await)
}

/// Check if a string contains any SSRF-attempt URLs.
/// Returns the first blocked URL found, if any.
fn check_for_ssrf_urls(input: &str) -> Option<String> {
    // Look for URLs in the input
    for word in input.split_whitespace() {
        let cleaned = word.trim_matches(|c: char| {
            c == '"' || c == '\'' || c == ',' || c == ')' || c == '(' || c == ']' || c == '['
        });

        // Check hostname-based blocks
        if let Some(url_host) = extract_hostname(cleaned) {
            let host_lower = url_host.to_lowercase();
            for &blocked in BLOCKED_HOSTNAMES {
                if host_lower == blocked || host_lower.ends_with(&format!(".{}", blocked)) {
                    return Some(cleaned.to_string());
                }
            }

            // Try parsing as IP
            if let Ok(ip) = url_host.parse::<IpAddr>() {
                if is_private_ip(ip) {
                    return Some(cleaned.to_string());
                }
            }
        }
    }
    None
}

/// Extract hostname from a URL string
fn extract_hostname(url: &str) -> Option<String> {
    // Handle various URL formats
    let without_protocol = if let Some(stripped) = url.strip_prefix("http://") {
        stripped
    } else if let Some(stripped) = url.strip_prefix("https://") {
        stripped
    } else {
        url
    };

    // Handle userinfo@host (e.g. http://user:pass@evil.com)
    // Always take the part after '@' if present
    let without_userinfo = if let Some(at_pos) = without_protocol.find('@') {
        &without_protocol[at_pos + 1..]
    } else {
        without_protocol
    };

    // Get hostname (before first /, :)
    let host = without_userinfo.split(['/', ':']).next()?.trim();

    if host.is_empty() || host.len() > 253 {
        return None;
    }

    Some(host.to_string())
}

/// Check if an IP address is in a private/reserved range
fn is_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_link_local()
                || v4.is_private()
                || v4.is_unspecified()
                || v4.octets() == [169, 254, 169, 254] // Cloud metadata
        }
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_unicast_link_local()
                || v6.is_unique_local()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_private_ip() {
        // RFC 1918
        assert!(is_private_ip("10.0.0.1".parse().unwrap()));
        assert!(is_private_ip("172.16.0.1".parse().unwrap()));
        assert!(is_private_ip("192.168.1.1".parse().unwrap()));

        // Loopback
        assert!(is_private_ip("127.0.0.1".parse().unwrap()));
        assert!(is_private_ip("::1".parse().unwrap()));

        // Cloud metadata
        assert!(is_private_ip("169.254.169.254".parse().unwrap()));

        // Link-local
        assert!(is_private_ip("169.254.1.1".parse().unwrap()));

        // Public
        assert!(!is_private_ip("8.8.8.8".parse().unwrap()));
        assert!(!is_private_ip("1.1.1.1".parse().unwrap()));
        assert!(!is_private_ip("203.0.113.1".parse().unwrap()));
    }

    #[test]
    fn test_check_for_ssrf_urls() {
        // Blocked
        assert!(
            check_for_ssrf_urls(r#"{"url": "http://169.254.169.254/latest/meta-data/"}"#).is_some()
        );
        assert!(check_for_ssrf_urls(r#"{"callback": "http://192.168.1.1/admin"}"#).is_some());
        assert!(check_for_ssrf_urls(r#"{"host": "http://127.0.0.1:8080"}"#).is_some());
        assert!(check_for_ssrf_urls(r#"{"host": "http://10.0.0.1/admin"}"#).is_some());
        assert!(check_for_ssrf_urls(r#"{"host": "http://localhost/admin"}"#).is_some());
        assert!(check_for_ssrf_urls(
            r#"{"host": "http://metadata.google.internal/computeMetadata"}"#
        )
        .is_some());

        // Allowed
        assert!(check_for_ssrf_urls(r#"{"url": "https://example.com"}"#).is_none());
        assert!(check_for_ssrf_urls(r#"{"url": "https://api.razorpay.com/v1/orders"}"#).is_none());
        assert!(check_for_ssrf_urls(r#"{"url": "https://hooks.slack.com/xxx"}"#).is_none());
    }

    #[test]
    fn test_extract_hostname() {
        assert_eq!(
            extract_hostname("http://example.com/path"),
            Some("example.com".to_string())
        );
        assert_eq!(
            extract_hostname("https://192.168.1.1:8080/api"),
            Some("192.168.1.1".to_string())
        );
        assert_eq!(
            extract_hostname("http://user:pass@evil.com"),
            Some("evil.com".to_string())
        );
        assert_eq!(
            extract_hostname("just-a-hostname"),
            Some("just-a-hostname".to_string())
        );
    }
}
