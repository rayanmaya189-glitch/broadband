-- Phase 6: Inventory, Payment Gateway, and Realtime Enhancements

-- ════════════════════════════════════════════════════════════════
-- INVENTORY MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- Add missing columns to inventory_items
DO $$ BEGIN
    ALTER TABLE inventory_items ADD COLUMN IF NOT EXISTS assigned_to_branch_id BIGINT REFERENCES branches(id);
    ALTER TABLE inventory_items ADD COLUMN IF NOT EXISTS notes TEXT;
    ALTER TABLE inventory_items ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- Inventory Movements table
CREATE TABLE IF NOT EXISTS inventory_movements (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGINT NOT NULL REFERENCES inventory_items(id),
    movement_type VARCHAR(30) NOT NULL
        CHECK (movement_type IN ('received', 'assigned', 'installed', 'returned', 'transferred', 'scrapped')),
    from_branch_id BIGINT REFERENCES branches(id),
    to_branch_id BIGINT REFERENCES branches(id),
    reference_type VARCHAR(50),
    reference_id BIGINT,
    performed_by BIGINT NOT NULL REFERENCES users(id),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_inventory_movements_item ON inventory_movements(item_id);

-- ════════════════════════════════════════════════════════════════
-- PAYMENT GATEWAY MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- Add missing columns to payment_gateways
DO $$ BEGIN
    ALTER TABLE payment_gateways ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
    ALTER TABLE payment_gateways ADD COLUMN IF NOT EXISTS supported_methods TEXT[] DEFAULT '{}';
    ALTER TABLE payment_gateways ADD COLUMN IF NOT EXISTS currency VARCHAR(10) DEFAULT 'INR';
    ALTER TABLE payment_gateways ADD COLUMN IF NOT EXISTS webhook_secret TEXT;
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- Payment Transactions table
CREATE TABLE IF NOT EXISTS payment_transactions (
    id BIGSERIAL PRIMARY KEY,
    gateway_id VARCHAR(50) NOT NULL,
    invoice_id BIGINT REFERENCES invoices(id),
    customer_id BIGINT REFERENCES customers(id),
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'INR',
    payment_method VARCHAR(50) NOT NULL,
    gateway_transaction_id VARCHAR(255),
    status VARCHAR(30) DEFAULT 'pending'
        CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'refunded', 'expired')),
    idempotency_key VARCHAR(255) UNIQUE,
    failure_reason TEXT,
    webhook_received_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_payment_transactions_gateway ON payment_transactions(gateway_id);
CREATE INDEX IF NOT EXISTS idx_payment_transactions_status ON payment_transactions(status);
CREATE INDEX IF NOT EXISTS idx_payment_transactions_idempotency ON payment_transactions(idempotency_key);

-- Payment Links table
CREATE TABLE IF NOT EXISTS payment_links (
    id BIGSERIAL PRIMARY KEY,
    transaction_id BIGINT NOT NULL REFERENCES payment_transactions(id),
    payment_url TEXT NOT NULL,
    short_url TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    is_used BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_payment_links_transaction ON payment_links(transaction_id);

-- Webhook Logs table
CREATE TABLE IF NOT EXISTS webhook_logs (
    id BIGSERIAL PRIMARY KEY,
    gateway_id VARCHAR(50) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    processed BOOLEAN DEFAULT FALSE,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_webhook_logs_gateway ON webhook_logs(gateway_id);
CREATE INDEX IF NOT EXISTS idx_webhook_logs_event ON webhook_logs(event_type);
