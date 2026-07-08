# AeroXe Admin Portal — Discovery Module

> **Req Ref:** §6.6 Plug-and-Play Device Detection, §16 Admin Portal

---

## 1. Overview

Real-time device discovery dashboard — shows newly detected network devices pending approval, auto-registered devices, rejected devices, and active scan status. NOC engineers use this to monitor and approve discovered hardware.

## 2. Pages

### Discovery Dashboard (`/discovery`)

```
┌──────────────────────────────────────────────────────────┐
│  Device Discovery Dashboard                              │
├──────────┬──────────┬──────────┬──────────────────────────┤
│ Pending  │ Today's  │ Today's  │ Active Scans             │
│ Review:  │ Auto-Reg:│ Rejected:│ 3 running                │
│ 7        │ 12       │ 1        │ Next: 15 min             │
├──────────┴──────────┴──────────┴──────────────────────────┤
│                                                          │
│  Recent Discoveries (Live via WebSocket)                 │
│  ┌──────────────────────────────────────────────────┐   │
│  │ ⏳ 10.10.5.20 │ Huawei HG8245H │ ONT │ 2 min ago │   │
│  │    MAC: AA:BB:CC:DD:EE:FF                        │   │
│  │    Matched: device_models#42                     │   │
│  │    [Approve] [Reject] [View Details]             │   │
│  ├──────────────────────────────────────────────────┤   │
│  │ ✅ 10.10.3.15 │ MikroTik RB760 │ Router │ 5 min  │   │
│  │    Auto-registered → Device ID #156              │   │
│  ├──────────────────────────────────────────────────┤   │
│  │ ❓ 10.10.8.50 │ Unknown        │ ??? │ 8 min ago │   │
│  │    No match in device_models                     │   │
│  │    [Create Model] [Reject]                       │   │
│  └──────────────────────────────────────────────────┘   │
│                                                          │
│  Scan Status                                            │
│  ┌──────────────────────────────────────────────────┐   │
│  │ Jalgaon SNMP Walk │ Last: 10:30 │ Next: 10:45    │   │
│  │ Jalgaon LLDP      │ Last: 10:29 │ Next: 10:30    │   │
│  │ Bhusawal SNMP     │ Last: 10:25 │ Next: 10:40    │   │
│  └──────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘
```

### Discovery Results (`/discovery/results`)

Full table of all discovered devices with filtering and bulk actions.

### Scan Configuration (`/discovery/scans`)

CRUD for scan configurations — target subnets, intervals, SNMP communities.

## 3. Key Features

### Approve & Register
```
1. NOC engineer clicks "Approve" on a discovered device
2. System creates network_devices entry
3. Assigns to branch (via subnet → location mapping)
4. Maps to parent device (via LLDP/CDP)
5. Starts monitoring (SNMP polling)
6. Publishes device.auto_registered event
```

### Reject Unknown Device
```
1. NOC engineer clicks "Reject"
2. Enters rejection reason
3. Device marked as rejected in discovery_results
4. Publishes device.rejected event
5. Security alert if device appears on multiple subnets
```

## 4. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/discovery/scans` | GET/POST | List/create scan configs |
| `/api/v1/discovery/scans/:id` | PUT/DELETE | Update/delete scan |
| `/api/v1/discovery/scans/:id/start` | POST | Start scan |
| `/api/v1/discovery/scans/:id/stop` | POST | Stop scan |
| `/api/v1/discovery/results` | GET | List discovered devices |
| `/api/v1/discovery/results/:id/approve` | POST | Approve & register |
| `/api/v1/discovery/results/:id/reject` | POST | Reject device |
| `/api/v1/discovery/dashboard` | GET | Dashboard summary |

## 5. WebSocket Channel

```
ws:noc:discovery → Real-time discovery events
  - device.discovered (new device detected)
  - device.auto_registered (auto-registered)
  - device.rejected (rejected by NOC)
```

## 6. RBAC

| Action | Required Permission |
|--------|-------------------|
| View scans | `discovery.scan.view` |
| Create scan | `discovery.scan.create` |
| Start/stop scan | `discovery.scan.start` / `discovery.scan.stop` |
| View results | `discovery.result.view` |
| Approve device | `discovery.result.approve` |
| Reject device | `discovery.result.reject` |
