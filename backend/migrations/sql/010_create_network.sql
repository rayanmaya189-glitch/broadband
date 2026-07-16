-- AeroXe Backend Migration 010: Create Network Management Tables
-- VLANs, IP pools, PPPoE, DHCP, MAC bindings, and customer sessions

-- VLANs
CREATE TABLE IF NOT EXISTS vlans (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    vlan_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    vlan_type VARCHAR(30) NOT NULL
        CHECK (vlan_type IN ('management', 'customer_residential', 'customer_business',
                             'iptv', 'voip', 'monitoring')),
    is_active BOOLEAN DEFAULT TRUE,
    created_by BIGINT REFERENCES users(id),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, vlan_id)
);

CREATE INDEX IF NOT EXISTS idx_vlans_branch ON vlans(branch_id);
CREATE INDEX IF NOT EXISTS idx_vlans_type ON vlans(vlan_type);

-- VLANs History
CREATE TABLE IF NOT EXISTS vlans_history (
    id BIGSERIAL PRIMARY KEY,
    vlan_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

-- IP Pools
CREATE TABLE IF NOT EXISTS ip_pools (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    cidr CIDR NOT NULL,
    gateway INET NOT NULL,
    dns_primary INET DEFAULT '1.1.1.1',
    dns_secondary INET DEFAULT '8.8.8.8',
    dhcp_range_start INET,
    dhcp_range_end INET,
    vlan_id BIGINT REFERENCES vlans(id),
    pool_type VARCHAR(30) DEFAULT 'customer'
        CHECK (pool_type IN ('customer', 'management', 'shared_services')),
    allocated_count INTEGER DEFAULT 0,
    total_count INTEGER NOT NULL,
    utilization_percent DECIMAL(5,2) GENERATED ALWAYS AS
        (CASE WHEN total_count > 0 THEN (allocated_count::DECIMAL / total_count) * 100 ELSE 0 END) STORED,
    status VARCHAR(20) DEFAULT 'healthy'
        CHECK (status IN ('healthy', 'warning', 'critical', 'exhausted')),
    warning_threshold_percent DECIMAL(5,2) DEFAULT 80.0,
    critical_threshold_percent DECIMAL(5,2) DEFAULT 95.0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, cidr)
);

CREATE INDEX IF NOT EXISTS idx_ip_pools_branch ON ip_pools(branch_id);
CREATE INDEX IF NOT EXISTS idx_ip_pools_vlan ON ip_pools(vlan_id);

-- IP Pools History
CREATE TABLE IF NOT EXISTS ip_pools_history (
    id BIGSERIAL PRIMARY KEY,
    pool_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

-- IP Addresses
CREATE TABLE IF NOT EXISTS ip_addresses (
    id BIGSERIAL PRIMARY KEY,
    ip_pool_id BIGINT NOT NULL REFERENCES ip_pools(id),
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

-- PPPoE Sessions
CREATE TABLE IF NOT EXISTS pppoe_sessions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL,
    subscription_id BIGINT NOT NULL,
    username VARCHAR(100) NOT NULL UNIQUE,
    password_encrypted VARCHAR(255) NOT NULL,
    pppoe_server_ip INET,
    assigned_ip INET,
    nas_port_id VARCHAR(100),
    nas_ip_address INET,
    nas_session_id VARCHAR(100),
    session_start TIMESTAMPTZ,
    session_duration_seconds BIGINT DEFAULT 0,
    bytes_in BIGINT DEFAULT 0,
    bytes_out BIGINT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'inactive'
        CHECK (status IN ('active', 'inactive', 'terminated')),
    device_id BIGINT REFERENCES network_devices(id),
    last_activity_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_pppoe_sessions_customer ON pppoe_sessions(customer_id);
CREATE INDEX IF NOT EXISTS idx_pppoe_sessions_status ON pppoe_sessions(status);
CREATE INDEX IF NOT EXISTS idx_pppoe_sessions_branch ON pppoe_sessions(branch_id);

-- PPPoE Sessions History
CREATE TABLE IF NOT EXISTS pppoe_sessions_history (
    id BIGSERIAL PRIMARY KEY,
    session_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

-- DHCP Leases
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
    customer_id BIGINT,
    subscription_id BIGINT,
    device_id BIGINT REFERENCES network_devices(id),
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'expired', 'released')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dhcp_leases_mac ON dhcp_leases(mac_address);
CREATE INDEX IF NOT EXISTS idx_dhcp_leases_ip ON dhcp_leases(ip_address);
CREATE INDEX IF NOT EXISTS idx_dhcp_leases_pool ON dhcp_leases(ip_pool_id);

-- MAC Bindings
CREATE TABLE IF NOT EXISTS mac_bindings (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL,
    subscription_id BIGINT NOT NULL,
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

CREATE INDEX IF NOT EXISTS idx_mac_bindings_customer ON mac_bindings(customer_id);
CREATE INDEX IF NOT EXISTS idx_mac_bindings_mac ON mac_bindings(mac_address);

-- Customer Sessions (active online sessions)
CREATE TABLE IF NOT EXISTS customer_sessions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL,
    subscription_id BIGINT NOT NULL,
    pppoe_session_id BIGINT REFERENCES pppoe_sessions(id),
    dhcp_lease_id BIGINT REFERENCES dhcp_leases(id),
    mac_address MACADDR NOT NULL,
    ip_address INET NOT NULL,
    device_id BIGINT REFERENCES network_devices(id),
    port_id BIGINT,
    vlan_id BIGINT REFERENCES vlans(id),
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    disconnected_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    bytes_in BIGINT DEFAULT 0,
    bytes_out BIGINT DEFAULT 0,
    is_online BOOLEAN DEFAULT TRUE,
    latency_ms DECIMAL(7,2),
    packet_loss_percent DECIMAL(5,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

CREATE TABLE IF NOT EXISTS customer_sessions_2026_07 PARTITION OF customer_sessions
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
