/// OpenAPI schemas and stub handlers for Monitoring endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct MetricsQuery {
    /// Filter by device ID
    #[serde(default)]
    pub device_id: Option<i64>,
    /// Filter by metric name (e.g. cpu_usage, bandwidth)
    #[serde(default)]
    pub metric_name: Option<String>,
    /// Max records to return (default 100, max 500)
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AlertsQuery {
    /// Filter by severity (critical, high, medium, low)
    #[serde(default)]
    pub severity: Option<String>,
    /// Filter by status (firing, acknowledged, resolved)
    #[serde(default)]
    pub status: Option<String>,
    /// Filter by branch ID
    #[serde(default)]
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAlertRequest {
    /// Device ID the alert is for
    pub device_id: i64,
    /// Branch ID
    pub branch_id: i64,
    /// Severity level (critical, high, medium, low)
    pub severity: String,
    /// Alert title
    pub title: String,
    /// Alert message
    pub message: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AcknowledgeAlertRequest {
    /// User ID acknowledging the alert
    pub user_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ResolveAlertRequest {
    /// User ID resolving the alert
    pub user_id: i64,
    /// Resolution notes
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MetricRecordResponse {
    /// Metric record ID
    pub id: i64,
    /// Device ID
    pub device_id: i64,
    /// Branch ID
    pub branch_id: i64,
    /// Metric name
    pub metric_name: String,
    /// Metric value
    pub metric_value: f64,
    /// Unit of measurement
    pub unit: String,
    /// Timestamp when recorded
    pub recorded_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MetricsResponse {
    /// List of metric records
    pub metrics: Vec<MetricRecordResponse>,
    /// Total number of records returned
    pub total: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AlertResponse {
    /// Alert ID
    pub id: i64,
    /// Device ID
    pub device_id: i64,
    /// Branch ID
    pub branch_id: i64,
    /// Severity level
    pub severity: String,
    /// Alert status
    pub status: String,
    /// Alert title
    pub title: String,
    /// Alert message
    pub message: String,
    /// User who acknowledged
    pub acknowledged_by: Option<i64>,
    /// User who resolved
    pub resolved_by: Option<i64>,
    /// Resolution timestamp
    pub resolved_at: Option<String>,
    /// Creation timestamp
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AlertsResponse {
    /// List of alerts
    pub alerts: Vec<AlertResponse>,
    /// Total number of alerts
    pub total: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AlertStatsResponse {
    /// Total active alerts
    pub total_active: i64,
    /// Critical severity count
    pub critical: i64,
    /// High severity count
    pub high: i64,
    /// Medium severity count
    pub medium: i64,
    /// Low severity count
    pub low: i64,
    /// Alerts grouped by branch ID
    pub by_branch: std::collections::HashMap<i64, i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthSummaryResponse {
    /// Overall system health status
    pub status: String,
    /// Number of active devices
    pub active_devices: i64,
    /// Number of active alerts
    pub active_alerts: i64,
    /// System uptime percentage
    pub uptime_percentage: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IncidentResponse {
    /// Incident ID
    pub id: i64,
    /// Incident title
    pub title: String,
    /// Incident severity
    pub severity: String,
    /// Incident status
    pub status: String,
    /// Affected branch IDs
    pub affected_branches: Vec<i64>,
    /// Creation timestamp
    pub created_at: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List metrics with optional device and name filters
#[utoipa::path(
    get,
    path = "/api/v1/monitoring/metrics",
    tag = "Monitoring",
    params(
        ("device_id" = Option<i64>, Query, description = "Filter by device ID"),
        ("metric_name" = Option<String>, Query, description = "Filter by metric name"),
        ("limit" = Option<i64>, Query, description = "Max records (default 100, max 500)"),
    ),
    responses(
        (status = 200, description = "List of metric records", body = MetricsResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_metrics() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get metrics for a specific device
#[utoipa::path(
    get,
    path = "/api/v1/monitoring/metrics/{device_id}",
    tag = "Monitoring",
    params(
        ("device_id" = i64, Path, description = "Device ID"),
        ("metric_name" = Option<String>, Query, description = "Filter by metric name"),
        ("limit" = Option<i64>, Query, description = "Max records (default 100, max 500)"),
    ),
    responses(
        (status = 200, description = "Device metrics", body = MetricsResponse),
        (status = 404, description = "Device not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_device_metrics() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List monitoring alerts with optional filters
#[utoipa::path(
    get,
    path = "/api/v1/monitoring/alerts",
    tag = "Monitoring",
    params(
        ("severity" = Option<String>, Query, description = "Filter by severity"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("branch_id" = Option<i64>, Query, description = "Filter by branch ID"),
    ),
    responses(
        (status = 200, description = "List of alerts", body = AlertsResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_alerts() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get alert statistics across the system
#[utoipa::path(
    get,
    path = "/api/v1/monitoring/alerts/stats",
    tag = "Monitoring",
    responses(
        (status = 200, description = "Alert statistics", body = AlertStatsResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_alert_stats() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new monitoring alert
#[utoipa::path(
    post,
    path = "/api/v1/monitoring/alerts",
    tag = "Monitoring",
    request_body = CreateAlertRequest,
    responses(
        (status = 201, description = "Alert created", body = AlertResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_alert() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Acknowledge a monitoring alert
#[utoipa::path(
    post,
    path = "/api/v1/monitoring/alerts/{id}/acknowledge",
    tag = "Monitoring",
    params(("id" = i64, Path, description = "Alert ID")),
    request_body = AcknowledgeAlertRequest,
    responses(
        (status = 200, description = "Alert acknowledged"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Alert not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn acknowledge_alert() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Resolve a monitoring alert
#[utoipa::path(
    post,
    path = "/api/v1/monitoring/alerts/{id}/resolve",
    tag = "Monitoring",
    params(("id" = i64, Path, description = "Alert ID")),
    request_body = ResolveAlertRequest,
    responses(
        (status = 200, description = "Alert resolved"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Alert not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn resolve_alert() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get system health summary
#[utoipa::path(
    get,
    path = "/api/v1/monitoring/health",
    tag = "Monitoring",
    responses(
        (status = 200, description = "Health summary", body = HealthSummaryResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_health_summary() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List system incidents
#[utoipa::path(
    get,
    path = "/api/v1/monitoring/incidents",
    tag = "Monitoring",
    responses(
        (status = 200, description = "List of incidents"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_incidents() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
