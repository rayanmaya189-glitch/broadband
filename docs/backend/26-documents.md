# AeroXe Backend — Documents Module

> **Req Ref:** §18A Document Storage (MinIO)

---

## 1. Overview

S3-compatible object storage using MinIO for managing uploaded documents: KYC files, installation photos, ticket attachments, and invoices. Uses presigned URLs for secure upload/download without exposing credentials.

## 2. Architecture

```
Client requests presigned upload URL
    ↓
Backend generates URL (valid 15 min)
    ↓
Client uploads directly to MinIO
    ↓
MinIO sends webhook notification
    ↓
Backend validates file (type, size, hash)
    ↓
Stores metadata in database
    ↓
Associates with entity (customer, ticket, etc.)
```

## 3. Database Tables

```sql
CREATE TABLE document_files (
    id BIGSERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL,
    file_hash VARCHAR(255),
    storage_bucket VARCHAR(100) NOT NULL,
    storage_key VARCHAR(500) NOT NULL,
    storage_url TEXT,
    uploaded_by BIGINT NOT NULL REFERENCES users(id),
    entity_type VARCHAR(50),
    entity_id BIGINT,
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'deleted', 'expired')),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE document_access_logs (
    id BIGSERIAL PRIMARY KEY,
    document_id BIGINT NOT NULL REFERENCES document_files(id),
    accessed_by BIGINT REFERENCES users(id),
    access_type VARCHAR(20) NOT NULL
        CHECK (access_type IN ('upload', 'download', 'view', 'delete')),
    ip_address INET,
    user_agent TEXT,
    accessed_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (accessed_at);
```

## 4. MinIO Configuration

```yaml
# minio config
endpoint: "minio.aeroxe.internal:9000"
access_key: "${MINIO_ACCESS_KEY}"
secret_key: "${MINIO_SECRET_KEY}"
buckets:
  kyc-documents: "aeroxe-kyc"
  installation-photos: "aeroxe-installations"
  ticket-attachments: "aeroxe-tickets"
  invoices: "aeroxe-invoices"
  avatars: "aeroxe-avatars"
presigned_url_expiry: 900  # 15 minutes
max_file_size_mb: 10
```

## 5. File Validation Rules

| Document Type | Allowed Extensions | Max Size | Required |
|--------------|-------------------|----------|----------|
| KYC - Aadhaar | jpg, jpeg, png, pdf | 5 MB | Yes |
| KYC - PAN | jpg, jpeg, png, pdf | 5 MB | Yes |
| KYC - Address Proof | jpg, jpeg, png, pdf | 5 MB | Yes |
| Installation Photo | jpg, jpeg, png | 10 MB | No |
| Ticket Attachment | jpg, jpeg, png, pdf, doc, docx | 10 MB | No |
| Invoice PDF | pdf | 2 MB | Auto-generated |
| User Avatar | jpg, jpeg, png | 2 MB | No |

## 6. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/documents/upload-url` | Yes | Get presigned upload URL |
| POST | `/api/v1/documents/confirm` | Yes | Confirm upload completed |
| GET | `/api/v1/documents/:id/download` | Yes | Get presigned download URL |
| GET | `/api/v1/documents/:id` | Yes | Get document metadata |
| DELETE | `/api/v1/documents/:id` | Yes | Soft-delete document |
| GET | `/api/v1/documents` | Yes | List documents (filtered) |
| GET | `/api/v1/documents/entity/:type/:id` | Yes | List docs for entity |

## 7. Upload Flow

```rust
pub async fn get_upload_url(
    State(state): State<SharedState>,
    user: UserContext,
    req: UploadRequest,
) -> Result<UploadResponse> {
    // 1. Validate file type and size
    validate_file(&req)?;

    // 2. Generate storage key
    let key = generate_storage_key(&req.entity_type, &req.filename);

    // 3. Generate presigned upload URL
    let url = state.storage.presigned_put_url(
        &req.bucket, &key, Duration::from_secs(900)
    ).await?;

    // 4. Create document record (status: pending)
    let doc = db.create_document(InsertDocument {
        filename: key.clone(),
        original_filename: req.filename,
        mime_type: req.mime_type,
        file_size: req.file_size,
        storage_bucket: req.bucket,
        storage_key: key,
        uploaded_by: user.id,
        entity_type: req.entity_type,
        entity_id: req.entity_id,
        status: "pending".to_string(),
    }).await?;

    Ok(UploadResponse {
        document_id: doc.id,
        upload_url: url,
        expires_in: 900,
    })
}
```

## 8. Storage Lifecycle

- Active documents: retained indefinitely
- Deleted documents: soft-deleted, purged after 30 days
- Orphaned documents (no entity): flagged and cleaned up weekly
- KYC documents: retained for 7 years (regulatory requirement)

## 9. RBAC Permissions

```
document.upload
document.download
document.view
document.delete
document.list
```
