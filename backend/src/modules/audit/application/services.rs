use crate::modules::audit::domain::entities::{AuditLog, AuditLogActiveModel};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, Set};

pub struct AuditService;

impl AuditService {
    pub async fn record_action(
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
    ) -> Result<crate::modules::audit::domain::entities::audit_log::Model, AppError> {
        let log = AuditLogActiveModel {
            user_id: Set(user_id),
            user_email: Set(user_email),
            user_role: Set(user_role),
            action: Set(action),
            resource_type: Set(resource_type),
            resource_id: Set(resource_id),
            ip_address: Set(ip_address),
            result: Set(result),
            old_data: Set(old_data),
            new_data: Set(new_data),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(log.insert(db).await?)
    }

    pub async fn list_logs(
        db: &DatabaseConnection,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::audit::domain::entities::audit_log::Model>,
            u64,
        ),
        AppError,
    > {
        {
            let q = AuditLog::find();
            let t = q.clone().count(db).await?;
            Ok((q.all(db).await?, t))
        }
    }
}
