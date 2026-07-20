use crate::modules::document::domain::entities::{
    DocumentFile, DocumentFileActiveModel, DocumentFileColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

pub struct DocumentService;

impl DocumentService {
    pub async fn list_documents(
        db: &DatabaseConnection,
        uploaded_by: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::document::domain::entities::document_file::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = DocumentFile::find();
        if let Some(uid) = uploaded_by {
            query = query.filter(DocumentFileColumn::UploadedBy.eq(uid));
        }
        let total = query.clone().count(db).await?;
        let docs = query.all(db).await?;
        Ok((docs, total))
    }

    pub async fn create_document(
        db: &DatabaseConnection,
        filename: String,
        original_filename: String,
        mime_type: String,
        file_size: i64,
        storage_bucket: String,
        storage_key: String,
        uploaded_by: i64,
    ) -> Result<crate::modules::document::domain::entities::document_file::Model, AppError> {
        let now = chrono::Utc::now();
        let doc = DocumentFileActiveModel {
            filename: Set(filename),
            original_filename: Set(original_filename),
            mime_type: Set(mime_type),
            file_size: Set(file_size),
            storage_bucket: Set(storage_bucket),
            storage_key: Set(storage_key),
            uploaded_by: Set(uploaded_by),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(doc.insert(db).await?)
    }

    pub async fn get_document(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::document::domain::entities::document_file::Model, AppError> {
        DocumentFile::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Document {} not found", id)))
    }

    pub async fn list_entity_documents(
        db: &DatabaseConnection,
        entity_type: &str,
        entity_id: i64,
        page: u64,
        limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::document::domain::entities::document_file::Model>,
            u64,
        ),
        AppError,
    > {
        let query = DocumentFile::find()
            .filter(DocumentFileColumn::EntityType.eq(entity_type))
            .filter(DocumentFileColumn::EntityId.eq(entity_id));
        let total = query.clone().count(db).await?;
        let docs = query
            .paginate(db, limit)
            .fetch_page(page.saturating_sub(1))
            .await?;
        Ok((docs, total))
    }

    pub async fn delete_document(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let doc = DocumentFile::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Document {} not found", id)))?;
        let mut active = <crate::modules::document::domain::entities::document_file::Entity as sea_orm::EntityTrait>::ActiveModel::from(doc);
        active.status = Set("deleted".to_string());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }
}
