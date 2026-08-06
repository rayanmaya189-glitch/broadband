-- AeroXe Backend Migration 032: Realign workflow.approval_requests to the SeaORM entity contract
-- Entity expects: workflow_type, resource_type, resource_id, requested_by, branch_id, status,
-- payload, reason, reviewer_id, reviewer_comment, requested_at, reviewed_at, expires_at,
-- created_at, updated_at. The DDL (m003) created the older shape (workflow_id, reviewed_by,
-- comment) which the workflow module never used at runtime.

ALTER TABLE workflow.approval_requests
    DROP CONSTRAINT IF EXISTS fk_approval_requests_workflow;
ALTER TABLE workflow.approval_requests
    DROP CONSTRAINT IF EXISTS fk_approval_requests_reviewed_by;

ALTER TABLE workflow.approval_requests
    RENAME COLUMN workflow_id TO workflow_type;
ALTER TABLE workflow.approval_requests
    ALTER COLUMN workflow_type TYPE VARCHAR(100) USING workflow_type::text;

ALTER TABLE workflow.approval_requests
    RENAME COLUMN reviewed_by TO reviewer_id;
ALTER TABLE workflow.approval_requests
    RENAME COLUMN comment TO reviewer_comment;

ALTER TABLE workflow.approval_requests
    ADD COLUMN IF NOT EXISTS branch_id BIGINT;
ALTER TABLE workflow.approval_requests
    ADD COLUMN IF NOT EXISTS reason TEXT;
ALTER TABLE workflow.approval_requests
    ADD COLUMN IF NOT EXISTS requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE workflow.approval_requests
    ADD COLUMN IF NOT EXISTS expires_at TIMESTAMPTZ;
