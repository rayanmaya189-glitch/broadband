# AeroXe Broadband — Architecture Patterns & Network Operations Gap Analysis v3.0

**Date:** 2026-07-21
**Author:** Backend Architecture Team
**Scope:** Architecture pattern gaps, resilience patterns, missing workers, and network operations gaps
**Previous:** v1.0 (84 gaps), v2.0 (68 gaps) — this is v3.0 (63 gaps: 18 patterns + 8 workers + 10 network ops domains + 15 SRS design gaps + 12 missing entities)

---

## Executive Summary

The backend has excellent CRUD patterns but **lacks production-grade resilience, observability, and network operations capabilities**. No circuit breaker pattern exists for external calls, no distributed tracing, no health check endpoints, and 8 critical background workers are missing entirely. The network operations layer has zero real SNMP polling, no fiber plant topology, and no provisioning automation.

**Combined gap count:** 215 total (84 + 68 + 63)

---

## PART 1: ARCHITECTURE PATTERN GAPS (P-01 to P-18)

### P-01: No Circuit Breaker Pattern

- **Impact:** Cascade failure when MikroTik/Huawei/RADIUS goes down
- **What's missing:**
  - No `circuit-breaker` crate or custom implementation
  - No failure threshold tracking per external service
  - No half-open state for recovery testing
  - All external calls fail synchronously if device unreachable
- **Current behavior:** MikroTik adapter creates new HTTP connection per call, blocks on timeout
- **Fix:**
```rust
// infrastructure/resilience/circuit_breaker.rs
pub struct CircuitBreaker {
    failure_count: AtomicU32,
    state: Arc<RwLock<CircuitState>>,
    failure_threshold: u32,
    recovery_timeout: Duration,
}
impl CircuitBreaker {
    pub async fn call<F, Fut, T>(&self, f: F) -> Result<T, CircuitBreakerError>
    where F: FnOnce() -> Fut, Fut: Future<Output = Result<T, anyhow::Error>> {
        if self.state().await == CircuitState::Open {
            return Err(CircuitBreakerError::Open);
        }
        match f().await {
            Ok(val) => { self.on_success().await; Ok(val) }
            Err(e) => { self.on_failure().await; Err(CircuitBreakerError::Inner(e)) }
        }
    }
}
```
- **Apply to:** `MikroTikAdapter`, `HuaweiAdapter`, `RadiusClient`, `PaymentGateway`

---

### P-02: No Bulkhead Isolation

- **Impact:** Billing day worker blocks all database connections, starving provisioning
- **What's missing:**
  - All workers share one DB connection pool
  - No per-worker pool limits
  - No resource isolation between critical and background tasks
- **Fix:** Use separate connection pools per worker category:
  - Critical: provisioning, RADIUS, payment processing (10 connections)
  - Background: billing, reports, device sync (5 connections)
  - Analytics: dashboards, exports (3 connections)

---

### P-03: No Saga Compensation for Provisioning

- **Impact:** Partial provisioning leaves zombie states (PPPoE created but bandwidth not applied)
- **What's missing:**
  - ProvisioningWorker has no rollback logic
  - If step 3 of 5 fails, steps 1-2 are not undone
  - No saga state tracking
  - No compensation actions defined
- **Fix:**
```rust
// workers/provisioning_worker.rs
enum ProvisioningStep {
    CreateRADIUSAccount,
    PushBandwidthProfile,
    ConfigureVLAN,
    VerifyConnectivity,
    ActivateService,
}

fn compensation(step: ProvisioningStep) -> Option<Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()>>>>> {
    match step {
        ProvisioningStep::CreateRADIUSAccount => Some(Box::new(|| Box::pin(delete_radius_account()))),
        ProvisioningStep::PushBandwidthProfile => Some(Box::new(|| Box::pin(remove_bandwidth_queue()))),
        ProvisioningStep::ConfigureVLAN => Some(Box::new(|| Box::pin(remove_vlan_config()))),
        _ => None,
    }
}
```

---

### P-04: No Backpressure for Bursty Network Events

- **Impact:** Device offline events flood system during network outage, causing memory pressure
- **What's missing:**
  - No channel capacity limits on NATS consumers
  - No event deduplication for rapid status changes
  - No batched processing for high-volume events
  - A single OLT failure generates 500+ events in seconds
- **Fix:** Use bounded channels with `tokio::sync::mpsc::channel(1000)`, batch events by 100ms window, dedup by device_id.

---

### P-05: No Standardized Retry Policy

