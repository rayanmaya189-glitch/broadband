/// OpenAPI schemas and stub handlers for Audit endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request Types ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct HistoryQuery {
    /// Filter by entity ID
    #[serde(default)]
    pub entity_id: Option<String>,
    /// Filter by action type (create, update, delete)
    #[serde(default)]
    pub action: Option<String>,
    /// Filter by user ID
    #[serde(default)]
    pub user_id: Option<i64>,
    /// Filter from date
    #[serde(default)]
    pub from: Option<String>,
    /// Filter to date
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RollbackRequest {
    /// History entry ID to rollback to
    pub history_id: String,
    /// Reason for the rollback
    pub reason: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CompareQuery {
    /// Entity type (e.g. "customer", "subscription")
    pub entity_type: String,
    /// Entity ID
    pub entity_id: String,
    /// First version hash to compare
    pub version_a: String,
    /// Second version hash to compare
    pub version_b: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExportHistoryQuery {
    /// Entity type to export history for
    pub entity_type: String,
    /// Entity ID
    pub entity_id: String,
    /// Export format (json, csv)
    #[serde(default)]
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuditLogQuery {
    /// Filter by user ID
    #[serde(default)]
    pub user_id: Option<i64>,
    /// Filter by action
    #[serde(default)]
    pub action: Option<String>,
    /// Filter by resource type
    #[serde(default)]
    pub resource_type: Option<String>,
    /// Filter by result (success, failure)
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EventQuery {
    /// Filter by event type
    #[serde(default)]
    pub event_type: Option<String>,
    /// Filter by aggregate type
    #[serde(default)]
    pub aggregate_type: Option<String>,
    /// Filter by status (published, pending, dead_letter)
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EventExportQuery {
    /// Filter by event type
    #[serde(default)]
    pub event_type: Option<String>,
    /// Filter from datetime
    #[serde(default)]
    pub from: Option<String>,
    /// Filter to datetime
    #[serde(default)]
    pub to: Option<String>,
    /// Export format
    #[serde(default)]
    pub format: Option<String>,
}

// ── Response Types ───────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct HistoryEntryResponse {
    /// History entry ID
    pub history_id: String,
    /// Entity type
    pub entity_type: String,
    /// Entity ID
    pub entity_id: String,
    /// Action performed (create, update, delete)
    pub action: String,
    /// User who performed the action
    pub user_id: Option<i64>,
    /// Timestamp of the change
    pub timestamp: String,
    /// Snapshot of the entity after the change
    pub snapshot: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HistoryComparisonResponse {
    /// Entity type
    pub entity_type: String,
    /// Entity ID
    pub entity_id: String,
    /// Differences between version A and version B
    pub diffs: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuditLogResponse {
    pub id: i64,
    pub user_id: Option<i64>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<i64>,
    pub result: String,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EventResponse {
    pub id: i64,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: serde_json::Value,
    pub published: bool,
    pub dead_letter: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EventReplayResult {
    /// Status of the replay
    pub status: String,
    /// Event ID that was replayed
    pub event_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RollbackResult {
    /// Rollback status
    pub status: String,
    /// New history ID created by the rollback
    pub history_id: String,
    /// Entity type rolled back
    pub entity_type: String,
    /// Entity ID rolled back
    pub entity_id: String,
    /// Version restored from
    pub restored_from: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse {
    pub data: serde_json::Value,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// Search entity history with filters
#[utoipa::path(
    get,
    path = "/api/v1/audit/history/{entity_type}",
    tag = "Audit",
    params(
        ("entity_type" = String, Path, description = "Entity type (customer, subscription, etc.)")
    ),
    params(
        ("entity_id" = Option<String>, Query, description = "Filter by entity ID"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("user_id" = Option<i64>, Query, description = "Filter by user ID"),
        ("from" = Option<String>, Query, description = "From date"),
        ("to" = Option<String>, Query, description = "To date"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Paginated history entries"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn search_history() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a specific history entry
#[utoipa::path(
    get,
    path = "/api/v1/audit/history/{entity_type}/{history_id}",
    tag = "Audit",
    params(
        ("entity_type" = String, Path, description = "Entity type"),
        ("history_id" = String, Path, description = "History entry ID")
    ),
    responses(
        (status = 200, description = "History entry details", body = HistoryEntryResponse),
        (status = 404, description = "History entry not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_history_entry() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Rollback an entity to a previous version
#[utoipa::path(
    post,
    path = "/api/v1/audit/rollback/{entity_type}/{entity_id}",
    tag = "Audit",
    params(
        ("entity_type" = String, Path, description = "Entity type"),
        ("entity_id" = String, Path, description = "Entity ID")
    ),
    request_body = RollbackRequest,
    responses(
        (status = 200, description = "Entity rolled back", body = RollbackResult),
        (status = 403, description = "Only administrators can perform rollbacks"),
        (status = 404, description = "Entity or history entry not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn rollback_entity() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List allowed entity types for history tracking
#[utoipa::path(
    get,
    path = "/api/v1/audit/entity-types",
    tag = "Audit",
    responses(
        (status = 200, description = "List of allowed entity types", body = Vec<String>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_entity_types() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Compare two versions of an entity
#[utoipa::path(
    get,
    path = "/api/v1/audit/history/compare",
    tag = "Audit",
    params(
        ("entity_type" = String, Query, description = "Entity type"),
        ("entity_id" = String, Query, description = "Entity ID"),
        ("version_a" = String, Query, description = "First version hash"),
        ("version_b" = String, Query, description = "Second version hash")
    ),
    responses(
        (status = 200, description = "Version comparison", body = HistoryComparisonResponse),
        (status = 404, description = "Version not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn compare_history() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Export entity history
#[utoipa::path(
    get,
    path = "/api/v1/audit/history/export",
    tag = "Audit",
    params(
        ("entity_type" = String, Query, description = "Entity type"),
        ("entity_id" = String, Query, description = "Entity ID"),
        ("format" = Option<String>, Query, description = "Export format (json, csv)")
    ),
    responses(
        (status = 200, description = "Exported history data"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn export_history() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Search audit logs with filters
#[utoipa::path(
    get,
    path = "/api/v1/audit/logs",
    tag = "Audit",
    params(
        ("user_id" = Option<i64>, Query, description = "Filter by user ID"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("resource_type" = Option<String>, Query, description = "Filter by resource type"),
        ("result" = Option<String>, Query, description = "Filter by result"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Paginated audit logs"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn search_audit_logs() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a specific audit log entry
#[utoipa::path(
    get,
    path = "/api/v1/audit/logs/{id}",
    tag = "Audit",
    params(("id" = i64, Path, description = "Audit log ID")),
    responses(
        (status = 200, description = "Audit log details", body = AuditLogResponse),
        (status = 404, description = "Audit log not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_audit_log() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get user activity log
#[utoipa::path(
    get,
    path = "/api/v1/audit/user/{user_id}",
    tag = "Audit",
    params(("user_id" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User activity logs", body = Vec<AuditLogResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_user_activity() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Export audit logs as JSON (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/audit/export",
    tag = "Audit",
    params(
        ("user_id" = Option<i64>, Query, description = "Filter by user ID"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("resource_type" = Option<String>, Query, description = "Filter by resource type"),
        ("result" = Option<String>, Query, description = "Filter by result")
    ),
    responses(
        (status = 200, description = "Exported audit logs"),
        (status = 403, description = "Only administrators can export audit logs")
    ),
    security(("bearer_auth" = []))
)]
pub async fn export_audit_logs() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List events from the outbox
#[utoipa::path(
    get,
    path = "/api/v1/audit/events",
    tag = "Audit",
    params(
        ("event_type" = Option<String>, Query, description = "Filter by event type"),
        ("aggregate_type" = Option<String>, Query, description = "Filter by aggregate type"),
        ("status" = Option<String>, Query, description = "Filter by status (published, pending, dead_letter)"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Paginated event list"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_events() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Export events matching criteria
#[utoipa::path(
    get,
    path = "/api/v1/audit/events/export",
    tag = "Audit",
    params(
        ("event_type" = Option<String>, Query, description = "Filter by event type"),
        ("from" = Option<String>, Query, description = "From datetime"),
        ("to" = Option<String>, Query, description = "To datetime"),
        ("format" = Option<String>, Query, description = "Export format")
    ),
    responses(
        (status = 200, description = "Exported events"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn export_events() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Replay a single event by resetting it to pending
#[utoipa::path(
    post,
    path = "/api/v1/audit/events/{id}/replay",
    tag = "Audit",
    params(("id" = i64, Path, description = "Event ID")),
    responses(
        (status = 200, description = "Event replayed", body = EventReplayResult),
        (status = 404, description = "Event not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn replay_event() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
