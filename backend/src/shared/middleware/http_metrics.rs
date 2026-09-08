//! HTTP request metrics middleware for Prometheus.
//!
//! Records every request into `http_requests_total` (counter) and
//! `http_request_duration_seconds` (histogram), labeled by method, status
//! code, and a normalised path.
//!
//! The path is normalised by stripping numeric IDs (e.g. `/customers/42`
//! → `/customers/:id`) to prevent label cardinality explosion.

use std::time::Instant;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::debug;

/// Middleware that records request metrics to Prometheus.
///
/// Expects `AppState` to be available via `axum::extract::State`.
/// Placed inside the router's layer stack *after* branch_scope so that the
/// response status is available.
pub async fn http_metrics_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    // Clone the metrics reference BEFORE the request is moved into next.run()
    let metrics = request
        .extensions()
        .get::<crate::infrastructure::metrics::SharedMetrics>()
        .cloned();

    let response = next.run(request).await;

    let duration_secs = start.elapsed().as_secs_f64();
    let status = response.status();

    // Only record if metrics are initialised
    if let Some(metrics) = metrics {
        let normalised = normalise_path(&path);
        let status_str = status.as_u16().to_string();
        let labels: [&str; 3] = [method.as_str(), normalised.as_str(), status_str.as_str()];

        let m = metrics.read().await;
        m.http_requests_total.with_label_values(&labels).inc();
        m.http_request_duration_seconds
            .with_label_values(&labels)
            .observe(duration_secs);
    } else {
        debug!(
            method = %method,
            path = %path,
            status = status.as_u16(),
            duration_ms = (duration_secs * 1000.0) as u64,
            "HTTP request (metrics not initialised)"
        );
    }

    response
}

/// Normalise a URI path to prevent label cardinality explosion.
///
/// Rules:
/// - Trailing numeric segments become `:id`
/// - Fixed segments like `me`, `overdue`, `my-assignments` are kept as-is
///   (they are semantic, not user-supplied)
fn normalise_path(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();
    let mut result = Vec::with_capacity(segments.len());

    for segment in &segments {
        if segment.is_empty() {
            continue;
        }

        // Keep well-known fixed segments
        if matches!(
            *segment,
            "me" | "overdue"
                | "my-assignments"
                | "hierarchy"
                | "stats"
                | "list"
                | "search"
                | "history"
                | "topology"
                | "summary"
                | "pending"
                | "export"
                | "approve"
                | "unapprove"
                | "pipeline"
                | "auto-generate"
                | "retry"
                | "channels"
                | "entity-types"
                | "compare"
                | "lease"
                | "leases"
                | "sessions"
        ) {
            result.push(segment.to_string());
            continue;
        }

        // If the segment looks like a numeric ID, normalise it
        if segment.chars().all(|c| c.is_ascii_digit()) {
            result.push(":id".to_string());
            continue;
        }

        // Check if the segment contains a numeric suffix after a known prefix
        // e.g. "comments", "items", "ports", "address", "users"
        // These are sub-resource identifiers and should be normalised too
        if let Some(prefix) = segment.rsplit_once('-').map(|(p, _)| p) {
            if !prefix.is_empty()
                && prefix
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-')
            {
                result.push(segment.to_string());
                continue;
            }
        }

        result.push(segment.to_string());
    }

    format!("/{}", result.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise_path_numeric_id() {
        assert_eq!(
            normalise_path("/api/v1/customers/42"),
            "/api/v1/customers/:id"
        );
        assert_eq!(normalise_path("/api/v1/plans/123"), "/api/v1/plans/:id");
        assert_eq!(
            normalise_path("/api/v1/billing/invoices/456"),
            "/api/v1/billing/invoices/:id"
        );
    }

    #[test]
    fn test_normalise_path_fixed_segments() {
        assert_eq!(
            normalise_path("/api/v1/customers/me"),
            "/api/v1/customers/me"
        );
        assert_eq!(
            normalise_path("/api/v1/billing/invoices/overdue"),
            "/api/v1/billing/invoices/overdue"
        );
        assert_eq!(
            normalise_path("/api/v1/customers/list"),
            "/api/v1/customers/list"
        );
    }

    #[test]
    fn test_normalise_path_no_id() {
        assert_eq!(normalise_path("/api/v1/customers"), "/api/v1/customers");
        assert_eq!(normalise_path("/api/v1/plans"), "/api/v1/plans");
    }

    #[test]
    fn test_normalise_path_nested_ids() {
        assert_eq!(
            normalise_path("/api/v1/network/ip-pools/42/allocate"),
            "/api/v1/network/ip-pools/:id/allocate"
        );
        assert_eq!(
            normalise_path("/api/v1/tickets/55/comments"),
            "/api/v1/tickets/:id/comments"
        );
    }
}