- **Impact:** Some retries use fixed delay, some use exponential, some don't retry at all
- **What's missing:**
  - No system-wide retry configuration
  - No jitter to prevent thundering herd
  - No per-service retry policy
  - No max retry budget
- **Fix:** Define `RetryPolicy` in shared infrastructure:
```rust
pub struct RetryPolicy {
    max_retries: u32,
    base_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
    jitter: bool,
}
```

---

### P-06: No Graceful Degradation

- **Impact:** If Huawei OLT goes down, entire provisioning pipeline fails
- **What's missing:**
  - No fallback strategy when external service unavailable
  - No partial provisioning (skip OLT config, apply later)
  - No degraded mode for dashboard
  - No "maintenance page" mode
- **Fix:** Define degradation levels: full → partial → minimal → maintenance

---

### P-07: No API/Webhook Retry + DLQ

- **Impact:** Failed payment gateway webhooks are silently lost
- **What's missing:**
  - No retry queue for failed HTTP calls (webhooks, notifications)
  - No dead-letter queue for API failures
  - No webhook delivery tracking
  - Outbox pattern exists for domain events but not for API operations
- **Fix:** Extend outbox pattern to cover external HTTP calls. Add `api_call_outbox` table.

---

### P-08: No Time-Series Strategy for Metrics

- **Impact:** `device_metrics` table grows unbounded, queries degrade after 30 days
- **What's missing:**
  - No time-series specific table design
  - No rollup/aggregation strategy (5min → 1hr → 1day → 1week)
  - No downsampled data for dashboards
  - No retention policy for metrics
- **Fix:** Create rollup tables: `device_metrics_5min`, `device_metrics_hourly`, `device_metrics_daily`. Worker to aggregate and delete raw data after 7 days.

---

### P-09: No Hot/Cold Data Separation

- **Impact:** Active customers and terminated customers share same tables, degrading query performance
- **What's missing:**
  - No table partitioning for hot (active) vs cold (archived) data
  - No automatic archival process
  - Terminated customer queries scan same index as active customers
- **Fix:** Use PostgreSQL native partitioning: `customers` (active), `customers_archive` (terminated). Partition by `status`.

---

### P-10: No CDR Storage Schema

- **Impact:** Cannot resolve usage disputes — no session-level data
- **What's missing:**
  - No `cdr_records` table
  - No session-level accounting data storage
  - RADIUS accounting data not persisted
  - Cannot answer: "How many GB did customer X use on July 15?"
- **Fix:**
```sql
CREATE TABLE cdr_records (
    id BIGSERIAL PRIMARY KEY,
    session_id VARCHAR(64) NOT NULL,
    customer_id BIGINT NOT NULL,
    nas_ip_address INET NOT NULL,
    calling_station_id VARCHAR(17),
    framed_ip_address INET,
    acct_status_time TIMESTAMPTZ NOT NULL,
    acct_stop_time TIMESTAMPTZ,
    acct_input_octets BIGINT DEFAULT 0,
    acct_output_octets BIGINT DEFAULT 0,
    acct_session_time INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (acct_status_time);
```

---

### P-11: No Data Archival / Auto-Purge

- **Impact:** Database grows unbounded, exceeds storage within 12-18 months
- **What's missing:**
  - `AutoPurgeWorker` doesn't exist
  - No retention policy enforcement
  - No cold storage migration to S3/MinIO
  - Device metrics, CDRs, audit logs accumulate forever
- **Fix:** Create `retention_policies` table and `RetentionWorker` that runs nightly.

---

### P-12: No Materialized Views for Dashboards

- **Impact:** Dashboard queries scan full tables, 30-second load times at 10K+ customers
- **What's missing:**
  - No pre-computed aggregations
  - No `REFRESH MATERIALIZED VIEW CONCURRENTLY` strategy
  - Dashboard metrics recalculated on every request
- **Fix:** Create materialized views: `mv_daily_revenue`, `mv_customer_count_by_status`, `mv_device_health_summary`. Refresh every 5 minutes via worker.

---

### P-13: No IPAM Data Model

- **Impact:** IP over-allocation, no recycling, no conflict detection
- **What's missing:**
  - No CIDR math library (`ipnetwork` crate not in Cargo.toml)
  - No per-address allocation tracking
  - No IP conflict prevention
  - No public vs private IP classification
  - No IP recycling on subscription cancellation
- **See:** `GAP-code-bugs.md` CODE-NET-01

---

### P-14: Health Check Endpoints Don't Check Dependencies

