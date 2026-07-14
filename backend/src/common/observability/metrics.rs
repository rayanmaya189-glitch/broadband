//! Metrics collection for the AeroXe Broadband platform.
//!
//! Provides counters, gauges, and histograms for monitoring system health.

use metrics::{counter, gauge, histogram};

/// Record an HTTP request metric.
pub fn record_http_request(method: &str, path: &str, status: u16, duration_ms: f64) {
    counter!(
        "http_requests_total",
        "method" => method.to_string(),
        "path" => path.to_string(),
        "status" => status.to_string(),
    )
    .increment(1);

    histogram!(
        "http_request_duration_ms",
        "method" => method.to_string(),
        "path" => path.to_string(),
    )
    .record(duration_ms);
}

/// Record a database query metric.
pub fn record_db_query(operation: &str, table: &str, duration_ms: f64, success: bool) {
    counter!(
        "db_queries_total",
        "operation" => operation.to_string(),
        "table" => table.to_string(),
        "success" => success.to_string(),
    )
    .increment(1);

    histogram!(
        "db_query_duration_ms",
        "operation" => operation.to_string(),
        "table" => table.to_string(),
    )
    .record(duration_ms);
}

/// Record a NATS publish metric.
pub fn record_nats_publish(subject: &str, success: bool) {
    counter!(
        "nats_publish_total",
        "subject" => subject.to_string(),
        "success" => success.to_string(),
    )
    .increment(1);
}

/// Record a NATS subscribe metric.
pub fn record_nats_subscribe(subject: &str) {
    counter!(
        "nats_subscribe_total",
        "subject" => subject.to_string(),
    )
    .increment(1);
}

/// Record active connections gauge.
pub fn set_active_connections(count: f64) {
    gauge!("active_connections").set(count);
}

/// Record active WebSocket connections.
pub fn set_websocket_connections(count: f64) {
    gauge!("websocket_connections").set(count);
}

/// Record queue size.
pub fn set_queue_size(queue: &str, size: f64) {
    gauge!("queue_size", "queue" => queue.to_string()).set(size);
}

/// Record a business metric.
pub fn record_business_metric(name: &str, value: f64) {
    gauge!("business_metric", "name" => name.to_string()).set(value);
}

/// Record an error counter.
pub fn record_error(error_type: &str, module: &str) {
    counter!(
        "errors_total",
        "type" => error_type.to_string(),
        "module" => module.to_string(),
    )
    .increment(1);
}
