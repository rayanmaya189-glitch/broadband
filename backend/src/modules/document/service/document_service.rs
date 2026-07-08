use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::document::repository::document_repository::DocumentRepository;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::*;

pub struct DocumentService<'a> {
    repo: DocumentRepository<'a>,
}

impl<'a> DocumentService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { repo: DocumentRepository::new(pool) }
    }

    pub async fn list(&self, q: DocumentQuery) -> Result<Vec<DocumentResponse>, AppError> {
        let docs = self.repo.list(q.entity_type.as_deref(), q.entity_id).await?;
        Ok(docs.iter().map(|d| DocumentResponse {
            id: d.id,
            filename: d.filename.clone(),
            original_filename: d.original_filename.clone(),
            mime_type: d.mime_type.clone(),
            file_size: d.file_size,
            status: d.status.clone(),
            created_at: d.created_at,
        }).collect())
    }

    pub async fn upload(&self, req: UploadRequest, uploaded_by: i64) -> Result<UploadResponse, AppError> {
        let key = format!("{}/{}", uuid::Uuid::new_v4(), req.filename);
        let doc = self.repo.create(&req.filename, &req.filename, &req.mime_type, req.file_size, &req.bucket, &key, uploaded_by).await?;
        let url = format!("/storage/{}/{}", req.bucket, key);
        Ok(UploadResponse { document_id: doc.id, upload_url: url, expires_in: 900 })
    }

    pub async fn delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? {
            return Err(AppError::NotFound("Not found".into()));
        }
        Ok(MessageResponse { message: "Deleted".into() })
    }
}
