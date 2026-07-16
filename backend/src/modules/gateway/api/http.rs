use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::modules::gateway::application::services::GatewayService;

// ── Rate Limit Rules ──

#[derive(Debug, Serialize)]
pub struct RateLimitRuleResponse {
    pub id: i64,
    pub route_pattern: String,
    pub methods: String,
    pub max_requests: i32,
    pub window_seconds: i32,
    pub role: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateRateLimitRuleRequest {
    pub route_pattern: String,
    pub methods: String,
    pub max_requests: i32,
    pub window_seconds: i32,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub branch_id: Option<i64>,
}

pub async fn list_rate_limit_rules(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<RateLimitRuleResponse>>, AppError> {
    let rules = GatewayService::list_rate_limit_rules(&state.db).await?;
    Ok(Json(rules.into_iter().map(|r| RateLimitRuleResponse {
        id: r.id,
        route_pattern: r.route_pattern,
        methods: r.methods,
        max_requests: r.max_requests,
        window_seconds: r.window_seconds,
        role: r.role,
        is_active: r.is_active,
    }).collect()))
}

pub async fn create_rate_limit_rule(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateRateLimitRuleRequest>,
) -> Result<(StatusCode, Json<RateLimitRuleResponse>), AppError> {
    require_permission(&user, "gateway.ratelimit.create").map_err(|e| AppError::Forbidden(e.1))?;
    let rule = GatewayService::create_rate_limit_rule(
        &state.db, req.route_pattern, req.methods, req.max_requests,
        req.window_seconds, req.role, req.branch_id,
    ).await?;
    Ok((StatusCode::CREATED, Json(RateLimitRuleResponse {
        id: rule.id, route_pattern: rule.route_pattern, methods: rule.methods,
        max_requests: rule.max_requests, window_seconds: rule.window_seconds,
        role: rule.role, is_active: rule.is_active,
    })))
}

pub async fn delete_rate_limit_rule(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    user: UserContext,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "gateway.ratelimit.delete").map_err(|e| AppError::Forbidden(e.1))?;
    GatewayService::delete_rate_limit_rule(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ── API Keys ──

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: i64,
    pub name: String,
    pub key_prefix: String,
    pub permissions: String,
    pub is_active: bool,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: String,
    #[serde(default)]
    pub branch_id: Option<i64>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

pub async fn list_api_keys(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<ApiKeyResponse>>, AppError> {
    let keys = GatewayService::list_api_keys(&state.db).await?;
    Ok(Json(keys.into_iter().map(|k| ApiKeyResponse {
        id: k.id, name: k.name, key_prefix: k.key_prefix,
        permissions: k.permissions, is_active: k.is_active,
        expires_at: k.expires_at.map(|e| e.to_rfc3339()),
    }).collect()))
}

pub async fn create_api_key(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiKeyResponse>), AppError> {
    require_permission(&user, "gateway.apikey.create").map_err(|e| AppError::Forbidden(e.1))?;
    // Generate a random API key
    let raw_key = format!("ax_{}_{}", uuid::Uuid::new_v4().to_string().replace("-", ""), chrono::Utc::now().timestamp());
    let key_hash = format!("{:x}", md5::compute(raw_key.as_bytes()));
    let key_prefix = raw_key[..12].to_string();

    let expires_at = req.expires_at
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let key = GatewayService::create_api_key(
        &state.db, req.name, key_hash, key_prefix,
        req.branch_id, req.permissions, expires_at,
    ).await?;

    Ok((StatusCode::CREATED, Json(ApiKeyResponse {
        id: key.id, name: key.name, key_prefix: key.key_prefix,
        permissions: key.permissions, is_active: key.is_active,
        expires_at: key.expires_at.map(|e| e.to_rfc3339()),
    })))
}

pub async fn revoke_api_key(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    user: UserContext,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "gateway.apikey.revoke").map_err(|e| AppError::Forbidden(e.1))?;
    GatewayService::revoke_api_key(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ── Request Logs & Stats ──

#[derive(Debug, Serialize)]
pub struct RequestLogResponse {
    pub id: i64,
    pub method: String,
    pub path: String,
    pub status_code: i32,
    pub response_time_ms: i32,
    pub rate_limited: bool,
    pub created_at: String,
}

pub async fn list_request_logs(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<RequestLogResponse>>, AppError> {
    let logs = GatewayService::list_request_logs(&state.db, 100).await?;
    Ok(Json(logs.into_iter().map(|l| RequestLogResponse {
        id: l.id, method: l.method, path: l.path,
        status_code: l.status_code, response_time_ms: l.response_time_ms,
        rate_limited: l.rate_limited, created_at: l.created_at.to_rfc3339(),
    }).collect()))
}

pub async fn get_request_stats(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(GatewayService::get_request_stats(&state.db).await?))
}
