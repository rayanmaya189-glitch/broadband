# AeroXe Admin Portal — Devices Module

> **Req Ref:** §6 Hardware Device Management, §16 Admin Portal

---

## 1. Overview

Network device management — register, monitor, configure, restart, shutdown, and manage firmware for OLT, ONT, Router, Switch, and Access Point devices. Includes health monitoring dashboard and device control.

## 2. Pages

### Device List (`/devices`)

```
┌──────────────────────────────────────────────────────────┐
│  Network Devices                        [+ Register] [Export] │
├──────────────────────────────────────────────────────────┤
│  Type: [All ▼] Status: [All ▼] Vendor: [All ▼] Branch: [All ▼] │
├──────────────────────────────────────────────────────────┤
│  Name                    │ Type │ Vendor  │ IP          │ Status    │ Health │
│  Jalgaon-CityCenter-OLT  │ OLT  │ Huawei  │ 10.0.0.1   │ ● Online  │ 95/100 │
│  Jalgaon-CityCenter-R01  │ Rout │ MikroTik│ 10.0.0.10  │ ● Online  │ 88/100 │
│  Jalgaon-MIDC-SW01       │ Swit │ Cisco   │ 10.0.0.20  │ ● Offline │ 0/100  │
│  Bhusawal-OLT-01         │ OLT  │ ZTE     │ 10.0.1.1   │ ● Degraded│ 45/100 │
└──────────────────────────────────────────────────────────┘
```

### Device Detail (`/devices/:id`)

```
┌──────────────────────────────────────────────────────────┐
│  Jalgaon-CityCenter-OLT-01      [Restart] [Configure] [Firmware] │
│  Huawei MA5683T  │  10.0.0.1  │  ● Online  │  Health: 95  │
├──────────────────────────────────────────────────────────┤
│  [Overview] [Ports] [Metrics] [Logs] [Configuration] [History] │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Device Info           │  Health Metrics (Live)          │
│  ──────────            │  ────────────────────           │
│  Serial: HW-2100-OLT   │  CPU: 32% [████░░░░░░]         │
│  Firmware: V800R017C10 │  Memory: 45% [█████░░░░░]      │
│  Location: City Center │  Temperature: 42°C [███░░░░░░]  │
│  Last Seen: 2 min ago  │  Uplink: ● Up                   │
│                        │  Bandwidth: 65% [███████░░░░]   │
│                        │                                 │
│  PON Ports: 16         │  Connected ONTs: 247            │
│  Active ONTs: 231      │  Offline ONTs: 16               │
└──────────────────────────────────────────────────────────┘
```

## 3. Device Control Actions

| Action | Permission | Approval Required | Confirmation Dialog |
|--------|-----------|-------------------|-------------------|
| Restart | `device.*.restart` | No | "Are you sure?" |
| Shutdown | `device.*.shutdown` | Yes | "This will interrupt service" |
| Configure | `device.*.configure` | Yes | Configuration editor |
| Update Firmware | `device.*.update_firmware` | Yes (isp_owner) | Version selector + confirmation |
| Decommission | `device.*.remove` | Yes (isp_owner) | "This is permanent" |

## 4. Port Management

```
PON Port List (OLT):
┌────────┬──────────┬──────────┬──────────┬──────────┐
│ Port   │ Status   │ ONTs     │ BW Util  │ Actions  │
├────────┼──────────┼──────────┼──────────┼──────────┤
│ PON 0/0│ ● Up     │ 16/32    │ 45%      │ [Disable]│
│ PON 0/1│ ● Up     │ 18/32    │ 52%      │ [Disable]│
│ PON 0/2│ ● Down   │ 0/32     │ 0%       │ [Enable] │
│ PON 0/3│ ● Up     │ 12/32    │ 28%      │ [Disable]│
└────────┴──────────┴──────────┴──────────┴──────────┘
```

## 5. Firmware Management

```
Firmware Updates:
├── Current Version: V800R017C10
├── Available Update: V800R018C02
├── [Check for Updates]
├── [Schedule Update] → Date/time picker
└── Update History:
    ├── V800R017C10 ← Installed Jul 1, 2026
    └── V800R016C10 ← Installed Jun 15, 2026
```

## 6. Real-Time Metrics Charts

- **CPU Usage** (line chart, last 24h)
- **Memory Usage** (line chart, last 24h)
- **Bandwidth Utilization** (area chart, last 24h)
- **Temperature** (line chart, last 24h)
- **Packet Loss** (line chart, last 24h)
- **Latency** (line chart, last 24h)

## 7. API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/devices/list` | POST | List devices |
| `/api/v1/devices/create` | POST | Register device |
| `/api/v1/devices/get` | POST | Get device details |
| `/api/v1/devices/update` | PATCH | Update device |
| `/api/v1/devices/restart` | POST | Restart device |
| `/api/v1/devices/shutdown` | POST | Shutdown device |
| `/api/v1/devices/configure` | PATCH | Configure device |
| `/api/v1/devices/ports/list` | POST | List ports |
| `/api/v1/devices/ports/update` | PATCH | Enable/disable port |
| `/api/v1/devices/logs/list` | POST | Get device logs |
| `/api/v1/devices/metrics/list` | POST | Get device metrics |
| `/api/v1/devices/firmware/update` | POST | Update firmware |
| `/api/v1/devices/models/list` | POST | List device models |
| `/api/v1/devices/models/create` | POST | Register device model |

## 8. RBAC

| Action | Required Permission |
|--------|-------------------|
| View devices | `device.*.view` |
| Register device | `device.*.register` |
| Restart device | `device.*.restart` |
| Configure device | `device.*.configure` |
| Update firmware | `device.*.update_firmware` |
| Enable/disable port | `device.port.enable` / `device.port.disable` |
