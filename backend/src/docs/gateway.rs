/// OpenAPI schemas and stub handlers for Gateway endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

// ── Rate Limit Rules ──

#[derive(Debug, Serialize, ToSchema)]
pub struct RateLimitRuleResponse {
    /// Rule ID
    pub id: i64,
    /// Route pattern (e.g. "/api/v1/*")
    pub route_pattern: String,
    /// HTTP methods (e.g. "GET,POST")
    pub methods: String,
    /// Maximum requests per window
    pub max_requests: i32,
    /// Window size in seconds
    pub window_seconds: i32,
    /// Optional role restriction
    pub role: Option<String>,
    /// Whether the rule is active
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRateLimitRuleRequest {
    /// Route pattern
    pub route_pattern: String,
    /// HTTP methods
    pub methods: String,
    /// Maximum requests per window
    pub max_requests: i32,
    /// Window size in seconds
    pub window_seconds: i32,
    /// Optional role restriction
    #[serde(default)]
    pub role: Option<String>,
    /// Optional branch ID
    #[serde(default)]
    pub branch_id: Option<i64>,
}

// ── API Keys ──

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyResponse {
    /// API key ID
    pub id: i64,
    /// Key name
    pub name: String,
    /// Key prefix (first chars, for identification)
    pub key_prefix: String,
    /// Permissions string
    pub permissions: String,
    /// Whether the key is active
    pub is_active: bool,
    /// Expiry timestamp
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateApiKeyRequest {
    /// Key name
    pub name: String,
    /// Permissions string
    pub permissions: String,
    /// Optional branch ID
    #[serde(default)]
    pub branch_id: Option<i64>,
    /// Optional expiry (RFC 3339)
    #[serde(default)]
    pub expires_at: Option<String>,
}

// ── Request Logs ──

#[derive(Debug, Serialize, ToSchema)]
pub struct RequestLogResponse {
    /// Log entry ID
    pub id: i64,
    /// HTTP method
    pub method: String,
    /// Request path
    pub path: String,
    /// Response status code
    pub status_code: i32,
    /// Response time in milliseconds
    pub response_time_ms: i32,
    /// Whether request was rate-limited
    pub rate_limited: bool,
    /// Timestamp
    pub created_at: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

// ── Rate Limit Rules ──

/// List all rate limit rules (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/gateway/rate-limit-rules",
    tag = "Gateway",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "List of rate limit rules"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_rate_limit_rules() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new rate limit rule
#[utoipa::path(
    post,
    path = "/api/v1/gateway/rate-limit-rules",
    tag = "Gateway",
    request_body = CreateRateLimitRuleRequest,
    responses(
        (status = 201, description = "Rate limit rule created", body = RateLimitRuleResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_rate_limit_rule() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a rate limit rule
#[utoipa::path(
    delete,
    path = "/api/v1/gateway/rate-limit-rules/{id}",
    tag = "Gateway",
    params(("id" = i64, Path, description = "Rule ID")),
    responses(
        (status = 204, description = "Rule deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Rule not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_rate_limit_rule() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

// ── API Keys ──

/// List all API keys (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/gateway/api-keys",
    tag = "Gateway",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "List of API keys"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_api_keys() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new API key
#[utoipa::path(
    post,
    path = "/api/v1/gateway/api-keys",
    tag = "Gateway",
    request_body = CreateApiKeyRequest,
    responses(
        (status = 201, description = "API key created (raw key returned once)", body = ApiKeyResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_api_key() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Revoke an API key
#[utoipa::path(
    delete,
    path = "/api/v1/gateway/api-keys/{id}",
    tag = "Gateway",
    params(("id" = i64, Path, description = "API key ID")),
    responses(
        (status = 204, description = "API key revoked"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "API key not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn revoke_api_key() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

// ── Request Logs & Stats ──

/// List recent request logs (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/gateway/logs",
    tag = "Gateway",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "List of request logs"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_request_logs() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get aggregated request statistics
#[utoipa::path(
    get,
    path = "/api/v1/gateway/stats",
    tag = "Gateway",
    responses(
        (status = 200, description = "Request statistics"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_request_stats() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
