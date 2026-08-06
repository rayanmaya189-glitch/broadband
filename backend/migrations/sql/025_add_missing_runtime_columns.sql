-- AeroXe Backend Migration 031: Add missing runtime columns to align DB with SeaORM entities
-- bandwidth_applications: entity expects last_error (DDL historically created failed_reason)
ALTER TABLE bandwidth.bandwidth_applications
    ADD COLUMN IF NOT EXISTS last_error TEXT;

-- subscriptions: entity expects bytes_used, last_session_duration, last_session_at
ALTER TABLE subscription.subscriptions
    ADD COLUMN IF NOT EXISTS bytes_used BIGINT;
ALTER TABLE subscription.subscriptions
    ADD COLUMN IF NOT EXISTS last_session_duration BIGINT;
ALTER TABLE subscription.subscriptions
    ADD COLUMN IF NOT EXISTS last_session_at TIMESTAMPTZ;

-- notification_channels: entity expects name/channel_type; DDL seeded channel/provider
ALTER TABLE notification.notification_channels
    ADD COLUMN IF NOT EXISTS name VARCHAR(100);
UPDATE notification.notification_channels SET name = channel WHERE name IS NULL;
ALTER TABLE notification.notification_channels ALTER COLUMN name SET NOT NULL;

ALTER TABLE notification.notification_channels
    ADD COLUMN IF NOT EXISTS channel_type VARCHAR(50);
UPDATE notification.notification_channels SET channel_type = provider WHERE channel_type IS NULL;
ALTER TABLE notification.notification_channels ALTER COLUMN channel_type SET NOT NULL;

-- user_branches: entity expects role/assigned_at (DDL has is_primary/created_at)
ALTER TABLE branches.user_branches
    ADD COLUMN IF NOT EXISTS role VARCHAR(50) NOT NULL DEFAULT 'member';
ALTER TABLE branches.user_branches
    ADD COLUMN IF NOT EXISTS assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- branch_working_hours: entity expects updated_at
ALTER TABLE branches.branch_working_hours
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
