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
        Ok(docs.into_iter().map(|d| DocumentResponse {
            id: d.id, filename: d.filename, original_filename: d.original_filename,
            mime_type: d.mime_type, file_size: d.file_size, status: d.status,
            entity_type: d.entity_type, entity_id: d.entity_id,
            created_at: d.created_at, updated_at: d.updated_at,
        }).collect())
    }

    pub async fn get_by_id(&self, id: i64) -> Result<DocumentResponse, AppError> {
        let d = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Document not found".into()))?;
        Ok(DocumentResponse { id: d.id, filename: d.filename, original_filename: d.original_filename, mime_type: d.mime_type, file_size: d.file_size, status: d.status, entity_type: d.entity_type, entity_id: d.entity_id, created_at: d.created_at, updated_at: d.updated_at })
    }

    pub async fn upload(&self, req: UploadRequest, uploaded_by: i64) -> Result<UploadResponse, AppError> {
        let key = format!("{}/{}", uuid::Uuid::new_v4(), req.filename);
        let doc = self.repo.create(&req.filename, &req.filename, &req.mime_type, req.file_size, &req.bucket, &key, uploaded_by, req.entity_type.as_deref(), req.entity_id).await?;
        let url = format!("/storage/{}/{}", req.bucket, key);
        Ok(UploadResponse { document_id: doc.id, upload_url: url, expires_in: 900 })
    }

    pub async fn confirm_upload(&self, id: i64, req: ConfirmUploadRequest) -> Result<DocumentResponse, AppError> {
        let d = self.repo.confirm_upload(id, req.file_hash.as_deref(), req.storage_url.as_deref()).await.map_err(|_| AppError::NotFound("Document not found".into()))?;
        Ok(DocumentResponse { id: d.id, filename: d.filename, original_filename: d.original_filename, mime_type: d.mime_type, file_size: d.file_size, status: d.status, entity_type: d.entity_type, entity_id: d.entity_id, created_at: d.created_at, updated_at: d.updated_at })
    }

    pub async fn associate_entity(&self, id: i64, req: AssociateEntityRequest) -> Result<DocumentResponse, AppError> {
        self.repo.associate_entity(id, &req.entity_type, req.entity_id).await.map_err(|_| AppError::NotFound("Document not found".into()))?;
        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.soft_delete(id).await? { return Err(AppError::NotFound("Not found".into())); }
        Ok(MessageResponse { message: "Document deleted".into() })
    }

    pub async fn log_access(&self, document_id: i64, accessed_by: Option<i64>, access_type: &str) -> Result<(), AppError> {
        self.repo.log_access(document_id, accessed_by, access_type, None, None).await?;
        Ok(())
    }

    pub async fn get_access_logs(&self, document_id: i64) -> Result<Vec<DocumentAccessLogResponse>, AppError> {
        self.repo.get_by_id(document_id).await?.ok_or_else(|| AppError::NotFound("Document not found".into()))?;
        let logs = self.repo.get_access_logs(document_id).await?;
        Ok(logs.into_iter().map(|l| DocumentAccessLogResponse { id: l.id, document_id: l.document_id, accessed_by: l.accessed_by, access_type: l.access_type, ip_address: l.ip_address, accessed_at: l.accessed_at }).collect())
    }
}
