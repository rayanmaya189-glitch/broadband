-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — Phase 2: KYC, Speed Profiles, Plan Pricing
-- ═══════════════════════════════════════════════════════════════

-- ── Customer Profiles (KYC & personal details) ───────────────
CREATE TABLE IF NOT EXISTS customer_profiles (
    id                  BIGSERIAL PRIMARY KEY,
    customer_id         BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE UNIQUE,
    date_of_birth       DATE,
    gender              VARCHAR(10),           -- male, female, other
    nationality         VARCHAR(100),
    id_proof_type       VARCHAR(50),           -- aadhaar, pan, passport, voter_id, driving_license
    id_proof_number     VARCHAR(100),
    id_proof_expiry     DATE,
    pan_number          VARCHAR(20),
    aadhaar_number      VARCHAR(20),
    gstin               VARCHAR(20),
    company_name        VARCHAR(255),
    designation         VARCHAR(100),
    occupation          VARCHAR(100),
    annual_income_range VARCHAR(50),
    preferred_language  VARCHAR(20) DEFAULT 'en',
    communication_opt_in BOOLEAN NOT NULL DEFAULT true,
    notes               TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_customer_profiles_customer ON customer_profiles(customer_id);

-- ── KYC Documents ────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS kyc_documents (
    id                  BIGSERIAL PRIMARY KEY,
    customer_id         BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    document_type       VARCHAR(50) NOT NULL,  -- aadhaar_front, aadhaar_back, pan_card, passport, photo, address_proof, other
    document_url        TEXT NOT NULL,
    file_name           VARCHAR(255),
    file_size_bytes     BIGINT,
    mime_type           VARCHAR(100),
    verification_status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, approved, rejected
    rejection_reason    TEXT,
    verified_by         BIGINT REFERENCES users(id),
    verified_at         TIMESTAMPTZ,
    uploaded_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_kyc_documents_customer ON kyc_documents(customer_id);
CREATE INDEX idx_kyc_documents_status ON kyc_documents(verification_status);

-- ── Speed Profiles (Mikrotik/TikSP integration) ──────────────
CREATE TABLE IF NOT EXISTS speed_profiles (
    id                      BIGSERIAL PRIMARY KEY,
    plan_id                 BIGINT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    name                    VARCHAR(100) NOT NULL,
    download_limit_kbps     INTEGER NOT NULL,
    upload_limit_kbps       INTEGER NOT NULL,
    burst_download_kbps     INTEGER,
    burst_upload_kbps       INTEGER,
    burst_duration_seconds  INTEGER NOT NULL DEFAULT 30,
    priority_queue          INTEGER NOT NULL DEFAULT 1,    -- 1=highest
    qos_marking             VARCHAR(10),                   -- DSCP marking
    htb_parent_queue        VARCHAR(20),                   -- HTB parent class
    fq_codel_enabled        BOOLEAN NOT NULL DEFAULT true,
    device_type             VARCHAR(50) NOT NULL DEFAULT 'mikrotik',  -- mikrotik, cisco, juniper
    is_active               BOOLEAN NOT NULL DEFAULT true,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_speed_profiles_plan ON speed_profiles(plan_id);
CREATE INDEX idx_speed_profiles_device ON speed_profiles(device_type);

-- ── Plan Pricing (multi-period pricing) ──────────────────────
CREATE TABLE IF NOT EXISTS plan_pricing (
    id                      BIGSERIAL PRIMARY KEY,
    plan_id                 BIGINT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    billing_period_months   INTEGER NOT NULL,              -- 1, 3, 6, 12
    price                   DECIMAL(10,2) NOT NULL,
    savings_amount          DECIMAL(10,2),                 -- savings vs monthly * months
    savings_percent         DECIMAL(5,2),                  -- savings percentage
    is_active               BOOLEAN NOT NULL DEFAULT true,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(plan_id, billing_period_months)
);

CREATE INDEX idx_plan_pricing_plan ON plan_pricing(plan_id);

-- ── Plan publish/unpublish tracking ──────────────────────────
ALTER TABLE plans ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ;
ALTER TABLE plans ADD COLUMN IF NOT EXISTS unpublished_at TIMESTAMPTZ;
ALTER TABLE plans ADD COLUMN IF NOT EXISTS published_by BIGINT REFERENCES users(id);

-- ── Subscription upgrade/downgrade tracking ──────────────────
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS previous_plan_id BIGINT REFERENCES plans(id);
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS upgraded_at TIMESTAMPTZ;
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS downgrade_scheduled_date DATE;
