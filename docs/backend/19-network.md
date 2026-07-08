# AeroXe Backend — Network Management Module

> **Req Ref:** §7 Network Management Module

---

## 1. Overview

Manages VLANs, IP pools, PPPoE sessions, DHCP leases, MAC bindings, and customer session tracking. Provides the network infrastructure layer that connects customers to the internet.

## 2. Network Topology

```
Internet Upstream (Tier 1/2 ISP)
    ↓
Core Router (MikroTik CCR / Cisco ASR)
    ↓
Distribution Switch (MikroTik CRS / Cisco Catalyst)
    ↓
OLT (Huawei MA5683T / ZTE C300)
    ↓ (Fiber split 1:32 / 1:64)
Splitter
    ↓
ONT (Huawei HG8245H / ZTE F670L)
    ↓ (Ethernet / WiFi)
Customer Premises
```

## 3. VLAN Ranges

| VLAN ID Range | Purpose | Example |
|---------------|---------|---------|
| 100–199 | Management | VLAN 100 — OLT management |
| 200–299 | Customer Data (Residential) | VLAN 200 — Jalgaon City Center |
| 300–399 | Customer Data (Business) | VLAN 300 — MIDC Area |
| 400–499 | IPTV/Multicast | VLAN 400 — IPTV |
| 500–599 | VoIP | VLAN 500 — SIP Trunk |
| 900–999 | Monitoring/SNMP | VLAN 900 — SNMP |

## 4. Database Tables

```sql
CREATE TABLE vlans (
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

CREATE TABLE ip_pools (
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

CREATE TABLE ip_addresses (
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

CREATE TABLE pppoe_sessions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
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

CREATE TABLE dhcp_leases (
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

CREATE TABLE mac_bindings (
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

CREATE TABLE customer_sessions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    pppoe_session_id BIGINT REFERENCES pppoe_sessions(id),
    dhcp_lease_id BIGINT REFERENCES dhcp_leases(id),
    mac_address MACADDR NOT NULL,
    ip_address INET NOT NULL,
    device_id BIGINT REFERENCES network_devices(id),
    port_id BIGINT REFERENCES device_ports(id),
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
```

## 5. Online/Offline Status

Status determined by:
1. PPPoE session active → Online
2. DHCP lease active → Online
3. SNMP heartbeat within 5 minutes → Online
4. ONT OMCI link up → Online
5. None of above → Offline

## 6. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/network/vlans` | network.*.view | List VLANs |
| POST | `/api/v1/network/vlans` | network.vlan.create | Create VLAN |
| PUT | `/api/v1/network/vlans/:id` | network.vlan.update | Update VLAN |
| DELETE | `/api/v1/network/vlans/:id` | network.vlan.delete | Delete VLAN |
| GET | `/api/v1/network/ip-pools` | network.ippool.view | List IP pools |
| POST | `/api/v1/network/ip-pools` | network.ippool.create | Create IP pool |
| PUT | `/api/v1/network/ip-pools/:id` | network.ippool.update | Update IP pool |
| GET | `/api/v1/network/ip-pools/:id/addresses` | network.ippool.view | List addresses |
| POST | `/api/v1/network/ip-pools/:id/allocate` | network.ippool.allocate | Allocate IP |
| POST | `/api/v1/network/ip-pools/:id/release` | network.ippool.release | Release IP |
| GET | `/api/v1/network/pppoe/sessions` | network.pppoe.view | List PPPoE sessions |
| POST | `/api/v1/network/pppoe/sessions` | network.pppoe.create | Create PPPoE session |
| POST | `/api/v1/network/pppoe/sessions/:id/terminate` | network.pppoe.terminate | Terminate session |
| GET | `/api/v1/network/dhcp/leases` | network.dhcp.view | List DHCP leases |
| GET | `/api/v1/network/mac-bindings` | network.mac_binding.view | List MAC bindings |
| POST | `/api/v1/network/mac-bindings` | network.mac_binding.create | Create MAC binding |
| GET | `/api/v1/network/sessions` | noc_engineer+ | Customer sessions |
| GET | `/api/v1/network/topology` | noc_engineer+ | Network topology |

## 7. Events Published

```yaml
vlan.created:
  payload: { vlan_id, branch_id, vlan_tag, vlan_type }
vlan.deleted:
  payload: { vlan_id, branch_id }
ippool.exhausted:
  payload: { pool_id, branch_id, utilization_percent }
ippool.warning:
  payload: { pool_id, branch_id, utilization_percent }
pppoe.session.started:
  payload: { session_id, customer_id, assigned_ip }
pppoe.session.ended:
  payload: { session_id, customer_id, duration_seconds, bytes_in, bytes_out }
customer.session.connected:
  payload: { session_id, customer_id, ip_address, mac_address }
customer.session.disconnected:
  payload: { session_id, customer_id, reason }
```

## 8. RBAC Permissions

```
network.vlan.view
network.vlan.create
network.vlan.update
network.vlan.delete
network.vlan.assign
network.vlan.unassign
network.ippool.view
network.ippool.create
network.ippool.update
network.ippool.delete
network.ippool.allocate
network.ippool.release
network.pppoe.view
network.pppoe.create
network.pppoe.update
network.pppoe.delete
network.pppoe.authenticate
network.pppoe.terminate
network.dhcp.view
network.dhcp.create
network.dhcp.update
network.dhcp.delete
network.dhcp.lease
network.mac_binding.view
network.mac_binding.create
network.mac_binding.update
network.mac_binding.delete
network_topology.view
network_topology.update
```
