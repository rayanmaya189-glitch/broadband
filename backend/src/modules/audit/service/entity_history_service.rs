use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::repository::entity_history_repository::EntityHistoryRepository;
use crate::modules::audit::request::entity_history_request::*;
use crate::modules::audit::response::entity_history_response::*;

pub struct EntityHistoryService<'a> { repo: EntityHistoryRepository<'a> }
impl<'a> EntityHistoryService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: EntityHistoryRepository::new(pool) } }

    pub async fn search(&self, q: EntityHistoryQuery) -> Result<EntityHistoryListResponse, AppError> {
        let page = q.page.unwrap_or(1);
        let per_page = q.per_page.unwrap_or(50);
        let (entries, total) = self.repo.search(q.entity_type.as_deref(), q.entity_id, q.action.as_deref(), q.user_id, q.from.as_deref(), q.to.as_deref(), page, per_page).await?;
        Ok(EntityHistoryListResponse {
            entries: entries.into_iter().map(|e| EntityHistoryResponse {
                id: e.id, entity_type: e.entity_type, entity_id: e.entity_id, action: e.action,
                old_data: e.old_data, new_data: e.new_data, changed_fields: e.changed_fields,
                user_id: e.user_id, reason: e.reason, rollback_reference: e.rollback_reference,
                created_at: e.created_at,
            }).collect(),
            total, page, per_page,
        })
    }

    pub async fn get_by_id(&self, id: i64) -> Result<EntityHistoryResponse, AppError> {
        let e = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("History entry not found".into()))?;
        Ok(EntityHistoryResponse { id: e.id, entity_type: e.entity_type, entity_id: e.entity_id, action: e.action, old_data: e.old_data, new_data: e.new_data, changed_fields: e.changed_fields, user_id: e.user_id, reason: e.reason, rollback_reference: e.rollback_reference, created_at: e.created_at })
    }

    pub async fn get_entity_history(&self, entity_type: &str, entity_id: i64) -> Result<Vec<EntityHistoryResponse>, AppError> {
        let entries = self.repo.get_entity_history(entity_type, entity_id).await?;
        Ok(entries.into_iter().map(|e| EntityHistoryResponse { id: e.id, entity_type: e.entity_type, entity_id: e.entity_id, action: e.action, old_data: e.old_data, new_data: e.new_data, changed_fields: e.changed_fields, user_id: e.user_id, reason: e.reason, rollback_reference: e.rollback_reference, created_at: e.created_at }).collect())
    }

    pub async fn rollback(&self, req: RollbackRequest) -> Result<EntityHistoryResponse, AppError> {
        let e = self.repo.rollback(req.history_id, req.user_id, &req.reason).await.map_err(|_| AppError::BadRequest("Cannot rollback: no previous state available".into()))?;
        Ok(EntityHistoryResponse { id: e.id, entity_type: e.entity_type, entity_id: e.entity_id, action: e.action, old_data: e.old_data, new_data: e.new_data, changed_fields: e.changed_fields, user_id: e.user_id, reason: e.reason, rollback_reference: e.rollback_reference, created_at: e.created_at })
    }

    pub async fn get_stats(&self) -> Result<EntityHistoryStatsResponse, AppError> {
        let s = self.repo.get_stats().await?;
        Ok(EntityHistoryStatsResponse { total_entries: s.total_entries, total_entities: s.total_entities, total_rollbacks: s.total_rollbacks })
    }
}
