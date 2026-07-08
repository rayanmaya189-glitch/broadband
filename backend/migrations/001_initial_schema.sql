-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — Initial Schema Migration
-- ═══════════════════════════════════════════════════════════════

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- ── Branches ─────────────────────────────────────────────────
CREATE TABLE branches (
    id          BIGSERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    code        VARCHAR(50) UNIQUE NOT NULL,
    address     TEXT,
    city        VARCHAR(100),
    state       VARCHAR(100),
    pincode     VARCHAR(10),
    phone       VARCHAR(20),
    email       VARCHAR(255),
    is_active   BOOLEAN NOT NULL DEFAULT true,
    timezone    VARCHAR(50) NOT NULL DEFAULT 'Asia/Kolkata',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_branches_code ON branches(code);
CREATE INDEX idx_branches_active ON branches(is_active) WHERE is_active = true;

-- ── Permissions ──────────────────────────────────────────────
CREATE TABLE permissions (
    id          BIGSERIAL PRIMARY KEY,
    name        VARCHAR(100) UNIQUE NOT NULL,   -- e.g. "customer.view", "billing.invoice.create"
    description TEXT,
    module      VARCHAR(50) NOT NULL,            -- e.g. "customer", "billing"
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_permissions_module ON permissions(module);

-- ── Roles ────────────────────────────────────────────────────
CREATE TABLE roles (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(50) UNIQUE NOT NULL, -- e.g. "super_admin", "branch_operator"
    display_name    VARCHAR(100) NOT NULL,
    description     TEXT,
    is_system       BOOLEAN NOT NULL DEFAULT false, -- system roles cannot be deleted
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── Role ↔ Permission mapping ───────────────────────────────
CREATE TABLE role_permissions (
    role_id       BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- ── Users ────────────────────────────────────────────────────
CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY,
    email           VARCHAR(255) UNIQUE NOT NULL,
    password_hash   VARCHAR(255) NOT NULL,
    name            VARCHAR(255) NOT NULL,
    phone           VARCHAR(20),
    avatar_url      TEXT,
    role_id         BIGINT NOT NULL REFERENCES roles(id),
    branch_id       BIGINT REFERENCES branches(id),
    is_company_wide BOOLEAN NOT NULL DEFAULT false,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    is_locked       BOOLEAN NOT NULL DEFAULT false,
    locked_until    TIMESTAMPTZ,
    failed_attempts INTEGER NOT NULL DEFAULT 0,
    last_login_at   TIMESTAMPTZ,
    two_factor_enabled  BOOLEAN NOT NULL DEFAULT false,
    two_factor_secret   BYTEA,               -- AES-256-GCM encrypted TOTP secret
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role_id);
CREATE INDEX idx_users_branch ON users(branch_id);
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = true;

-- ── Refresh Tokens ───────────────────────────────────────────
CREATE TABLE refresh_tokens (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash  VARCHAR(64) NOT NULL,         -- SHA-256 of the raw token
    device_info TEXT,
    ip_address  INET,
    expires_at  TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at  TIMESTAMPTZ
);

CREATE INDEX idx_refresh_tokens_user ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_hash ON refresh_tokens(token_hash);

-- ── Customers (ISP subscribers) ─────────────────────────────
CREATE TABLE customers (
    id              BIGSERIAL PRIMARY KEY,
    customer_code   VARCHAR(50) UNIQUE NOT NULL,   -- AX-XXXXXXXXXXXX
    first_name      VARCHAR(255) NOT NULL,
    last_name       VARCHAR(255),
    email           VARCHAR(255),
    phone           VARCHAR(20) NOT NULL,
    alternate_phone VARCHAR(20),
    status          VARCHAR(30) NOT NULL DEFAULT 'lead',  -- lead, prospect, active, suspended, deactivated, blacklist
    branch_id       BIGINT NOT NULL REFERENCES branches(id),
    lead_id         BIGINT,                          -- converted from lead
    referred_by     BIGINT REFERENCES customers(id),
    created_by      BIGINT REFERENCES users(id),
    kyc_status      VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, submitted, verified, rejected
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_customers_code ON customers(customer_code);
CREATE INDEX idx_customers_phone ON customers(phone);
CREATE INDEX idx_customers_status ON customers(status);
CREATE INDEX idx_customers_branch ON customers(branch_id);

-- ── Customer Addresses ───────────────────────────────────────
CREATE TABLE customer_addresses (
    id              BIGSERIAL PRIMARY KEY,
    customer_id     BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    type            VARCHAR(20) NOT NULL DEFAULT 'installation',  -- installation, billing, correspondence
    address_line1   VARCHAR(500) NOT NULL,
    address_line2   VARCHAR(500),
    city            VARCHAR(100) NOT NULL,
    state           VARCHAR(100) NOT NULL,
    pincode         VARCHAR(10) NOT NULL,
    location        GEOMETRY(POINT, 4326),  -- PostGIS point
    landmark        VARCHAR(255),
    is_primary      BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_customer_addresses_customer ON customer_addresses(customer_id);
CREATE INDEX idx_customer_addresses_location ON customer_addresses USING GIST(location);

-- ── Plans ────────────────────────────────────────────────────
CREATE TABLE plans (
    id                  BIGSERIAL PRIMARY KEY,
    name                VARCHAR(255) NOT NULL,
    code                VARCHAR(50) UNIQUE NOT NULL,
    description         TEXT,
    speed_down_mbps     INTEGER NOT NULL,        -- download speed in Mbps
    speed_up_mbps       INTEGER NOT NULL,        -- upload speed in Mbps
    data_cap_gb         INTEGER,                  -- NULL = unlimited
    price_monthly       DECIMAL(10,2) NOT NULL,
    price_quarterly     DECIMAL(10,2),
    price_half_yearly   DECIMAL(10,2),
    price_yearly        DECIMAL(10,2),
    gst_percent         DECIMAL(5,2) NOT NULL DEFAULT 18.00,
    is_active           BOOLEAN NOT NULL DEFAULT true,
    is_promotional      BOOLEAN NOT NULL DEFAULT false,
    category            VARCHAR(50) NOT NULL DEFAULT 'standard',  -- basic, standard, premium, enterprise
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_plans_code ON plans(code);
CREATE INDEX idx_plans_active ON plans(is_active) WHERE is_active = true;
CREATE INDEX idx_plans_category ON plans(category);

-- ── Subscriptions ────────────────────────────────────────────
CREATE TABLE subscriptions (
    id                  BIGSERIAL PRIMARY KEY,
    customer_id         BIGINT NOT NULL REFERENCES customers(id),
    plan_id             BIGINT NOT NULL REFERENCES plans(id),
    status              VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, active, suspended, expired, cancelled
    start_date          DATE NOT NULL,
    end_date            DATE,
    next_renewal_date   DATE,
    billing_cycle       VARCHAR(20) NOT NULL DEFAULT 'monthly',  -- monthly, quarterly, half_yearly, yearly
    is_auto_renew       BOOLEAN NOT NULL DEFAULT true,
    installation_date   DATE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_subscriptions_customer ON subscriptions(customer_id);
CREATE INDEX idx_subscriptions_plan ON subscriptions(plan_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);
CREATE INDEX idx_subscriptions_renewal ON subscriptions(next_renewal_date) WHERE status = 'active';

-- ── Audit Log (partitioned) ─────────────────────────────────
CREATE TABLE audit_logs (
    id              BIGSERIAL,
    user_id         BIGINT,
    branch_id       BIGINT,
    action          VARCHAR(100) NOT NULL,
    entity_type     VARCHAR(50),
    entity_id       VARCHAR(50),
    old_value       JSONB,
    new_value       JSONB,
    ip_address      INET,
    user_agent      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

-- Create partitions for current and next 12 months
CREATE TABLE audit_logs_default PARTITION OF audit_logs DEFAULT;

-- ── Seed system roles ───────────────────────────────────────
INSERT INTO roles (name, display_name, description, is_system) VALUES
    ('super_admin',     'Super Admin',     'Full system access', true),
    ('isp_owner',       'ISP Owner',       'Business owner access', true),
    ('admin',           'Admin',           'Administrative access', true),
    ('finance_manager', 'Finance Manager', 'Financial operations', true),
    ('branch_manager',  'Branch Manager',  'Branch-level management', false),
    ('branch_operator', 'Branch Operator', 'Branch-level operations', false),
    ('network_engineer','Network Engineer', 'Network management', false),
    ('field_technician','Field Technician', 'Installation & field work', false),
    ('customer_support', 'Customer Support', 'Ticket handling', false),
    ('reseller',        'Reseller',        'Reseller portal access', false);

-- ── Seed base permissions ───────────────────────────────────
INSERT INTO permissions (name, description, module) VALUES
    -- Auth
    ('auth.login', 'Login', 'auth'),
    ('auth.logout', 'Logout', 'auth'),
    ('auth.change_password', 'Change password', 'auth'),
    ('auth.reset_password', 'Reset password', 'auth'),
    -- User management
    ('user.view', 'View users', 'user'),
    ('user.create', 'Create users', 'user'),
    ('user.update', 'Update users', 'user'),
    ('user.delete', 'Delete users', 'user'),
    -- Branch
    ('branch.view', 'View branches', 'branch'),
    ('branch.create', 'Create branches', 'branch'),
    ('branch.update', 'Update branches', 'branch'),
    -- Customer
    ('customer.view', 'View customers', 'customer'),
    ('customer.create', 'Create customers', 'customer'),
    ('customer.update', 'Update customers', 'customer'),
    ('customer.suspend', 'Suspend customer', 'customer'),
    ('customer.kyc', 'Verify KYC', 'customer'),
    -- Plans
    ('plan.view', 'View plans', 'plan'),
    ('plan.create', 'Create plans', 'plan'),
    ('plan.update', 'Update plans', 'plan'),
    ('plan.delete', 'Delete plans', 'plan'),
    -- Billing
    ('billing.invoice.view', 'View invoices', 'billing'),
    ('billing.invoice.create', 'Create invoices', 'billing'),
    ('billing.invoice.approve', 'Approve invoices', 'billing'),
    ('billing.payment.view', 'View payments', 'billing'),
    ('billing.payment.record', 'Record payments', 'billing'),
    ('billing.refund', 'Process refunds', 'billing'),
    ('billing.discount', 'Manage discounts', 'billing'),
    -- Network
    ('network.vlan.view', 'View VLANs', 'network'),
    ('network.vlan.manage', 'Manage VLANs', 'network'),
    ('network.ip_pool.view', 'View IP pools', 'network'),
    ('network.ip_pool.manage', 'Manage IP pools', 'network'),
    -- Devices
    ('device.view', 'View devices', 'device'),
    ('device.manage', 'Manage devices', 'device'),
    ('device.reboot', 'Reboot devices', 'device'),
    ('device.firmware', 'Update firmware', 'device'),
    -- Bandwidth
    ('bandwidth.view', 'View bandwidth', 'bandwidth'),
    ('bandwidth.manage', 'Manage bandwidth', 'bandwidth'),
    -- Tickets
    ('ticket.view', 'View tickets', 'ticket'),
    ('ticket.create', 'Create tickets', 'ticket'),
    ('ticket.assign', 'Assign tickets', 'ticket'),
    ('ticket.resolve', 'Resolve tickets', 'ticket'),
    -- Audit
    ('audit.view', 'View audit logs', 'audit'),
    -- Settings
    ('settings.view', 'View settings', 'settings'),
    ('settings.manage', 'Manage settings', 'settings'),
    -- Reports
    ('reports.view', 'View reports', 'reports'),
    ('reports.export', 'Export reports', 'reports');
