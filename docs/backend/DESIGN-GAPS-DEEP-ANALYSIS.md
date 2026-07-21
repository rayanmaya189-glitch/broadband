# AeroXe Broadband — Deep ISP Operational Design Gap Analysis

**Date:** 2026-07-21
**Author:** Backend Architecture Team
**Scope:** Complete backend codebase + all 35 module docs + 6303-line requirement spec
**Methodology:** Real-world FTTH ISP operations perspective — what does a broadband ISP in Jalgaon, India actually need to run daily operations?

---

## Executive Summary

The AeroXe backend has **excellent CRUD coverage** (~229 endpoints) and **strong architectural patterns** (DDD, outbox, events, partitioning). However, the **ISP operational core** — the layer that talks to real network devices, collects real usage data, and enforces real speed limits — is almost entirely stubbed.

**Bottom line:** The system works as an admin panel but not as a live ISP operations platform.

| Category | Total Gaps | Critical | High | Medium | Low |
|----------|-----------|----------|------|--------|-----|
| ISP Network Operations | 12 | 5 | 4 | 3 | 0 |
| Billing & Revenue | 8 | 2 | 3 | 3 | 0 |
| Customer Operations | 6 | 1 | 3 | 2 | 0 |
| Compliance & Security | 7 | 1 | 2 | 3 | 1 |
| Infrastructure & DevOps | 5 | 1 | 2 | 1 | 1 |
| **TOTAL** | **38** | **10** | **14** | **12** | **2** |

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
- **Required Fix:** Create `customer-portal` route group with: `GET /me/profile`, `GET /me/invoices`, `POST /me/pay`, `GET /me/usage`, `POST /me/tickets`, `POST /me/plan-change`, `POST /me/kyc-upload`, `GET /me/referral-code`.

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
