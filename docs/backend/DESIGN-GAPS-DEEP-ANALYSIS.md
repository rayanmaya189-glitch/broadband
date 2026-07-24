# AeroXe Broadband — Deep ISP Operational Design Gap Analysis v2.0

**Date:** 2026-07-21
**Author:** Backend Architecture Team
**Scope:** Complete backend codebase + all 35 module docs + 6303-line requirement spec + 13 source code files
**Methodology:** 4 parallel deep-dive agents analyzing docs, code, and requirements from real-world FTTH ISP perspective

---

## Executive Summary

The AeroXe backend has **excellent CRUD coverage** (~229 endpoints) and **strong architectural patterns** (DDD, outbox, events, partitioning). However, the **ISP operational core** — the layer that talks to real network devices, collects real usage data, and enforces real speed limits — is almost entirely stubbed. Additionally, **critical security vulnerabilities** and **data integrity bugs** were discovered in the code.

**Bottom line:** The system works as an admin panel but not as a live ISP operations platform. It also has security holes that must be fixed before any deployment.

| Category | Total Gaps | Critical | High | Medium | Low |
|----------|-----------|----------|------|--------|-----|
| Security & Compliance | 13 | 7 | 2 | 4 | 0 |
| ISP Network Operations | 13 | 4 | 5 | 4 | 0 |
| Billing & Revenue | 11 | 4 | 5 | 2 | 0 |
| Customer Operations | 10 | 3 | 4 | 3 | 0 |
| Infrastructure & DevOps | 9 | 3 | 2 | 4 | 0 |
| Regulatory (TRAI/GST/IT Act) | 12 | 2 | 3 | 5 | 2 |
| **v1.0 + v2.0 Subtotal** | **68** | **23** | **21** | **22** | **2** |
| Indian Finance & Tax Compliance (v3.0) | 25 | 6 | 10 | 7 | 2 |
| Architecture Patterns & Resilience (v3.0) | 18 | 4 | 8 | 5 | 1 |
| Network Operations & Field Ops (v3.0) | 10 | 4 | 4 | 2 | 0 |
| SRS Design Gaps (v3.0) | 15 | 2 | 5 | 6 | 2 |
| Missing Workers (v3.0) | 8 | 3 | 3 | 2 | 0 |
| **v3.0 Subtotal** | **76** | **19** | **30** | **22** | **5** |
| **GRAND TOTAL (v1.0 + v2.0 + v3.0)** | **144** | **42** | **51** | **44** | **7** |

**Previous analysis (v1.0):** 84 gaps (47 API/design + 37 ISP operational)
**This analysis (v2.0):** 68 new gaps from code-level deep dive
**This analysis (v3.0):** 76 new gaps from SRS deep dive, finance compliance, architecture patterns, network ops
**Combined total:** 215 unique gaps identified (71 additional addressed incrementally within phases)

---

## TIER 0: SECURITY / DATA INTEGRITY (Immediate Fix — Before Any Deployment)

### SEC-001: Aadhaar Hash Uses Static Salt
- **File:** `28-security.md:111`, `shared/utils/pii.rs`
- **Code:** `format!("aeroxe:{}", aadhaar)` — same Aadhaar always produces same hash
- **Impact:** Rainbow table attack trivial. If DB dumped, all Aadhaar numbers recoverable.
- **Fix:** Use per-record random salt stored alongside hash. `hash = SHA256(random_salt + aadhaar)`. Store `salt:hash` together.

### SEC-002: MikroTik `execute_command` Allows Arbitrary RouterOS
- **File:** `integrations/mikrotik/adapter.rs:501-508`
- **Code:** `rest_post("/run", body)` — any command string accepted
- **Impact:** `/system shutdown`, `/interface delete`, `/user set` — full device takeover.
- **Fix:** Implement command whitelist: `["/queue/simple/*", "/ppp/secret/*", "/ip/dhcp-server/lease/*"]`. Reject all others.

### SEC-003: MikroTik `danger_accept_invalid_certs(true)`
- **File:** `integrations/mikrotik/adapter.rs:167`
- **Impact:** MITM attacks trivial. Admin credentials exposed on network.
- **Fix:** Use proper CA certificate validation. Add MikroTik CA cert to trusted store.

### SEC-004: WebSocket Exposed Without Authentication
- **File:** `routes/mod.rs:12`
- **Code:** `.route("/ws", get(ws_handler))` under `health_routes()` — no auth middleware. **Note:** WebSocket upgrade requires HTTP GET per RFC 6455; this is a protocol requirement, not a REST design choice.
- **Impact:** Anonymous users access real-time ISP data (device status, customer sessions).
- **Fix:** Move `/ws` to authenticated route group. Require JWT validation on WebSocket upgrade.

### SEC-005: Swagger UI Publicly Accessible in Production
- **File:** `routes/mod.rs:13-16`
- **Code:** `SwaggerUi::new("/swagger-ui")` — no environment check
- **Impact:** Attackers get complete API documentation and endpoint map.
- **Fix:** Gate Swagger behind `#[cfg(debug_assertions)]` or environment check.

### SEC-006: No Distributed Rate Limiting
- **File:** `28-security.md:51-57`
- **Impact:** Per-server limits bypassed with load balancer. 3 instances = 3× rate.
- **Fix:** Use Redis-based sliding window rate limiting (already have Redis in stack).

### SEC-007: RADIUS Password Encoding Broken for >16 Bytes
- **File:** `integrations/radius/adapter.rs:231-252`
- **Code:** `let h = if i < 16 { hash[i] } else { hash[i % 16] };`
- **Impact:** Per RFC 2865, password chaining should XOR with MD5(previous_ciphertext + secret). Passwords >16 bytes won't decrypt on RADIUS server.
- **Fix:** Implement proper RFC 2865 password chaining algorithm.

### SEC-008: RADIUS Response Authenticator Not Validated
- **File:** `integrations/radius/adapter.rs:355-424`
- **Impact:** Any UDP response matching identifier is accepted. Vulnerable to spoofing.
- **Fix:** Validate response authenticator = MD5(packet + response_auth + secret).

### SEC-009: No JWT Refresh Token Rotation
- **File:** `28-security.md:15-22`
- **Impact:** Stolen refresh token = 7 days unlimited access.
- **Fix:** Single-use refresh tokens with rotation. Invalidate old token on use.

### SEC-010: Account Lockout is DoS Vector
- **File:** `28-security.md:15-22`
- **Impact:** Attacker locks out any user with 5 failed attempts.
- **Fix:** Add CAPTCHA after 3 attempts. Progressive delays instead of hard lockout.

### SEC-011: No DPDP Act 2023 Compliance
- **File:** `28-security.md:154`
- **Impact:** India's data protection law requires consent management, data principal rights, breach notification.
- **Fix:** Add consent table, data access/erasure APIs, 72-hour breach notification workflow.

### SEC-012: No IT Act Section 43A Compliance
- **File:** `28-security.md:153`
- **Impact:** CERT-In directive requires 6-hour incident reporting.
- **Fix:** Add incident response workflow with CERT-In reporting.

### SEC-013: No Aadhaar Act UIDAI Authorization
- **File:** `28-security.md:155`
- **Impact:** Storing Aadhaar data (even hashed) requires UIDAI authorization.
- **Fix:** Obtain UIDAI requesting entity authorization or remove Aadhaar storage.

---

## TIER 1: CRITICAL — ISP Cannot Function Without These

