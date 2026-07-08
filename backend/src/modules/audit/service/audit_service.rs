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
        let (logs, total) = self.repo.list(q.user_id, q.action.as_deref(), q.resource_type.as_deref(), page, per_page).await?;
        Ok(AuditListResponse {
            logs: logs.iter().map(|l| AuditLogResponse {
                id: l.id, user_id: l.user_id, user_email: l.user_email.clone(),
                action: l.action.clone(), resource_type: l.resource_type.clone(),
                result: l.result.clone(), created_at: l.created_at,
            }).collect(),
            total, page, per_page,
        })
    }
}
