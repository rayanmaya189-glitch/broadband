-- Migration 020: Create Missing Tables for Entity Modules
-- Creates tables for 25 entities that have SeaORM definitions but no database tables
-- NOTE: No BEGIN/COMMIT - migration runner handles transaction

-- ============================================================
-- compliance schema
-- ============================================================
CREATE SCHEMA IF NOT EXISTS compliance;

CREATE TABLE IF NOT EXISTS compliance.kyc_verifications (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    document_type TEXT NOT NULL,
    document_number_hash TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    provider TEXT,
    provider_reference TEXT,
    rejection_reason TEXT,
    verified_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS compliance.data_retention_policies (
    id BIGSERIAL PRIMARY KEY,
    entity_type TEXT NOT NULL,
    retention_days INTEGER NOT NULL,
    action TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    description TEXT,
    legal_basis TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS compliance.consents (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    consent_type TEXT NOT NULL,
    granted BOOLEAN NOT NULL DEFAULT TRUE,
    collection_channel TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- gateway schema
-- ============================================================
CREATE SCHEMA IF NOT EXISTS gateway;

CREATE TABLE IF NOT EXISTS gateway.api_keys (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    key_prefix TEXT NOT NULL,
    branch_id BIGINT,
    permissions TEXT NOT NULL,
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS gateway.rate_limit_rules (
    id BIGSERIAL PRIMARY KEY,
    route_pattern TEXT NOT NULL,
    methods TEXT NOT NULL,
    max_requests INTEGER NOT NULL,
    window_seconds INTEGER NOT NULL,
    role TEXT,
    branch_id BIGINT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS gateway.request_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT,
    branch_id BIGINT,
    method TEXT NOT NULL,
    path TEXT NOT NULL,
    status_code INTEGER NOT NULL,
    response_time_ms INTEGER NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    rate_limited BOOLEAN NOT NULL DEFAULT FALSE,
    api_key_id BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- installation schema (supplemental tables)
-- ============================================================
CREATE TABLE IF NOT EXISTS installation.installation_equipment (
    id BIGSERIAL PRIMARY KEY,
    installation_order_id BIGINT NOT NULL REFERENCES installation.installation_orders(id),
    equipment_type TEXT NOT NULL,
    model_name TEXT,
    serial_number TEXT,
    quantity INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'pending',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS installation.installation_photos (
    id BIGSERIAL PRIMARY KEY,
    installation_order_id BIGINT NOT NULL REFERENCES installation.installation_orders(id),
    storage_key TEXT NOT NULL,
    storage_bucket TEXT NOT NULL,
    photo_type TEXT NOT NULL,
    uploaded_by BIGINT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- monitoring schema
-- ============================================================
CREATE SCHEMA IF NOT EXISTS monitoring;

CREATE TABLE IF NOT EXISTS monitoring.alert_rules (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    metric_name TEXT NOT NULL,
    condition TEXT NOT NULL,
    threshold_value DOUBLE PRECISION NOT NULL,
    severity TEXT NOT NULL,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    cooldown_seconds INTEGER NOT NULL DEFAULT 300,
    notification_channels JSONB NOT NULL DEFAULT '[]',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS monitoring.monitoring_alerts (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL,
    branch_id BIGINT NOT NULL,
    alert_rule_id BIGINT,
    alert_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    metric_name TEXT,
    metric_value DOUBLE PRECISION,
    threshold_value DOUBLE PRECISION,
    acknowledged_by BIGINT,
    acknowledged_at TIMESTAMPTZ,
    resolved_by BIGINT,
    resolved_at TIMESTAMPTZ,
    resolution_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS monitoring.metric_records (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL,
    branch_id BIGINT NOT NULL,
    metric_name TEXT NOT NULL,
    metric_value DOUBLE PRECISION NOT NULL,
    unit TEXT,
    tags JSONB,
    recorded_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- payment schema
-- ============================================================
CREATE SCHEMA IF NOT EXISTS payment;

CREATE TABLE IF NOT EXISTS payment.gateway_configs (
    id BIGSERIAL PRIMARY KEY,
    gateway_id TEXT NOT NULL,
    name TEXT NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    credentials JSONB NOT NULL DEFAULT '{}',
    webhook_secret TEXT,
    fee_percentage NUMERIC(5,2) NOT NULL DEFAULT 0,
    fee_fixed NUMERIC(10,2) NOT NULL DEFAULT 0,
    gst_on_fee NUMERIC(5,2) NOT NULL DEFAULT 18,
    supported_methods JSONB NOT NULL DEFAULT '[]',
    currency TEXT NOT NULL DEFAULT 'INR',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS payment.webhook_logs (
    id BIGSERIAL PRIMARY KEY,
    gateway_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'received',
    error_message TEXT,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS payment.payment_links (
    id BIGSERIAL PRIMARY KEY,
    link_id TEXT NOT NULL UNIQUE,
    invoice_id BIGINT NOT NULL,
    customer_id BIGINT NOT NULL,
    branch_id BIGINT NOT NULL,
    amount NUMERIC(12,2) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'INR',
    gateway_id TEXT NOT NULL,
    gateway_order_id TEXT,
    payment_url TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    idempotency_key TEXT NOT NULL UNIQUE,
    metadata JSONB,
    expires_at TIMESTAMPTZ,
    paid_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- referral schema
-- ============================================================
CREATE SCHEMA IF NOT EXISTS referral;

CREATE TABLE IF NOT EXISTS referral.customer_wallets (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL UNIQUE,
    balance NUMERIC(12,2) NOT NULL DEFAULT 0,
    total_earned NUMERIC(12,2) NOT NULL DEFAULT 0,
    total_used NUMERIC(12,2) NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'INR',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS referral.referral_programs (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    reward_type TEXT NOT NULL,
    reward_value NUMERIC(12,2) NOT NULL,
    max_referrals_per_user INTEGER,
    valid_from DATE NOT NULL,
    valid_until DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS referral.referral_tracking (
    id BIGSERIAL PRIMARY KEY,
    program_id BIGINT NOT NULL REFERENCES referral.referral_programs(id),
    referrer_id BIGINT NOT NULL,
    referee_id BIGINT,
    referral_code TEXT NOT NULL,
    referee_phone TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'shared',
    referrer_reward_status TEXT,
    referrer_reward_amount NUMERIC(12,2),
    referee_reward_status TEXT,
    referee_reward_amount NUMERIC(12,2),
    shared_at TIMESTAMPTZ NOT NULL,
    registered_at TIMESTAMPTZ,
    activated_at TIMESTAMPTZ,
    rewarded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS referral.wallet_transactions (
    id BIGSERIAL PRIMARY KEY,
    wallet_id BIGINT NOT NULL REFERENCES referral.customer_wallets(id),
    transaction_type TEXT NOT NULL,
    amount NUMERIC(12,2) NOT NULL,
    reference_id BIGINT,
    reference_type TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- scheduler schema
-- ============================================================
CREATE SCHEMA IF NOT EXISTS scheduler;

CREATE TABLE IF NOT EXISTS scheduler.job_definitions (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    job_type TEXT NOT NULL,
    schedule TEXT NOT NULL,
    target_module TEXT NOT NULL,
    action TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    timeout_seconds INTEGER,
    next_run_at TIMESTAMPTZ,
    last_run_at TIMESTAMPTZ,
    last_run_status TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS scheduler.job_executions (
    id BIGSERIAL PRIMARY KEY,
    job_definition_id BIGINT NOT NULL REFERENCES scheduler.job_definitions(id),
    status TEXT NOT NULL DEFAULT 'running',
    input_payload JSONB NOT NULL DEFAULT '{}',
    output_payload JSONB,
    error_message TEXT,
    duration_ms BIGINT,
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ
);

-- ============================================================
-- workflow schema (supplemental tables)
-- ============================================================
CREATE TABLE IF NOT EXISTS workflow.workflow_instances (
    id BIGSERIAL PRIMARY KEY,
    workflow_type TEXT NOT NULL,
    reference_type TEXT NOT NULL,
    reference_id BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    current_step INTEGER NOT NULL DEFAULT 0,
    total_steps INTEGER NOT NULL,
    input_data JSONB NOT NULL DEFAULT '{}',
    output_data JSONB,
    error_message TEXT,
    initiated_by BIGINT,
    branch_id BIGINT,
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS workflow.workflow_steps (
    id BIGSERIAL PRIMARY KEY,
    workflow_instance_id BIGINT NOT NULL REFERENCES workflow.workflow_instances(id),
    step_name TEXT NOT NULL,
    step_order INTEGER NOT NULL,
    target_module TEXT NOT NULL,
    action TEXT NOT NULL,
    input_payload JSONB NOT NULL DEFAULT '{}',
    output_payload JSONB,
    status TEXT NOT NULL DEFAULT 'pending',
    error_message TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    compensation_action TEXT,
    compensation_executed BOOLEAN NOT NULL DEFAULT FALSE,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- bandwidth schema (supplemental table)
-- ============================================================
CREATE TABLE IF NOT EXISTS bandwidth.bandwidth_policies (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    policy_type TEXT NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    priority INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    review_status TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- branches schema (supplemental table)
-- NOTE: user_branches already exists from migration 003 and is
-- moved to branches schema by migration 019. No new table needed.
-- ============================================================

-- ============================================================
-- notification schema (supplemental table)
-- ============================================================
CREATE TABLE IF NOT EXISTS notification.notification_delivery_history (
    id BIGSERIAL PRIMARY KEY,
    notification_id BIGINT NOT NULL REFERENCES notification.notifications(id),
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- Indexes for new tables
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_kyc_verifications_customer ON compliance.kyc_verifications(customer_id);
CREATE INDEX IF NOT EXISTS idx_kyc_verifications_status ON compliance.kyc_verifications(status);
CREATE INDEX IF NOT EXISTS idx_consents_customer ON compliance.consents(customer_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_hash ON gateway.api_keys(key_hash);
CREATE INDEX IF NOT EXISTS idx_api_keys_prefix ON gateway.api_keys(key_prefix);
CREATE INDEX IF NOT EXISTS idx_request_logs_user ON gateway.request_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_request_logs_created ON gateway.request_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_installation_equipment_order ON installation.installation_equipment(installation_order_id);
CREATE INDEX IF NOT EXISTS idx_installation_photos_order ON installation.installation_photos(installation_order_id);
CREATE INDEX IF NOT EXISTS idx_monitoring_alerts_device ON monitoring.monitoring_alerts(device_id);
CREATE INDEX IF NOT EXISTS idx_monitoring_alerts_status ON monitoring.monitoring_alerts(status);
CREATE INDEX IF NOT EXISTS idx_metric_records_device ON monitoring.metric_records(device_id);
CREATE INDEX IF NOT EXISTS idx_metric_records_recorded ON monitoring.metric_records(recorded_at);
CREATE INDEX IF NOT EXISTS idx_webhook_logs_gateway ON payment.webhook_logs(gateway_id);
CREATE INDEX IF NOT EXISTS idx_payment_links_invoice ON payment.payment_links(invoice_id);
CREATE INDEX IF NOT EXISTS idx_payment_links_customer ON payment.payment_links(customer_id);
CREATE INDEX IF NOT EXISTS idx_referral_tracking_code ON referral.referral_tracking(referral_code);
CREATE INDEX IF NOT EXISTS idx_referral_tracking_referrer ON referral.referral_tracking(referrer_id);
CREATE INDEX IF NOT EXISTS idx_wallet_transactions_wallet ON referral.wallet_transactions(wallet_id);
CREATE INDEX IF NOT EXISTS idx_job_definitions_next_run ON scheduler.job_definitions(next_run_at);
CREATE INDEX IF NOT EXISTS idx_job_executions_job ON scheduler.job_executions(job_definition_id);
CREATE INDEX IF NOT EXISTS idx_workflow_instances_ref ON workflow.workflow_instances(reference_type, reference_id);
CREATE INDEX IF NOT EXISTS idx_workflow_steps_instance ON workflow.workflow_steps(workflow_instance_id);
CREATE INDEX IF NOT EXISTS idx_delivery_history_notification ON notification.notification_delivery_history(notification_id);
