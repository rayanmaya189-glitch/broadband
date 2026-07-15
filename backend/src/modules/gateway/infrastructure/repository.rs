use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::shared::errors::AppError;
use crate::modules::gateway::domain::entities::{ApiKey, RateLimitRule, RequestLog};

/// Gateway repository for database queries.
pub struct GatewayRepository;

impl GatewayRepository {
    /// Find active rate limit rules matching a route pattern.
    pub async fn find_rate_limit_rules_for_route(
        db: &DatabaseConnection,
        route_pattern: &str,
    ) -> Result<Vec<rate_limit_rule::Model>, AppError> {
        Ok(RateLimitRule::find()
            .filter(rate_limit_rule::Column::RoutePattern.eq(route_pattern))
            .filter(rate_limit_rule::Column::IsActive.eq(true))
            .all(db)
            .await?)
    }

    /// Find an active API key by its hash.
    pub async fn find_active_api_key(
        db: &DatabaseConnection,
        key_hash: &str,
    ) -> Result<Option<api_key::Model>, AppError> {
        Ok(ApiKey::find()
            .filter(api_key::Column::KeyHash.eq(key_hash))
            .filter(api_key::Column::IsActive.eq(true))
            .one(db)
            .await?)
    }

    /// Count requests in a time window for rate limiting.
    pub async fn count_requests_in_window(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        route_pattern: &str,
        window_seconds: i32,
    ) -> Result<i64, AppError> {
        let cutoff = chrono::Utc::now() - chrono::Duration::seconds(window_seconds as i64);
        let count = RequestLog::find()
            .filter(request_log::Column::Path.eq(route_pattern))
            .filter(request_log::Column::CreatedAt.gte(cutoff))
            .count(db)
            .await? as i64;
        Ok(count)
    }
}

use crate::modules::gateway::domain::entities::{rate_limit_rule, api_key, request_log};
