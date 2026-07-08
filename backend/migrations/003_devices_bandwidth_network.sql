-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — Devices, Bandwidth, Network
-- ═══════════════════════════════════════════════════════════════

-- ── Device Models ────────────────────────────────────────────
CREATE TABLE device_models (
    id                  BIGSERIAL PRIMARY KEY,
    vendor              VARCHAR(100) NOT NULL,
    model               VARCHAR(100) NOT NULL,
    device_type         VARCHAR(50) NOT NULL,
    management_protocol VARCHAR(50) NOT NULL,
    default_port        INTEGER,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(vendor, model)
);

-- ── Network Devices ──────────────────────────────────────────
CREATE TABLE network_devices (
    id                  BIGSERIAL PRIMARY KEY,
    branch_id           BIGINT NOT NULL REFERENCES branches(id),
    name                VARCHAR(255) NOT NULL,
    device_model_id     BIGINT NOT NULL REFERENCES device_models(id),
    serial_number       VARCHAR(255) NOT NULL UNIQUE,
    management_ip       INET NOT NULL,
    management_port     INTEGER DEFAULT 22,
    firmware_version    VARCHAR(50),
    status              VARCHAR(20) DEFAULT 'offline'
        CHECK (status IN ('online', 'offline', 'degraded', 'maintenance', 'decommissioned')),
    health_score        INTEGER DEFAULT 0,
    location_city       VARCHAR(100),
    location_area       VARCHAR(100),
    created_by          BIGINT REFERENCES users(id),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_network_devices_branch ON network_devices(branch_id);
CREATE INDEX idx_network_devices_status ON network_devices(status);

-- ── Device Ports ─────────────────────────────────────────────
CREATE TABLE device_ports (
    id                  BIGSERIAL PRIMARY KEY,
    device_id           BIGINT NOT NULL REFERENCES network_devices(id) ON DELETE CASCADE,
    port_number         INTEGER NOT NULL,
    port_name           VARCHAR(50),
    port_type           VARCHAR(50),
    speed_mbps          INTEGER,
    status              VARCHAR(20) DEFAULT 'down'
        CHECK (status IN ('up', 'down', 'disabled')),
    connected_device_id BIGINT REFERENCES network_devices(id),
    customer_id         BIGINT REFERENCES customers(id),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(device_id, port_number)
);

-- ── Bandwidth Profiles ───────────────────────────────────────
CREATE TABLE bandwidth_profiles (
    id                      BIGSERIAL PRIMARY KEY,
    name                    VARCHAR(100) NOT NULL,
    description             TEXT,
    plan_id                 BIGINT REFERENCES plans(id),
    download_kbps           INTEGER NOT NULL,
    upload_kbps             INTEGER NOT NULL,
    burst_download_kbps     INTEGER,
    burst_upload_kbps       INTEGER,
    burst_duration_seconds  INTEGER DEFAULT 30,
    priority                INTEGER DEFAULT 1,
    is_active               BOOLEAN DEFAULT TRUE,
    created_at              TIMESTAMPTZ DEFAULT NOW(),
    updated_at              TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_bandwidth_profiles_plan ON bandwidth_profiles(plan_id);

-- ── VLANs ────────────────────────────────────────────────────
CREATE TABLE vlans (
    id              BIGSERIAL PRIMARY KEY,
    branch_id       BIGINT NOT NULL REFERENCES branches(id),
    vlan_id         INTEGER NOT NULL,
    name            VARCHAR(100) NOT NULL,
    description     TEXT,
    vlan_type       VARCHAR(30) NOT NULL
        CHECK (vlan_type IN ('management', 'customer_residential', 'customer_business',
                             'iptv', 'voip', 'monitoring')),
    is_active       BOOLEAN DEFAULT TRUE,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, vlan_id)
);

-- ── IP Pools ─────────────────────────────────────────────────
CREATE TABLE ip_pools (
    id                          BIGSERIAL PRIMARY KEY,
    branch_id                   BIGINT NOT NULL REFERENCES branches(id),
    name                        VARCHAR(100) NOT NULL,
    cidr                        CIDR NOT NULL,
    gateway                     INET NOT NULL,
    dns_primary                 INET DEFAULT '1.1.1.1',
    dns_secondary               INET DEFAULT '8.8.8.8',
    vlan_id                     BIGINT REFERENCES vlans(id),
    pool_type                   VARCHAR(30) DEFAULT 'customer'
        CHECK (pool_type IN ('customer', 'management', 'shared_services')),
    allocated_count             INTEGER DEFAULT 0,
    total_count                 INTEGER NOT NULL,
    status                      VARCHAR(20) DEFAULT 'healthy'
        CHECK (status IN ('healthy', 'warning', 'critical', 'exhausted')),
    is_active                   BOOLEAN DEFAULT TRUE,
    created_at                  TIMESTAMPTZ DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, cidr)
);

