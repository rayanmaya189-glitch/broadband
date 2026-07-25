-- AeroXe Backend Migration 022: Add GST Tax Breakdown to Invoices
-- Adds CGST/SGST/IGST columns, place-of-supply, HSN/SAC, and late fee GST

-- Invoice GST breakdown columns
ALTER TABLE billing.invoices
    ADD COLUMN cgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    ADD COLUMN sgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    ADD COLUMN igst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    ADD COLUMN place_of_supply_state VARCHAR(50) NOT NULL DEFAULT 'Maharashtra',
    ADD COLUMN supplier_gstin VARCHAR(15),
    ADD COLUMN reverse_charge BOOLEAN NOT NULL DEFAULT FALSE;

-- Late fee GST columns
ALTER TABLE billing.invoices
    ADD COLUMN late_fee_subtotal DECIMAL(10,2) NOT NULL DEFAULT 0,
    ADD COLUMN late_fee_gst DECIMAL(10,2) NOT NULL DEFAULT 0;

-- Line item HSN/SAC code
ALTER TABLE billing.invoice_line_items
    ADD COLUMN hsn_sac_code VARCHAR(20),
    ADD COLUMN tax_type VARCHAR(10) NOT NULL DEFAULT 'CGST_SGST';

-- Line item GST breakdown
ALTER TABLE billing.invoice_line_items
    ADD COLUMN cgst_rate DECIMAL(5,2) NOT NULL DEFAULT 0,
    ADD COLUMN sgst_rate DECIMAL(5,2) NOT NULL DEFAULT 0,
    ADD COLUMN igst_rate DECIMAL(5,2) NOT NULL DEFAULT 0,
    ADD COLUMN cgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    ADD COLUMN sgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    ADD COLUMN igst_amount DECIMAL(10,2) NOT NULL DEFAULT 0;

-- Credit/debit notes table
CREATE TABLE IF NOT EXISTS billing.credit_debit_notes (
    id BIGSERIAL PRIMARY KEY,
    note_number VARCHAR(30) NOT NULL UNIQUE,
    note_type VARCHAR(10) NOT NULL CHECK (note_type IN ('credit', 'debit')),
    original_invoice_id BIGINT NOT NULL REFERENCES billing.invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customer.customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches.branches(id),
    reason TEXT NOT NULL,
    subtotal DECIMAL(10,2) NOT NULL,
    cgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    sgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    igst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    total_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'applied', 'rejected')),
    approved_by BIGINT REFERENCES identity.users(id),
    approved_at TIMESTAMPTZ,
    applied_to_invoice_id BIGINT REFERENCES billing.invoices(id),
    created_by BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_credit_debit_notes_customer ON billing.credit_debit_notes(customer_id);
CREATE INDEX IF NOT EXISTS idx_credit_debit_notes_invoice ON billing.credit_debit_notes(original_invoice_id);
CREATE INDEX IF NOT EXISTS idx_credit_debit_notes_status ON billing.credit_debit_notes(status);

-- Security deposit ledger
CREATE TABLE IF NOT EXISTS billing.security_deposits (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customer.customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches.branches(id),
    amount DECIMAL(10,2) NOT NULL,
    deposit_type VARCHAR(20) NOT NULL CHECK (deposit_type IN ('security', 'advance', 'equipment')),
    status VARCHAR(20) NOT NULL DEFAULT 'held' CHECK (status IN ('held', 'refunded', 'adjusted', 'forfeited')),
    collected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    refunded_at TIMESTAMPTZ,
    refund_amount DECIMAL(10,2),
    refund_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_security_deposits_customer ON billing.security_deposits(customer_id);
CREATE INDEX IF NOT EXISTS idx_security_deposits_status ON billing.security_deposits(status);

-- Reverse charge mechanism tracking
CREATE TABLE IF NOT EXISTS billing.rcm_entries (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT REFERENCES billing.invoices(id),
    vendor_name VARCHAR(255) NOT NULL,
    vendor_gstin VARCHAR(15),
    service_description TEXT NOT NULL,
    taxable_value DECIMAL(10,2) NOT NULL,
    cgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    sgst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    igst_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    total_gst DECIMAL(10,2) NOT NULL DEFAULT 0,
    itc_claimed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_rcm_entries_invoice ON billing.rcm_entries(invoice_id);

-- Revenue recognition (Ind AS 115) - deferred revenue tracking
CREATE TABLE IF NOT EXISTS billing.deferred_revenue (
    id BIGSERIAL PRIMARY KEY,
    subscription_id BIGINT NOT NULL REFERENCES subscription.subscriptions(id),
    customer_id BIGINT NOT NULL REFERENCES customer.customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches.branches(id),
    total_amount DECIMAL(10,2) NOT NULL,
    recognized_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    deferred_amount DECIMAL(10,2) NOT NULL,
    recognition_period_start DATE NOT NULL,
    recognition_period_end DATE NOT NULL,
    monthly_recognition_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'fully_recognized', 'cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_deferred_revenue_subscription ON billing.deferred_revenue(subscription_id);
CREATE INDEX IF NOT EXISTS idx_deferred_revenue_status ON billing.deferred_revenue(status);
CREATE INDEX IF NOT EXISTS idx_deferred_revenue_period ON billing.deferred_revenue(recognition_period_end);
