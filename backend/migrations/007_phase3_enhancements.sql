-- ============================================================
-- Phase 3 Enhancements: Billing, Device, Bandwidth, Tickets
-- ============================================================

-- ── Billing Config (dunning, tax) ───────────────────────────
CREATE TABLE IF NOT EXISTS billing_config (
    key VARCHAR(50) PRIMARY KEY,
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add review columns to invoices if not present
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'invoices' AND column_name = 'review_status') THEN
        ALTER TABLE invoices ADD COLUMN review_status VARCHAR(20) DEFAULT 'pending'
            CHECK (review_status IN ('pending', 'approved', 'rejected'));
        ALTER TABLE invoices ADD COLUMN review_notes TEXT;
        ALTER TABLE invoices ADD COLUMN reviewed_by BIGINT REFERENCES users(id);
        ALTER TABLE invoices ADD COLUMN reviewed_at TIMESTAMPTZ;
        ALTER TABLE invoices ADD COLUMN approved_by BIGINT REFERENCES users(id);
        ALTER TABLE invoices ADD COLUMN approved_at TIMESTAMPTZ;
    END IF;
END $$;

-- ── Device Ports ────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS device_ports (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id) ON DELETE CASCADE,
    port_number INTEGER NOT NULL,
    port_name VARCHAR(50),
    port_type VARCHAR(50),
    speed_mbps INTEGER,
    status VARCHAR(20) DEFAULT 'down'
        CHECK (status IN ('up', 'down', 'disabled')),
    connected_device_id BIGINT REFERENCES network_devices(id),
    customer_id BIGINT REFERENCES customers(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(device_id, port_number)
);

-- ── Firmware Updates ────────────────────────────────────────
CREATE TABLE IF NOT EXISTS firmware_updates (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    from_version VARCHAR(50),
    to_version VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'downloading', 'installing', 'completed', 'failed', 'rolled_back')),
    initiated_by BIGINT REFERENCES users(id),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    failure_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Device Metrics ──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS device_metrics (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(15,4) NOT NULL,
    unit VARCHAR(20),
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_device_metrics_device ON device_metrics(device_id);
CREATE INDEX IF NOT EXISTS idx_device_metrics_recorded ON device_metrics(recorded_at);

-- ── Bandwidth Applications ──────────────────────────────────
CREATE TABLE IF NOT EXISTS bandwidth_applications (
    id BIGSERIAL PRIMARY KEY,
    profile_id BIGINT NOT NULL REFERENCES bandwidth_profiles(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'applying', 'applied', 'failed')),
    applied_at TIMESTAMPTZ,
    failed_reason TEXT,
    retry_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Bandwidth Usage ─────────────────────────────────────────
CREATE TABLE IF NOT EXISTS bandwidth_usage (
    id BIGSERIAL PRIMARY KEY,
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    download_bytes BIGINT DEFAULT 0,
    upload_bytes BIGINT DEFAULT 0,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_bandwidth_usage_sub ON bandwidth_usage(subscription_id);
CREATE INDEX IF NOT EXISTS idx_bandwidth_usage_recorded ON bandwidth_usage(recorded_at);

-- ── Ticket Escalations ──────────────────────────────────────
CREATE TABLE IF NOT EXISTS ticket_escalations (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id),
    from_user_id BIGINT NOT NULL REFERENCES users(id),
    to_user_id BIGINT NOT NULL REFERENCES users(id),
    from_priority VARCHAR(10),
    to_priority VARCHAR(10),
    reason TEXT NOT NULL,
    escalated_at TIMESTAMPTZ DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Ticket Status History ───────────────────────────────────
CREATE TABLE IF NOT EXISTS ticket_status_history (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    old_status VARCHAR(30),
    new_status VARCHAR(30) NOT NULL,
    changed_by BIGINT NOT NULL REFERENCES users(id),
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ticket_status_history_ticket ON ticket_status_history(ticket_id);
