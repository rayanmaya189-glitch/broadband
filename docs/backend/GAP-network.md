# Network Module — ISP Design Gaps

**Module:** `network`
**Cross-reference:** `DESIGN-GAPS-DEEP-ANALYSIS.md` (ISP-NET-C01 through ISP-NET-M03)

---

## Critical Gaps

### ISP-NET-C01: No RADIUS Accounting Listener

**Files Affected:**
- `src/modules/integrations/radius/adapter.rs`
- `src/modules/network/domain/entities/pppoe_session.rs`
- `src/modules/network/application/service.rs`

**Current State:**
```rust
// RadiusClient trait can SEND packets:
// - authenticate(AccessRequest)
// - accounting_start(AccountingStart)
// - accounting_stop(AccountingStop)
// - accounting_interim(AccountingInterim)
// - change_authorization(CoaRequest)
```

**Missing:**
- No UDP listener for inbound RADIUS Accounting packets
- `pppoe_sessions.bytes_in` and `pppoe_sessions.bytes_out` never updated from real data
- `Calling-Station-Id` (MAC) not sent in Access-Request
- `Called-Station-Id` (BNG identifier) not sent
- `NAS-Identifier` not sent
- `MessageAuthenticator` (RFC 3579) defined but never set
- No `State` attribute tracking for multi-round auth
- No Response Authenticator verification
- No RADIUS proxy/failover
- No `radsec` (TLS) support
- Retry logic configured but never used

**Required Implementation:**
1. Create `RadiusAccountingListener` that binds UDP socket on RADIUS accounting port (1813)
2. Parse incoming Accounting-Request packets
3. Correlate `Acct-Session-Id` to `pppoe_sessions` table
4. Update `bytes_in` (Acct-Input-Octets) and `bytes_out` (Acct-Output-Octets) on each Interim-Update
5. Update `session_end` on Accounting-Stop
6. Add `Calling-Station-Id` to all outbound Access-Request packets
7. Implement RADIUS proxy with primary/secondary failover

**New Entities Required:**
```rust
// radius_accounting entity
pub struct RadiusAccounting {
    pub id: Uuid,
    pub branch_id: Uuid,
    pub pppoe_session_id: Option<Uuid>,
    pub acct_session_id: String,
    pub acct_status_type: AcctStatusType, // Start/Stop/Interim-Update
    pub nas_ip_address: IpAddr,
    pub nas_port_id: Option<String>,
    pub calling_station_id: Option<String>, // MAC
    pub framed_ip_address: Option<IpAddr>,
    pub acct_input_octets: i64,
    pub acct_output_octets: i64,
    pub acct_session_time: i32,
    pub acct_terminate_cause: Option<String>,
    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
```

**New Worker Required:**
```rust
// RadiusAccountingWorker
// - Binds UDP socket on port 1813
// - Parses RADIUS Accounting-Request packets
// - Correlates to pppoe_sessions via Acct-Session-Id
// - Updates bytes_in/bytes_out in real-time
// - Publishes accounting events to NATS
```

---

### ISP-NET-C02: IP Allocation is Fake

**Files Affected:**
- `src/modules/network/application/service.rs` — `allocate_ip()`, `release_ip()`
- `src/modules/network/domain/entities/ip_pool.rs`

**Current State:**
```rust
pub async fn allocate_ip(&self, pool_id: Uuid) -> Result<IpPool> {
    let pool = self.repo.get_pool(pool_id).await?;
    // ONLY increments counter:
    let updated = pool.into_active_model();
    updated.allocated_count = Set(pool.allocated_count + 1);
    // NO actual IP address is allocated from the CIDR range
    self.repo.update_pool(updated).await
}
```

**Missing:**
- No CIDR parsing
- No subnet traversal
- No available IP detection
- No IP conflict prevention
- No per-address allocation records
- `release_ip()` only decrements counter

**Required Implementation:**
1. Parse CIDR into IP range (e.g., `192.168.1.0/24` → `192.168.1.1` to `192.168.1.254`)
2. Create `ip_address` entity for each address in pool
3. `allocate_ip()` finds first available address, marks as allocated
4. `release_ip()` marks address as available
5. Conflict detection: check before allocation
6. Static IP reservation support

**New Entity Required:**
```rust
pub struct IpAddress {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub branch_id: Uuid,
    pub ip_address: IpAddr,
    pub status: IpAddressStatus, // Available, Allocated, Reserved, Conflict
    pub allocated_to_type: Option<String>, // customer, device, pool_gateway
    pub allocated_to_id: Option<Uuid>,
    pub allocated_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
```

---

### ISP-NET-C03: No Device Provisioning Automation

**Files Affected:**
- `src/modules/network/application/service.rs`
- `src/modules/bandwidth/application/service.rs`
- `src/modules/installations/application/service.rs`
- `src/modules/integrations/mikrotik/adapter.rs`
- `src/modules/integrations/huawei/adapter.rs`

**Current State:** Creating a subscription requires manual NOC intervention for each step.

**Required Workflow (ProvisioningWorker):**
```
subscription.activated event
  ↓
1. Create PPPoE credentials on RADIUS server
  ↓
2. Push bandwidth profile to BNG/MikroTik (simple queue or HTB)
  ↓
3. Configure ONT VLAN/QoS on Huawei OLT
  ↓
4. Verify connectivity (SNMP ping or OMCI link check)
  ↓
5. Publish customer.provisioned event
  ↓
6. Notify customer: "Your service is now active!"
```

