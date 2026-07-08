-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — History Tables for Critical Entities
-- ═══════════════════════════════════════════════════════════════

-- ── Customer History ─────────────────────────────────────────
CREATE TABLE customer_history (
    id              BIGSERIAL PRIMARY KEY,
    customer_id     BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_customer_history_customer ON customer_history(customer_id);

-- ── Subscription History ─────────────────────────────────────
CREATE TABLE subscription_history (
    id                  BIGSERIAL PRIMARY KEY,
    subscription_id     BIGINT NOT NULL,
    action              VARCHAR(20) NOT NULL,
    old_data            JSONB,
    new_data            JSONB,
    performed_by        BIGINT REFERENCES users(id),
    performed_at        TIMESTAMPTZ DEFAULT NOW(),
    reason              TEXT
);

CREATE INDEX idx_subscription_history_subscription ON subscription_history(subscription_id);

-- ── Invoice History ──────────────────────────────────────────
CREATE TABLE invoice_history (
    id              BIGSERIAL PRIMARY KEY,
    invoice_id      BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_invoice_history_invoice ON invoice_history(invoice_id);

-- ── Device History ───────────────────────────────────────────
CREATE TABLE device_history (
    id              BIGSERIAL PRIMARY KEY,
    device_id       BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_device_history_device ON device_history(device_id);

-- ── Ticket History ───────────────────────────────────────────
CREATE TABLE ticket_history (
    id              BIGSERIAL PRIMARY KEY,
    ticket_id       BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_ticket_history_ticket ON ticket_history(ticket_id);

-- ── Refund History ───────────────────────────────────────────
CREATE TABLE refund_history (
    id              BIGSERIAL PRIMARY KEY,
    refund_id       BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_refund_history_refund ON refund_history(refund_id);

-- ── Discount History ─────────────────────────────────────────
CREATE TABLE discount_history (
    id              BIGSERIAL PRIMARY KEY,
    discount_id     BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_discount_history_discount ON discount_history(discount_id);

-- ── Bandwidth Profile History ────────────────────────────────
CREATE TABLE bandwidth_profile_history (
    id                  BIGSERIAL PRIMARY KEY,
    profile_id          BIGINT NOT NULL,
    action              VARCHAR(20) NOT NULL,
    old_data            JSONB,
    new_data            JSONB,
    performed_by        BIGINT REFERENCES users(id),
    performed_at        TIMESTAMPTZ DEFAULT NOW(),
    reason              TEXT
);

CREATE INDEX idx_bandwidth_profile_history_profile ON bandwidth_profile_history(profile_id);

-- ── Payment History ──────────────────────────────────────────
CREATE TABLE payment_history (
    id              BIGSERIAL PRIMARY KEY,
    payment_id      BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_payment_history_payment ON payment_history(payment_id);

-- ── Installation History ─────────────────────────────────────
CREATE TABLE installation_history (
    id                  BIGSERIAL PRIMARY KEY,
    installation_id     BIGINT NOT NULL,
    action              VARCHAR(20) NOT NULL,
    old_data            JSONB,
    new_data            JSONB,
    performed_by        BIGINT REFERENCES users(id),
    performed_at        TIMESTAMPTZ DEFAULT NOW(),
    reason              TEXT
);

CREATE INDEX idx_installation_history_installation ON installation_history(installation_id);

-- ── Lead History ─────────────────────────────────────────────
CREATE TABLE lead_history (
    id              BIGSERIAL PRIMARY KEY,
    lead_id         BIGINT NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    BIGINT REFERENCES users(id),
    performed_at    TIMESTAMPTZ DEFAULT NOW(),
    reason          TEXT
);

CREATE INDEX idx_lead_history_lead ON lead_history(lead_id);
