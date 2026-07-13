//! SeaORM-based service for the EntityHistory domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::audit::repository::entity_history_repository::{EntityHistoryRepository, RollbackResult};

pub struct EntityHistoryService<'a> {
    repo: EntityHistoryRepository<'a>,
}

impl<'a> EntityHistoryService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: EntityHistoryRepository::new(db) }
    }

    pub async fn search(
        &self, entity_type: Option<&str>, entity_id: Option<i64>, action: Option<&str>,
        user_id: Option<i64>, from: Option<&str>, to: Option<&str>,
        page: i64, per_page: i64,
    ) -> Result<(Vec<crate::modules::audit::model::entity_history_entity::Model>, i64), AppError> {
        self.repo.search(entity_type, entity_id, action, user_id, from, to, page, per_page).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<crate::modules::audit::model::entity_history_entity::Model, AppError> {
        self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("History entry not found".into()))
    }

    pub async fn get_entity_history(&self, entity_type: &str, entity_id: i64) -> Result<Vec<crate::modules::audit::model::entity_history_entity::Model>, AppError> {
        self.repo.get_entity_history(entity_type, entity_id).await
    }

    /// Perform a full entity rollback with safety checks and data restoration.
    ///
    /// This is the main service entry point for rollback operations.
    /// It delegates to the repository which handles:
    /// 1. Fetching the history entry
    /// 2. Validating old_data exists
    /// 3. Running entity-type-specific safety checks
    /// 4. Restoring the entity data via dynamic UPDATE
    /// 5. Recording a rollback history entry
    pub async fn rollback(
        &self, history_id: i64, user_id: i64, reason: &str,
    ) -> Result<RollbackResult, AppError> {
        self.repo.rollback(history_id, user_id, reason).await
    }
}
