-- ============================================================
-- Phase 4 Enhancements: Notifications, Referrals, Accounting, Discovery
-- ============================================================

-- ── Notification Channels ───────────────────────────────────
CREATE TABLE IF NOT EXISTS notification_channels (
    id BIGSERIAL PRIMARY KEY,
    channel VARCHAR(20) NOT NULL UNIQUE,
    provider VARCHAR(50) NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Notification History ────────────────────────────────────
CREATE TABLE IF NOT EXISTS notification_history (
    id BIGSERIAL PRIMARY KEY,
    notification_id BIGINT NOT NULL,
    event VARCHAR(50) NOT NULL,
    details JSONB,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_notification_history_notif ON notification_history(notification_id);

-- Add columns to notification_templates if not present
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'notification_templates' AND column_name = 'variables') THEN
        ALTER TABLE notification_templates ADD COLUMN variables JSONB;
        ALTER TABLE notification_templates ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();
    END IF;
END $$;

-- Add columns to notifications if not present
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'notifications' AND column_name = 'template_id') THEN
        ALTER TABLE notifications ADD COLUMN template_id BIGINT REFERENCES notification_templates(id);
        ALTER TABLE notifications ADD COLUMN variables JSONB;
        ALTER TABLE notifications ADD COLUMN max_retries INTEGER DEFAULT 3;
        ALTER TABLE notifications ADD COLUMN last_error TEXT;
        ALTER TABLE notifications ADD COLUMN sent_at TIMESTAMPTZ;
        ALTER TABLE notifications ADD COLUMN delivered_at TIMESTAMPTZ;
    END IF;
END $$;

-- ── Journal Entry Lines ─────────────────────────────────────
CREATE TABLE IF NOT EXISTS journal_entry_lines (
    id BIGSERIAL PRIMARY KEY,
    journal_entry_id BIGINT NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id BIGINT NOT NULL REFERENCES chart_of_accounts(id),
    debit DECIMAL(12,2) DEFAULT 0,
    credit DECIMAL(12,2) DEFAULT 0,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    CHECK (debit >= 0 AND credit >= 0),
    CHECK (debit > 0 OR credit > 0)
);

CREATE INDEX IF NOT EXISTS idx_journal_entry_lines_entry ON journal_entry_lines(journal_entry_id);

-- ── Trial Balances ──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS trial_balances (
    id BIGSERIAL PRIMARY KEY,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    account_id BIGINT NOT NULL REFERENCES chart_of_accounts(id),
    opening_balance DECIMAL(12,2) DEFAULT 0,
    total_debit DECIMAL(12,2) DEFAULT 0,
    total_credit DECIMAL(12,2) DEFAULT 0,
    closing_balance DECIMAL(12,2) DEFAULT 0,
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(period_start, period_end, account_id)
);

-- ── Referral Tracking ───────────────────────────────────────
-- Add columns to referral_programs if not present
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'referral_programs' AND column_name = 'updated_at') THEN
        ALTER TABLE referral_programs ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();
    END IF;
END $$;

-- Add columns to referral_tracking if not present
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'referral_tracking' AND column_name = 'updated_at') THEN
        ALTER TABLE referral_tracking ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();
    END IF;
END $$;
