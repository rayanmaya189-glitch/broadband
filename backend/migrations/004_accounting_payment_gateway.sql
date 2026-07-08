-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — Accounting & Payment Gateway
-- ═══════════════════════════════════════════════════════════════

-- ── Chart of Accounts ────────────────────────────────────────
CREATE TABLE chart_of_accounts (
    id              BIGSERIAL PRIMARY KEY,
    code            VARCHAR(20) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    account_type    VARCHAR(30) NOT NULL
        CHECK (account_type IN ('asset', 'liability', 'equity', 'revenue', 'expense')),
    parent_id       BIGINT REFERENCES chart_of_accounts(id),
    is_group        BOOLEAN DEFAULT FALSE,
    is_active       BOOLEAN DEFAULT TRUE,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

-- ── Journal Entries ──────────────────────────────────────────
CREATE TABLE journal_entries (
    id                  BIGSERIAL PRIMARY KEY,
    entry_number        VARCHAR(30) NOT NULL UNIQUE,
    entry_date          DATE NOT NULL,
    description         TEXT NOT NULL,
    reference_type      VARCHAR(50),
    reference_id        BIGINT,
    total_debit         DECIMAL(12,2) NOT NULL,
    total_credit        DECIMAL(12,2) NOT NULL,
    status              VARCHAR(20) DEFAULT 'draft'
        CHECK (status IN ('draft', 'posted', 'voided')),
    created_by          BIGINT REFERENCES users(id),
    posted_at           TIMESTAMPTZ,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_journal_entries_date ON journal_entries(entry_date);
CREATE INDEX idx_journal_entries_status ON journal_entries(status);

-- ── Journal Entry Lines ──────────────────────────────────────
CREATE TABLE journal_entry_lines (
    id                  BIGSERIAL PRIMARY KEY,
    journal_entry_id    BIGINT NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id          BIGINT NOT NULL REFERENCES chart_of_accounts(id),
    debit               DECIMAL(12,2) DEFAULT 0,
    credit              DECIMAL(12,2) DEFAULT 0,
    description         TEXT,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    CHECK (debit >= 0 AND credit >= 0),
    CHECK (debit > 0 OR credit > 0)
);

CREATE INDEX idx_journal_entry_lines_entry ON journal_entry_lines(journal_entry_id);

-- ── Payment Gateways ─────────────────────────────────────────
CREATE TABLE payment_gateways (
    id              BIGSERIAL PRIMARY KEY,
    gateway_id      VARCHAR(50) NOT NULL UNIQUE,
    name            VARCHAR(100) NOT NULL,
    is_primary      BOOLEAN DEFAULT FALSE,
    is_active       BOOLEAN DEFAULT TRUE,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

-- ── Seed Chart of Accounts ───────────────────────────────────
INSERT INTO chart_of_accounts (code, name, account_type, is_group) VALUES
    -- Assets
    ('1000', 'Cash in Hand', 'asset', false),
    ('1100', 'Bank Accounts', 'asset', false),
    ('1200', 'Accounts Receivable', 'asset', false),
    ('1300', 'Prepaid Expenses', 'asset', false),
    ('1400', 'Inventory (Hardware)', 'asset', false),
    ('1500', 'Fixed Assets', 'asset', false),
    -- Liabilities
    ('2000', 'Accounts Payable', 'liability', false),
    ('2100', 'GST Payable (Output)', 'liability', false),
    ('2200', 'GST Receivable (Input)', 'liability', false),
    ('2300', 'Customer Wallet Balance', 'liability', false),
    -- Equity
    ('3000', "Owner's Equity", 'equity', false),
    ('3100', 'Retained Earnings', 'equity', false),
    -- Revenue
    ('4000', 'Subscription Revenue', 'revenue', false),
    ('4100', 'Installation Revenue', 'revenue', false),
    ('4200', 'Hardware Sales Revenue', 'revenue', false),
    ('4300', 'Priority Support Revenue', 'revenue', false),
    ('4400', 'Late Fee Revenue', 'revenue', false),
    -- Expenses
    ('5000', 'Bandwidth Cost', 'expense', false),
    ('5100', 'Hardware Cost', 'expense', false),
    ('5200', 'Staff Salaries', 'expense', false),
    ('5300', 'Marketing Expense', 'expense', false),
    ('5400', 'Referral Rewards', 'expense', false),
    ('5500', 'Payment Gateway Fees', 'expense', false),
    ('5600', 'Depreciation', 'expense', false),
    ('5700', 'Utilities', 'expense', false);

-- ── Seed Payment Gateways ────────────────────────────────────
INSERT INTO payment_gateways (gateway_id, name, is_primary, is_active) VALUES
    ('razorpay', 'Razorpay', true, true),
    ('payu', 'PayU', false, false),
    ('instamojo', 'InstaMojo', false, false);