- **Impact:** `/health` returns 200 even when PostgreSQL, Redis, NATS, or RADIUS are down
- **What's missing:**
  - No DB connectivity check
  - No Redis ping
  - No NATS connection check
  - No RADIUS reachability check
  - Load balancer routes traffic to unhealthy instances
- **Fix:**
```rust
// routes/health.rs
async fn deep_health(State(state): State<AppState>) -> proto_response<HealthResponse> {
    let db = state.db.ping().await.is_ok();
    let redis = state.redis.ping().await.is_ok();
    let nats = state.nats.is_connected();
    let radius = state.radius.ping().await.is_ok();
    let healthy = db && redis && nats && radius;
    proto_response(HealthResponse {
        status: if healthy { "healthy" } else { "degraded" },
        components: Some(HealthComponents { db, redis, nats, radius }),
    })
}
```

---

### P-15: No Worker Job Dead-Letter Queue

- **Impact:** A single bad record blocks the entire worker queue forever
- **What's missing:**
  - `BillingWorker` retries failed invoices infinitely
  - No max retry count per job
  - No DLQ for permanently failed jobs
  - No manual retry/discard UI
- **Fix:** Add `max_retries` and `dlq` to job processing. Outbox DLQ pattern exists but not applied to worker jobs.

---

### P-16: No External Service Health Monitoring

- **Impact:** MikroTik/Huawei/RADIUS status is always "unknown" or faked
- **What's missing:**
  - No periodic ping/connectivity check for managed devices
  - No reachability tracking per device
  - Random health scores when adapter unavailable (`rng.gen_range(70..100)`)
- **See:** `GAP-code-bugs.md` CODE-MON-04

---

### P-17: No Distributed Tracing

- **Impact:** Debugging a provisioning failure requires reading 8 log files across 5 modules
- **What's missing:**
  - No OpenTelemetry integration
  - No Jaeger/Tempo
  - No correlation IDs across async event chains
  - No trace context propagation through NATS
- **Fix:** Add `tracing-opentelemetry` crate. Propagate `trace_id` through NATS message headers.

---

### P-18: No SLO/SLI Definitions

- **Impact:** Cannot measure ISP SLA delivery to enterprise customers
- **What's missing:**
  - No formal availability targets (99.9% = 8.76 hours/year downtime)
  - No latency SLIs (P99 < 200ms)
  - No error budget tracking
  - No SLA breach alerting
- **Fix:** Define SLIs in `sla_definitions` table. Monitor with `SlaMonitorWorker`.

---

## PART 2: MISSING WORKERS (W-01 to W-08)

| # | Worker | Priority | Purpose | Implementation Effort |
|---|--------|----------|---------|----------------------|
| W-01 | **CdrProcessingWorker** | CRITICAL | Parse BNG CDR files → `cdr_records` → usage data → FUP enforcement | 1 week |
| W-02 | **RadiusAccountingWorker** | CRITICAL | Listen for RADIUS Accounting packets → session tracking → billing | 1 week |
| W-03 | **UsageMeteringWorker** | CRITICAL | Track per-customer data usage, enforce FUP speed caps | 3 days |
| W-04 | **SlaMonitorWorker** | HIGH | SLA timer management, auto-escalation, breach alerts | 3 days |
| W-05 | **CapacityAlertingWorker** | HIGH | SNMP polling, threshold alerts, mass incident detection | 3 days |
| W-06 | **ReportGenerationWorker** | MEDIUM | Daily revenue, subscriber metrics, GST data | 3 days |
| W-07 | **CertificateRenewalWorker** | MEDIUM | TLS/JWT/RADIUS secret rotation | 2 days |
| W-08 | **RetentionWorker** | MEDIUM | Redis key expiry, outbox cleanup, old event purge | 2 days |

**Total effort:** ~4.5 weeks

---

## PART 3: NETWORK OPERATIONS GAPS (N-01 to N-10)

### N-01: No Fiber Plant Topology (CRITICAL)

- **What's missing:**
  - No OLT→Splitter→ONT hierarchy entities
  - No `fiber_segment` table (cable route, length, fiber count)
  - No `splice_point` table (fusion splices, location)
  - No `splitter` table (ratio, location, port count)
  - No `olt_port` → `splitter` → `ont` mapping
  - No OTDR test result storage
  - No fiber cut impact analysis (which 500 customers affected?)
