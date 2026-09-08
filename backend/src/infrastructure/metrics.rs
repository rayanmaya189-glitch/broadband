use std::sync::Arc;

use prometheus::{Gauge, IntCounter, IntCounterVec, IntGauge, Registry};
type RwLock<T> = tokio::sync::RwLock<T>;

/// Prometheus metrics for the AeroXe backend per §29 DevOps.
pub struct Metrics {
    pub registry: Registry,

    // HTTP metrics
    pub http_requests_total: IntCounterVec,
    pub http_request_duration_seconds: prometheus::HistogramVec,

    // Database metrics
    pub db_connections_active: IntGauge,
    pub db_health_check_latency_seconds: prometheus::Histogram,

    // Business metrics
    pub active_subscriptions: IntGauge,
    pub invoices_generated_total: IntCounter,
    pub device_online_count: IntGauge,
    pub revenue_total: Gauge,

    // Worker metrics
    pub worker_cycles_total: IntCounterVec,
    pub worker_errors_total: IntCounterVec,

    // NATS metrics
    pub nats_messages_published: IntCounter,
    pub nats_messages_consumed: IntCounter,

    // DB pool stats
    pub db_pool_active: IntGauge,
    pub db_pool_idle: IntGauge,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let http_requests_total = IntCounterVec::new(
            prometheus::opts!("aeroxe_http_requests_total", "Total HTTP requests"),
            &["method", "path", "status"],
        )
        .expect("valid metric name");
        let http_request_duration_seconds = prometheus::HistogramVec::new(
            prometheus::histogram_opts!(
                "aeroxe_http_request_duration_seconds",
                "Request latency in seconds"
            )
            .buckets(vec![0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0, 5.0]),
            &["method", "path", "status"],
        )
        .expect("valid metric name");
        let db_connections_active = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_db_connections_active",
            "Active DB connections"
        ))
        .expect("valid metric name");
        let active_subscriptions = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_active_subscriptions",
            "Current active subscriptions"
        ))
        .expect("valid metric name");
        let invoices_generated_total = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_invoices_generated_total",
            "Total invoices generated"
        ))
        .expect("valid metric name");
        let device_online_count = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_device_online_count",
            "Online device count"
        ))
        .expect("valid metric name");
        let revenue_total = Gauge::with_opts(prometheus::opts!(
            "aeroxe_revenue_total",
            "Total revenue in INR"
        ))
        .expect("valid metric name");
        let worker_cycles_total = IntCounterVec::new(
            prometheus::opts!("aeroxe_worker_cycles_total", "Total worker cycles"),
            &["worker"],
        )
        .expect("valid metric name");
        let worker_errors_total = IntCounterVec::new(
            prometheus::opts!("aeroxe_worker_errors_total", "Worker cycle errors"),
            &["worker"],
        )
        .expect("valid metric name");
        let nats_messages_published = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_nats_messages_published",
            "NATS messages published"
        ))
        .expect("valid metric name");
        let nats_messages_consumed = IntCounter::with_opts(prometheus::opts!(
            "aeroxe_nats_messages_consumed",
            "NATS messages consumed"
        ))
        .expect("valid metric name");

        registry
            .register(Box::new(http_requests_total.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(http_request_duration_seconds.clone()))
            .expect("metric already registered");
        let db_health_check_latency_seconds = prometheus::Histogram::with_opts(
            prometheus::histogram_opts!(
                "aeroxe_db_health_check_latency_seconds",
                "Database health check latency in seconds"
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]),
        )
        .expect("valid metric name");

        registry
            .register(Box::new(db_connections_active.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(db_health_check_latency_seconds.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(active_subscriptions.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(invoices_generated_total.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(device_online_count.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(revenue_total.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(worker_cycles_total.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(worker_errors_total.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(nats_messages_published.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(nats_messages_consumed.clone()))
            .expect("metric already registered");

        let db_pool_active = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_db_pool_active",
            "Active connections in the DB pool"
        ))
        .expect("valid metric name");
        let db_pool_idle = IntGauge::with_opts(prometheus::opts!(
            "aeroxe_db_pool_idle",
            "Idle connections in the DB pool"
        ))
        .expect("valid metric name");
        registry
            .register(Box::new(db_pool_active.clone()))
            .expect("metric already registered");
        registry
            .register(Box::new(db_pool_idle.clone()))
            .expect("metric already registered");

        Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
            db_connections_active,
            db_health_check_latency_seconds,
            active_subscriptions,
            invoices_generated_total,
            device_online_count,
            revenue_total,
            worker_cycles_total,
            worker_errors_total,
            nats_messages_published,
            nats_messages_consumed,
            db_pool_active,
            db_pool_idle,
        }
    }
}

/// Shared metrics reference.
pub type SharedMetrics = Arc<RwLock<Metrics>>;

/// Create shared metrics instance.
pub fn create_metrics() -> SharedMetrics {
    Arc::new(RwLock::new(Metrics::new()))
}
