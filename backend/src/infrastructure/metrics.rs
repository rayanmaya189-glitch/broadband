use std::sync::Arc;

use prometheus::{Gauge, IntCounter, IntGauge, Registry};
type RwLock<T> = tokio::sync::RwLock<T>;

/// Prometheus metrics for the AeroXe backend per §29 DevOps.
pub struct Metrics {
    pub registry: Registry,

    // HTTP metrics
    pub http_requests_total: IntCounter,
    pub http_request_duration_seconds: prometheus::Histogram,

    // Database metrics
    pub db_connections_active: IntGauge,

    // Business metrics
    pub active_subscriptions: IntGauge,
    pub invoices_generated_total: IntCounter,
    pub device_online_count: IntGauge,
    pub revenue_total: Gauge,

    // Worker metrics
    pub worker_cycles_total: IntCounter,
    pub worker_errors_total: IntCounter,

    // NATS metrics
    pub nats_messages_published: IntCounter,
    pub nats_messages_consumed: IntCounter,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let http_requests_total = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_http_requests_total",
            "Total HTTP requests"
        ))
        .unwrap();
        let http_request_duration_seconds = prometheus::Histogram::with_opts(
            prometheus::histogram_opts!(
                "aeroxe_http_request_duration_seconds",
                "Request latency in seconds"
            )
            .buckets(vec![0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0, 5.0]),
        )
        .unwrap();
        let db_connections_active = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_db_connections_active",
            "Active DB connections"
        ))
        .unwrap();
        let active_subscriptions = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_active_subscriptions",
            "Current active subscriptions"
        ))
        .unwrap();
        let invoices_generated_total = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_invoices_generated_total",
            "Total invoices generated"
        ))
        .unwrap();
        let device_online_count = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_device_online_count",
            "Online device count"
        ))
        .unwrap();
        let revenue_total = Gauge::with_opts(prometheus::opts!(
            "aeroxe_revenue_total",
            "Total revenue in INR"
        ))
        .unwrap();
        let worker_cycles_total = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_worker_cycles_total",
            "Total worker cycles"
        ))
        .unwrap();
        let worker_errors_total = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_worker_errors_total",
            "Worker cycle errors"
        ))
        .unwrap();
        let nats_messages_published = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_nats_messages_published",
            "NATS messages published"
        ))
        .unwrap();
        let nats_messages_consumed = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_nats_messages_consumed",
            "NATS messages consumed"
        ))
        .unwrap();

        registry
            .register(Box::new(http_requests_total.clone()))
            .unwrap();
        registry
            .register(Box::new(http_request_duration_seconds.clone()))
            .unwrap();
        registry
            .register(Box::new(db_connections_active.clone()))
            .unwrap();
        registry
            .register(Box::new(active_subscriptions.clone()))
            .unwrap();
        registry
            .register(Box::new(invoices_generated_total.clone()))
            .unwrap();
        registry
            .register(Box::new(device_online_count.clone()))
            .unwrap();
        registry.register(Box::new(revenue_total.clone())).unwrap();
        registry
            .register(Box::new(worker_cycles_total.clone()))
            .unwrap();
        registry
            .register(Box::new(worker_errors_total.clone()))
            .unwrap();
        registry
            .register(Box::new(nats_messages_published.clone()))
            .unwrap();
        registry
            .register(Box::new(nats_messages_consumed.clone()))
            .unwrap();

        Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
            db_connections_active,
            active_subscriptions,
            invoices_generated_total,
            device_online_count,
            revenue_total,
            worker_cycles_total,
            worker_errors_total,
            nats_messages_published,
            nats_messages_consumed,
        }
    }
}

/// Shared metrics reference.
pub type SharedMetrics = Arc<RwLock<Metrics>>;

/// Create shared metrics instance.
pub fn create_metrics() -> SharedMetrics {
    Arc::new(RwLock::new(Metrics::new()))
}
