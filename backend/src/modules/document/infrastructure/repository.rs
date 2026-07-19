use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};

use crate::modules::document::domain::entities::document_file;
use crate::shared::errors::AppError;

pub struct DocumentRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DocumentRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<document_file::Model>, AppError> {
        Ok(document_file::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_by_entity(
        &self,
        entity_type: &str,
        entity_id: i64,
    ) -> Result<Vec<document_file::Model>, AppError> {
        Ok(document_file::Entity::find()
            .filter(document_file::Column::EntityType.eq(entity_type))
            .filter(document_file::Column::EntityId.eq(entity_id))
            .filter(document_file::Column::Status.eq("active"))
            .order_by_desc(document_file::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn list_documents(
        &self,
        uploaded_by: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<document_file::Model>, AppError> {
        let mut query =
            document_file::Entity::find().filter(document_file::Column::Status.eq("active"));
        if let Some(uid) = uploaded_by {
            query = query.filter(document_file::Column::UploadedBy.eq(uid));
        }
        Ok(query
            .order_by_desc(document_file::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn create_document(
        &self,
        filename: String,
        original_filename: String,
        mime_type: String,
        file_size: i64,
        file_hash: Option<String>,
        storage_bucket: String,
        storage_key: String,
        storage_url: Option<String>,
        uploaded_by: i64,
        entity_type: Option<String>,
        entity_id: Option<i64>,
    ) -> Result<document_file::Model, AppError> {
        let now = chrono::Utc::now();
        let model = document_file::ActiveModel {
            filename: Set(filename),
            original_filename: Set(original_filename),
            mime_type: Set(mime_type),
            file_size: Set(file_size),
            file_hash: Set(file_hash),
            storage_bucket: Set(storage_bucket),
            storage_key: Set(storage_key),
            storage_url: Set(storage_url),
            uploaded_by: Set(uploaded_by),
            entity_type: Set(entity_type),
            entity_id: Set(entity_id),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        if let Some(model) = document_file::Entity::find_by_id(id).one(self.db).await? {
            let mut active: document_file::ActiveModel = model.into();
            active.status = Set("deleted".to_string());
            active.updated_at = Set(chrono::Utc::now());
            active.update(self.db).await?;
        }
        Ok(())
    }
}