- **Entities needed:**
```sql
CREATE TABLE fiber_segments (
    id BIGSERIAL PRIMARY KEY,
    segment_name VARCHAR(100) NOT NULL,
    cable_type VARCHAR(20) NOT NULL, -- ADSS, OPGW, direct-buried
    fiber_count INTEGER NOT NULL,
    length_meters DECIMAL(10,2),
    start_location GEOGRAPHY(POINT, 4326),
    end_location GEOGRAPHY(POINT, 4326),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE splitters (
    id BIGSERIAL PRIMARY KEY,
    splitter_type VARCHAR(10) NOT NULL, -- 1:8, 1:16, 1:32, 1:64
    location GEOGRAPHY(POINT, 4326),
    olt_id BIGINT REFERENCES network_devices(id),
    olt_port INTEGER,
    input_fiber_id BIGINT REFERENCES fiber_segments(id),
    output_fiber_ids BIGINT[],
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ont_mappings (
    id BIGSERIAL PRIMARY KEY,
    ont_serial VARCHAR(50) NOT NULL,
    customer_id BIGINT,
    splitter_id BIGINT REFERENCES splitters(id),
    splitter_port INTEGER,
    olt_id BIGINT REFERENCES network_devices(id),
    olt_port INTEGER,
    pon_slot INTEGER,
    pon_port INTEGER,
    ont_distance_meters DECIMAL(10,2),
    rx_power_dbm DECIMAL(6,2),
    tx_power_dbm DECIMAL(6,2),
    status VARCHAR(20) DEFAULT 'active',
    installed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

### N-02: No IPAM (IP Address Management) (CRITICAL)

- **What's missing:**
  - No public IPv4 address block tracking
  - No CGNAT pool modeling (100.64.0.0/10)
  - No IPv6 prefix delegation
  - No IP recycling on customer disconnect
  - No DHCP scope management
  - No static IP reservation workflow
  - No IP utilization reporting
- **Impact:** IP exhaustion undetected. Enterprise static IP requests unfulfillable.

---

### N-03: Zero Real Network Monitoring (CRITICAL)

- **What's missing:**
  - No `snmp` or `agent` crate in `Cargo.toml`
  - `device_metrics` table never populated from real devices
  - Health scores computed from nothing
  - `monitoring_worker` never spawned in `main.rs`
  - No SNMP GET/WALK/BULK operations
  - No ONT optical power monitoring (Rx/Tx dBm)
  - No interface-level monitoring (errors, discards, speed/duplex)
  - No threshold-based alerting
- **See:** `GAP-code-bugs.md` CODE-MON-01 through CODE-MON-05

---

### N-04: No Bandwidth Enforcement (HIGH)

- **What's missing:**
  - No HTB (Hierarchical Token Bucket) implementation
  - No CIR/PIR per subscriber
  - No PCQ (Per Connection Queue) for fair sharing
  - No time-based bandwidth profiles (night boost)
  - No post-FUP speed reduction
  - Simple Queues only, and they're never actually pushed to devices
- **See:** `GAP-code-bugs.md` CODE-BW-01 through CODE-BW-05

---

### N-05: No Provisioning Automation (CRITICAL)

- **What's missing:**
  - No RADIUS CoA (Change of Authorization) for session changes
  - No OMCI provisioning for ONT configuration
  - No TR-069 ACS for remote ONT management
  - Creating subscription does NOT create PPPoE account
  - Creating subscription does NOT push bandwidth to BNG
  - Creating subscription does NOT configure ONT VLAN
  - Every installation requires 30-60 minutes of manual NOC time
- **See:** `DESIGN-GAPS-DEEP-ANALYSIS.md` ISP-NET-C03

---

### N-06: No Incident Management (HIGH)

- **What's missing:**
  - No shift handover workflow
  - No escalation matrix (L1→L2→L3 with time thresholds)
  - No MTTR/MTBF tracking
  - No mass incident correlation (single OLT failure → 500 tickets → no grouping)
  - No post-mortem template
  - No change management workflow
- **Impact:** During outage, support flooded. No root cause tracking.

---

### N-07: No Field Operations (HIGH)

- **What's missing:**
  - No mobile app for technicians
  - No GPS check-in/check-out
  - No installation checklist (photo, speed test, customer sign-off)
  - No OTDR test gate before marking installation complete
  - No barcode/QR scanning for equipment
  - No offline sync for low-connectivity areas
  - No route optimization
  - No real-time technician tracking
- **Impact:** No installation quality control. No field visibility.

---

### N-08: No Capacity Planning (HIGH)

- **What's missing:**
  - No PON port utilization tracking
  - No backbone link utilization trending
  - No growth forecasting
  - No "capacity exhausted" alerting
  - No fiber route capacity vs demand analysis
- **Impact:** Unexpected capacity exhaustion. Emergency procurement at premium prices.

---

### N-09: No Vendor/AMC Management (MEDIUM-HIGH)

- **What's missing:**
  - No vendor contracts table
  - No AMC (Annual Maintenance Contract) tracking
  - No upstream SLA tracking
  - No spare parts inventory
  - No vendor performance scoring
  - No equipment warranty tracking
- **Impact:** Expired AMCs go unnoticed. Warranty claims missed.

---

### N-10: No Regulatory Compliance (CRITICAL)

- **What's missing:**
  - No TRAI QoS metric reporting
  - No lawful intercept interface (Section 5 TRAI Act)
  - No CDR retention (5 years per IT Act)
  - No TRAI shutdown compliance workflow
  - No subscriber verification (CAF per TRAI)
  - No MNP (Mobile Number Portability) support
- **Impact:** License non-compliance. Potential cancellation.

---

## PART 4: MISSING ENTITIES (E-01 to E-12)

| # | Entity | Purpose | Priority |
|---|--------|---------|----------|
| E-01 | `ip_address` | Individual IP allocation records | CRITICAL |
| E-02 | `radius_accounting` | RADIUS accounting packet logs | CRITICAL |
| E-03 | `provisioning_job` | Customer provisioning task tracking | CRITICAL |
| E-04 | `cdr_records` | Call detail records for usage tracking | CRITICAL |
| E-05 | `fiber_segment` | Physical fiber route segments | HIGH |
| E-06 | `olt_port` | OLT PON port → ONT mapping | HIGH |
| E-07 | `splitters` | Optical splitter locations and mappings | HIGH |
| E-08 | `customer_equipment` | Customer-premises equipment (ONT, router) | HIGH |
| E-09 | `mass_incident` | Area-wide outage tracking | HIGH |
| E-10 | `sla_definition` | SLA targets per plan/tier | HIGH |
| E-11 | `sla_measurement` | Actual SLA performance per customer | HIGH |
| E-12 | `usage_record` | Per-customer daily usage aggregation | HIGH |

---

## PART 5: SRS DESIGN GAPS (D-01 to D-15)

| # | Gap | Impact |
|---|-----|--------|
| D-01 | **No `customer_type` field** (residential vs enterprise) | Cannot differentiate SLA, billing, support |
| D-02 | **No `relocation` installation type** | Ghost equipment on customer moves |
| D-03 | **No `disconnection_order` entity** | No structured termination workflow |
| D-04 | **No `customer_notes` / communication log** | Poor support experience |
| D-05 | **No fiber route / physical layer entities** | Cannot diagnose fiber cuts |
| D-06 | **No `sla_agreement` for enterprise** | No per-customer SLA tracking |
| D-07 | **No `static_ip_assignment`** | No static IP lifecycle or billing |
| D-08 | **No `invoice_pdf_url`** | No PDF generation pipeline |
| D-09 | **No `service_outage` / `maintenance_window`** | Planned maintenance not modeled |
| D-10 | **No `ticket_sla_timer`** | SLA timer pauses on wrong statuses |
| D-11 | **No plan versioning** | Price changes affect historical records |
| D-12 | **No `amount_paid` on invoices** | Dunning can't calculate balance |
| D-13 | **No `customer_contacts` for enterprise** | Single contact for all purposes |
| D-14 | **No `referral_wallet` linkage** | Referral rewards can't auto-credit |
| D-15 | **No `ont_provisioning_profile`** | No OMCI-based ONT configuration |

---

## Implementation Priority

| Priority | Gaps | Est. Effort |
|----------|------|-------------|
| **P0 (Immediate)** | P-01, P-14, W-01, W-02, W-03, N-03, N-05 | 4 weeks |
| **P1 (Pre-launch)** | P-02, P-03, P-05, P-10, P-11, P-13, W-04, W-05, N-01, N-02, N-04, E-01 to E-12 | 4 weeks |
| **P2 (Post-launch)** | P-04, P-06, P-07, P-08, P-09, P-12, P-17, W-06, W-07, W-08, N-06, N-07, N-08, N-10, D-01 to D-15 | 4 weeks |
| **P3 (Enhancement)** | P-15, P-16, P-18, N-09 | 2 weeks |

**Total estimated effort:** 14 weeks

---

*Document version: 3.0 — 2026-07-21*
*Combined total: 215 gaps (84 v1.0 + 68 v2.0 + 63 v3.0)*
*See also: DESIGN-GAPS-DEEP-ANALYSIS.md, GAP-IMPLEMENTATION-ROADMAP.md, GAP-finance-compliance.md*
