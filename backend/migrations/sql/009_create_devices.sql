-- AeroXe Backend Migration 009: Create Device Management Tables
-- Hardware device catalog, registration, monitoring, and discovery

-- Device Models
CREATE TABLE IF NOT EXISTS device_models (
    id BIGSERIAL PRIMARY KEY,
    vendor VARCHAR(100) NOT NULL,
    model VARCHAR(100) NOT NULL,
    device_type VARCHAR(50) NOT NULL,
    management_protocol VARCHAR(50) NOT NULL,
    default_port INTEGER,
    firmware_versions TEXT[],
    specs JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(vendor, model)
);

-- Network Devices
CREATE TABLE IF NOT EXISTS network_devices (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    device_model_id BIGINT NOT NULL REFERENCES device_models(id),
    serial_number VARCHAR(255) NOT NULL UNIQUE,
    management_ip INET NOT NULL,
    management_port INTEGER DEFAULT 22,
    snmp_community_encrypted TEXT,
    ssh_key_id UUID,
    firmware_version VARCHAR(50),
    firmware_update_available VARCHAR(50),
    status VARCHAR(20) DEFAULT 'offline'
        CHECK (status IN ('online', 'offline', 'degraded', 'maintenance', 'decommissioned')),
    health_score INTEGER DEFAULT 0,
    location_city VARCHAR(100),
    location_area VARCHAR(100),
    location_address TEXT,
    location_latitude DECIMAL(10, 7),
    location_longitude DECIMAL(10, 7),
    parent_device_id BIGINT REFERENCES network_devices(id),
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_network_devices_branch ON network_devices(branch_id);
CREATE INDEX IF NOT EXISTS idx_network_devices_status ON network_devices(status);
CREATE INDEX IF NOT EXISTS idx_network_devices_ip ON network_devices(management_ip);
CREATE INDEX IF NOT EXISTS idx_network_devices_model ON network_devices(device_model_id);

-- Network Devices History
CREATE TABLE IF NOT EXISTS network_devices_history (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE INDEX IF NOT EXISTS idx_network_devices_history_device ON network_devices_history(device_id);

-- Device Ports
CREATE TABLE IF NOT EXISTS device_ports (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id) ON DELETE CASCADE,
    port_number INTEGER NOT NULL,
    port_name VARCHAR(50),
    port_type VARCHAR(50),
    speed_mbps INTEGER,
    status VARCHAR(20) DEFAULT 'down'
        CHECK (status IN ('up', 'down', 'disabled')),
    connected_device_id BIGINT REFERENCES network_devices(id),
    customer_id BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(device_id, port_number)
);

CREATE INDEX IF NOT EXISTS idx_device_ports_device ON device_ports(device_id);

-- Device Logs (will be partitioned by created_at)
CREATE TABLE IF NOT EXISTS device_logs (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    level VARCHAR(10) NOT NULL CHECK (level IN ('info', 'warning', 'error', 'critical')),
    message TEXT NOT NULL,
    source VARCHAR(50),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Create initial partition for current month
CREATE TABLE IF NOT EXISTS device_logs_2026_07 PARTITION OF device_logs
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');

-- Device Metrics (will be partitioned by recorded_at)
CREATE TABLE IF NOT EXISTS device_metrics (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(15,4) NOT NULL,
    unit VARCHAR(20),
    recorded_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (recorded_at);

CREATE TABLE IF NOT EXISTS device_metrics_2026_07 PARTITION OF device_metrics
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');

-- Firmware Updates
CREATE TABLE IF NOT EXISTS firmware_updates (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    from_version VARCHAR(50),
    to_version VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'downloading', 'installing', 'completed', 'failed', 'rolled_back')),
    initiated_by BIGINT REFERENCES users(id),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    failure_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_firmware_updates_device ON firmware_updates(device_id);

-- Discovery Scans
CREATE TABLE IF NOT EXISTS discovery_scans (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    scan_type VARCHAR(30) NOT NULL
        CHECK (scan_type IN ('snmp_walk', 'lldp', 'cdp', 'arp_scan',
                            'mac_table', 'pon_scan', 'dhcp_scan', 'icmp_sweep')),
    target_subnets CIDR[],
    target_devices BIGINT[],
    snmp_community_id BIGINT,
    scan_interval_seconds INTEGER DEFAULT 900,
    is_active BOOLEAN DEFAULT TRUE,
    last_scan_at TIMESTAMPTZ,
    next_scan_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_discovery_scans_branch ON discovery_scans(branch_id);

-- Discovery Results
CREATE TABLE IF NOT EXISTS discovery_results (
    id BIGSERIAL PRIMARY KEY,
    scan_id BIGINT NOT NULL REFERENCES discovery_scans(id),
    discovered_ip INET NOT NULL,
    discovered_mac MACADDR,
    sys_descr TEXT,
    sys_object_id VARCHAR(255),
    sys_name VARCHAR(255),
    sys_uptime INTERVAL,
    vendor VARCHAR(100),
    vendor_enterprise_id INTEGER,
    model VARCHAR(100),
    firmware_version VARCHAR(50),
    port_count INTEGER,
    management_protocol VARCHAR(50),
    capabilities VARCHAR(100)[],
    lldp_neighbors JSONB,
    cdp_neighbors JSONB,
    matched_model_id BIGINT REFERENCES device_models(id),
    matched_device_id BIGINT REFERENCES network_devices(id),
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'auto_registered', 'manual_review',
                          'approved', 'rejected', 'duplicate')),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    rejection_reason TEXT,
    raw_snmp_data JSONB,
    discovered_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_discovery_results_scan ON discovery_results(scan_id);
CREATE INDEX IF NOT EXISTS idx_discovery_results_ip ON discovery_results(discovered_ip);

-- Discovery Scan History
CREATE TABLE IF NOT EXISTS discovery_scan_history (
    id BIGSERIAL PRIMARY KEY,
    scan_id BIGINT NOT NULL REFERENCES discovery_scans(id),
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

-- Subnet Location Map
CREATE TABLE IF NOT EXISTS subnet_location_map (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    subnet CIDR NOT NULL UNIQUE,
    city VARCHAR(100) NOT NULL,
    area VARCHAR(100),
    location_latitude DECIMAL(10, 7),
    location_longitude DECIMAL(10, 7),
    location_address TEXT,
    vlan_id INTEGER,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Inventory Items
CREATE TABLE IF NOT EXISTS inventory_items (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    item_type VARCHAR(50) NOT NULL,
    device_model_id BIGINT REFERENCES device_models(id),
    serial_number VARCHAR(255) UNIQUE,
    barcode VARCHAR(100) UNIQUE,
    purchase_date DATE,
    purchase_price DECIMAL(10,2),
    warranty_expiry DATE,
    supplier VARCHAR(255),
    status VARCHAR(30) DEFAULT 'in_stock'
        CHECK (status IN ('in_stock', 'assigned', 'installed', 'returned',
                          'damaged', 'scrapped', 'in_transit')),
    assigned_to BIGINT REFERENCES users(id),
    assigned_to_branch_id BIGINT REFERENCES branches(id),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_inventory_items_branch ON inventory_items(branch_id);
CREATE INDEX IF NOT EXISTS idx_inventory_items_status ON inventory_items(status);

-- Inventory Movements
CREATE TABLE IF NOT EXISTS inventory_movements (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGINT NOT NULL REFERENCES inventory_items(id),
    movement_type VARCHAR(30) NOT NULL
        CHECK (movement_type IN ('received', 'assigned', 'installed',
                                 'returned', 'transferred', 'scrapped')),
    from_branch_id BIGINT REFERENCES branches(id),
    to_branch_id BIGINT REFERENCES branches(id),
    reference_type VARCHAR(50),
    reference_id BIGINT,
    performed_by BIGINT NOT NULL REFERENCES users(id),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_inventory_movements_item ON inventory_movements(item_id);
