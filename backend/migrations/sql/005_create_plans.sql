-- AeroXe Backend Migration 005: Create Plans & Pricing Tables
-- Internet plans with multi-period pricing and speed profiles

-- Plans
CREATE TABLE IF NOT EXISTS plans (
    id BIGSERIAL PRIMARY KEY,
    slug VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    speed_label VARCHAR(20) NOT NULL,
    download_mbps INTEGER NOT NULL,
    upload_mbps INTEGER NOT NULL,
    burst_mbps INTEGER,
    data_quota VARCHAR(50) DEFAULT 'unlimited',
    fair_usage_policy JSONB,
    qos_priority VARCHAR(20) DEFAULT 'standard',
    sla_uptime_percent DECIMAL(5,2) DEFAULT 99.5,
    is_popular BOOLEAN DEFAULT FALSE,
    is_business BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    sort_order INTEGER DEFAULT 0,
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_plans_slug ON plans(slug);
CREATE INDEX IF NOT EXISTS idx_plans_active ON plans(is_active);
CREATE INDEX IF NOT EXISTS idx_plans_status ON plans(review_status);

-- Plans history
CREATE TABLE IF NOT EXISTS plans_history (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE INDEX IF NOT EXISTS idx_plans_history_plan ON plans_history(plan_id);

-- Plan pricing (multi-period: monthly, quarterly, half-yearly, annual)
CREATE TABLE IF NOT EXISTS plan_pricing (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    billing_period_months INTEGER NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    savings DECIMAL(10,2),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(plan_id, billing_period_months)
);

CREATE INDEX IF NOT EXISTS idx_plan_pricing_plan ON plan_pricing(plan_id);

-- Speed profiles (technical bandwidth config per plan)
CREATE TABLE IF NOT EXISTS speed_profiles (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES plans(id),
    name VARCHAR(100) NOT NULL,
    download_limit_kbps INTEGER NOT NULL,
    upload_limit_kbps INTEGER NOT NULL,
    burst_download_kbps INTEGER,
    burst_upload_kbps INTEGER,
    burst_duration_seconds INTEGER DEFAULT 30,
    priority_queue INTEGER DEFAULT 1,
    qos_marking VARCHAR(10),
    htb_parent_queue VARCHAR(20),
    fq_codel_enabled BOOLEAN DEFAULT TRUE,
    device_type VARCHAR(50) NOT NULL DEFAULT 'mikrotik',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_speed_profiles_plan ON speed_profiles(plan_id);

-- Service packages (add-ons)
CREATE TABLE IF NOT EXISTS service_packages (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    type VARCHAR(50) NOT NULL,
    monthly_price DECIMAL(10,2),
    config JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Plan-Service Package mapping
CREATE TABLE IF NOT EXISTS plan_service_packages (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    package_id BIGINT NOT NULL REFERENCES service_packages(id) ON DELETE CASCADE,
    is_included BOOLEAN DEFAULT FALSE,
    additional_price DECIMAL(10,2),
    UNIQUE(plan_id, package_id)
);

-- Bandwidth profiles
CREATE TABLE IF NOT EXISTS bandwidth_profiles (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    plan_id BIGINT REFERENCES plans(id),
    download_kbps INTEGER NOT NULL,
    upload_kbps INTEGER NOT NULL,
    burst_download_kbps INTEGER,
    burst_upload_kbps INTEGER,
    burst_duration_seconds INTEGER DEFAULT 30,
    priority INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT TRUE,
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_bandwidth_profiles_plan ON bandwidth_profiles(plan_id);

-- Bandwidth profiles history
CREATE TABLE IF NOT EXISTS bandwidth_profiles_history (
    id BIGSERIAL PRIMARY KEY,
    profile_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

-- Bandwidth applications (profile-to-subscription mapping)
CREATE TABLE IF NOT EXISTS bandwidth_applications (
    id BIGSERIAL PRIMARY KEY,
    profile_id BIGINT NOT NULL REFERENCES bandwidth_profiles(id),
    subscription_id BIGINT NOT NULL,
    device_id BIGINT,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'applying', 'applied', 'failed')),
    applied_at TIMESTAMPTZ,
    failed_reason TEXT,
    retry_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_bandwidth_applications_profile ON bandwidth_applications(profile_id);
CREATE INDEX IF NOT EXISTS idx_bandwidth_applications_subscription ON bandwidth_applications(subscription_id);
