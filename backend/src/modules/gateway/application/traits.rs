use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type RateLimitRuleModel = crate::modules::gateway::domain::entities::rate_limit_rule::Model;
pub type ApiKeyModel = crate::modules::gateway::domain::entities::api_key::Model;
pub type RequestLogModel = crate::modules::gateway::domain::entities::request_log::Model;

#[async_trait]
pub trait GatewayServiceTrait: Send + Sync {
    async fn list_rate_limit_rules(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<RateLimitRuleModel>, AppError>;

    async fn create_rate_limit_rule(
        &self,
        db: &DatabaseConnection,
        name: String,
        max_requests: i32,
        window_seconds: i32,
    ) -> Result<RateLimitRuleModel, AppError>;

    async fn delete_rate_limit_rule(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError>;

    async fn list_api_keys(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<ApiKeyModel>, AppError>;

    async fn create_api_key(
        &self,
        db: &DatabaseConnection,
        name: String,
        permissions: serde_json::Value,
    ) -> Result<ApiKeyModel, AppError>;

    async fn revoke_api_key(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError>;

    async fn list_request_logs(
        &self,
        db: &DatabaseConnection,
        limit: u64,
    ) -> Result<Vec<RequestLogModel>, AppError>;
}
