//! SeaORM-based service for the Document domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::document::repository::document_repository::DocumentRepository;
use crate::modules::document::response::document_response::*;

pub struct DocumentService<'a> {
    repo: DocumentRepository<'a>,
}

impl<'a> DocumentService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: DocumentRepository::new(db) }
    }

    pub async fn list(&self, entity_type: Option<&str>, entity_id: Option<i64>) -> Result<Vec<DocumentFileResponse>, AppError> {
        let files = self.repo.list(entity_type, entity_id).await?;
        Ok(files.into_iter().map(|f| DocumentFileResponse { id: f.id, filename: f.filename, original_filename: f.original_filename, mime_type: f.mime_type, file_size: f.file_size, storage_url: f.storage_url, uploaded_by: f.uploaded_by, entity_type: f.entity_type, entity_id: f.entity_id, status: f.status, created_at: f.created_at.into() }).collect())
    }

    pub async fn get_by_id(&self, id: i64) -> Result<DocumentFileResponse, AppError> {
        let f = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Document not found".into()))?;
        Ok(DocumentFileResponse { id: f.id, filename: f.filename, original_filename: f.original_filename, mime_type: f.mime_type, file_size: f.file_size, storage_url: f.storage_url, uploaded_by: f.uploaded_by, entity_type: f.entity_type, entity_id: f.entity_id, status: f.status, created_at: f.created_at.into() })
    }

    pub async fn confirm_upload(&self, id: i64, file_hash: Option<&str>, storage_url: Option<&str>) -> Result<DocumentFileResponse, AppError> {
        let f = self.repo.confirm_upload(id, file_hash, storage_url).await?;
        Ok(DocumentFileResponse { id: f.id, filename: f.filename, original_filename: f.original_filename, mime_type: f.mime_type, file_size: f.file_size, storage_url: f.storage_url, uploaded_by: f.uploaded_by, entity_type: f.entity_type, entity_id: f.entity_id, status: f.status, created_at: f.created_at.into() })
    }

    pub async fn soft_delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.soft_delete(id).await? { return Err(AppError::NotFound("Document not found".into())); }
        Ok(MessageResponse { message: "Document deleted".into() })
    }
}
