//! SeaORM-based service for the Audit domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::audit::repository::audit_repository_seaorm::AuditRepositorySeaorm;

pub struct AuditServiceSeaorm<'a> {
    repo: AuditRepositorySeaorm<'a>,
}

impl<'a> AuditServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: AuditRepositorySeaorm::new(db) }
    }

    pub async fn list(&self, user_id: Option<i64>, action: Option<&str>, resource_type: Option<&str>, result: Option<&str>, from: Option<&str>, to: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<crate::modules::audit::model::audit_log_entity::Model>, i64), AppError> {
        self.repo.list(user_id, action, resource_type, result, from, to, page, per_page).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<crate::modules::audit::model::audit_log_entity::Model, AppError> {
        self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Audit log not found".into()))
    }

    pub async fn get_by_resource(&self, resource_type: &str, resource_id: &str) -> Result<Vec<crate::modules::audit::model::audit_log_entity::Model>, AppError> {
        self.repo.get_by_resource(resource_type, resource_id).await
    }
}
