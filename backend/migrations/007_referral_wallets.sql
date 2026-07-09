-- ── Customer Wallets ──────────────────────────────────────────
-- Stores wallet balances per customer for referral rewards and credits.

CREATE TABLE IF NOT EXISTS customer_wallets (
    id              BIGSERIAL PRIMARY KEY,
    customer_id     BIGINT NOT NULL REFERENCES customers(id),
    balance         NUMERIC(12,2) NOT NULL DEFAULT 0.00,
    total_earned    NUMERIC(12,2) NOT NULL DEFAULT 0.00,
    total_spent     NUMERIC(12,2) NOT NULL DEFAULT 0.00,
    status          VARCHAR(20) NOT NULL DEFAULT 'active',  -- active, frozen, closed
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_customer_wallet UNIQUE (customer_id)
);

CREATE INDEX idx_wallets_customer ON customer_wallets (customer_id);
CREATE INDEX idx_wallets_status ON customer_wallets (status);

-- ── Wallet Transactions ──────────────────────────────────────
-- Immutable ledger of all wallet credits, debits, and adjustments.

CREATE TABLE IF NOT EXISTS wallet_transactions (
    id              BIGSERIAL PRIMARY KEY,
    wallet_id       BIGINT NOT NULL REFERENCES customer_wallets(id),
    transaction_type VARCHAR(30) NOT NULL,  -- credit, debit, adjustment, referral_reward, refund, expiry
    amount          NUMERIC(12,2) NOT NULL,
    balance_after   NUMERIC(12,2) NOT NULL,
    reference_type  VARCHAR(50),  -- 'referral_tracking', 'invoice', 'refund', 'manual'
    reference_id    BIGINT,       -- ID of the related entity
    description     TEXT,
    performed_by    BIGINT REFERENCES users(id),
    expires_at      TIMESTAMPTZ,  -- optional expiry for credits
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_wallet_txn_wallet ON wallet_transactions (wallet_id);
CREATE INDEX idx_wallet_txn_type ON wallet_transactions (transaction_type);
CREATE INDEX idx_wallet_txn_ref ON wallet_transactions (reference_type, reference_id);
CREATE INDEX idx_wallet_txn_created ON wallet_transactions (created_at DESC);
