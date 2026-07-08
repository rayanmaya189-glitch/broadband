-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — Billing, Tickets, Leads
-- ═══════════════════════════════════════════════════════════════

-- ── Tickets ──────────────────────────────────────────────────
CREATE TABLE tickets (
    id                  BIGSERIAL PRIMARY KEY,
    ticket_number       VARCHAR(20) NOT NULL UNIQUE,
    branch_id           BIGINT NOT NULL REFERENCES branches(id),
    customer_id         BIGINT REFERENCES customers(id),
    subscription_id     BIGINT REFERENCES subscriptions(id),
    created_by          BIGINT NOT NULL REFERENCES users(id),
    assigned_to         BIGINT REFERENCES users(id),
    escalated_to        BIGINT REFERENCES users(id),
    category            VARCHAR(50) NOT NULL,
    subcategory         VARCHAR(50),
    priority            VARCHAR(10) DEFAULT 'medium'
        CHECK (priority IN ('critical', 'high', 'medium', 'low')),
    status              VARCHAR(30) DEFAULT 'open'
        CHECK (status IN ('open', 'assigned', 'in_progress', 'waiting_customer',
                          'escalated', 'resolved', 'closed', 'reopened')),
    subject             VARCHAR(255) NOT NULL,
    description         TEXT NOT NULL,
    source              VARCHAR(20) DEFAULT 'customer'
        CHECK (source IN ('customer', 'phone', 'email', 'whatsapp', 'agent', 'system')),
    resolution_notes    TEXT,
    sla_response_at     TIMESTAMPTZ,
    sla_resolution_at   TIMESTAMPTZ,
    first_response_at   TIMESTAMPTZ,
    resolved_at         TIMESTAMPTZ,
    closed_at           TIMESTAMPTZ,
    reopen_count        INTEGER DEFAULT 0,
    satisfaction_rating INTEGER CHECK (satisfaction_rating BETWEEN 1 AND 5),
    satisfaction_feedback TEXT,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_tickets_branch ON tickets(branch_id);
CREATE INDEX idx_tickets_customer ON tickets(customer_id);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_priority ON tickets(priority);
CREATE INDEX idx_tickets_assigned ON tickets(assigned_to);
CREATE INDEX idx_tickets_sla ON tickets(sla_resolution_at) WHERE status NOT IN ('resolved', 'closed');

-- ── Ticket Comments ──────────────────────────────────────────
CREATE TABLE ticket_comments (
    id          BIGSERIAL PRIMARY KEY,
    ticket_id   BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    user_id     BIGINT REFERENCES users(id),
    is_customer BOOLEAN DEFAULT FALSE,
    comment     TEXT NOT NULL,
    is_internal BOOLEAN DEFAULT FALSE,
    attachments JSONB,
    created_at  TIMESTAMPTZ DEFAULT NOW(),
    updated_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_ticket_comments_ticket ON ticket_comments(ticket_id);

-- ── Leads ────────────────────────────────────────────────────
CREATE TABLE leads (
    id                      BIGSERIAL PRIMARY KEY,
    branch_id               BIGINT NOT NULL REFERENCES branches(id),
    assigned_to             BIGINT REFERENCES users(id),
    name                    VARCHAR(255) NOT NULL,
    phone                   VARCHAR(20) NOT NULL,
    email                   VARCHAR(255),
    source                  VARCHAR(50) NOT NULL
        CHECK (source IN ('landing_page', 'whatsapp', 'referral', 'walk_in',
                          'cold_call', 'social_media', 'field_visit')),
    status                  VARCHAR(30) DEFAULT 'new'
        CHECK (status IN ('new', 'contacted', 'interested', 'surveyed',
                          'quoted', 'converted', 'lost')),
    interested_plan_id      BIGINT REFERENCES plans(id),
    estimated_install_date  DATE,
    address                 TEXT,
    latitude                DECIMAL(10, 7),
    longitude               DECIMAL(10, 7),
    lost_reason             TEXT,
    notes                   TEXT,
    converted_customer_id   BIGINT REFERENCES customers(id),
    converted_at            TIMESTAMPTZ,
    created_at              TIMESTAMPTZ DEFAULT NOW(),
    updated_at              TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_leads_branch ON leads(branch_id);
CREATE INDEX idx_leads_status ON leads(status);
CREATE INDEX idx_leads_assigned ON leads(assigned_to);
CREATE INDEX idx_leads_source ON leads(source);

-- ── Lead Activities ──────────────────────────────────────────
CREATE TABLE lead_activities (
    id              BIGSERIAL PRIMARY KEY,
    lead_id         BIGINT NOT NULL REFERENCES leads(id) ON DELETE CASCADE,
    activity_type   VARCHAR(30) NOT NULL
        CHECK (activity_type IN ('call', 'whatsapp', 'visit', 'email', 'note', 'status_change')),
    description     TEXT NOT NULL,
    performed_by    BIGINT NOT NULL REFERENCES users(id),
    scheduled_at    TIMESTAMPTZ,
    completed_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_lead_activities_lead ON lead_activities(lead_id);

-- ── Invoices ─────────────────────────────────────────────────
CREATE TABLE invoices (
    id                      BIGSERIAL PRIMARY KEY,
    invoice_number          VARCHAR(20) NOT NULL UNIQUE,
    customer_id             BIGINT NOT NULL REFERENCES customers(id),
    branch_id               BIGINT NOT NULL REFERENCES branches(id),
    subscription_id         BIGINT NOT NULL REFERENCES subscriptions(id),
    billing_period_start    DATE NOT NULL,
    billing_period_end      DATE NOT NULL,
    subtotal                DECIMAL(10,2) NOT NULL,
    discount_amount         DECIMAL(10,2) DEFAULT 0,
    tax_amount              DECIMAL(10,2) DEFAULT 0,
    total_amount            DECIMAL(10,2) NOT NULL,
    currency                VARCHAR(3) DEFAULT 'INR',
    status                  VARCHAR(20) DEFAULT 'draft'
        CHECK (status IN ('draft', 'pending', 'sent', 'paid', 'partial', 'overdue', 'void', 'refunded')),
    due_date                DATE NOT NULL,
    paid_at                 TIMESTAMPTZ,
    payment_method          VARCHAR(50),
    payment_reference       VARCHAR(255),
    created_by              BIGINT REFERENCES users(id),
    notes                   TEXT,
    created_at              TIMESTAMPTZ DEFAULT NOW(),
    updated_at              TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_invoices_customer ON invoices(customer_id);
CREATE INDEX idx_invoices_branch ON invoices(branch_id);
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_invoices_due ON invoices(due_date) WHERE status NOT IN ('paid', 'void', 'refunded');

-- ── Invoice Line Items ───────────────────────────────────────
CREATE TABLE invoice_line_items (
    id              BIGSERIAL PRIMARY KEY,
    invoice_id      BIGINT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description     TEXT NOT NULL,
    quantity        DECIMAL(10,2) DEFAULT 1,
    unit_price      DECIMAL(10,2) NOT NULL,
    amount          DECIMAL(10,2) NOT NULL,
    tax_rate        DECIMAL(5,2) DEFAULT 0,
    tax_amount      DECIMAL(10,2) DEFAULT 0,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_invoice_line_items_invoice ON invoice_line_items(invoice_id);

-- ── Payments ─────────────────────────────────────────────────
CREATE TABLE payments (
    id                      BIGSERIAL PRIMARY KEY,
    payment_number          VARCHAR(20) NOT NULL UNIQUE,
    invoice_id              BIGINT NOT NULL REFERENCES invoices(id),
    customer_id             BIGINT NOT NULL REFERENCES customers(id),
    branch_id               BIGINT NOT NULL REFERENCES branches(id),
    amount                  DECIMAL(10,2) NOT NULL,
    currency                VARCHAR(3) DEFAULT 'INR',
    payment_method          VARCHAR(50) NOT NULL,
    payment_gateway         VARCHAR(50),
    gateway_transaction_id  VARCHAR(255),
    status                  VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'refunded')),
    processed_at            TIMESTAMPTZ,
    created_at              TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_payments_invoice ON payments(invoice_id);
CREATE INDEX idx_payments_customer ON payments(customer_id);
CREATE INDEX idx_payments_status ON payments(status);

-- ── Refunds ──────────────────────────────────────────────────
CREATE TABLE refunds (
    id              BIGSERIAL PRIMARY KEY,
    refund_number   VARCHAR(20) NOT NULL UNIQUE,
    payment_id      BIGINT NOT NULL REFERENCES payments(id),
    invoice_id      BIGINT NOT NULL REFERENCES invoices(id),
    customer_id     BIGINT NOT NULL REFERENCES customers(id),
    amount          DECIMAL(10,2) NOT NULL,
    reason          TEXT NOT NULL,
    requested_by    BIGINT REFERENCES users(id),
    approved_by     BIGINT REFERENCES users(id),
    approved_at     TIMESTAMPTZ,
    status          VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'processed', 'rejected')),
    processed_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

-- ── Discounts ────────────────────────────────────────────────
CREATE TABLE discounts (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    code            VARCHAR(50) UNIQUE,
    type            VARCHAR(20) NOT NULL CHECK (type IN ('percentage', 'fixed')),
    value           DECIMAL(10,2) NOT NULL,
    max_uses        INTEGER,
    current_uses    INTEGER DEFAULT 0,
    valid_from      DATE NOT NULL,
    valid_until     DATE NOT NULL,
    is_active       BOOLEAN DEFAULT TRUE,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

-- ── Referral Programs ────────────────────────────────────────
CREATE TABLE referral_programs (
    id                          BIGSERIAL PRIMARY KEY,
    name                        VARCHAR(100) NOT NULL,
    status                      VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'paused', 'ended')),
    referrer_reward_type        VARCHAR(20) NOT NULL,
    referrer_reward_value       DECIMAL(10,2) NOT NULL,
    referee_reward_type         VARCHAR(20) NOT NULL,
    referee_reward_value        DECIMAL(10,2) NOT NULL,
    max_referrals_per_customer  INTEGER DEFAULT 10,
    start_date                  DATE NOT NULL,
    end_date                    DATE NOT NULL,
    created_at                  TIMESTAMPTZ DEFAULT NOW()
);

-- ── Referral Tracking ────────────────────────────────────────
CREATE TABLE referral_tracking (
    id                  BIGSERIAL PRIMARY KEY,
    program_id          BIGINT NOT NULL REFERENCES referral_programs(id),
    referrer_id         BIGINT NOT NULL REFERENCES customers(id),
    referee_id          BIGINT REFERENCES customers(id),
    referral_code       VARCHAR(10) NOT NULL,
    referee_phone       VARCHAR(15) NOT NULL,
    status              VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'registered', 'activated', 'rewarded')),
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_referral_tracking_referrer ON referral_tracking(referrer_id);
CREATE INDEX idx_referral_tracking_code ON referral_tracking(referral_code);

-- ── Notification Templates ───────────────────────────────────
CREATE TABLE notification_templates (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL UNIQUE,
    channel         VARCHAR(20) NOT NULL
        CHECK (channel IN ('email', 'sms', 'whatsapp', 'push', 'in_app')),
    subject_template TEXT,
    body_template   TEXT NOT NULL,
    is_active       BOOLEAN DEFAULT TRUE,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

-- ── Notifications ────────────────────────────────────────────
CREATE TABLE notifications (
    id                  BIGSERIAL PRIMARY KEY,
    channel             VARCHAR(20) NOT NULL,
    recipient_type      VARCHAR(20) NOT NULL,
    recipient_id        BIGINT NOT NULL,
    recipient_address   VARCHAR(255) NOT NULL,
    subject             TEXT,
    body                TEXT NOT NULL,
    status              VARCHAR(20) DEFAULT 'queued'
        CHECK (status IN ('queued', 'sent', 'delivered', 'failed', 'retrying')),
    retry_count         INTEGER DEFAULT 0,
    last_error          TEXT,
    sent_at             TIMESTAMPTZ,
    delivered_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_recipient ON notifications(recipient_id);

-- ── Events ───────────────────────────────────────────────────
CREATE TABLE events (
    id                      BIGSERIAL PRIMARY KEY,
    event_type              VARCHAR(100) NOT NULL,
    aggregate_type          VARCHAR(50) NOT NULL,
    aggregate_id            BIGINT NOT NULL,
    payload                 JSONB NOT NULL,
    caused_by_user_id       BIGINT REFERENCES users(id),
    caused_by_branch_id     BIGINT REFERENCES branches(id),
    sequence_number         BIGSERIAL,
    published_at            TIMESTAMPTZ DEFAULT NOW(),
    processed               BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_aggregate ON events(aggregate_type, aggregate_id);
CREATE INDEX idx_events_processed ON events(processed) WHERE processed = false;

-- ── Document Files ───────────────────────────────────────────
CREATE TABLE document_files (
    id                  BIGSERIAL PRIMARY KEY,
    filename            VARCHAR(255) NOT NULL,
    original_filename   VARCHAR(255) NOT NULL,
    mime_type           VARCHAR(100) NOT NULL,
    file_size           BIGINT NOT NULL,
    storage_bucket      VARCHAR(100) NOT NULL,
    storage_key         VARCHAR(500) NOT NULL,
    uploaded_by         BIGINT NOT NULL REFERENCES users(id),
    entity_type         VARCHAR(50),
    entity_id           BIGINT,
    status              VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'deleted', 'expired')),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_document_files_entity ON document_files(entity_type, entity_id);
