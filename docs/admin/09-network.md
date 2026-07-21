# AeroXe Admin Portal — Network Module

> **Req Ref:** §7 Network Management Module, §16 Admin Portal

---

## 1. Overview

Network infrastructure management — VLANs, IP pools, PPPoE sessions, DHCP leases, MAC bindings, and customer session tracking. Includes network topology visualization and real-time session monitoring.

## 2. Pages

### VLAN Management (`/network/vlans`)

```
┌──────────────────────────────────────────────────────────┐
│  VLANs                                      [+ Add VLAN] │
├──────────────────────────────────────────────────────────┤
│  Branch: [All ▼]  Type: [All ▼]                         │
├──────────────────────────────────────────────────────────┤
│  VLAN ID │ Name                  │ Type         │ Branch │ Status │
│  100     │ OLT Management        │ Management   │ JLG    │ ● Active│
│  200     │ Jalgaon City Center   │ Residential  │ JLG    │ ● Active│
│  300     │ MIDC Area Business    │ Business     │ JLG    │ ● Active│
│  400     │ IPTV Multicast        │ IPTV         │ JLG    │ ● Active│
│  900     │ SNMP Monitoring       │ Monitoring   │ JLG    │ ● Active│
│  200     │ Bhusawal Residential  │ Residential  │ BHL    │ ● Active│
└──────────────────────────────────────────────────────────┘
```

### IP Pool Management (`/network/ip-pools`)

```
┌──────────────────────────────────────────────────────────┐
│  IP Pools                                    [+ Add Pool] │
├──────────────────────────────────────────────────────────┤
│  Name                  │ CIDR           │ Utilization │ Status │
│  Jalgaon City Center   │ 10.10.0.0/16   │ 1.9% █░░░░ │ Healthy│
│  MIDC Area Business    │ 10.20.0.0/16   │ 12.5% ██░░░ │ Healthy│
│  Bhusawal Residential  │ 10.30.0.0/16   │ 78.5% ████████│ Warning│
│  Management Network    │ 10.0.0.0/24    │ 45.0% █████░░░│ Healthy│
└──────────────────────────────────────────────────────────┘

Pool Detail: Jalgaon City Center
├── Gateway: 10.10.0.1
├── DNS: 1.1.1.1 / 8.8.8.8
├── DHCP Range: 10.10.1.1 - 10.10.254.254
├── VLAN: 200
├── Allocated: 1,250 / 65,534 (1.9%)
├── Available: 64,284
├── Reserved: 12
└── [Allocate IP] [Release IP] [View Addresses]
```

### PPPoE Sessions (`/network/pppoe`)

```
┌──────────────────────────────────────────────────────────┐
│  PPPoE Sessions                                          │
├──────────────────────────────────────────────────────────┤
│  Customer      │ Username       │ IP            │ Status │ Duration │
│  Rahul Sharma  │ rahul@aeroxe   │ 10.10.1.100   │ ● Active│ 6h 30m  │
│  Priya Patil   │ priya@aeroxe   │ 10.10.1.101   │ ● Active│ 4h 15m  │
│  Amit Deshmukh │ amit@aeroxe    │ —             │ ● Inactive│ —     │
└──────────────────────────────────────────────────────────┘
```

### DHCP Leases (`/network/dhcp`)

### MAC Bindings (`/network/mac-bindings`)

### Customer Sessions (`/network/sessions`)

Real-time view of all online customers with bandwidth usage, latency, packet loss.

### Network Topology (`/network/topology`)

Interactive network topology visualization:
```
Internet → Core Router → Distribution Switch → OLT → ONT → Customer
```
Shows device status, connections, and bandwidth utilization on each link.

## 3. API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/network/vlans/list` | POST | List VLANs |
| `/api/v1/network/vlans/create` | POST | Create VLAN |
| `/api/v1/network/vlans/update` | PATCH | Update VLAN |
| `/api/v1/network/vlans/delete` | DELETE | Delete VLAN |
| `/api/v1/network/ip-pools/list` | POST | List IP pools |
| `/api/v1/network/ip-pools/create` | POST | Create IP pool |
| `/api/v1/network/ip-pools/update` | PATCH | Update pool |
| `/api/v1/network/ip-pools/addresses/list` | POST | List addresses |
| `/api/v1/network/ip-pools/allocate` | POST | Allocate IP |
| `/api/v1/network/ip-pools/release` | POST | Release IP |
| `/api/v1/network/pppoe/sessions/list` | POST | List PPPoE sessions |
| `/api/v1/network/pppoe/sessions/terminate` | POST | Terminate session |
| `/api/v1/network/dhcp/leases/list` | POST | List DHCP leases |
| `/api/v1/network/mac-bindings/list` | POST | List MAC bindings |
| `/api/v1/network/mac-bindings/create` | POST | Create MAC binding |
| `/api/v1/network/sessions/list` | POST | Customer sessions |
| `/api/v1/network/topology/list` | POST | Network topology data |

## 4. RBAC

| Action | Required Permission |
|--------|-------------------|
| View VLANs | `network.vlan.view` |
| Create VLAN | `network.vlan.create` |
| View IP pools | `network.ippool.view` |
| Allocate IP | `network.ippool.allocate` |
| View PPPoE sessions | `network.pppoe.view` |
| Terminate session | `network.pppoe.terminate` |
| View DHCP leases | `network.dhcp.view` |
| View MAC bindings | `network.mac_binding.view` |
| View topology | `network_topology.view` |
