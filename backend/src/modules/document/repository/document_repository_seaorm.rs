//! SeaORM-based repository for the Document domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::document::model::document_file_entity::{self, Model as DocumentFileModel};
use crate::modules::document::model::document_access_log_entity::{self, Model as DocumentAccessLogModel};

pub struct DocumentRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DocumentRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, entity_type: Option<&str>, entity_id: Option<i64>) -> Result<Vec<DocumentFileModel>, AppError> {
        let mut select = document_file_entity::Entity::find()
            .filter(document_file_entity::Column::Status.ne("deleted"));
        if let Some(et) = entity_type {
            select = select.filter(document_file_entity::Column::EntityType.eq(et));
        }
        if let Some(eid) = entity_id {
            select = select.filter(document_file_entity::Column::EntityId.eq(eid));
        }
        let files = select.order_by_desc(document_file_entity::Column::CreatedAt).all(self.db).await?;
        Ok(files)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<DocumentFileModel>, AppError> {
        Ok(document_file_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(
        &self, filename: &str, orig: &str, mime: &str, size: i64,
        bucket: &str, key: &str, uploaded_by: i64,
        entity_type: Option<&str>, entity_id: Option<i64>,
    ) -> Result<DocumentFileModel, AppError> {
        let now = chrono::Utc::now();
        let active = document_file_entity::ActiveModel {
            filename: Set(filename.to_owned()),
            original_filename: Set(orig.to_owned()),
            mime_type: Set(mime.to_owned()),
            file_size: Set(size),
            storage_bucket: Set(bucket.to_owned()),
            storage_key: Set(key.to_owned()),
            uploaded_by: Set(uploaded_by),
            entity_type: Set(entity_type.map(|s| s.to_owned())),
            entity_id: Set(entity_id),
            status: Set("pending".to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn confirm_upload(&self, id: i64, file_hash: Option<&str>, storage_url: Option<&str>) -> Result<DocumentFileModel, AppError> {
        let existing = document_file_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Document not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("active".to_owned());
        if let Some(h) = file_hash { active.file_hash = Set(Some(h.to_owned())); }
        if let Some(u) = storage_url { active.storage_url = Set(Some(u.to_owned())); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn soft_delete(&self, id: i64) -> Result<bool, AppError> {
        let existing = document_file_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.status = Set("deleted".to_owned());
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn associate_entity(&self, id: i64, entity_type: &str, entity_id: i64) -> Result<bool, AppError> {
        let existing = document_file_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.entity_type = Set(Some(entity_type.to_owned()));
                active.entity_id = Set(Some(entity_id));
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn log_access(
        &self, document_id: i64, accessed_by: Option<i64>, access_type: &str,
        ip_address: Option<&str>, user_agent: Option<&str>,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now();
        let active = document_access_log_entity::ActiveModel {
            document_id: Set(document_id),
            accessed_by: Set(accessed_by),
            access_type: Set(access_type.to_owned()),
            ip_address: Set(ip_address.map(|s| s.to_owned())),
            user_agent: Set(user_agent.map(|s| s.to_owned())),
            accessed_at: Set(now.into()),
            ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }

    pub async fn get_access_logs(&self, document_id: i64) -> Result<Vec<DocumentAccessLogModel>, AppError> {
        let logs = document_access_log_entity::Entity::find()
            .filter(document_access_log_entity::Column::DocumentId.eq(document_id))
            .order_by_desc(document_access_log_entity::Column::AccessedAt)
            .all(self.db).await?;
        Ok(logs)
    }
}
