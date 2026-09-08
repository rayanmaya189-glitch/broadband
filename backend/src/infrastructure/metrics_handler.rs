use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use prometheus::{Encoder, TextEncoder};
use serde::Serialize;
use tracing::debug;

use crate::shared::app_state::SharedState;

/// Require `Authorization: Bearer <token>` on metrics endpoints.
///
/// - When `METRICS_TOKEN` is set, it must match.
/// - In production, a token is REQUIRED (metrics expose business counters).
/// - In development, unset token means the endpoint stays open.
pub fn require_metrics_auth(headers: &HeaderMap) -> Result<(), StatusCode> {
    let is_production = std::env::var("APP_ENV")
        .unwrap_or_default()
        .eq_ignore_ascii_case("production");
    let configured = std::env::var("METRICS_TOKEN").unwrap_or_default();

    if configured.is_empty() {
        if is_production {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        return Ok(());
    }

    let provided = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");
    if provided == configured {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// GET /api/v1/metrics — Prometheus scrape endpoint.
pub async fn metrics_handler(
    headers: HeaderMap,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    if let Err(status) = require_metrics_auth(&headers) {
        return (status, "Metrics auth required".to_string()).into_response();
    }

    let Some(metrics) = &state.metrics else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Metrics not initialized".to_string(),
        )
            .into_response();
    };

    let encoder = TextEncoder::new();
    let m = metrics.read().await;
    let metric_families = m.registry.gather();
    let mut buffer = Vec::new();
    match encoder.encode(&metric_families, &mut buffer) {
        Ok(()) => {
            debug!(bytes = buffer.len(), "Prometheus metrics scraped");
            (
                StatusCode::OK,
                [(
                    axum::http::header::CONTENT_TYPE,
                    "text/plain; version=0.0.4",
                )],
                buffer,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to encode Prometheus metrics");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Metrics encoding failed: {}", e),
            )
                .into_response()
        }
    }
}

/// GET /api/v1/metrics/summary — JSON summary of key metrics for dashboards.
#[derive(Serialize)]
pub struct MetricsSummary {
    pub active_subscriptions: i64,
    pub invoices_generated: u64,
    pub devices_online: i64,
    pub revenue_total: f64,
    pub worker_cycles: u64,
    pub worker_errors: u64,
    pub nats_published: u64,
    pub nats_consumed: u64,
    pub db_pool_active: i64,
    pub db_pool_idle: i64,
}

pub async fn metrics_summary_handler(
    headers: HeaderMap,
    State(state): State<SharedState>,
) -> Result<Json<MetricsSummary>, StatusCode> {
    require_metrics_auth(&headers)?;
    let Some(metrics) = &state.metrics else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };
    let m = metrics.read().await;
    Ok(Json(MetricsSummary {
        active_subscriptions: m.active_subscriptions.get(),
        invoices_generated: m.invoices_generated_total.get(),
        devices_online: m.device_online_count.get(),
        revenue_total: m.revenue_total.get(),
        worker_cycles: {
            let counter = &m.worker_cycles_total;
            let mut total = 0u64;
            for name in &[
                "billing",
                "notification",
                "device_sync",
                "bandwidth",
                "radius",
                "scheduler",
                "monitoring",
                "partition",
            ] {
                if let Ok(c) = counter.get_metric_with_label_values(&[name]) {
                    total += c.get();
                }
            }
            total
        },
        worker_errors: {
            let counter = &m.worker_errors_total;
            let mut total = 0u64;
            for name in &[
                "billing",
                "notification",
                "device_sync",
                "bandwidth",
                "radius",
                "scheduler",
                "monitoring",
                "partition",
            ] {
                if let Ok(c) = counter.get_metric_with_label_values(&[name]) {
                    total += c.get();
                }
            }
            total
        },
        nats_published: m.nats_messages_published.get(),
        nats_consumed: m.nats_messages_consumed.get(),
        db_pool_active: m.db_pool_active.get(),
        db_pool_idle: m.db_pool_idle.get(),
    }))
}
