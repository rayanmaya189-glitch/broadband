-- AeroXe Backend Migration 004: Create Customer Management Tables
-- Customer lifecycle: registered → kyc_pending → kyc_verified → installation → active → suspended → terminated

-- Customer profiles and KYC
CREATE TABLE IF NOT EXISTS customer_profiles (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    aadhaar_hash VARCHAR(255),
    pan_hash VARCHAR(255),
    gender VARCHAR(10),
    date_of_birth DATE,
    occupation VARCHAR(100),
    kyc_status VARCHAR(20) DEFAULT 'pending'
        CHECK (kyc_status IN ('pending', 'submitted', 'verified', 'rejected')),
    kyc_verified_at TIMESTAMPTZ,
    kyc_verified_by BIGINT REFERENCES users(id),
    kyc_rejection_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Customers (main customer entity - owned by customer module)
CREATE TABLE IF NOT EXISTS customers (
    id BIGSERIAL PRIMARY KEY,
    customer_code VARCHAR(20) NOT NULL UNIQUE,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(20) NOT NULL,
    alternate_phone VARCHAR(20),
    status VARCHAR(30) NOT NULL DEFAULT 'registered'
        CHECK (status IN ('registered', 'kyc_pending', 'kyc_verified',
                          'installation_scheduled', 'installation_in_progress',
                          'active', 'suspended', 'terminated')),
    referral_code VARCHAR(20) UNIQUE,
    referred_by BIGINT REFERENCES customers(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_customers_branch ON customers(branch_id);
CREATE INDEX IF NOT EXISTS idx_customers_phone ON customers(phone);
CREATE INDEX IF NOT EXISTS idx_customers_status ON customers(status);
CREATE INDEX IF NOT EXISTS idx_customers_referral ON customers(referral_code);
CREATE INDEX IF NOT EXISTS idx_customers_code ON customers(customer_code);

-- Customer history for audit trail
CREATE TABLE IF NOT EXISTS customers_history (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE INDEX IF NOT EXISTS idx_customers_history_customer ON customers_history(customer_id);

-- KYC documents
CREATE TABLE IF NOT EXISTS kyc_documents (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    document_type VARCHAR(50) NOT NULL,
    file_url TEXT NOT NULL,
    file_hash VARCHAR(255),
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected')),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_kyc_documents_customer ON kyc_documents(customer_id);

-- Customer addresses
CREATE TABLE IF NOT EXISTS addresses (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    address_type VARCHAR(20) DEFAULT 'installation'
        CHECK (address_type IN ('installation', 'billing', 'correspondence')),
    line1 VARCHAR(255) NOT NULL,
    line2 VARCHAR(255),
    area VARCHAR(100),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    pincode VARCHAR(10) NOT NULL,
    country VARCHAR(50) DEFAULT 'India',
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    landmark VARCHAR(255),
    is_primary BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_addresses_customer ON addresses(customer_id);
CREATE INDEX IF NOT EXISTS idx_addresses_pincode ON addresses(pincode);

-- Installation orders
CREATE TABLE IF NOT EXISTS installation_orders (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    subscription_id BIGINT,
    assigned_technician_id BIGINT REFERENCES users(id),
    status VARCHAR(30) DEFAULT 'pending'
        CHECK (status IN ('pending', 'scheduled', 'in_progress', 'completed', 'cancelled', 'rescheduled')),
    scheduled_date DATE,
    scheduled_time_slot VARCHAR(20),
    completed_at TIMESTAMPTZ,
    installation_type VARCHAR(20) DEFAULT 'new',
    equipment_issued JSONB,
    fiber_drop_length_meters INTEGER,
    onu_power_dbm DECIMAL(5,2),
    notes TEXT,
    photos TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_installation_orders_customer ON installation_orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_installation_orders_branch ON installation_orders(branch_id);
CREATE INDEX IF NOT EXISTS idx_installation_orders_status ON installation_orders(status);

-- Coverage areas
CREATE TABLE IF NOT EXISTS coverage_areas (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    area_type VARCHAR(30) DEFAULT 'polygon'
        CHECK (area_type IN ('polygon', 'circle', 'pincode')),
    boundary GEOMETRY(Polygon, 4326),
    center_point GEOMETRY(Point, 4326),
    radius_meters INTEGER,
    pincodes TEXT[],
    is_active BOOLEAN DEFAULT TRUE,
    max_customers INTEGER,
    current_customers INTEGER DEFAULT 0,
    fiber_available BOOLEAN DEFAULT TRUE,
    estimated_installation_days INTEGER DEFAULT 3,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_coverage_areas_branch ON coverage_areas(branch_id);
CREATE INDEX IF NOT EXISTS idx_coverage_areas_boundary ON coverage_areas USING GIST(boundary);
CREATE INDEX IF NOT EXISTS idx_coverage_areas_center ON coverage_areas USING GIST(center_point);

CREATE TABLE IF NOT EXISTS coverage_pincode_map (
    id BIGSERIAL PRIMARY KEY,
    coverage_area_id BIGINT NOT NULL REFERENCES coverage_areas(id) ON DELETE CASCADE,
    pincode VARCHAR(10) NOT NULL,
    city VARCHAR(100) NOT NULL,
    district VARCHAR(100),
    state VARCHAR(100) DEFAULT 'Maharashtra',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(coverage_area_id, pincode)
);

CREATE INDEX IF NOT EXISTS idx_coverage_pincode_pincode ON coverage_pincode_map(pincode);

-- Leads (sales pipeline)
CREATE TABLE IF NOT EXISTS leads (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    assigned_to BIGINT REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    email VARCHAR(255),
    source VARCHAR(50) NOT NULL
        CHECK (source IN ('landing_page', 'whatsapp', 'referral', 'walk_in',
                          'cold_call', 'social_media', 'field_visit')),
    status VARCHAR(30) DEFAULT 'new'
        CHECK (status IN ('new', 'contacted', 'interested', 'surveyed',
                          'quoted', 'converted', 'lost')),
    interested_plan_id BIGINT,
    estimated_install_date DATE,
    address TEXT,
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    lost_reason TEXT,
    notes TEXT,
    converted_customer_id BIGINT REFERENCES customers(id),
    converted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_leads_branch ON leads(branch_id);
CREATE INDEX IF NOT EXISTS idx_leads_status ON leads(status);
CREATE INDEX IF NOT EXISTS idx_leads_assigned ON leads(assigned_to);

CREATE TABLE IF NOT EXISTS lead_activities (
    id BIGSERIAL PRIMARY KEY,
    lead_id BIGINT NOT NULL REFERENCES leads(id) ON DELETE CASCADE,
    activity_type VARCHAR(30) NOT NULL
        CHECK (activity_type IN ('call', 'whatsapp', 'visit', 'email', 'note', 'status_change')),
    description TEXT NOT NULL,
    performed_by BIGINT NOT NULL REFERENCES users(id),
    scheduled_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_lead_activities_lead ON lead_activities(lead_id);