### ISP-NET-C01: No RADIUS Accounting Listener
- **Files:** `integrations/radius/adapter.rs`, `network/domain/entities/pppoe_session.rs`
- **What Exists:** `RadiusClient` trait can send Access-Request, Accounting-Start, Accounting-Stop, Accounting-Interim to a RADIUS server
- **What's Missing:**
  - No listener for inbound RADIUS Accounting packets (Acct-Start/Stop/Interim-Update)
  - `pppoe_sessions.bytes_in` and `pppoe_sessions.bytes_out` **never updated from real RADIUS data**
  - `Calling-Station-Id` (customer MAC) not sent in Access-Request
  - `Called-Station-Id` (BNG identifier) not sent
  - `NAS-Identifier` not sent
  - `Message-Authenticator` (RFC 3579) defined as enum variant but never set
  - No `State` attribute tracking for multi-round authentication
  - No Response Authenticator verification (spoofed responses accepted)
  - No RADIUS proxy/failover for primary + secondary RADIUS servers
  - No `radsec` (TLS-encrypted RADIUS) support
  - Retry logic configured but never used (`send_and_receive` sends once)
- **Business Impact:** Cannot track who is online, session duration, or data consumed. Usage-based billing is impossible. Session accounting is fake.
- **Required Fix:** Implement RADIUS Accounting-Server listener (UDP socket), correlate accounting packets to `pppoe_sessions`, update `bytes_in`/`bytes_out` in real-time. Add RADIUS proxy with failover.

### ISP-NET-C02: IP Allocation is Fake
- **Files:** `network/application/service.rs`, `network/domain/entities/ip_pool.rs`
- **What Exists:** `ip_pool` tracks `allocated_count` and `total_count`
- **What's Missing:**
  - `allocate_ip()` only increments `allocated_count` — **no actual IP address is allocated from the CIDR range**
  - No CIDR parsing, no subnet traversal, no available IP detection
  - No IP conflict prevention — two customers can get the same IP
  - `release_ip()` only decrements counter
  - No `ip_address` entity with individual allocation records
  - No DHCP scope management
  - No static IP reservation tracking
- **Business Impact:** IP conflicts cause customer outages. DHCP exhaustion undetected. Cannot assign static IPs to enterprise customers.
- **Required Fix:** Implement proper IPAM (IP Address Management): CIDR parser, IP range generator, allocation tracking per address, conflict detection, DHCP scope integration.

### ISP-NET-C03: No Device Provisioning Automation
- **Files:** `network/application/service.rs`, `bandwidth/application/service.rs`, `installations/application/service.rs`
- **What Exists:** adapters for MikroTik (REST) and Huawei (SSH CLI), subscription creation endpoint
- **What's Missing:**
  - Creating a subscription **does not** create PPPoE account on RADIUS server
  - Creating a subscription **does not** push bandwidth queue to MikroTik router
  - Creating a subscription **does not** configure ONT VLAN/QoS on Huawei OLT
  - No `ProvisioningWorker` to automate the sequence: create PPPoE → apply bandwidth → configure ONT → verify → activate
  - Each step requires manual NOC engineer intervention
  - No rollback if any step fails
- **Business Impact:** Every new customer installation takes 30-60 minutes of NOC engineer time instead of 2 minutes automated. Cannot scale beyond 100 customers/month.
- **Required Fix:** Implement `ProvisioningWorker`: on `subscription.activated` event, automatically: (1) create PPPoE credentials on RADIUS, (2) push bandwidth profile to BNG/MikroTik, (3) configure ONT on OLT, (4) verify connectivity, (5) publish `customer.provisioned` event.

### ISP-NET-C04: No SNMP Polling
- **Files:** `workers/monitoring_worker.rs`, `Cargo.toml`
- **What Exists:** `monitoring_worker` polls devices via adapter, creates health alerts
- **What's Missing:**
  - No `snmp` or `agent` crate in `Cargo.toml`
  - `device_metrics` table **never populated from real devices**
  - Health scores computed from nothing (adapter returns mock data)
  - Cannot poll: CPU, memory, uptime, interface counters, optical power (ONU), temperature
  - Cannot detect: device offline, link down, high utilization, optical power degradation
  - No interface-level monitoring (errors, discards, speed/duplex)
- **Business Impact:** NOC dashboard shows fake data. Device failures go undetected until customers complain. Cannot monitor ONT optical power (critical for FTTH). SLA monitoring is blind.
- **Required Fix:** Add `snmp` crate, implement SNMPv2c/v3 GET/WALK for MikroTik, Huawei, ZTE devices. Poll: sysUpTime, CPU, memory, interface counters, ONT optical power. Populate `device_metrics` in real-time.

