use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type DocumentFileModel = crate::modules::document::domain::entities::document_file::Model;

#[async_trait]
pub trait DocumentServiceTrait: Send + Sync {
    async fn list_documents(
        &self,
        db: &DatabaseConnection,
        customer_id: Option<i64>,
    ) -> Result<Vec<DocumentFileModel>, AppError>;

    async fn upload_document(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        file_name: String,
        file_type: String,
        file_size: i64,
        storage_path: String,
    ) -> Result<DocumentFileModel, AppError>;

    async fn delete_document(&self, db: &DatabaseConnection, id: i64) -> Result<(), AppError>;
}
