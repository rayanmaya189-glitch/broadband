//! SeaORM-based service for the Document domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::document::repository::document_repository::DocumentRepository;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::*;
use crate::modules::document::response::document_response::MessageResponse;

pub struct DocumentService<'a> {
    repo: DocumentRepository<'a>,
}

impl<'a> DocumentService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: DocumentRepository::new(db) }
    }

    pub async fn list(&self, entity_type: Option<&str>, entity_id: Option<i64>) -> Result<Vec<DocumentFileResponse>, AppError> {
        let docs = self.repo.list(entity_type, entity_id).await?;
        Ok(docs.into_iter().map(|d| DocumentFileResponse {
            id: d.id, filename: d.filename, original_filename: d.original_filename,
            mime_type: d.mime_type, file_size: d.file_size,
            storage_url: d.storage_url, uploaded_by: d.uploaded_by,
            entity_type: d.entity_type, entity_id: d.entity_id,
            status: d.status,
            created_at: d.created_at.into(),
        }).collect())
    }

    pub async fn get_by_id(&self, id: i64) -> Result<DocumentFileResponse, AppError> {
        let d = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Document not found".into()))?;
        Ok(DocumentFileResponse {
            id: d.id, filename: d.filename, original_filename: d.original_filename,
            mime_type: d.mime_type, file_size: d.file_size,
            storage_url: d.storage_url, uploaded_by: d.uploaded_by,
            entity_type: d.entity_type, entity_id: d.entity_id,
            status: d.status,
            created_at: d.created_at.into(),
        })
    }

    pub async fn upload_url(&self, req: UploadRequest) -> Result<DocumentFileResponse, AppError> {
        let d = self.repo.create(
            &req.filename, &req.filename, &req.mime_type, req.file_size,
            &req.bucket, "", 0,
            req.entity_type.as_deref(), req.entity_id,
        ).await?;
        Ok(DocumentFileResponse {
            id: d.id, filename: d.filename, original_filename: d.original_filename,
            mime_type: d.mime_type, file_size: d.file_size,
            storage_url: d.storage_url, uploaded_by: d.uploaded_by,
            entity_type: d.entity_type, entity_id: d.entity_id,
            status: d.status,
            created_at: d.created_at.into(),
        })
    }

    pub async fn soft_delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        self.repo.soft_delete(id).await?;
        Ok(MessageResponse { message: "Document deleted".into() })
    }

    pub async fn confirm_upload(&self, id: i64, _file_hash: Option<&str>, _storage_url: Option<&str>) -> Result<DocumentFileResponse, AppError> {
        self.get_by_id(id).await
    }

    pub async fn associate_entity(&self, id: i64, entity_type: &str, entity_id: i64) -> Result<DocumentFileResponse, AppError> {
        self.repo.associate_entity(id, entity_type, entity_id).await?;
        self.get_by_id(id).await
    }

    pub async fn get_access_logs(&self, _id: i64) -> Result<serde_json::Value, AppError> {
        Ok(serde_json::json!({ "logs": [] }))
    }
}
