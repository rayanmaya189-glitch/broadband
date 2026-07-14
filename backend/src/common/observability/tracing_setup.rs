//! Distributed tracing configuration.
//!
//! Provides span creation and context propagation for request tracing.

use tracing::info_span;

/// Create a request span for tracing.
pub fn request_span(method: &str, path: &str) -> tracing::Span {
    info_span!(
        "http_request",
        method = %method,
        path = %path,
    )
}

/// Create a database query span for tracing.
pub fn db_query_span(operation: &str, table: &str) -> tracing::Span {
    info_span!(
        "db_query",
        operation = %operation,
        table = %table,
    )
}

/// Create a NATS event span for tracing.
pub fn nats_event_span(subject: &str) -> tracing::Span {
    info_span!(
        "nats_event",
        subject = %subject,
    )
}

/// Create a business operation span for tracing.
pub fn business_span(operation: &str, entity: &str) -> tracing::Span {
    info_span!(
        "business_operation",
        operation = %operation,
        entity = %entity,
    )
}
