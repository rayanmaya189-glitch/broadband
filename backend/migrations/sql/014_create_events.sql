-- AeroXe Backend Migration 014: Create Event Store Tables
-- Event sourcing with NATS JetStream integration

CREATE TABLE IF NOT EXISTS events (
    id BIGSERIAL,
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    aggregate_type VARCHAR(50) NOT NULL,
    aggregate_id BIGINT NOT NULL,
    payload JSONB NOT NULL,
    metadata JSONB,
    caused_by_user_id BIGINT REFERENCES users(id),
    caused_by_branch_id BIGINT REFERENCES branches(id),
    sequence_number BIGSERIAL,
    published_at TIMESTAMPTZ DEFAULT NOW(),
    processed BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (id, published_at)
) PARTITION BY RANGE (published_at);

CREATE TABLE IF NOT EXISTS events_2026_07 PARTITION OF events
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');

CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type, published_at);
CREATE INDEX IF NOT EXISTS idx_events_aggregate ON events(aggregate_type, aggregate_id);
CREATE INDEX IF NOT EXISTS idx_events_processed ON events(processed, published_at);

-- Event subscriptions (subscriber tracking)
CREATE TABLE IF NOT EXISTS event_subscriptions (
    id BIGSERIAL PRIMARY KEY,
    subscriber_name VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    last_processed_id BIGINT DEFAULT 0,
    last_processed_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(subscriber_name, event_type)
);

-- Outbox pattern for reliable event publishing
CREATE TABLE IF NOT EXISTS outbox_events (
    id BIGSERIAL PRIMARY KEY,
    event_id VARCHAR(255) NOT NULL UNIQUE,
    event_type VARCHAR(100) NOT NULL,
    aggregate_type VARCHAR(50) NOT NULL,
    aggregate_id BIGINT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    is_published BOOLEAN DEFAULT FALSE
);

CREATE INDEX IF NOT EXISTS idx_outbox_events_unpublished ON outbox_events(is_published, created_at);
