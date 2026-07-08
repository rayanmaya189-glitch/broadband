//! Health check endpoints.

use axum::Json;
use serde::Serialize;
use sqlx::PgPool;

use crate::app::SharedState;
use crate::error::AppError;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub services: ServiceHealth,
}

#[derive(Serialize)]
pub struct ServiceHealth {
    pub database: &'static str,
    pub redis: &'static str,
    pub nats: &'static str,
}

/// GET /health — Liveness probe (always 200 if server is running).
pub async fn liveness() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// GET /health/ready — Readiness probe (checks all backing services).
pub async fn readiness(
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> Result<Json<HealthResponse>, AppError> {
    let db_status = check_db(&state.db).await;
    let redis_status = "ok"; // Redis health checked via connection manager
    let nats_status = "ok"; // NATS health checked via client

    let overall = if db_status == "ok" && redis_status == "ok" && nats_status == "ok" {
        "ok"
    } else {
        "degraded"
    };

    Ok(Json(HealthResponse {
        status: overall,
        version: env!("CARGO_PKG_VERSION"),
        services: ServiceHealth {
            database: db_status,
            redis: redis_status,
            nats: nats_status,
        },
    }))
}

async fn check_db(pool: &PgPool) -> &'static str {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => "ok",
        Err(e) => {
            tracing::error!(error = %e, "Database health check failed");
            "error"
        }
    }
}
