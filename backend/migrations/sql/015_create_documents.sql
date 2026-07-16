-- AeroXe Backend Migration 015: Create Document Storage Tables
-- S3/MinIO compatible document management with presigned URLs

CREATE TABLE IF NOT EXISTS document_files (
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

CREATE INDEX IF NOT EXISTS idx_document_files_entity ON document_files(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_document_files_uploaded ON document_files(uploaded_by);

-- Document access logs
CREATE TABLE IF NOT EXISTS document_access_logs (
    id BIGSERIAL PRIMARY KEY,
    document_id BIGINT NOT NULL REFERENCES document_files(id),
    accessed_by BIGINT REFERENCES users(id),
    access_type VARCHAR(20) NOT NULL
        CHECK (access_type IN ('upload', 'download', 'view', 'delete')),
    ip_address INET,
    user_agent TEXT,
    accessed_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (accessed_at);

CREATE TABLE IF NOT EXISTS document_access_logs_2026_07 PARTITION OF document_access_logs
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