-- ── PPPoE Sessions ───────────────────────────────────────────
CREATE TABLE pppoe_sessions (
    id                  BIGSERIAL PRIMARY KEY,
    branch_id           BIGINT NOT NULL REFERENCES branches(id),
    customer_id         BIGINT NOT NULL REFERENCES customers(id),
    subscription_id     BIGINT NOT NULL REFERENCES subscriptions(id),
    username            VARCHAR(100) NOT NULL UNIQUE,
    assigned_ip         INET,
    status              VARCHAR(20) DEFAULT 'inactive'
        CHECK (status IN ('active', 'inactive', 'terminated')),
    session_start       TIMESTAMPTZ,
    bytes_in            BIGINT DEFAULT 0,
    bytes_out           BIGINT DEFAULT 0,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_pppoe_sessions_customer ON pppoe_sessions(customer_id);
CREATE INDEX idx_pppoe_sessions_status ON pppoe_sessions(status);

-- ── Coverage Areas ───────────────────────────────────────────
CREATE TABLE coverage_areas (
    id                          BIGSERIAL PRIMARY KEY,
    branch_id                   BIGINT NOT NULL REFERENCES branches(id),
    name                        VARCHAR(255) NOT NULL,
    description                 TEXT,
    area_type                   VARCHAR(30) DEFAULT 'polygon'
        CHECK (area_type IN ('polygon', 'circle', 'pincode')),
    pincodes                    TEXT[],
    is_active                   BOOLEAN DEFAULT TRUE,
    max_customers               INTEGER,
    current_customers           INTEGER DEFAULT 0,
    fiber_available             BOOLEAN DEFAULT TRUE,
    estimated_installation_days INTEGER DEFAULT 3,
    created_at                  TIMESTAMPTZ DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_coverage_areas_branch ON coverage_areas(branch_id);

-- ── Coverage Pincode Map ─────────────────────────────────────
CREATE TABLE coverage_pincode_map (
    id                  BIGSERIAL PRIMARY KEY,
    coverage_area_id    BIGINT NOT NULL REFERENCES coverage_areas(id) ON DELETE CASCADE,
    pincode             VARCHAR(10) NOT NULL,
    city                VARCHAR(100) NOT NULL,
    district            VARCHAR(100),
    state               VARCHAR(100) DEFAULT 'Maharashtra',
    is_active           BOOLEAN DEFAULT TRUE,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(coverage_area_id, pincode)
);

CREATE INDEX idx_coverage_pincode_pincode ON coverage_pincode_map(pincode);

-- ── Installation Orders ──────────────────────────────────────
CREATE TABLE installation_orders (
    id                      BIGSERIAL PRIMARY KEY,
    customer_id             BIGINT NOT NULL REFERENCES customers(id),
    branch_id               BIGINT NOT NULL REFERENCES branches(id),
    subscription_id         BIGINT REFERENCES subscriptions(id),
    assigned_technician_id  BIGINT REFERENCES users(id),
    status                  VARCHAR(30) DEFAULT 'pending'
        CHECK (status IN ('pending', 'scheduled', 'in_progress', 'completed', 'cancelled', 'rescheduled')),
    scheduled_date          DATE,
    scheduled_time_slot     VARCHAR(20),
    completed_at            TIMESTAMPTZ,
    installation_type       VARCHAR(20) DEFAULT 'new',
    equipment_issued        JSONB,
    fiber_drop_length_meters INTEGER,
    onu_power_dbm           DECIMAL(5,2),
    notes                   TEXT,
    created_at              TIMESTAMPTZ DEFAULT NOW(),
    updated_at              TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_installation_orders_customer ON installation_orders(customer_id);
CREATE INDEX idx_installation_orders_branch ON installation_orders(branch_id);
CREATE INDEX idx_installation_orders_technician ON installation_orders(assigned_technician_id);
CREATE INDEX idx_installation_orders_status ON installation_orders(status);

-- ── Inventory Items ──────────────────────────────────────────
CREATE TABLE inventory_items (
    id                  BIGSERIAL PRIMARY KEY,
    branch_id           BIGINT NOT NULL REFERENCES branches(id),
    item_type           VARCHAR(50) NOT NULL,
    device_model_id     BIGINT REFERENCES device_models(id),
    serial_number       VARCHAR(255) UNIQUE,
    barcode             VARCHAR(100) UNIQUE,
    purchase_date       DATE,
    purchase_price      DECIMAL(10,2),
    warranty_expiry     DATE,
    supplier            VARCHAR(255),
    status              VARCHAR(30) DEFAULT 'in_stock'
        CHECK (status IN ('in_stock', 'assigned', 'installed', 'returned',
                          'damaged', 'scrapped', 'in_transit')),
    assigned_to         BIGINT REFERENCES users(id),
    notes               TEXT,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_inventory_items_branch ON inventory_items(branch_id);
CREATE INDEX idx_inventory_items_status ON inventory_items(status);

-- ── Inventory Movements ──────────────────────────────────────
CREATE TABLE inventory_movements (
    id                  BIGSERIAL PRIMARY KEY,
    item_id             BIGINT NOT NULL REFERENCES inventory_items(id),
    movement_type       VARCHAR(30) NOT NULL
        CHECK (movement_type IN ('received', 'assigned', 'installed',
                                 'returned', 'transferred', 'scrapped')),
    from_branch_id      BIGINT REFERENCES branches(id),
    to_branch_id        BIGINT REFERENCES branches(id),
    performed_by        BIGINT NOT NULL REFERENCES users(id),
    notes               TEXT,
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

-- ── Discovery Scans ──────────────────────────────────────────
CREATE TABLE discovery_scans (
    id                  BIGSERIAL PRIMARY KEY,
    branch_id           BIGINT NOT NULL REFERENCES branches(id),
    name                VARCHAR(100) NOT NULL,
    scan_type           VARCHAR(30) NOT NULL
        CHECK (scan_type IN ('snmp_walk', 'lldp', 'cdp', 'arp_scan',
                            'mac_table', 'pon_scan', 'dhcp_scan', 'icmp_sweep')),
    is_active           BOOLEAN DEFAULT TRUE,
    last_scan_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

-- ── Discovery Results ────────────────────────────────────────
CREATE TABLE discovery_results (
    id                  BIGSERIAL PRIMARY KEY,
    scan_id             BIGINT NOT NULL REFERENCES discovery_scans(id),
    discovered_ip       INET NOT NULL,
    vendor              VARCHAR(100),
    model               VARCHAR(100),
    firmware_version    VARCHAR(50),
    status              VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'auto_registered', 'manual_review',
                          'approved', 'rejected', 'duplicate')),
    discovered_at       TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_discovery_results_scan ON discovery_results(scan_id);
CREATE INDEX idx_discovery_results_status ON discovery_results(status);
