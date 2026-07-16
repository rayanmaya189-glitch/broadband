use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use prometheus::{Encoder, TextEncoder};
use serde::Serialize;
use tracing::debug;

use crate::shared::app_state::SharedState;

/// GET /api/v1/metrics — Prometheus scrape endpoint.
pub async fn metrics_handler(
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let Some(metrics) = &state.metrics else {
        return (StatusCode::SERVICE_UNAVAILABLE, "Metrics not initialized".to_string()).into_response();
    };

    let encoder = TextEncoder::new();
    let m = metrics.read().await;
    let metric_families = m.registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    debug!(bytes = buffer.len(), "Prometheus metrics scraped");

    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        buffer,
    ).into_response()
}

/// GET /api/v1/metrics/summary — JSON summary of key metrics for dashboards.
#[derive(Serialize)]
pub struct MetricsSummary {
    pub http_requests_total: u64,
    pub active_subscriptions: i64,
    pub invoices_generated: u64,
    pub devices_online: i64,
    pub revenue_total: f64,
    pub worker_cycles: u64,
    pub worker_errors: u64,
    pub nats_published: u64,
    pub nats_consumed: u64,
}

pub async fn metrics_summary_handler(
    State(state): State<SharedState>,
) -> Result<Json<MetricsSummary>, StatusCode> {
    let Some(metrics) = &state.metrics else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };
    let m = metrics.read().await;
    Ok(Json(MetricsSummary {
        http_requests_total: m.http_requests_total.get(),
        active_subscriptions: m.active_subscriptions.get(),
        invoices_generated: m.invoices_generated_total.get(),
        devices_online: m.device_online_count.get(),
        revenue_total: m.revenue_total.get(),
        worker_cycles: m.worker_cycles_total.get(),
        worker_errors: m.worker_errors_total.get(),
        nats_published: m.nats_messages_published.get(),
        nats_consumed: m.nats_messages_consumed.get(),
    }))
}
