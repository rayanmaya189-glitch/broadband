use crate::modules::document::domain::value_objects::{DocumentFileId, DocumentStatus};

/// DocumentFile aggregate root - represents a stored document
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentFile {
    pub id: DocumentFileId,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    pub storage_bucket: String,
    pub storage_key: String,
    pub uploaded_by: i64,
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub status: DocumentStatus,
}

/// Domain errors for DocumentFile aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentDomainError {
    DocumentNotFound(i64),
    InvalidFileType(String),
    FileTooLarge(i64),
    CannotDeleteActiveDocument,
}

impl std::fmt::Display for DocumentDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DocumentNotFound(id) => write!(f, "Document {} not found", id),
            Self::InvalidFileType(t) => write!(f, "Invalid file type: {}", t),
            Self::FileTooLarge(s) => write!(f, "File too large: {} bytes", s),
            Self::CannotDeleteActiveDocument => write!(f, "Cannot delete an active document"),
        }
    }
}

impl std::error::Error for DocumentDomainError {}

impl DocumentFile {
    pub fn new(
        original_filename: String,
        mime_type: String,
        file_size: i64,
        storage_bucket: String,
        storage_key: String,
        uploaded_by: i64,
    ) -> Result<Self, DocumentDomainError> {
        let max_size = 10 * 1024 * 1024; // 10MB
        if file_size > max_size {
            return Err(DocumentDomainError::FileTooLarge(file_size));
        }
        Ok(Self {
            id: DocumentFileId::new(0),
            filename: storage_key.clone(),
            original_filename,
            mime_type,
            file_size,
            file_hash: None,
            storage_bucket,
            storage_key,
            uploaded_by,
            entity_type: None,
            entity_id: None,
            status: DocumentStatus::Active,
        })
    }

    pub fn soft_delete(&mut self) -> Result<(), DocumentDomainError> {
        if self.status == DocumentStatus::Deleted {
            return Err(DocumentDomainError::CannotDeleteActiveDocument);
        }
        self.status = DocumentStatus::Deleted;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.status == DocumentStatus::Active
    }

    pub fn is_kyc_document(&self) -> bool {
        self.entity_type.as_deref() == Some("kyc")
    }

    pub fn is_invoice(&self) -> bool {
        self.mime_type == "application/pdf" && self.entity_type.as_deref() == Some("invoice")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document() {
        let doc = DocumentFile::new(
            "photo.jpg".to_string(), "image/jpeg".to_string(), 1024,
            "bucket".to_string(), "key".to_string(), 1,
        );
        assert!(doc.is_ok());
        assert!(doc.unwrap().is_active());
    }

    #[test]
    fn test_file_too_large() {
        let doc = DocumentFile::new(
            "large.pdf".to_string(), "application/pdf".to_string(), 20 * 1024 * 1024,
            "bucket".to_string(), "key".to_string(), 1,
        );
        assert_eq!(doc, Err(DocumentDomainError::FileTooLarge(20 * 1024 * 1024)));
    }
}
