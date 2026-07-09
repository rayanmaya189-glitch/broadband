use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::repository::audit_repository::AuditRepository;
use crate::modules::audit::request::audit_request::*;
use crate::modules::audit::response::audit_response::*;

pub struct AuditService<'a> { repo: AuditRepository<'a> }
impl<'a> AuditService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: AuditRepository::new(pool) } }

    pub async fn list(&self, q: AuditQuery) -> Result<AuditListResponse, AppError> {
        let page = q.page.unwrap_or(1);
        let per_page = q.per_page.unwrap_or(50);
        let (logs, total) = self.repo.list(q.user_id, q.action.as_deref(), q.resource_type.as_deref(), q.result.as_deref(), q.from.as_deref(), q.to.as_deref(), page, per_page).await?;
        Ok(AuditListResponse {
            logs: logs.into_iter().map(|l| AuditLogResponse {
                id: l.id, user_id: l.user_id, user_email: l.user_email, user_role: l.user_role,
                action: l.action, resource_type: l.resource_type, resource_id: l.resource_id,
                result: l.result, old_data: l.old_data, new_data: l.new_data, metadata: l.metadata,
                created_at: l.created_at,
            }).collect(),
            total, page, per_page,
        })
    }

    pub async fn get_by_id(&self, id: i64) -> Result<AuditLogResponse, AppError> {
        let l = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Audit log not found".into()))?;
        Ok(AuditLogResponse { id: l.id, user_id: l.user_id, user_email: l.user_email, user_role: l.user_role, action: l.action, resource_type: l.resource_type, resource_id: l.resource_id, result: l.result, old_data: l.old_data, new_data: l.new_data, metadata: l.metadata, created_at: l.created_at })
    }

    pub async fn get_user_activity(&self, user_id: i64, page: i64, per_page: i64) -> Result<AuditListResponse, AppError> {
        let (logs, total) = self.repo.list(Some(user_id), None, None, None, None, None, page, per_page).await?;
        Ok(AuditListResponse {
            logs: logs.into_iter().map(|l| AuditLogResponse { id: l.id, user_id: l.user_id, user_email: l.user_email, user_role: l.user_role, action: l.action, resource_type: l.resource_type, resource_id: l.resource_id, result: l.result, old_data: l.old_data, new_data: l.new_data, metadata: l.metadata, created_at: l.created_at }).collect(),
            total, page, per_page,
        })
    }

    pub async fn get_resource_history(&self, resource_type: &str, resource_id: &str) -> Result<Vec<AuditLogResponse>, AppError> {
        let logs = self.repo.get_by_resource(resource_type, resource_id).await?;
        Ok(logs.into_iter().map(|l| AuditLogResponse { id: l.id, user_id: l.user_id, user_email: l.user_email, user_role: l.user_role, action: l.action, resource_type: l.resource_type, resource_id: l.resource_id, result: l.result, old_data: l.old_data, new_data: l.new_data, metadata: l.metadata, created_at: l.created_at }).collect())
    }

    pub async fn export_logs(&self, req: ExportAuditRequest) -> Result<Vec<AuditLogResponse>, AppError> {
        let logs = self.repo.export_csv(req.user_id, req.action.as_deref(), req.from.as_deref(), req.to.as_deref()).await?;
        Ok(logs.into_iter().map(|l| AuditLogResponse { id: l.id, user_id: l.user_id, user_email: l.user_email, user_role: l.user_role, action: l.action, resource_type: l.resource_type, resource_id: l.resource_id, result: l.result, old_data: l.old_data, new_data: l.new_data, metadata: l.metadata, created_at: l.created_at }).collect())
    }
}
