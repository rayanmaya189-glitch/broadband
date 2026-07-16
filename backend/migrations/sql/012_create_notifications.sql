-- AeroXe Backend Migration 012: Create Notification Tables
-- Multi-channel notification system: Email, SMS, WhatsApp, Push, In-App

-- Notification Templates
CREATE TABLE IF NOT EXISTS notification_templates (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    channel VARCHAR(20) NOT NULL
        CHECK (channel IN ('email', 'sms', 'whatsapp', 'push', 'in_app')),
    subject_template TEXT,
    body_template TEXT NOT NULL,
    variables JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Notification Channels (provider configurations)
CREATE TABLE IF NOT EXISTS notification_channels (
    id BIGSERIAL PRIMARY KEY,
    channel VARCHAR(20) NOT NULL UNIQUE,
    provider VARCHAR(50) NOT NULL,
    config JSONB NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Notifications (sent notifications)
CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,
    template_id BIGINT REFERENCES notification_templates(id),
    channel VARCHAR(20) NOT NULL,
    recipient_type VARCHAR(20) NOT NULL,
    recipient_id BIGINT NOT NULL,
    recipient_address VARCHAR(255) NOT NULL,
    subject TEXT,
    body TEXT NOT NULL,
    variables JSONB,
    status VARCHAR(20) DEFAULT 'queued'
        CHECK (status IN ('queued', 'sent', 'delivered', 'failed', 'retrying')),
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    last_error TEXT,
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

CREATE TABLE IF NOT EXISTS notifications_2026_07 PARTITION OF notifications
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');

-- Notification History (delivery tracking)
CREATE TABLE IF NOT EXISTS notification_history (
    id BIGSERIAL PRIMARY KEY,
    notification_id BIGINT NOT NULL,
    event VARCHAR(50) NOT NULL,
    details JSONB,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (recorded_at);

CREATE TABLE IF NOT EXISTS notification_history_2026_07 PARTITION OF notification_history
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
