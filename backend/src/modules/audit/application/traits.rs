use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type AuditLogModel = crate::modules::audit::domain::entities::audit_log::Model;

#[async_trait]
pub trait AuditServiceTrait: Send + Sync {
    async fn record_action(
        &self,
        db: &DatabaseConnection,
        user_id: Option<i64>,
        user_email: Option<String>,
        user_role: Option<String>,
        action: String,
        resource_type: Option<String>,
        resource_id: Option<String>,
        ip_address: Option<String>,
        result: String,
        old_data: Option<serde_json::Value>,
        new_data: Option<serde_json::Value>,
    ) -> Result<AuditLogModel, AppError>;

    async fn list_logs(&self, db: &DatabaseConnection) -> Result<Vec<AuditLogModel>, AppError>;
}
