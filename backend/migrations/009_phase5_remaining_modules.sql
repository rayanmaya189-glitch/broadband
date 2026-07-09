-- Phase 5: Remaining Module Enhancements
-- Network, Coverage, Installation, Events, Documents, Audit, Entity History

-- ════════════════════════════════════════════════════════════════
-- NETWORK MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- IP Addresses table (for allocate/release)
CREATE TABLE IF NOT EXISTS ip_addresses (
    id BIGSERIAL PRIMARY KEY,
    ip_pool_id BIGINT NOT NULL REFERENCES ip_pools(id) ON DELETE CASCADE,
    ip_address INET NOT NULL UNIQUE,
    status VARCHAR(20) DEFAULT 'available'
        CHECK (status IN ('available', 'allocated', 'reserved', 'excluded')),
    allocated_to_type VARCHAR(50),
    allocated_to_id BIGINT,
    allocated_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_ip_addresses_pool ON ip_addresses(ip_pool_id);
CREATE INDEX IF NOT EXISTS idx_ip_addresses_status ON ip_addresses(status);

-- MAC Bindings table
CREATE TABLE IF NOT EXISTS mac_bindings (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    mac_address MACADDR NOT NULL,
    assigned_ip INET NOT NULL,
    vlan_id BIGINT REFERENCES vlans(id),
    bound_at TIMESTAMPTZ DEFAULT NOW(),
    bound_by BIGINT REFERENCES users(id),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, mac_address)
);
CREATE INDEX IF NOT EXISTS idx_mac_bindings_branch ON mac_bindings(branch_id);
CREATE INDEX IF NOT EXISTS idx_mac_bindings_customer ON mac_bindings(customer_id);

-- DHCP Leases table
CREATE TABLE IF NOT EXISTS dhcp_leases (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    mac_address MACADDR NOT NULL,
    ip_address INET NOT NULL,
    hostname VARCHAR(255),
    vlan_id BIGINT REFERENCES vlans(id),
    ip_pool_id BIGINT NOT NULL REFERENCES ip_pools(id),
    lease_start TIMESTAMPTZ NOT NULL,
    lease_end TIMESTAMPTZ NOT NULL,
    lease_type VARCHAR(20) DEFAULT 'dynamic'
        CHECK (lease_type IN ('dynamic', 'static', 'reserved')),
    client_id VARCHAR(255),
    customer_id BIGINT REFERENCES customers(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    device_id BIGINT REFERENCES network_devices(id),
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'expired', 'released')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_dhcp_leases_branch ON dhcp_leases(branch_id);
CREATE INDEX IF NOT EXISTS idx_dhcp_leases_mac ON dhcp_leases(mac_address);

-- Customer Sessions table
CREATE TABLE IF NOT EXISTS customer_sessions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    mac_address MACADDR NOT NULL,
    ip_address INET NOT NULL,
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    disconnected_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    bytes_in BIGINT DEFAULT 0,
    bytes_out BIGINT DEFAULT 0,
    is_online BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_customer_sessions_branch ON customer_sessions(branch_id);
CREATE INDEX IF NOT EXISTS idx_customer_sessions_customer ON customer_sessions(customer_id);
CREATE INDEX IF NOT EXISTS idx_customer_sessions_online ON customer_sessions(is_online);

-- Add missing VLAN columns
DO $$ BEGIN
    ALTER TABLE vlans ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
    ALTER TABLE vlans ADD COLUMN IF NOT EXISTS created_by BIGINT REFERENCES users(id);
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- ════════════════════════════════════════════════════════════════
-- COVERAGE MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- Add missing CoverageArea columns
DO $$ BEGIN
    ALTER TABLE coverage_areas ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- ════════════════════════════════════════════════════════════════
-- INSTALLATION MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- Add photos array column to installation_orders
DO $$ BEGIN
    ALTER TABLE installation_orders ADD COLUMN IF NOT EXISTS photos TEXT[];
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- ════════════════════════════════════════════════════════════════
-- EVENT MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- Add metadata column to events
DO $$ BEGIN
    ALTER TABLE events ADD COLUMN IF NOT EXISTS metadata JSONB;
    ALTER TABLE events ADD COLUMN IF NOT EXISTS caused_by_branch_id BIGINT REFERENCES branches(id);
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- Event Subscriptions table
CREATE TABLE IF NOT EXISTS event_subscriptions (
    id BIGSERIAL PRIMARY KEY,
    subscriber_name VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    last_processed_id BIGINT DEFAULT 0,
    last_processed_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(subscriber_name, event_type)
);
CREATE INDEX IF NOT EXISTS idx_event_subscriptions_subscriber ON event_subscriptions(subscriber_name);

-- ════════════════════════════════════════════════════════════════
-- DOCUMENT MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- Add missing document_files columns
DO $$ BEGIN
    ALTER TABLE document_files ADD COLUMN IF NOT EXISTS file_hash VARCHAR(255);
    ALTER TABLE document_files ADD COLUMN IF NOT EXISTS storage_url TEXT;
    ALTER TABLE document_files ADD COLUMN IF NOT EXISTS metadata JSONB;
    ALTER TABLE document_files ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- Document Access Logs table
CREATE TABLE IF NOT EXISTS document_access_logs (
    id BIGSERIAL PRIMARY KEY,
    document_id BIGINT NOT NULL REFERENCES document_files(id),
    accessed_by BIGINT REFERENCES users(id),
    access_type VARCHAR(20) NOT NULL
        CHECK (access_type IN ('upload', 'download', 'view', 'delete')),
    ip_address INET,
    user_agent TEXT,
    accessed_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_doc_access_logs_document ON document_access_logs(document_id);

-- ════════════════════════════════════════════════════════════════
-- AUDIT MODULE ENHANCEMENTS
-- ════════════════════════════════════════════════════════════════

-- Add missing audit_logs columns
DO $$ BEGIN
    ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS user_agent TEXT;
    ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS resource_id VARCHAR(255);
    ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS metadata JSONB;
EXCEPTION WHEN duplicate_column THEN NULL;
END $$;

-- ════════════════════════════════════════════════════════════════
-- ENTITY HISTORY
-- ════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS entity_history (
    id BIGSERIAL PRIMARY KEY,
    entity_type VARCHAR(100) NOT NULL,
    entity_id BIGINT NOT NULL,
    action VARCHAR(50) NOT NULL
        CHECK (action IN ('created', 'updated', 'deleted', 'status_changed', 'rollback')),
    old_data JSONB,
    new_data JSONB,
    changed_fields TEXT[],
    user_id BIGINT REFERENCES users(id),
    branch_id BIGINT REFERENCES branches(id),
    ip_address INET,
    user_agent TEXT,
    reason TEXT,
    rollback_reference BIGINT REFERENCES entity_history(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_entity_history_type ON entity_history(entity_type);
CREATE INDEX IF NOT EXISTS idx_entity_history_entity ON entity_history(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_entity_history_action ON entity_history(action);
CREATE INDEX IF NOT EXISTS idx_entity_history_user ON entity_history(user_id);
CREATE INDEX IF NOT EXISTS idx_entity_history_created ON entity_history(created_at);