**New Worker Required:**
```rust
pub struct ProvisioningWorker {
    db: DatabaseConnection,
    radius: Arc<dyn RadiusClient>,
    mikrotik: Arc<dyn MikrotikAdapter>,
    huawei: Arc<dyn HuaweiOltAdapter>,
    event_publisher: EventPublisher,
}

impl ProvisioningWorker {
    pub async fn run(&self) {
        // Listen for subscription.activated events
        // Execute provisioning sequence
        // Handle failures with rollback
        // Retry 3x with exponential backoff
    }
}
```

**New Entity Required:**
```rust
pub struct ProvisioningJob {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub customer_id: Uuid,
    pub branch_id: Uuid,
    pub status: ProvisioningStatus, // Pending, InProgress, Completed, Failed, RolledBack
    pub steps: Vec<ProvisioningStep>,
    pub current_step: ProvisioningStepType,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub enum ProvisioningStepType {
    RadiusAccountCreate,
    BandwidthProfilePush,
    OntVlanConfig,
    ConnectivityVerify,
}
```

---

### ISP-NET-C04: No SNMP Polling

**Files Affected:**
- `src/workers/monitoring_worker.rs`
- `Cargo.toml` (missing `snmp` crate)

**Current State:**
```rust
// monitoring_worker calls adapter.get_health_score()
// which returns a mock integer from the device adapter
// device_metrics table is never populated with real data
```

**Missing:**
- No `snmp` or `agent` crate in dependencies
- Cannot poll: CPU, memory, uptime, interface counters, optical power, temperature
- Cannot detect: device offline, link down, high utilization, ONT power degradation

**Required Implementation:**
1. Add `snmp` crate to Cargo.toml
2. Implement SNMPv2c/v3 GET/WALK for each device type
3. Create `SnmpPoller` service that queries devices on schedule
4. Populate `device_metrics` table with real data
5. Implement threshold-based alerting

**SNMP OIDs to Poll:**
| Metric | OID | Device Type |
|--------|-----|-------------|
| CPU Usage | 1.3.6.1.4.1.2021.11.9.0 | All |
| Memory Usage | 1.3.6.1.4.1.2021.11.11.0 | All |
| System Uptime | 1.3.6.1.2.1.1.3.0 | All |
| Interface Speed | 1.3.6.1.2.1.2.2.1.5.{ifIndex} | Routers/Switches |
| Interface In Octets | 1.3.6.1.2.1.2.2.1.10.{ifIndex} | Routers/Switches |
| Interface Out Octets | 1.3.6.1.2.1.2.2.1.16.{ifIndex} | Routers/Switches |
| ONT Optical Power | 1.3.6.1.4.1.2011.5.25.131.1.1.1.1.{ontId} | Huawei OLT |
| Temperature | 1.3.6.1.4.1.2021.13.16.2.1.0 | All |

---

### ISP-NET-C05: Bandwidth Limits are DB-Only

**Files Affected:**
- `src/workers/bandwidth_worker.rs`
- `src/modules/integrations/mikrotik/adapter.rs`

**Current State:**
```rust
// MikroTik adapter posts to "/run" endpoint
// This is NOT a valid RouterOS v7 REST endpoint
// Burst parameters always None
// verify_applied_profiles() is a no-op
```

**Required Fixes:**
1. Fix MikroTik REST endpoints:
   - Queue: POST `/rest/ip/queue/simple` (not `/run`)
   - PPPoE: POST `/rest/ppp/secret`
   - DHCP: POST `/rest/ip/dhcp-server/lease`
2. Implement Queue Tree / HTB for hierarchical QoS
3. Add SNMP verification after bandwidth push
4. Implement bandwidth rollback on deprovision

---

## High Gaps

### ISP-NET-H01: No CDR Ingestion Pipeline
- No CSV/binary CDR parsing from Huawei MA5800/MAXTEN exports
- `bandwidth_usage` table never populated
- Required: `CdrIngestionWorker` with CDR file parser

### ISP-NET-H02: No Fiber Plant Topology
- No OLT→Splitter→ONT hierarchy entities
- Required: `fiber_segment`, `olt_port`, `customer_equipment` entities

### ISP-NET-H03: No Mass Incident Management
- Single OLT failure creates 500 individual tickets
- Required: `mass_incident` entity with area-wide impact detection

### ISP-NET-H04: Fragile Huawei CLI Parsing
- `get_pon_status()` returns hardcoded values
- Required: Robust parsing with firmware version handling

---

## Medium Gaps

### ISP-NET-M01: No ZTE/Cisco/Nokia Adapters
### ISP-NET-M02: No TR-069/CWMP Support
### ISP-NET-M03: No RouterOS v6 Support

---

## New Dependencies Required

```toml
# Cargo.toml additions
snmp = "0.9"           # SNMP polling
radius = "0.5"         # RADIUS protocol (or implement raw)
ipnetwork = "0.20"     # CIDR parsing
```

---

## New Route Groups Required

```
/api/v1/network/olt/
  GET    /:id/onts              — List ONTs on OLT
  POST   /:id/onts/:sn/provision — Provision ONT
  GET    /:id/onts/:sn/optical  — Get ONT optical power
  POST   /:id/onts/:sn/config   — Configure ONT VLAN/QoS

/api/v1/network/snmp/
  GET    /walk                  — SNMP walk on device
  GET    /get                   — SNMP get single OID
  POST   /poll                  — Trigger immediate poll

/api/v1/network/ipam/
  GET    /pools/:id/addresses   — List IP addresses in pool
  POST   /pools/:id/allocate    — Allocate specific IP
  POST   /pools/:id/release     — Release specific IP
  POST   /pools/:id/reserve     — Reserve static IP
```