### ISP-NET-C05: Bandwidth Limits are DB-Only
- **Files:** `workers/bandwidth_worker.rs`, `integrations/mikrotik/adapter.rs`
- **What Exists:** `BandwidthWorker` fetches pending bandwidth applications, calls adapter
- **What's Missing:**
  - MikroTik `execute_command()` posts to `/run` — **invalid RouterOS v7 endpoint** (doesn't exist)
  - Burst parameters always `None` in queue listing
  - No Queue Tree / HTB implementation (only Simple Queues)
  - `verify_applied_profiles()` is a no-op — doesn't query device to verify queue exists
  - No bandwidth profile rollback on subscription cancellation
  - No per-device-type application (Huawei needs DBA profile + traffic table, MikroTik needs simple queue)
  - No FQ-CoDel implementation despite being specified
- **Business Impact:** Speed limits are purely declarative in the database. A "100 Mbps" plan customer actually gets unlimited speed. Revenue model collapses — all customers get the same (maximum) speed regardless of plan.
- **Required Fix:** Fix MikroTik REST API calls (use correct `/rest/ip/queue/simple` endpoint). Implement HTB Queue Tree. Add SNMP verification after push. Implement bandwidth rollback on deprovision.

---

### ISP-BILL-C01: No Tax Calculation
- **Files:** `billing/application/service.rs`
- **What Exists:** GST fields defined (CGST 9%, SGST 9%, HSN/SAC 998421), `tax_config` hardcoded
- **What's Missing:**
  - `create_invoice()` **never computes tax** — tax columns always 0
  - `auto_generate_invoices()` **never adds GST** to invoice total
  - No IGST handling for inter-state customers
  - No HSN/SAC code assignment per line item
  - No GSTIN validation on customer
  - No GSTR-1/GSTR-3B auto-generation from invoice data
- **Business Impact:** Invoices are non-compliant with Indian GST law. Cannot file GST returns. Risk of penalties from tax authorities.
- **Required Fix:** Implement GST calculation: `subtotal × CGST 9%` + `subtotal × SGST 9%` (intra-state) or `subtotal × IGST 18%` (inter-state). Add GSTIN validation. Auto-generate GSTR-1 data from paid invoices.

### ISP-BILL-C02: No Connection Pooling for Device Adapters
- **Files:** `integrations/mikrotik/adapter.rs`, `integrations/huawei/adapter.rs`
- **What Exists:** Each API call creates a new connection
- **What's Missing:**
  - MikroTik: new HTTP connection per REST call (no connection reuse)
  - Huawei: new SSH session per `execute_cli()` call (OLTs support 8-16 concurrent SSH sessions)
  - No connection pool for either adapter
  - No connection health check / keepalive
  - No connection timeout configuration
- **Business Impact:** At 50+ devices, connection refused errors. At 500+ devices, system unusable. OLT SSH session limit causes provisioning failures during peak installation hours.
- **Required Fix:** Implement connection pools: `mikrotik_pool: Arc<ConnectionPool>` and `huawei_ssh_pool: Arc<SshPool>` in `AppState`. Add health checks, max idle timeout, connection reuse.

---

### ISP-CUST-C01: No Customer Self-Service Portal
- **Files:** `customer/application/service.rs`, `routes/mod.rs`
- **What Exists:** `customer` module has admin-facing CRUD only
- **What's Missing:**
  - Zero `/api/v1/customer/me/*` routes registered
  - No customer auth separation (customers share staff auth)
  - No customer usage dashboard (view bandwidth usage, session history)
  - No bill view & pay (view invoices, payment history, online payment)
  - No self-service plan change (upgrade/downgrade request)
  - No service pause/resume by customer
  - No speed test integration
  - No referral code display
  - No KYC document upload by customer
  - No notification channel preferences
- **Business Impact:** Cannot launch Android/iOS customer app. All customer interactions require calling support. Customer acquisition cost remains high.
- **Required Fix:** Create `customer-portal` route group with: `POST /me/profile/get`, `POST /me/invoices/list`, `POST /me/pay`, `POST /me/usage/get`, `POST /me/tickets/create`, `POST /me/plan-change`, `POST /me/kyc/upload`, `POST /me/referral-code/get`.

---

### ISP-INFRA-C01: No Provisioning Worker
- **Files:** `workers/` directory
- **What Exists:** 8 workers (billing, notification, bandwidth, monitoring, device-sync, scheduler, outbox, partition)
- **What's Missing:**
  - No `ProvisioningWorker` — automated customer onboarding sequence
  - No `RadiusAccountingWorker` — correlating RADIUS sessions to customers
  - No `CdrIngestionWorker` — parsing CDR files from BNGs/OLTs
  - No `UsageMeteringWorker` — tracking per-customer data usage for FUP
  - No `NetworkDiscoveryWorker` — periodic SNMP scan for new devices
  - No `BackupWorker` — automated DB backup scheduling
  - No `CertificateRenewalWorker` — auto-renew TLS certs
  - No `ReportGenerationWorker` — nightly/weekly aggregate reports
  - No `WatchdogWorker` — process health watchdog (restart on panic)
- **Business Critical:** Without `ProvisioningWorker`, every customer requires manual NOC intervention. Without `RadiusAccountingWorker`, usage data is fake. Without `CdrIngestionWorker`, billing is inaccurate.
- **Required Fix:** Implement at minimum: `ProvisioningWorker`, `RadiusAccountingWorker`, `CdrIngestionWorker`, `UsageMeteringWorker`.

---

## TIER 2: HIGH — Won't Scale / Major Revenue Impact

### ISP-NET-H01: No CDR Ingestion Pipeline
- **What's Missing:** No CSV/binary CDR parsing from Huawei MA5800/MAXTEN CDR exports. No `CdrIngestionWorker`. `bandwidth_usage` table never populated with real data.
- **Impact:** Cannot implement usage-based billing, FUP enforcement, or usage analytics.

### ISP-NET-H02: No Fiber Plant Topology
- **What's Missing:** No OLT→Splitter→ONT hierarchy entities. No `fiber_segment`, `olt_port`, `customer_equipment` entities. No splitter ratio tracking (1:32/1:64). No fiber cut impact analysis.
- **Impact:** When a fiber cut occurs, technicians can't identify which 500 customers are affected.

### ISP-NET-H03: No Mass Incident / Outage Management
- **What's Missing:** Single OLT failure creates 500 individual tickets. No area-wide impact detection. No bulk customer notification ("We're aware of an outage in Jalgaon City Center"). No root cause correlation.
- **Impact:** During outage, support is flooded. No proactive customer communication.

### ISP-NET-H04: Fragile Huawei CLI Parsing
- **What Exists:** `parse_ont_status()` uses line-by-line string matching. `get_pon_status()` returns **hardcoded** values.
- **Impact:** Any firmware change breaks parsing. No real ONT optical power data.

### ISP-BILL-H01: No Pro-Rata Billing
- **What Exists:** `prorata_adjustments` table exists but is never written to
- **What's Missing:** Mid-cycle upgrade/downgrade doesn't generate partial invoices. No proper month-end calculation (uses `+ 30 days` instead of actual month end).
- **Impact:** Revenue loss or overcharging during plan changes.

### ISP-BILL-H02: No Invoice PDF Generation
- **What Exists:** `handlebars` crate included but no invoice template
- **Impact:** Cannot send professional invoices. Critical for billing operations.

### ISP-BILL-H03: No Late Fee Application
- **What Exists:** `late_fee_percent: "2.0"` defined but never applied
- **What's Missing:** No daily cron to calculate and apply late fees to overdue invoices.
- **Impact:** Revenue loss from uncollected late fees.

### ISP-CUST-H01: No WhatsApp Two-Way Bot
- **What Exists:** `whatsapp/mod.rs` is one-way notifications only
- **What's Missing:** No inbound message handling. Customers can't interact via WhatsApp.
- **Impact:** Missed India's #1 engagement channel. Customers expect to check balance, raise tickets via WhatsApp.

### ISP-CUST-H02: No SLA Enforcement
- **What Exists:** `escalate_ticket()` only changes status string. No SLA timers.
- **What's Missing:** No `sla_deadline`, no `first_response_at`, no auto-escalation matrix (L1→L2→L3), no breach detection, no SLA reporting.
- **Impact:** Enterprise customers paying premium have no guarantees. Critical tickets sit unresolved.

### ISP-CUST-H03: No Field Tech Mobile API
- **What's Missing:** No GPS check-in/check-out. No route optimization. No offline sync. No barcode scanning. Technicians use paper workflows.
- **Impact:** No real-time visibility into field operations. Installation tracking is manual.

### ISP-BILL-H04: Invoice Number Collision
- **What Exists:** Uses `timestamp_millis() % 10000` — two invoices in same millisecond get same number.
- **Impact:** Duplicate invoice numbers in accounting system.

### ISP-BILL-H05: No Partial Payment Support
- **What Exists:** `record_payment()` marks entire invoice paid even for partial amounts.
- **Impact:** Customers who pay partially are marked as fully paid. Revenue leakage.

### ISP-CUST-H04: No Customer Communication Preferences
- **What's Missing:** Notifications sent via all channels. No per-customer channel preferences, quiet hours, or opt-out management.
- **Impact:** Customer frustration from unwanted communications. Regulatory risk under TRAI regulations.

### ISP-CUST-H05: No Subscription Downgrade Protection
- **What's Missing:** No validation that customer's current bandwidth usage is below target plan limit before downgrade. Immediate speed reduction causes service disruption.
- **Impact:** Customer on 200Mbps plan downloading large files gets downgraded to 50Mbps mid-transfer.

---

## TIER 3: MEDIUM — Operational Pain / Compliance Risk

### ISP-NET-M01: No ZTE/Cisco/Nokia Adapters
- **Impact:** Only MikroTik + partial Huawei. ZTE is #2 GPON vendor in India. Cannot manage heterogeneous network.

### ISP-NET-M02: No TR-069/CWMP
- **Impact:** No remote ONT management protocol. Every ONT config change requires truck roll.

### ISP-NET-M03: No RouterOS v6 Support
- **Impact:** REST API is v7 only. Many ISPs still run v6. Legacy API (TCP/SSL text protocol) not supported.

### ISP-BILL-M01: No GST E-Invoice (IRN)
- **Impact:** No IRN generation via GSTN API. Compliance risk for large ISPs.

### ISP-BILL-M02: No Payment Reconciliation
- **Impact:** No bank statement import, no auto-matching of payments. Manual reconciliation is error-prone.

### ISP-BILL-M03: No TDS Deduction
- **Impact:** No TDS handling for enterprise customers. Compliance risk.

### ISP-CUST-M01: No Fraud Detection
- **Impact:** No concurrent login detection, no MAC spoofing check, no speed bypass detection. Estimated 5-15% revenue leakage.

### ISP-CUST-M02: No Data Retention Enforcement
- **What Exists:** `compliance` module has zero routes. Retention policies defined but no worker enforces them.
- **Impact:** DB grows unbounded. IT Act non-compliance risk.

### ISP-OPS-M01: No Reporting API
- **Impact:** No revenue reports, churn analysis, subscriber growth, bandwidth utilization analytics.

### ISP-OPS-M02: No Distributed Tracing
- **Impact:** Prometheus present but no OpenTelemetry/Jaeger. Debugging multi-module flows is blind.

### ISP-OPS-M03: No Customer Search Optimization
- **Impact:** Uses `LIKE '%query%'` — catastrophic at >100K customers. No full-text search.

### ISP-OPS-M04: No Backup Worker
- **Impact:** No automated DB backup scheduling, no config backup to S3.

---

## TIER 4: LOW — Nice-to-Have

### ISP-CUST-L01: No Multi-Language Notifications
- **Impact:** Regional language (Marathi) support missing for Jalgaon area.

### ISP-NET-L01: No GIS Integration
- **Impact:** No fiber route visualization on maps. No coverage area visualization.

---

## 3. ISP OPERATIONAL WORKFLOWS — What's Missing

### 3.1 Customer Onboarding Flow (End-to-End)

| Step | Current Status | Required |
|------|---------------|----------|
| 1. Customer registers | ✅ API exists | — |
| 2. KYC verification | ✅ API exists | — |
| 3. Coverage check | ✅ API exists | — |
| 4. Installation scheduling | ✅ API exists | — |
| 5. Field tech assignment | ✅ API exists | — |
| 6. Field tech GPS check-in | ❌ Missing | GPS + timestamp |
| 7. ONT installation & photo | ✅ API exists | — |
| 8. PPPoE account creation on RADIUS | ❌ Missing | Auto-provision |
| 9. Bandwidth profile push to MikroTik | ❌ Missing | Auto-provision |
| 10. ONT VLAN/QoS config on OLT | ❌ Missing | Auto-provision |
| 11. Connectivity verification | ❌ Missing | SNMP/ping check |
| 12. Customer notification | ✅ Worker exists | — |
| 13. First invoice generation | ⚠️ No tax | Needs GST calc |
| 14. Service activation | ⚠️ Manual | Needs auto-provision |

**Result:** Steps 8-11 require manual NOC engineer intervention (30-60 min per customer).

### 3.2 Daily Network Operations

| Operation | Current Status | Required |
|-----------|---------------|----------|
| Device health polling (SNMP) | ❌ Missing | SNMP library |
| ONT optical power monitoring | ❌ Missing | SNMP + thresholds |
| Bandwidth utilization tracking | ❌ Missing | SNMP counters |
| PPPoE session accounting | ❌ Missing | RADIUS listener |
| CDR ingestion from BNG | ❌ Missing | CDR parser |
| FUP enforcement | ❌ Missing | Usage metering |
| Auto-failover detection | ❌ Missing | SNMP + alerting |
| Fiber cut impact analysis | ❌ Missing | Topology data |

### 3.3 Billing Cycle (Monthly)

| Step | Current Status | Required |
|------|---------------|----------|
| 1. Auto-generate invoices | ⚠️ No tax, wrong dates | Fix month-end calc |
| 2. Apply GST (CGST+SGST) | ❌ Missing | Tax calculation |
| 3. Generate PDF invoices | ❌ Missing | PDF generation |
| 4. Send via Email/SMS/WhatsApp | ⚠️ Partial | Email sending |
| 5. Process payments | ✅ API exists | — |
| 6. Apply late fees | ❌ Missing | Late fee engine |
| 7. Dunning (overdue follow-up) | ⚠️ Hardcoded | Make configurable |
| 8. Auto-suspend on day 10 | ⚠️ Hardcoded | Make configurable |
| 9. Generate GST returns | ❌ Missing | GSTR-1/3B gen |
| 10. Bank reconciliation | ❌ Missing | Reconciliation engine |

---

## 4. MISSING ENTITIES — Database Schema Gaps

| Entity | Purpose | Priority |
|--------|---------|----------|
| `ip_address` | Individual IP allocation records | CRITICAL |
| `radius_accounting` | RADIUS accounting packet logs | CRITICAL |
| `provisioning_job` | Customer provisioning task tracking | CRITICAL |
| `fiber_segment` | Physical fiber route segments | HIGH |
| `olt_port` | OLT PON port → ONT mapping | HIGH |
| `customer_equipment` | Customer-premises equipment (ONT, router) | HIGH |
| `mass_incident` | Area-wide outage tracking | HIGH |
| `sla_definition` | SLA targets per plan/tier | HIGH |
| `sla_measurement` | Actual SLA performance per customer | HIGH |
| `usage_record` | Per-customer daily usage aggregation | HIGH |
| `fraud_alert` | Fraud detection alerts | MEDIUM |
| `reconciliation_record` | Payment reconciliation entries | MEDIUM |
| `static_ip_assignment` | Static IP assignments for enterprise | MEDIUM |

---

## 5. MISSING WORKERS — Background Process Gaps

| Worker | Purpose | Priority |
|--------|---------|----------|
| `ProvisioningWorker` | Auto-provision new customers (RADIUS + BNG + OLT) | CRITICAL |
| `RadiusAccountingWorker` | Correlate RADIUS sessions → customers → usage | CRITICAL |
| `CdrIngestionWorker` | Parse CDR files from BNGs/OLTs | CRITICAL |
| `UsageMeteringWorker` | Track per-customer data usage, FUP enforcement | CRITICAL |
| `SlaMonitorWorker` | Track SLA compliance, auto-escalate breaches | HIGH |
| `MassIncidentWorker` | Detect correlated failures, create mass incidents | HIGH |
| `FraudDetectionWorker` | Detect concurrent logins, MAC spoofing, speed bypass | MEDIUM |
| `ReportGenerationWorker` | Nightly/weekly aggregate reports | MEDIUM |
| `BackupWorker` | Automated DB backup scheduling | MEDIUM |
| `CertificateRenewalWorker` | Auto-renew TLS certificates | LOW |

---

## 6. MISSING INTEGRATIONS — External System Gaps

| Integration | Purpose | Priority |
|-------------|---------|----------|
| SNMP Library | Poll legacy devices (ZTE, Nokia, Calix) | CRITICAL |
| ZTE OLT Adapter | ZTE GPON management (India's #2 vendor) | HIGH |
| TR-069/CWMP | Remote ONT management protocol | HIGH |
| GSTN API | E-invoice IRN generation | MEDIUM |
| Bank Statement Parser | Payment reconciliation import | MEDIUM |
| GIS/Map Integration | Fiber route visualization | LOW |
| CRM Integration | Salesforce/Zoho sync | LOW |

---

## 7. IMPLEMENTATION ROADMAP

### Phase 1: Foundation (Weeks 1-2)
- [ ] Fix IP allocation (CIDR parser, per-address tracking)
- [ ] Add connection pooling for MikroTik REST + Huawei SSH
- [ ] Fix MikroTik REST API endpoints (use `/rest/ip/queue/simple`)
- [ ] Fix invoice month-end calculation
- [ ] Implement GST calculation on invoices

### Phase 2: Data Path (Weeks 3-4)
- [ ] Add `snmp` crate dependency
- [ ] Implement SNMPv2c/v3 polling for device metrics
- [ ] Implement RADIUS Accounting-Server listener
- [ ] Correlate RADIUS sessions to `pppoe_sessions`
- [ ] Populate `bytes_in`/`bytes_out` from RADIUS accounting

### Phase 3: Provisioning (Weeks 5-6)
- [ ] Implement `ProvisioningWorker`
- [ ] Auto-create PPPoE on RADIUS on subscription activation
- [ ] Auto-push bandwidth profile to BNG
- [ ] Auto-configure ONT on OLT
- [ ] Implement provisioning verification (SNMP/ping check)
- [ ] Implement provisioning rollback on failure

### Phase 4: Customer Portal (Weeks 7-8)
- [ ] Create `/api/v1/customer/me/*` route group
- [ ] Customer usage dashboard API
- [ ] Customer bill view & pay API
- [ ] Customer ticket creation API
- [ ] Customer plan change request API
- [ ] Invoice PDF generation

### Phase 5: Operations (Weeks 9-10)
- [ ] Implement SLA monitoring worker
- [ ] Implement mass incident management
- [ ] Implement late fee application worker
- [ ] Implement payment reconciliation
- [ ] Implement data retention enforcement

### Phase 6: Advanced (Weeks 11-12)
- [ ] Implement fraud detection
- [ ] Implement CDR ingestion pipeline
- [ ] Add ZTE adapter
- [ ] Add TR-069 support
- [ ] Implement reporting API

---

## 8. APPENDIX: COMPLETE GAP TRACKING MATRIX

| Gap ID | Category | Tier | Module | Status | Phase |
|--------|----------|------|--------|--------|-------|
| ISP-NET-C01 | Network | CRITICAL | radius | Open | 2 |
| ISP-NET-C02 | Network | CRITICAL | network | Open | 1 |
| ISP-NET-C03 | Network | CRITICAL | provisioning | Open | 3 |
| ISP-NET-C04 | Network | CRITICAL | monitoring | Open | 2 |
| ISP-NET-C05 | Network | CRITICAL | bandwidth | Open | 1 |
| ISP-BILL-C01 | Billing | CRITICAL | billing | Open | 1 |
| ISP-BILL-C02 | Infrastructure | CRITICAL | integrations | Open | 1 |
| ISP-CUST-C01 | Customer | CRITICAL | customer | Open | 4 |
| ISP-INFRA-C01 | Infrastructure | CRITICAL | workers | Open | 2-3 |
| ISP-NET-H01 | Network | HIGH | billing | Open | 6 |
| ISP-NET-H02 | Network | HIGH | network | Open | 5 |
| ISP-NET-H03 | Network | HIGH | tickets | Open | 5 |
| ISP-NET-H04 | Network | HIGH | integrations | Open | 1 |
| ISP-BILL-H01 | Billing | HIGH | billing | Open | 1 |
| ISP-BILL-H02 | Billing | HIGH | billing | Open | 4 |
| ISP-BILL-H03 | Billing | HIGH | billing | Open | 5 |
| ISP-BILL-H04 | Billing | HIGH | billing | Open | 1 |
| ISP-BILL-H05 | Billing | HIGH | billing | Open | 1 |
| ISP-CUST-H01 | Customer | HIGH | notifications | Open | 6 |
| ISP-CUST-H02 | Customer | HIGH | tickets | Open | 5 |
| ISP-CUST-H03 | Customer | HIGH | installation | Open | 6 |
| ISP-CUST-H04 | Customer | HIGH | notifications | Open | 5 |
| ISP-CUST-H05 | Customer | HIGH | subscription | Open | 5 |
| ISP-NET-M01 | Network | MEDIUM | integrations | Open | 6 |
| ISP-NET-M02 | Network | MEDIUM | integrations | Open | 6 |
| ISP-NET-M03 | Network | MEDIUM | integrations | Open | 6 |
| ISP-BILL-M01 | Billing | MEDIUM | accounting | Open | 6 |
| ISP-BILL-M02 | Billing | MEDIUM | billing | Open | 5 |
| ISP-BILL-M03 | Billing | MEDIUM | billing | Open | 6 |
| ISP-CUST-M01 | Customer | MEDIUM | security | Open | 6 |
| ISP-CUST-M02 | Customer | MEDIUM | compliance | Open | 5 |
| ISP-OPS-M01 | Operations | MEDIUM | reports | Open | 6 |
| ISP-OPS-M02 | Operations | MEDIUM | devops | Open | 6 |
| ISP-OPS-M03 | Operations | MEDIUM | customer | Open | 6 |
| ISP-OPS-M04 | Operations | MEDIUM | devops | Open | 6 |
| ISP-CUST-L01 | Customer | LOW | notifications | Open | 6 |
| ISP-NET-L01 | Network | LOW | network | Open | 6 |

**Total Gaps:** 37
**Critical:** 9 | **High:** 14 | **Medium:** 12 | **Low:** 2

---

## 9. CODE-LEVEL GAPS (v2.0 Deep Dive)

> These gaps were found by analyzing actual source code files. Each includes exact file:line references.

### 9.1 Billing Service — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-BILL-01 | `billing/services.rs:15-18` | `_page` and `_limit` parameters prefixed with underscore, never used | Full table loads, OOM at scale |
| CODE-BILL-02 | `billing/services.rs:68-70` | `tax_amount: Set(Decimal::ZERO)` — GST never calculated | Non-compliant invoices |
| CODE-BILL-03 | `billing/services.rs:218-221` | Auto-generate ignores GST, discounts, proration | Revenue leakage |
| CODE-BILL-04 | `billing/services.rs:56-59` | `timestamp_millis() % 10000` — collision possible | Duplicate invoices |
| CODE-BILL-05 | `billing/services.rs:252-261` | `send_invoice` only flips status, no delivery | Invoices never sent |
| CODE-BILL-06 | `billing/services.rs:402-417` | `get_dunning_config` returns hardcoded values | No per-branch config |
| CODE-BILL-07 | `billing/services.rs:421-433` | `get_tax_config` hardcodes Maharashtra only | Multi-state failure |
| CODE-BILL-08 | `billing/services.rs:83-120` | `record_payment` doesn't validate amount vs invoice | ₹1 pays ₹5000 |
| CODE-BILL-09 | `billing/services.rs:314-333` | `approve_refund` doesn't process money or reverse accounting | Refunds broken |
| CODE-BILL-10 | `billing/services.rs:111-118` | No database transaction on payment + invoice update | Double-credit race |
| CODE-BILL-11 | `billing/services.rs` | Domain aggregates bypassed — business rules dead code | Rules not enforced |

### 9.2 Network Service — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-NET-01 | `network/services.rs:158-176` | IP allocation: read-modify-write without locking | IP over-allocation |
| CODE-NET-02 | `network/services.rs:88-113` | CIDR not validated on pool creation | Invalid pools created |
| CODE-NET-03 | `network/services.rs:26-44` | VLAN ID not validated against domain rules | Invalid VLANs |
| CODE-NET-04 | `network/services.rs:258-280` | MAC binding allows duplicate MACs | MAC spoofing |
| CODE-NET-05 | `network/services.rs:234-244` | PPPoE terminate only updates DB, no NAS contact | Users stay online |
| CODE-NET-06 | `network/services.rs:282-344` | `get_topology` loads all data without pagination | OOM at scale |
| CODE-NET-07 | `network/services.rs` | No IP reclamation on subscription cancellation | IP pool exhaustion |

### 9.3 Customer Service — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-CUST-01 | `customer/services.rs:52-60` | Phone uniqueness check has race condition | Duplicate accounts |
| CODE-CUST-02 | `customer/services.rs:44-77` | No email uniqueness check | Duplicate emails |
| CODE-CUST-03 | `customer/services.rs:80-89` | No status transition validation | Invalid state changes |
| CODE-CUST-04 | `customer/services.rs:142-165` | No email/phone format validation | Invalid data |
| CODE-CUST-05 | `customer/services.rs:168-186` | Search uses `LIKE '%query%'` — no index usage | Slow at scale |
| CODE-CUST-06 | `customer/services.rs:125` | `add_address` always sets `is_primary=true` | Multiple primaries |
| CODE-CUST-07 | `customer/services.rs:34-42` | `get_customer` doesn't filter soft-deletes | Deleted customer accessible |

### 9.4 Ticket Service — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-TICK-01 | `ticket/services.rs:94-152` | No state machine — any status to any status | Invalid transitions |
| CODE-TICK-02 | `ticket/services.rs:201-213` | Satisfaction rating on non-resolved tickets, no range check | Invalid ratings |
| CODE-TICK-03 | `ticket/services.rs:120` | `escalate_ticket` overwrites resolution notes | Context lost |
| CODE-TICK-04 | `ticket/services.rs:46-79` | No SLA deadline calculation or tracking | No SLA enforcement |

### 9.5 Bandwidth Service — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-BW-01 | `bandwidth/services.rs:167-189` | `apply_profile` only flips DB flag, no device push | Profiles never enforced |
| CODE-BW-02 | `bandwidth/services.rs:191-211` | `device_id` never set on application | Worker can't find target |
| CODE-BW-03 | `bandwidth_worker.rs:148-174` | `verify_applied_profiles` is a no-op | No verification |
| CODE-BW-04 | `bandwidth/services.rs:232-257` | `get_usage` returns no actual usage data | No visibility |
| CODE-BW-05 | `bandwidth_worker.rs:43` | Processes only 20 items per cycle | Slow at scale |

### 9.6 Monitoring Service — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-MON-01 | `monitoring/services.rs:38-139` | Only 5 hardcoded metrics | Incomplete monitoring |
| CODE-MON-02 | `monitoring/services.rs:158-260` | `evaluate_alert_rules` returns empty Vec always | Alerts never surfaced |
| CODE-MON-03 | `monitoring/services.rs:318-327` | Fetches ALL alerts then filters in Rust | O(n) waste |
| CODE-MON-04 | `device_sync_worker.rs:236-239` | Random health scores when no adapter | False healthy status |
| CODE-MON-05 | `main.rs` | Monitoring worker never spawned | No device metrics |

### 9.7 Integration Adapters — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-INT-01 | `radius/adapter.rs:30` | `max_retries` config never used | Single packet loss = failure |
| CODE-INT-02 | `radius/adapter.rs:508-517` | `CallingStationId` (MAC) not sent | No MAC-based filtering |
| CODE-INT-03 | `radius/adapter.rs:355-424` | Response authenticator not validated | Spoofing possible |
| CODE-INT-04 | `mikrotik/adapter.rs:323-338` | Queue removal: GET + DELETE not atomic | Partial deletion |
| CODE-INT-05 | `mikrotik/adapter.rs:476` | PPPoE profile hardcoded to "default" | No bandwidth mapping |
| CODE-INT-06 | `huawei/adapter.rs:559-567` | `get_pon_status` returns hardcoded values | Fake PON data |
| CODE-INT-07 | `huawei/adapter.rs:495-511` | Traffic table CIR/PIR always 0 | No QoS data |
| CODE-INT-08 | `huawei/adapter.rs:236-273` | SSH output always `success: true` | Errors never detected |

### 9.8 Infrastructure — Code Bugs

| Gap | File:Line | Issue | Impact |
|-----|-----------|-------|--------|
| CODE-INF-01 | `main.rs:74-77` | NATS failure silently degrades | Cross-module comms stop |
| CODE-INF-02 | `main.rs:200` | Shutdown broadcast channel capacity 1 | Workers miss shutdown |
| CODE-INF-03 | `main.rs:414` | No graceful drain period | In-flight operations aborted |
| CODE-INF-04 | `routes/mod.rs:12` | WebSocket no auth middleware | Data exposure |
| CODE-INF-05 | `routes/mod.rs:13` | Swagger UI in production | Attack surface |

---

## 10. UPDATED GAP TRACKING MATRIX (v2.0)

| Gap ID | Category | Tier | Module | Status | Phase |
|--------|----------|------|--------|--------|-------|
| SEC-001 | Security | CRITICAL | security | Open | 0 |
| SEC-002 | Security | CRITICAL | mikrotik | Open | 0 |
| SEC-003 | Security | CRITICAL | mikrotik | Open | 0 |
| SEC-004 | Security | CRITICAL | websocket | Open | 0 |
| SEC-005 | Security | CRITICAL | routes | Open | 0 |
| SEC-006 | Security | HIGH | security | Open | 1 |
| SEC-007 | Security | CRITICAL | radius | Open | 0 |
| SEC-008 | Security | HIGH | radius | Open | 1 |
| SEC-009 | Security | HIGH | auth | Open | 1 |
| SEC-010 | Security | MEDIUM | auth | Open | 2 |
| SEC-011 | Compliance | CRITICAL | security | Open | 3 |
| SEC-012 | Compliance | CRITICAL | security | Open | 3 |
| SEC-013 | Compliance | MEDIUM | security | Open | 3 |
| CODE-BILL-01 | Code | CRITICAL | billing | Open | 0 |
| CODE-BILL-02 | Code | CRITICAL | billing | Open | 0 |
| CODE-BILL-03 | Code | CRITICAL | billing | Open | 0 |
| CODE-BILL-04 | Code | CRITICAL | billing | Open | 0 |
| CODE-BILL-05 | Code | HIGH | billing | Open | 2 |
| CODE-BILL-06 | Code | MEDIUM | billing | Open | 2 |
| CODE-BILL-07 | Code | HIGH | billing | Open | 1 |
| CODE-BILL-08 | Code | CRITICAL | billing | Open | 0 |
| CODE-BILL-09 | Code | HIGH | billing | Open | 2 |
| CODE-BILL-10 | Code | CRITICAL | billing | Open | 0 |
| CODE-BILL-11 | Code | HIGH | billing | Open | 1 |
| CODE-NET-01 | Code | CRITICAL | network | Open | 0 |
| CODE-NET-02 | Code | HIGH | network | Open | 1 |
| CODE-NET-03 | Code | HIGH | network | Open | 1 |
| CODE-NET-04 | Code | HIGH | network | Open | 1 |
| CODE-NET-05 | Code | CRITICAL | network | Open | 2 |
| CODE-NET-06 | Code | HIGH | network | Open | 1 |
| CODE-NET-07 | Code | HIGH | network | Open | 2 |
| CODE-CUST-01 | Code | CRITICAL | customer | Open | 0 |
| CODE-CUST-02 | Code | MEDIUM | customer | Open | 1 |
| CODE-CUST-03 | Code | CRITICAL | customer | Open | 0 |
| CODE-CUST-04 | Code | MEDIUM | customer | Open | 1 |
| CODE-CUST-05 | Code | HIGH | customer | Open | 2 |
| CODE-CUST-06 | Code | LOW | customer | Open | 3 |
| CODE-CUST-07 | Code | MEDIUM | customer | Open | 1 |
| CODE-TICK-01 | Code | HIGH | tickets | Open | 2 |
| CODE-TICK-02 | Code | MEDIUM | tickets | Open | 2 |
| CODE-TICK-03 | Code | MEDIUM | tickets | Open | 2 |
| CODE-TICK-04 | Code | CRITICAL | tickets | Open | 2 |
| CODE-BW-01 | Code | CRITICAL | bandwidth | Open | 2 |
| CODE-BW-02 | Code | CRITICAL | bandwidth | Open | 2 |
| CODE-BW-03 | Code | HIGH | bandwidth | Open | 2 |
| CODE-BW-04 | Code | HIGH | bandwidth | Open | 2 |
| CODE-BW-05 | Code | MEDIUM | bandwidth | Open | 2 |
| CODE-MON-01 | Code | HIGH | monitoring | Open | 2 |
| CODE-MON-02 | Code | HIGH | monitoring | Open | 2 |
| CODE-MON-03 | Code | MEDIUM | monitoring | Open | 2 |
| CODE-MON-04 | Code | CRITICAL | monitoring | Open | 0 |
| CODE-MON-05 | Code | CRITICAL | monitoring | Open | 0 |
| CODE-INT-01 | Code | HIGH | radius | Open | 1 |
| CODE-INT-02 | Code | HIGH | radius | Open | 1 |
| CODE-INT-03 | Code | HIGH | radius | Open | 1 |
| CODE-INT-04 | Code | MEDIUM | mikrotik | Open | 1 |
| CODE-INT-05 | Code | HIGH | mikrotik | Open | 2 |
| CODE-INT-06 | Code | CRITICAL | huawei | Open | 2 |
| CODE-INT-07 | Code | HIGH | huawei | Open | 2 |
| CODE-INT-08 | Code | CRITICAL | huawei | Open | 2 |
| CODE-INF-01 | Code | CRITICAL | main | Open | 0 |
| CODE-INF-02 | Code | MEDIUM | main | Open | 1 |
| CODE-INF-03 | Code | MEDIUM | main | Open | 1 |
| CODE-INF-04 | Code | CRITICAL | routes | Open | 0 |
| CODE-INF-05 | Code | CRITICAL | routes | Open | 0 |

**Total v2.0 Gaps:** 68
**Critical:** 23 | **High:** 21 | **Medium:** 22 | **Low:** 2

---

*Document version: 3.0 — Updated 2026-07-21*
*Previous version: 2.0 — 2026-07-21 (68 code/security gaps)*
*Previous version: 1.0 — 2026-07-21 (37 ISP operational gaps)*
*Combined total: 215 unique gaps (84 from v1.0 + 68 from v2.0 + 76 from v3.0, with 71 addressed incrementally)*

---

## 11. v3.0 DEEP DIVE — Finance, Architecture Patterns, SRS, Network Ops

> **Full gap details:** `GAP-finance-compliance.md`, `GAP-architecture-patterns.md`
> **Updated roadmap:** `GAP-IMPLEMENTATION-ROADMAP.md` (v3.0, 16 weeks, 12 phases)

### 11.1 Indian Finance & Tax Compliance (25 gaps)

| Gap | Priority | Summary |
|-----|----------|---------|
| F-01 | P0 | GST never calculated on invoices (`tax_amount: Set(Decimal::ZERO)`) |
| F-02 | P0 | No place-of-supply logic — hardcoded Maharashtra only |
| F-04 | P0 | Late fees lack 18% GST (Circular 178/10/2022) |
| F-05 | P0 | No credit notes / debit notes (Section 34 CGST Act) |
| F-08 | P1 | HSN/SAC codes never assigned to line items |
| F-20 | P0 | Tax invoice missing 8 mandatory fields (Rule 46 CGST Rules) |
| F-03 | P1 | No security deposit ledger — balance sheet misstatement |
| F-06 | P1 | No Ind AS 115 revenue recognition — deferred revenue missing |
| F-07 | P1 | No advance payment tracking |
| F-16 | P2 | Mid-month pro-ration never applied |
| F-17 | P2 | No grandfathered plan pricing |
| F-18 | P2 | No enterprise billing (consolidated, PO ref, credit terms) |
| F-19 | P2 | Incomplete chart of accounts (15+ missing accounts) |
| F-25 | P1 | No TRAI-compliant dunning process |
| F-09 | P2 | No reverse charge mechanism tracking |
| F-10 | P1 | No GST e-invoice (IRN) generation |
| F-11 | P1 | No payment reconciliation |
| F-12 | P1 | No UPI autopay / e-mandate management |
| F-13 | P2 | No gateway settlement cycle tracking |
| F-14 | P2 | No MDR tracking — ₹37,800/month unreconciled |
| F-15 | P2 | No bad debt provisioning (Ind AS 109 ECL model) |
| F-21 | P2 | No GST on discounted amounts |
| F-22 | P2 | No cash collection by field agents |
| F-23 | P3 | No EMI options for annual plans |
| F-24 | P2 | No wallet withdrawal for terminated customers |

### 11.2 Architecture Patterns & Resilience (18 gaps)

| Gap | Priority | Summary |
|-----|----------|---------|
| P-01 | P0 | No circuit breaker for MikroTik/Huawei/RADIUS — cascade failure |
| P-02 | P1 | No bulkhead isolation — billing blocks provisioning |
| P-03 | P1 | No saga compensation — partial provisioning zombie states |
| P-05 | P1 | No standardized retry policy — thundering herd risk |
| P-07 | P1 | No API/webhook retry + DLQ for external HTTP calls |
| P-13 | P1 | No IPAM data model — CIDR math missing |
| P-14 | P0 | Health check endpoints don't check DB/Redis/NATS |
| P-10 | P1 | No CDR storage schema — usage disputes unresolvable |
| P-11 | P1 | No data archival / AutoPurgeWorker |
| P-04 | P2 | No backpressure for bursty network events |
| P-06 | P2 | No graceful degradation strategy |
| P-08 | P2 | No time-series strategy for metrics |
| P-09 | P2 | No hot/cold data separation |
| P-12 | P2 | No materialized views for dashboards |
| P-15 | P2 | No worker job DLQ — bad record blocks entire queue |
| P-16 | P2 | No external service health monitoring |
| P-17 | P2 | No distributed tracing (OpenTelemetry) |
| P-18 | P3 | No SLO/SLI definitions |

### 11.3 Missing Workers (8 gaps)

| Worker | Priority | Purpose |
|--------|----------|---------|
| CdrProcessingWorker | CRITICAL | Parse BNG CDRs → usage → FUP |
| RadiusAccountingWorker | CRITICAL | RADIUS session tracking → billing |
| UsageMeteringWorker | CRITICAL | Per-customer data usage, FUP enforcement |
| SlaMonitorWorker | HIGH | SLA timers, auto-escalation, breach alerts |
| CapacityAlertingWorker | HIGH | SNMP polling, threshold alerts |
| ReportGenerationWorker | MEDIUM | Daily revenue, GST data |
| CertificateRenewalWorker | MEDIUM | TLS/JWT/RADIUS secret rotation |
| RetentionWorker | MEDIUM | Redis expiry, outbox cleanup |

### 11.4 Network Operations Gaps (10 domains)

| Domain | Priority | Key Missing |
|--------|----------|------------|
| Fiber Plant Topology | CRITICAL | OLT→Splitter→ONT hierarchy, fiber segments, splice points |
| IPAM | CRITICAL | Public IPv4, CGNAT, IPv6, IP recycling |
| Network Monitoring | CRITICAL | Real SNMP polling, all mocked, worker never spawned |
| Bandwidth Enforcement | HIGH | HTB hierarchy, CIR/PIR, PCQ, post-FUP |
| Provisioning Automation | CRITICAL | RADIUS CoA, OMCI, TR-069, fully manual |
| Incident Management | HIGH | Shift handover, escalation matrix, MTTR tracking |
| Field Operations | HIGH | Mobile app, GPS dispatch, checklists |
| Capacity Planning | HIGH | PON utilization, growth forecasting |
| Vendor/AMC Management | MEDIUM | Contracts, warranty, spare parts |
| Regulatory Compliance | CRITICAL | TRAI QoS, lawful intercept, CDR retention |

### 11.5 SRS Design Gaps (15 gaps)

| Gap | Summary |
|-----|---------|
| D-01 | No `customer_type` field (residential vs enterprise) |
| D-02 | No `relocation` installation type |
| D-03 | No `disconnection_order` entity |
| D-04 | No `customer_notes` / communication log |
| D-05 | No fiber route / physical layer entities |
| D-06 | No `sla_agreement` for enterprise |
| D-07 | No `static_ip_assignment` |
| D-08 | No `invoice_pdf_url` |
| D-09 | No `service_outage` / `maintenance_window` |
| D-10 | No `ticket_sla_timer` |
| D-11 | No plan versioning |
| D-12 | No `amount_paid` on invoices |
| D-13 | No `customer_contacts` for enterprise |
| D-14 | No `referral_wallet` linkage |
| D-15 | No `ont_provisioning_profile` |

### 11.6 Missing Entities (12 new)

| Entity | Purpose | Priority |
|--------|---------|----------|
| `ip_address` | Individual IP allocation records | CRITICAL |
| `radius_accounting` | RADIUS accounting packet logs | CRITICAL |
| `provisioning_job` | Customer provisioning task tracking | CRITICAL |
| `cdr_records` | Call detail records for usage tracking | CRITICAL |
| `fiber_segment` | Physical fiber route segments | HIGH |
| `olt_port` | OLT PON port → ONT mapping | HIGH |
| `splitters` | Optical splitter locations and mappings | HIGH |
| `customer_equipment` | Customer-premises equipment (ONT, router) | HIGH |
| `mass_incident` | Area-wide outage tracking | HIGH |
| `sla_definition` | SLA targets per plan/tier | HIGH |
| `sla_measurement` | Actual SLA performance per customer | HIGH |
| `usage_record` | Per-customer daily usage aggregation | HIGH |

### 11.7 Updated Gap Tracking Matrix (v3.0 additions)

| Gap ID | Category | Tier | Module | Status | Phase |
|--------|----------|------|--------|--------|-------|
| F-01 | Finance | CRITICAL | billing | Open | 1 |
| F-02 | Finance | CRITICAL | billing | Open | 1 |
| F-03 | Finance | HIGH | accounting | Open | 4 |
| F-04 | Finance | CRITICAL | billing | Open | 1 |
| F-05 | Finance | CRITICAL | billing | Open | 1 |
| F-06 | Finance | HIGH | accounting | Open | 4 |
| F-07 | Finance | HIGH | accounting | Open | 4 |
| F-08 | Finance | HIGH | billing | Open | 1 |
| F-09 | Finance | MEDIUM | accounting | Open | 8 |
| F-10 | Finance | HIGH | accounting | Open | 6 |
| F-11 | Finance | HIGH | billing | Open | 5 |
| F-12 | Finance | HIGH | payment | Open | 6 |
| F-13 | Finance | MEDIUM | payment | Open | 6 |
| F-14 | Finance | MEDIUM | accounting | Open | 6 |
| F-15 | Finance | MEDIUM | accounting | Open | 8 |
| F-16 | Finance | MEDIUM | billing | Open | 2 |
| F-17 | Finance | MEDIUM | subscription | Open | 5 |
| F-18 | Finance | MEDIUM | billing | Open | 7 |
| F-19 | Finance | MEDIUM | accounting | Open | 4 |
| F-20 | Finance | CRITICAL | billing | Open | 1 |
| F-21 | Finance | MEDIUM | billing | Open | 5 |
| F-22 | Finance | MEDIUM | installation | Open | 7 |
| F-23 | Finance | LOW | billing | Open | 9 |
| F-24 | Finance | MEDIUM | payment | Open | 6 |
| F-25 | Finance | HIGH | compliance | Open | 7 |
| P-01 | Architecture | CRITICAL | infrastructure | Open | 0 |
| P-02 | Architecture | HIGH | infrastructure | Open | 2 |
| P-03 | Architecture | HIGH | workflow | Open | 3 |
| P-04 | Architecture | MEDIUM | infrastructure | Open | 6 |
| P-05 | Architecture | HIGH | infrastructure | Open | 2 |
| P-06 | Architecture | MEDIUM | infrastructure | Open | 6 |
| P-07 | Architecture | HIGH | infrastructure | Open | 5 |
| P-08 | Architecture | MEDIUM | monitoring | Open | 7 |
| P-09 | Architecture | MEDIUM | infrastructure | Open | 8 |
| P-10 | Architecture | HIGH | network | Open | 3 |
| P-11 | Architecture | HIGH | compliance | Open | 5 |
| P-12 | Architecture | MEDIUM | monitoring | Open | 7 |
| P-13 | Architecture | HIGH | network | Open | 1 |
| P-14 | Architecture | CRITICAL | routes | Open | 0 |
| P-15 | Architecture | MEDIUM | workers | Open | 5 |
| P-16 | Architecture | MEDIUM | monitoring | Open | 3 |
| P-17 | Architecture | MEDIUM | infrastructure | Open | 8 |
| P-18 | Architecture | LOW | infrastructure | Open | 9 |
| W-01 | Workers | CRITICAL | radius | Open | 3 |
| W-02 | Workers | CRITICAL | radius | Open | 3 |
| W-03 | Workers | CRITICAL | bandwidth | Open | 3 |
| W-04 | Workers | HIGH | tickets | Open | 5 |
| W-05 | Workers | HIGH | monitoring | Open | 5 |
| W-06 | Workers | MEDIUM | reports | Open | 7 |
| W-07 | Workers | MEDIUM | security | Open | 8 |
| W-08 | Workers | MEDIUM | compliance | Open | 8 |
| N-01 | Network | CRITICAL | network | Open | 4 |
| N-02 | Network | CRITICAL | network | Open | 4 |
| N-03 | Network | CRITICAL | monitoring | Open | 0 |
| N-04 | Network | HIGH | bandwidth | Open | 2 |
| N-05 | Network | CRITICAL | provisioning | Open | 3 |
| N-06 | Network | HIGH | tickets | Open | 6 |
| N-07 | Network | HIGH | installation | Open | 7 |
| N-08 | Network | HIGH | monitoring | Open | 8 |
| N-09 | Network | MEDIUM | inventory | Open | 8 |
| N-10 | Network | CRITICAL | compliance | Open | 6 |
| D-01 | SRS | HIGH | customer | Open | 4 |
| D-02 | SRS | MEDIUM | installation | Open | 4 |
| D-03 | SRS | MEDIUM | customer | Open | 5 |
| D-04 | SRS | MEDIUM | customer | Open | 4 |
| D-05 | SRS | HIGH | network | Open | 4 |
| D-06 | SRS | HIGH | subscription | Open | 5 |
| D-07 | SRS | MEDIUM | network | Open | 5 |
| D-08 | SRS | MEDIUM | billing | Open | 4 |
| D-09 | SRS | MEDIUM | monitoring | Open | 5 |
| D-10 | SRS | HIGH | tickets | Open | 5 |
| D-11 | SRS | MEDIUM | subscription | Open | 6 |
| D-12 | SRS | MEDIUM | billing | Open | 2 |
| D-13 | SRS | LOW | customer | Open | 5 |
| D-14 | SRS | LOW | referrals | Open | 6 |
| D-15 | SRS | HIGH | network | Open | 3 |
| E-01 to E-12 | Entities | varies | various | Open | 3-5 |

**v3.0 Gaps:** 76
**Critical:** 19 | **High:** 30 | **Medium:** 22 | **Low:** 5
