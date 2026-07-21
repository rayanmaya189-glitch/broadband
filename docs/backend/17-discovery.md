# AeroXe Backend — Device Discovery Module

> **Req Ref:** §6.6 Plug-and-Play Device Detection

---

## 1. Overview

Automatic network device discovery — when a new OLT, ONT, router, switch, or access point is connected, it is detected, fingerprinted, and registered without manual configuration. Uses SNMP, LLDP, CDP, ARP scanning, and PON port scanning.

## 2. Discovery Architecture

```
Network Device Connected to Port
    ↓
Discovery Engine (Background Service)
    ├── SNMP Walk (sysDescr, sysObjectID, sysName)
    ├── LLDP Neighbor Discovery
    ├── CDP Neighbor Discovery (Cisco)
    ├── ARP Table Scanning
    ├── MAC Address Table Learning
    ├── PON Port Scanning (OLT → ONT)
    ├── DHCP Lease Table Scanning
    └── IP Range ICMP Sweep
    ↓
Device Fingerprinting
    ├── Vendor Identification (OUI lookup)
    ├── Model Detection (sysDescr parsing)
    ├── Firmware Version Extraction
    ├── Port Count & Speed Detection
    └── Management Protocol Detection
    ↓
Auto-Registration
    ├── Match against device_models table
    ├── Create network_devices entry
    ├── Map to parent device (LLDP/CDP)
    ├── Assign to city/area (subnet mapping)
    ├── Publish device.discovered event
    └── Alert NOC engineer for approval
```

## 3. Database Tables

```sql
CREATE TABLE discovery_scans (
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

CREATE TABLE discovery_results (
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

CREATE TABLE discovery_scan_history (
    id BIGSERIAL PRIMARY KEY,
    scan_id BIGINT NOT NULL REFERENCES discovery_scans(id),
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE subnet_location_map (
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
```

## 4. Discovery Protocols

| Protocol | Method | Frequency | Data Collected |
|----------|--------|-----------|----------------|
| SNMP Walk | OID tree walk | Every 15 min | sysDescr, sysObjectID, interfaces, MAC tables |
| LLDP | IEEE 802.1AB | Every 60s | Neighbor ID, port, chassis, management IP |
| CDP | Cisco proprietary | Every 60s | Neighbor ID, port, platform |
| ARP Scan | ARP requests | Every 5 min | IP-to-MAC mappings |
| MAC Table | Bridge tables | Every 5 min | Port-to-MAC mappings |
| PON Scan | OLT query | Every 2 min | ONT serial, distance, optical power |
| DHCP Scan | Lease tables | Every 5 min | Hostname, MAC, IP, lease time |
| ICMP Sweep | Ping sweep | Every 10 min | Live hosts, response time |

## 5. Vendor Identification (IANA Enterprise Numbers)

| Enterprise # | Vendor | Common Models |
|-------------|--------|---------------|
| 2011 | Huawei | MA5683T, HG8245H |
| 4881 | ZTE | C300, F670L |
| 14988 | MikroTik | RB760iGS, CCR1036 |
| 9 | Cisco | ISR, Catalyst |
| 4370 | TP-Link | Archer C6, EAP245 |
| 13014 | Ubiquiti | UniFi AP, EdgeRouter |

## 6. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/discovery/scans` | noc_engineer+ | List scan configs |
| POST | `/api/v1/discovery/scans` | network_admin+ | Create scan config |
| PUT | `/api/v1/discovery/scans/:id` | network_admin+ | Update scan config |
| POST | `/api/v1/discovery/scans/:id/start` | noc_engineer+ | Start scan |
| POST | `/api/v1/discovery/scans/:id/stop` | noc_engineer+ | Stop scan |
| GET | `/api/v1/discovery/results` | noc_engineer+ | List discovered devices |
| POST | `/api/v1/discovery/results/:id/approve` | noc_engineer+ | Approve & register |
| POST | `/api/v1/discovery/results/:id/reject` | noc_engineer+ | Reject device |
| GET | `/api/v1/discovery/dashboard` | noc_engineer+ | Discovery dashboard data |

## 7. Events Published

```yaml
device.discovered:
  payload: { discovery_result_id, discovered_ip, vendor, model, auto_registered }
device.auto_registered:
  payload: { device_id, device_name, device_type, vendor, model }
device.rejected:
  payload: { discovery_result_id, reason, rejected_by }
device.ont.discovered:
  payload: { olt_device_id, ont_serial, pon_port, ont_rx_power_dbm }
```

## 8. Security Considerations

| Threat | Mitigation |
|--------|------------|
| Rogue device | Unknown devices flagged for manual review |
| MAC spoofing | Cross-reference with OUI database |
| SNMP brute force | Rate limit attempts, use SNMPv3 |
| Unauthorized access | Restrict management VLAN, use ACLs |
| Device impersonation | Validate sysObjectID signatures |

## 9. RBAC Permissions

```
discovery.scan.view
discovery.scan.create
discovery.scan.start
discovery.scan.stop
discovery.result.view
discovery.result.approve
discovery.result.reject
discovery.config.view
discovery.config.update
```

---

## Monitoring & Device Sync Gap Reference (v2.0)

> **Cross-reference:** `GAP-code-bugs.md` §6, `DESIGN-GAPS-DEEP-ANALYSIS.md` §9.6

| Bug ID | Severity | Issue | Location |
|--------|----------|-------|----------|
| BUG-MON-01 | HIGH | Only 5 hardcoded metrics — incomplete device visibility | `monitoring/services.rs:38-139` |
| BUG-MON-02 | HIGH | `evaluate_alert_rules` returns empty Vec always — no alerts ever generated | `monitoring/services.rs:158-260` |
| BUG-MON-03 | MEDIUM | Fetches ALL alerts then filters in Rust — should filter at DB level | `monitoring/services.rs:318-327` |
| BUG-MON-04 | CRITICAL | Random health scores `rng.gen_range(70..100)` when adapter unreachable — fake healthy status | `device_sync_worker.rs:236-239` |
| BUG-MON-05 | CRITICAL | Monitoring worker never spawned in `main.rs` — zero device metrics | `main.rs` |
| BUG-INT-06 | CRITICAL | Huawei `get_pon_status` returns hardcoded fake values | `integrations/huawei/adapter.rs:559-567` |
| BUG-INT-07 | HIGH | Huawei traffic table CIR/PIR always 0 — no real QoS data | `integrations/huawei/adapter.rs:495-511` |
| BUG-INT-08 | CRITICAL | Huawei SSH output always `success: true` — errors never detected | `integrations/huawei/adapter.rs:236-273` |

**Priority:** Fix MON-04, MON-05, INT-06, INT-08 first (monitoring completely non-functional). See `GAP-IMPLEMENTATION-ROADMAP.md` Phase 0.
