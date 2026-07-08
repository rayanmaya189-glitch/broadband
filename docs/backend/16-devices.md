# AeroXe Backend — Hardware Device Management Module

> **Req Ref:** §6 Hardware Device Management

---

## 1. Overview

Manages registration, monitoring, configuration, and lifecycle of network hardware (OLT, ONT, Router, Switch, Access Point). Includes health monitoring via SNMP, firmware management, and device control permissions.

## 2. Supported Device Types

| Type | Vendor Examples | Protocol | Use Case |
|------|----------------|----------|----------|
| OLT | Huawei MA5683T, ZTE C300 | Telnet/SSH, SNMP, NETCONF | Fiber aggregation |
| ONT | Huawei HG8245H, ZTE F670L | TR-069, OMCI, SSH | Customer premises |
| Router | MikroTik RB760iGS, Cisco ISR | RouterOS API, SSH, SNMP | Distribution/core |
| Switch | MikroTik CRS, Cisco Catalyst | SNMP, SSH, API | Distribution/access |
| Access Point | Ubiquiti, TP-Link | SNMP, HTTP API | WiFi coverage |

## 3. Database Tables

```sql
CREATE TABLE device_models (
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

CREATE TABLE network_devices (
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

CREATE TABLE network_devices_history (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE device_ports (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id) ON DELETE CASCADE,
    port_number INTEGER NOT NULL,
    port_name VARCHAR(50),
    port_type VARCHAR(50),
    speed_mbps INTEGER,
    status VARCHAR(20) DEFAULT 'down'
        CHECK (status IN ('up', 'down', 'disabled')),
    connected_device_id BIGINT REFERENCES network_devices(id),
    customer_id BIGINT REFERENCES customers(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(device_id, port_number)
);

CREATE TABLE device_logs (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    level VARCHAR(10) NOT NULL CHECK (level IN ('info', 'warning', 'error', 'critical')),
    message TEXT NOT NULL,
    source VARCHAR(50),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

CREATE TABLE device_metrics (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(15,4) NOT NULL,
    unit VARCHAR(20),
    recorded_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (recorded_at);

CREATE TABLE firmware_updates (
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
```

## 4. Health Monitoring Metrics

| Metric | Critical | Warning | Method |
|--------|----------|---------|--------|
| CPU Usage | > 90% | > 70% | SNMP |
| Memory Usage | > 90% | > 80% | SNMP |
| Uplink Status | Down | Flapping | SNMP/ICMP |
| Temperature | > 70°C | > 60°C | SNMP |
| ONT Optical Power | < -28 dBm | < -25 dBm | OMCI |
| Packet Loss | > 5% | > 1% | SNMP counters |
| Latency | > 50ms | > 20ms | ICMP probe |
| Bandwidth Utilization | > 95% | > 80% | SNMP counters |

## 5. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/devices` | noc_engineer+ | List devices |
| POST | `/api/v1/devices` | network_admin+ | Register device |
| GET | `/api/v1/devices/:id` | noc_engineer+ | Get device details |
| PUT | `/api/v1/devices/:id` | network_admin+ | Update device |
| POST | `/api/v1/devices/:id/restart` | noc_engineer+ | Restart device |
| POST | `/api/v1/devices/:id/shutdown` | network_admin+ | Shutdown device |
| PUT | `/api/v1/devices/:id/configure` | network_admin+ | Configure device |
| GET | `/api/v1/devices/:id/ports` | noc_engineer+ | List ports |
| PUT | `/api/v1/devices/:id/ports/:pid` | noc_engineer+ | Enable/disable port |
| GET | `/api/v1/devices/:id/logs` | noc_engineer+ | Get device logs |
| GET | `/api/v1/devices/:id/metrics` | noc_engineer+ | Get device metrics |
| POST | `/api/v1/devices/:id/firmware/update` | network_admin+ | Update firmware |
| GET | `/api/v1/devices/:id/firmware` | noc_engineer+ | Firmware status |
| GET | `/api/v1/devices/models` | network_admin+ | List device models |
| POST | `/api/v1/devices/models` | network_admin+ | Register device model |

## 6. Device Control Permissions

| Operation | Required Role | Approval Required |
|-----------|---------------|-------------------|
| View device | noc_engineer+ | No |
| Restart device | noc_engineer+ | No |
| Shutdown device | network_admin+ | Yes |
| Configure device | network_admin+ | Yes |
| Update firmware | network_admin+ | Yes (isp_owner) |
| Register device | network_admin+ | No |
| Decommission device | network_admin+ | Yes (isp_owner) |
| Enable/disable port | noc_engineer+ | No |

## 7. Events Published

```yaml
device.registered:
  payload: { device_id, name, type, vendor, model, management_ip }
device.status.changed:
  payload: { device_id, old_status, new_status, health_score }
device.firmware.update.started:
  payload: { device_id, from_version, to_version }
device.firmware.update.completed:
  payload: { device_id, new_version }
device.firmware.update.failed:
  payload: { device_id, reason }
device.health.alert:
  payload: { device_id, metric, value, threshold, severity }
```

## 8. RBAC Permissions

```
device.router.view
device.router.register
device.router.configure
device.router.restart
device.router.shutdown
device.router.update_firmware
device.router.remove
device.switch.view
device.switch.register
device.switch.configure
device.switch.restart
device.switch.shutdown
device.switch.update_firmware
device.switch.remove
device.olt.view
device.olt.register
device.olt.configure
device.olt.restart
device.olt.update_firmware
device.olt.remove
device.olt.deploy_config
device.ont.view
device.ont.register
device.ont.configure
device.ont.restart
device.ont.update_firmware
device.ont.remove
device.ont.provision
device.access_point.view
device.access_point.register
device.access_point.configure
device.access_point.restart
device.access_point.update_firmware
device.access_point.remove
device.port.view
device.port.enable
device.port.disable
device.port.configure
```
