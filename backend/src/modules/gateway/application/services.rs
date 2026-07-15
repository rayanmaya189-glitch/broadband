use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait};
use chrono::Utc;
use crate::shared::errors::AppError;
use crate::modules::gateway::domain::entities::{
    RateLimitRule, RateLimitRuleActiveModel,
    ApiKey, ApiKeyActiveModel,
    RequestLog, RequestLogActiveModel,
};

/// Gateway service providing rate limiting, API key validation, and request logging.
pub struct GatewayService;

impl GatewayService {
    // ── Rate Limit Rules ──

    pub async fn list_rate_limit_rules(db: &DatabaseConnection) -> Result<Vec<rate_limit_rule::Model>, AppError> {
        Ok(RateLimitRule::find().all(db).await?)
    }

    pub async fn create_rate_limit_rule(
        db: &DatabaseConnection,
        route_pattern: String,
        methods: String,
        max_requests: i32,
        window_seconds: i32,
        role: Option<String>,
        branch_id: Option<i64>,
    ) -> Result<rate_limit_rule::Model, AppError> {
        let now = Utc::now();
        let rule = RateLimitRuleActiveModel {
            route_pattern: Set(route_pattern),
            methods: Set(methods),
            max_requests: Set(max_requests),
            window_seconds: Set(window_seconds),
            role: Set(role),
            branch_id: Set(branch_id),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(rule.insert(db).await?)
    }

    pub async fn delete_rate_limit_rule(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let rule = RateLimitRule::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Rate limit rule {} not found", id)))?;
        rule.delete(db).await?;
        Ok(())
    }

    // ── API Keys ──

    pub async fn list_api_keys(db: &DatabaseConnection) -> Result<Vec<api_key::Model>, AppError> {
        Ok(ApiKey::find().all(db).await?)
    }

    pub async fn create_api_key(
        db: &DatabaseConnection,
        name: String,
        key_hash: String,
        key_prefix: String,
        branch_id: Option<i64>,
        permissions: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<api_key::Model, AppError> {
        let now = Utc::now();
        let key = ApiKeyActiveModel {
            name: Set(name),
            key_hash: Set(key_hash),
            key_prefix: Set(key_prefix),
            branch_id: Set(branch_id),
            permissions: Set(permissions),
            expires_at: Set(expires_at),
            is_active: Set(true),
            last_used_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(key.insert(db).await?)
    }

    pub async fn revoke_api_key(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let key = ApiKey::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("API key {} not found", id)))?;
        let mut active: api_key::ActiveModel = key.into();
        active.is_active = Set(false);
        active.updated_at = Set(Utc::now());
        active.update(db).await?;
        Ok(())
    }

    pub async fn validate_api_key(db: &DatabaseConnection, key_hash: &str) -> Result<Option<api_key::Model>, AppError> {
        let key = ApiKey::find()
            .filter(api_key::Column::KeyHash.eq(key_hash))
            .filter(api_key::Column::IsActive.eq(true))
            .one(db)
            .await?;

        if let Some(ref k) = key {
            // Check expiry
            if let Some(expires) = k.expires_at {
                if expires < Utc::now() {
                    return Ok(None);
                }
            }
            // Update last_used_at
            let mut active: api_key::ActiveModel = k.clone().into();
            active.last_used_at = Set(Some(Utc::now()));
            active.update(db).await?;
        }

        Ok(key)
    }

    // ── Request Logging ──

    pub async fn log_request(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        branch_id: Option<i64>,
        method: String,
        path: String,
        status_code: i32,
        response_time_ms: i32,
        ip_address: Option<String>,
        user_agent: Option<String>,
        rate_limited: bool,
        api_key_id: Option<i64>,
    ) -> Result<request_log::Model, AppError> {
        let log = RequestLogActiveModel {
            user_id: Set(user_id),
            branch_id: Set(branch_id),
            method: Set(method),
            path: Set(path),
            status_code: Set(status_code),
            response_time_ms: Set(response_time_ms),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            rate_limited: Set(rate_limited),
            api_key_id: Set(api_key_id),
            created_at: Set(Utc::now()),
            ..Default::default()
        };
        Ok(log.insert(db).await?)
    }

    pub async fn list_request_logs(db: &DatabaseConnection, limit: u64) -> Result<Vec<request_log::Model>, AppError> {
        Ok(RequestLog::find()
            .order_by_desc(request_log::Column::CreatedAt)
            .limit(limit)
            .all(db)
            .await?)
    }

    pub async fn get_request_stats(db: &DatabaseConnection) -> Result<serde_json::Value, AppError> {
        let total = RequestLog::find().count(db).await? as i64;
        let rate_limited = RequestLog::find()
            .filter(request_log::Column::RateLimited.eq(true))
            .count(db)
            .await? as i64;

        Ok(serde_json::json!({
            "total_requests": total,
            "rate_limited_requests": rate_limited,
            "rate_limit_percentage": if total > 0 { (rate_limited as f64 / total as f64 * 100.0).round() } else { 0.0 },
        }))
    }
}

use sea_orm::ActiveModelTrait;
use crate::modules::gateway::domain::entities::{rate_limit_rule, api_key, request_log};
