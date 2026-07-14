-- Migration: add_new_modules
-- Description: Creates tables for CRM, Reporting, Monitoring, Traffic, Automation, Scheduler, and Workflow modules
-- Up

-- ── CRM ─────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS crm_interactions (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    interaction_type VARCHAR(30) NOT NULL,
    subject TEXT NOT NULL,
    body TEXT,
    channel VARCHAR(20) NOT NULL,
    duration_seconds INTEGER,
    sentiment VARCHAR(20),
    follow_up_date DATE,
    follow_up_done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_notes (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal',
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_tags (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    name VARCHAR(50) NOT NULL,
    color VARCHAR(7),
    category VARCHAR(30) NOT NULL,
    usage_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_customer_tags (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    tag_id BIGINT NOT NULL REFERENCES crm_tags(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_segments (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    criteria JSONB NOT NULL,
    customer_count BIGINT NOT NULL DEFAULT 0,
    is_dynamic BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_customer_segments (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    segment_id BIGINT NOT NULL REFERENCES crm_segments(id),
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── Reporting ───────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS reports (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    report_type VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    parameters JSONB,
    result JSONB,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    file_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS report_schedules (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    report_type VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    parameters JSONB,
    frequency VARCHAR(20) NOT NULL,
    next_run_at TIMESTAMPTZ,
    last_run_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── Monitoring ──────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS monitoring_health_checks (
    id BIGSERIAL PRIMARY KEY,
    service_name VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    response_time_ms INTEGER,
    error_message TEXT,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS monitoring_metrics (
    id BIGSERIAL PRIMARY KEY,
    metric_name VARCHAR(50) NOT NULL,
    metric_type VARCHAR(30) NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    tags JSONB,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS monitoring_alert_rules (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    service_name VARCHAR(50) NOT NULL,
    metric_name VARCHAR(30) NOT NULL,
    operator VARCHAR(10) NOT NULL,
    threshold DOUBLE PRECISION NOT NULL,
    severity VARCHAR(20) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS monitoring_alerts (
    id BIGSERIAL PRIMARY KEY,
    rule_id BIGINT REFERENCES monitoring_alert_rules(id),
    service_name VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    message TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    acknowledged_by BIGINT REFERENCES users(id),
    acknowledged_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── Traffic ─────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS traffic_samples (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT REFERENCES customers(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    branch_id BIGINT REFERENCES branches(id),
    interface_name VARCHAR(100),
    bytes_in BIGINT NOT NULL DEFAULT 0,
    bytes_out BIGINT NOT NULL DEFAULT 0,
    packets_in BIGINT NOT NULL DEFAULT 0,
    packets_out BIGINT NOT NULL DEFAULT 0,
    sample_duration_seconds INTEGER NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS traffic_policies (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    criteria JSONB NOT NULL,
    action JSONB NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS traffic_aggregates (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT REFERENCES customers(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    branch_id BIGINT REFERENCES branches(id),
    period VARCHAR(20) NOT NULL,
    total_bytes_in BIGINT NOT NULL DEFAULT 0,
    total_bytes_out BIGINT NOT NULL DEFAULT 0,
    peak_bytes_in BIGINT NOT NULL DEFAULT 0,
    peak_bytes_out BIGINT NOT NULL DEFAULT 0,
    sample_count BIGINT NOT NULL DEFAULT 0,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── Automation ──────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS automation_rules (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS automation_triggers (
    id BIGSERIAL PRIMARY KEY,
    rule_id BIGINT NOT NULL REFERENCES automation_rules(id),
    trigger_type VARCHAR(50) NOT NULL,
    config JSONB NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS automation_actions (
    id BIGSERIAL PRIMARY KEY,
    rule_id BIGINT NOT NULL REFERENCES automation_rules(id),
    action_type VARCHAR(50) NOT NULL,
    config JSONB NOT NULL,
    order_index INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS automation_executions (
    id BIGSERIAL PRIMARY KEY,
    rule_id BIGINT NOT NULL REFERENCES automation_rules(id),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    trigger_data JSONB,
    result JSONB,
    error_message TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- ── Scheduler ───────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS scheduled_tasks (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    task_type VARCHAR(50) NOT NULL,
    config JSONB,
    schedule_type VARCHAR(20) NOT NULL,
    schedule_value VARCHAR(50) NOT NULL,
    next_run_at TIMESTAMPTZ,
    last_run_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS task_executions (
    id BIGSERIAL PRIMARY KEY,
    task_id BIGINT NOT NULL REFERENCES scheduled_tasks(id),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    result JSONB,
    error_message TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    duration_ms BIGINT
);

-- ── Workflow ────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS workflow_definitions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    entity_type VARCHAR(50) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS workflow_steps (
    id BIGSERIAL PRIMARY KEY,
    definition_id BIGINT NOT NULL REFERENCES workflow_definitions(id),
    name VARCHAR(50) NOT NULL,
    step_type VARCHAR(30) NOT NULL,
    step_order INTEGER NOT NULL,
    required_role VARCHAR(50),
    config JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS workflow_instances (
    id BIGSERIAL PRIMARY KEY,
    definition_id BIGINT NOT NULL REFERENCES workflow_definitions(id),
    branch_id BIGINT REFERENCES branches(id),
    entity_id BIGINT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'in_progress',
    current_step_index INTEGER NOT NULL DEFAULT 0,
    started_by BIGINT NOT NULL REFERENCES users(id),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS workflow_step_instances (
    id BIGSERIAL PRIMARY KEY,
    instance_id BIGINT NOT NULL REFERENCES workflow_instances(id),
    step_id BIGINT NOT NULL REFERENCES workflow_steps(id),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    assigned_to BIGINT REFERENCES users(id),
    decided_by BIGINT REFERENCES users(id),
    decision VARCHAR(20),
    comments TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- ── Indexes ─────────────────────────────────────────────────

-- CRM
CREATE INDEX IF NOT EXISTS idx_crm_interactions_customer_id ON crm_interactions(customer_id);
CREATE INDEX IF NOT EXISTS idx_crm_interactions_branch_id ON crm_interactions(branch_id);
CREATE INDEX IF NOT EXISTS idx_crm_interactions_user_id ON crm_interactions(user_id);
CREATE INDEX IF NOT EXISTS idx_crm_interactions_type ON crm_interactions(interaction_type);
CREATE INDEX IF NOT EXISTS idx_crm_notes_customer_id ON crm_notes(customer_id);
CREATE INDEX IF NOT EXISTS idx_crm_notes_branch_id ON crm_notes(branch_id);
CREATE INDEX IF NOT EXISTS idx_crm_customer_tags_customer_id ON crm_customer_tags(customer_id);
CREATE INDEX IF NOT EXISTS idx_crm_customer_tags_tag_id ON crm_customer_tags(tag_id);
CREATE INDEX IF NOT EXISTS idx_crm_segments_branch_id ON crm_segments(branch_id);
CREATE INDEX IF NOT EXISTS idx_crm_customer_segments_customer_id ON crm_customer_segments(customer_id);
CREATE INDEX IF NOT EXISTS idx_crm_customer_segments_segment_id ON crm_customer_segments(segment_id);

-- Reporting
CREATE INDEX IF NOT EXISTS idx_reports_branch_id ON reports(branch_id);
CREATE INDEX IF NOT EXISTS idx_reports_user_id ON reports(user_id);
CREATE INDEX IF NOT EXISTS idx_reports_type ON reports(report_type);
CREATE INDEX IF NOT EXISTS idx_report_schedules_branch_id ON report_schedules(branch_id);
CREATE INDEX IF NOT EXISTS idx_report_schedules_next_run ON report_schedules(next_run_at);

-- Monitoring
CREATE INDEX IF NOT EXISTS idx_monitoring_health_checks_service ON monitoring_health_checks(service_name);
CREATE INDEX IF NOT EXISTS idx_monitoring_health_checks_checked_at ON monitoring_health_checks(checked_at);
CREATE INDEX IF NOT EXISTS idx_monitoring_metrics_name ON monitoring_metrics(metric_name);
CREATE INDEX IF NOT EXISTS idx_monitoring_metrics_recorded_at ON monitoring_metrics(recorded_at);
CREATE INDEX IF NOT EXISTS idx_monitoring_alerts_rule_id ON monitoring_alerts(rule_id);
CREATE INDEX IF NOT EXISTS idx_monitoring_alerts_status ON monitoring_alerts(status);
CREATE INDEX IF NOT EXISTS idx_monitoring_alerts_severity ON monitoring_alerts(severity);
CREATE INDEX IF NOT EXISTS idx_monitoring_alerts_service ON monitoring_alerts(service_name);

-- Traffic
CREATE INDEX IF NOT EXISTS idx_traffic_samples_customer_id ON traffic_samples(customer_id);
CREATE INDEX IF NOT EXISTS idx_traffic_samples_branch_id ON traffic_samples(branch_id);
CREATE INDEX IF NOT EXISTS idx_traffic_samples_recorded_at ON traffic_samples(recorded_at);
CREATE INDEX IF NOT EXISTS idx_traffic_policies_branch_id ON traffic_policies(branch_id);
CREATE INDEX IF NOT EXISTS idx_traffic_aggregates_customer_id ON traffic_aggregates(customer_id);
CREATE INDEX IF NOT EXISTS idx_traffic_aggregates_period ON traffic_aggregates(period);
CREATE INDEX IF NOT EXISTS idx_traffic_aggregates_period_start ON traffic_aggregates(period_start);

-- Automation
CREATE INDEX IF NOT EXISTS idx_automation_rules_branch_id ON automation_rules(branch_id);
CREATE INDEX IF NOT EXISTS idx_automation_triggers_rule_id ON automation_triggers(rule_id);
CREATE INDEX IF NOT EXISTS idx_automation_actions_rule_id ON automation_actions(rule_id);
CREATE INDEX IF NOT EXISTS idx_automation_executions_rule_id ON automation_executions(rule_id);
CREATE INDEX IF NOT EXISTS idx_automation_executions_status ON automation_executions(status);

-- Scheduler
CREATE INDEX IF NOT EXISTS idx_scheduled_tasks_branch_id ON scheduled_tasks(branch_id);
CREATE INDEX IF NOT EXISTS idx_scheduled_tasks_next_run ON scheduled_tasks(next_run_at);
CREATE INDEX IF NOT EXISTS idx_task_executions_task_id ON task_executions(task_id);
CREATE INDEX IF NOT EXISTS idx_task_executions_status ON task_executions(status);

-- Workflow
CREATE INDEX IF NOT EXISTS idx_workflow_definitions_branch_id ON workflow_definitions(branch_id);
CREATE INDEX IF NOT EXISTS idx_workflow_steps_definition_id ON workflow_steps(definition_id);
CREATE INDEX IF NOT EXISTS idx_workflow_instances_definition_id ON workflow_instances(definition_id);
CREATE INDEX IF NOT EXISTS idx_workflow_instances_branch_id ON workflow_instances(branch_id);
CREATE INDEX IF NOT EXISTS idx_workflow_instances_status ON workflow_instances(status);
CREATE INDEX IF NOT EXISTS idx_workflow_step_instances_instance_id ON workflow_step_instances(instance_id);
CREATE INDEX IF NOT EXISTS idx_workflow_step_instances_step_id ON workflow_step_instances(step_id);

-- Down (reverse order for FK deps)
-- DROP TABLE IF EXISTS workflow_step_instances;
-- DROP TABLE IF EXISTS workflow_instances;
-- DROP TABLE IF EXISTS workflow_steps;
-- DROP TABLE IF EXISTS workflow_definitions;
-- DROP TABLE IF EXISTS task_executions;
-- DROP TABLE IF EXISTS scheduled_tasks;
-- DROP TABLE IF EXISTS automation_executions;
-- DROP TABLE IF EXISTS automation_actions;
-- DROP TABLE IF EXISTS automation_triggers;
-- DROP TABLE IF EXISTS automation_rules;
-- DROP TABLE IF EXISTS traffic_aggregates;
-- DROP TABLE IF EXISTS traffic_policies;
-- DROP TABLE IF EXISTS traffic_samples;
-- DROP TABLE IF EXISTS monitoring_alerts;
-- DROP TABLE IF EXISTS monitoring_alert_rules;
-- DROP TABLE IF EXISTS monitoring_metrics;
-- DROP TABLE IF EXISTS monitoring_health_checks;
-- DROP TABLE IF EXISTS report_schedules;
-- DROP TABLE IF EXISTS reports;
-- DROP TABLE IF EXISTS crm_customer_segments;
-- DROP TABLE IF EXISTS crm_segments;
-- DROP TABLE IF EXISTS crm_customer_tags;
-- DROP TABLE IF EXISTS crm_tags;
-- DROP TABLE IF EXISTS crm_notes;
-- DROP TABLE IF EXISTS crm_interactions;
