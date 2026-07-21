# ISP Design Gap — Implementation Roadmap (v3.0)

**Date:** 2026-07-21
**Total Gaps:** 215 (84 from v1.0 + 68 from v2.0 + 76 from v3.0 finance/patterns/SRS/network)
**Target:** Production-ready FTTH ISP platform for Jalgaon, India

---

## Gap Distribution

| Category | v1.0 | v2.0 | v3.0 | Total | Critical | High | Medium | Low |
|----------|------|------|------|-------|----------|------|--------|-----|
| Security & Compliance | 2 | 13 | 0 | 15 | 7 | 4 | 4 | 0 |
| ISP Network Operations | 12 | 7 | 10 | 29 | 9 | 14 | 6 | 0 |
| Billing & Revenue | 8 | 11 | 25 | 44 | 16 | 17 | 9 | 2 |
| Customer Operations | 6 | 7 | 15 | 28 | 5 | 10 | 10 | 3 |
| Infrastructure & DevOps | 5 | 5 | 18 | 28 | 8 | 10 | 9 | 1 |
| Tickets & SLA | 4 | 4 | 2 | 10 | 1 | 5 | 4 | 0 |
| Bandwidth & Monitoring | 6 | 10 | 8 | 24 | 7 | 11 | 5 | 1 |
| Integration Adapters | 7 | 8 | 0 | 15 | 2 | 9 | 4 | 0 |
| Regulatory (TRAI/GST/IT) | 12 | 0 | 0 | 12 | 2 | 3 | 5 | 2 |
| Indian Finance & Tax (v3.0) | — | — | 25 | 25 | 6 | 10 | 7 | 2 |
| Architecture Patterns (v3.0) | — | — | 18 | 18 | 4 | 8 | 5 | 1 |
| Missing Workers (v3.0) | — | — | 8 | 8 | 3 | 3 | 2 | 0 |
| **TOTAL** | **62** | **65** | **76** | **215** | **70** | **104** | **72** | **11** |

> Note: 71 additional gaps addressed incrementally within each phase.

---

## Phase 0: Security Hardening (Days 1-5) ⚠️ NEW

**Goal:** Fix all security vulnerabilities that block deployment. **No code ships until Phase 0 is complete.**

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| SEC-002 | Add MikroTik command whitelist | `integrations/mikrotik/adapter.rs` | 0.5 |
| SEC-003 | Enable MikroTik TLS cert validation | `integrations/mikrotik/adapter.rs` | 0.5 |
| SEC-005 | Gate Swagger behind env check | `routes/mod.rs` | 0.5 |
| SEC-004 | Add WebSocket auth middleware | `routes/mod.rs`, `middleware/ws_auth.rs` | 1 |
| SEC-007 | Fix RADIUS password chaining | `integrations/radius/adapter.rs` | 1 |
| SEC-001 | Per-record Aadhaar salt | `shared/utils/pii.rs`, migration | 1 |
| BUG-INF-04 | WebSocket auth fix | `routes/mod.rs` | (same as SEC-004) |
| BUG-INF-05 | Swagger env gate | `routes/mod.rs` | (same as SEC-005) |
| BUG-INT-08 | Huawei SSH error detection | `integrations/huawei/adapter.rs` | 0.5 |
| BUG-INT-06 | Huawei real PON status | `integrations/huawei/adapter.rs` | 0.5 |
| BUG-MON-04 | Remove random health scores | `device_sync_worker.rs` | 0.5 |
| BUG-MON-05 | Spawn monitoring worker | `main.rs` | 0.5 |
| BUG-INF-01 | NATS failure = fatal | `main.rs` | 0.5 |

**Gaps Closed:** 13
**Cumulative:** 13 / 152

---

## Phase 1: Data Integrity Foundation (Days 6-12)

**Goal:** Fix data corruption bugs and critical CRUD gaps.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| BUG-BILL-01 | Wire pagination parameters | `billing/application/service.rs` | 0.5 |
| BUG-BILL-02 | Implement GST calculation | `billing/application/service.rs` | 1 |
| BUG-BILL-04 | Invoice number via DB sequence | `billing/application/service.rs`, migration | 0.5 |
| BUG-BILL-08 | Validate payment vs invoice amount | `billing/application/service.rs` | 0.5 |
| BUG-BILL-10 | Wrap payment in DB transaction | `billing/application/service.rs` | 0.5 |
| BUG-NET-01 | IP allocation SELECT FOR UPDATE | `network/application/service.rs` | 1 |
| BUG-CUST-01 | Phone uniqueness with DB constraint | `customer/application/service.rs`, migration | 0.5 |
| BUG-CUST-03 | Status transition validation | `customer/application/service.rs` | 0.5 |
| ISP-NET-C02 | CIDR math for IP allocation | `network/domain/entities/ip_address.rs` | 2 |
| ISP-NET-C05 | Fix MikroTik REST endpoints | `integrations/mikrotik/adapter.rs` | 1 |
| ISP-NET-H04 | Connection pooling (MikroTik + Huawei) | `integrations/*/pool.rs`, `shared/app_state.rs` | 2 |

**New Dependencies:** `ipnetwork = "0.20"`
**New Entities:** `ip_address`
**New Migrations:** `ip_address` table, `invoice_number_seq` sequence

**Gaps Closed:** 11
**Cumulative:** 24 / 152

---

## Phase 2: Revenue & Billing (Days 13-18)

**Goal:** Fix revenue leakage and enable compliant invoicing.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| BUG-BILL-03 | Auto-generate with tax/discount | `billing/application/service.rs` | 1 |
| BUG-BILL-05 | Invoice delivery via notification | `billing/application/service.rs` | 1 |
| BUG-BILL-06 | Per-branch dunning config | `billing/application/service.rs` | 0.5 |
| BUG-BILL-07 | Intra/inter-state GST logic | `billing/application/service.rs` | 1 |
| BUG-BILL-09 | Gateway refund on approval | `billing/application/service.rs`, gateway adapter | 1 |
| BUG-BILL-11 | Refactor to use domain aggregates | `billing/domain/`, `billing/application/service.rs` | 3 |
| ISP-BILL-C01 | GST fields and calculation | `billing/domain/entities/invoice.rs` | (same as BUG-BILL-02) |
| ISP-BILL-H04 | Invoice number collision fix | (same as BUG-BILL-04) | — |
| ISP-BILL-H05 | Partial payment support | `billing/application/service.rs` | 1 |
| ISP-BILL-H01 | Pro-rata billing | `billing/domain/primitives.rs` | 1 |

**Gaps Closed:** 10
**Cumulative:** 34 / 152

---

## Phase 3: Network & Provisioning (Days 19-28)

**Goal:** Wire network device management and auto-provisioning.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| BUG-NET-02 | CIDR validation on pool create | `network/application/service.rs` | 0.5 |
| BUG-NET-03 | VLAN ID range validation | `network/application/service.rs` | 0.5 |
| BUG-NET-04 | MAC binding uniqueness | `network/application/service.rs` | 0.5 |
| BUG-NET-05 | PPPoE terminate → RADIUS CoA | `network/application/service.rs`, `radius/adapter.rs` | 2 |
| BUG-NET-07 | IP reclamation on cancellation | `network/application/service.rs` | 1 |
| BUG-INT-01 | RADIUS retry loop | `integrations/radius/adapter.rs` | 0.5 |
| BUG-INT-02 | RADIUS CallingStationId | `integrations/radius/adapter.rs` | 0.5 |
| BUG-INT-03 | RADIUS response validation | `integrations/radius/adapter.rs` | 1 |
| BUG-INT-05 | PPPoE profile mapping | `integrations/mikrotik/adapter.rs` | 1 |
| ISP-NET-C01 | RADIUS Accounting listener | `integrations/radius/listener.rs`, `workers/radius_worker.rs` | 3 |
| ISP-NET-C03 | ProvisioningWorker | `workers/provisioning_worker.rs` | 3 |
| ISP-NET-C04 | SNMP polling library + adapter | `integrations/snmp/mod.rs` | 3 |

**New Dependencies:** `snmp = "0.9"`
**New Entities:** `radius_accounting`, `provisioning_job`, `cdr_record`
**New Workers:** `RadiusAccountingWorker`, `ProvisioningWorker`, `CdrIngestionWorker`

**Gaps Closed:** 12
**Cumulative:** 46 / 152

---

## Phase 4: Customer Portal & Service (Days 29-36)

**Goal:** Enable customer self-service and mobile app.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| BUG-CUST-02 | Email uniqueness check | `customer/application/service.rs` | 0.5 |
| BUG-CUST-04 | Email/phone format validation | `customer/application/service.rs` | 0.5 |
| BUG-CUST-05 | Full-text search optimization | `customer/application/service.rs`, migration | 1 |
| BUG-CUST-07 | Soft-delete filtering | `customer/application/service.rs` | 0.5 |
| BUG-TICK-01 | Ticket state machine | `ticket/application/service.rs` | 1 |
| BUG-TICK-04 | SLA deadline calculation | `ticket/application/service.rs` | 1 |
| ISP-CUST-C01 | `/customer/me/*` route group | `routes/customer_portal.rs` | 3 |
| ISP-CUST-C01 | Customer usage dashboard API | `customer/application/service.rs` | 2 |
| ISP-BILL-H02 | Invoice PDF generation | `billing/application/pdf.rs` | 2 |
| ISP-CUST-C01 | Customer auth separation | `middleware/customer_auth.rs` | 1 |

**New Dependencies:** `printpdf = "0.7"`
**New Route Groups:** `/api/v1/customer/me/*` (15+ endpoints)

**Gaps Closed:** 10
**Cumulative:** 56 / 152

---

## Phase 5: Operations & Monitoring (Days 37-44)

**Goal:** Real-time operations monitoring and enforcement.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| BUG-BW-01 | Profile → device push | `bandwidth/application/service.rs` | 1 |
| BUG-BW-02 | Device binding on profile apply | `bandwidth/application/service.rs` | 0.5 |
| BUG-BW-03 | Profile verification (real check) | `bandwidth_worker.rs` | 1 |
| BUG-BW-04 | Real usage data in get_usage | `bandwidth/application/service.rs` | 1 |
| BUG-BW-05 | Worker batch size scaling | `bandwidth_worker.rs` | 0.5 |
| BUG-MON-01 | Dynamic metric list | `monitoring/services.rs` | 0.5 |
| BUG-MON-02 | Implement alert evaluation | `monitoring/services.rs` | 2 |
| BUG-MON-03 | DB-level alert filtering | `monitoring/services.rs` | 0.5 |
| BUG-TICK-02 | Rating guard (resolved only) | `ticket/application/service.rs` | 0.5 |
| BUG-TICK-03 | Escalation append notes | `ticket/application/service.rs` | 0.5 |
| ISP-CUST-H02 | SLA monitoring worker | `workers/sla_worker.rs` | 2 |
| ISP-NET-H03 | Mass incident management | `tickets/application/service.rs` | 2 |
| ISP-BILL-H03 | Late fee worker | `workers/late_fee_worker.rs` | 1 |
| ISP-BILL-M02 | Payment reconciliation | `billing/application/service.rs` | 2 |
| ISP-CUST-M02 | Data retention enforcement | `workers/retention_worker.rs` | 1 |

**New Entities:** `sla_definition`, `sla_measurement`, `mass_incident`
**New Workers:** `SlaMonitorWorker`, `LateFeeWorker`, `RetentionWorker`

**Gaps Closed:** 15
**Cumulative:** 71 / 152

---

## Phase 6: Network Ops & Integration (Days 45-52)

**Goal:** Complete network integration and operations tooling.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| BUG-NET-06 | Paginated topology query | `network/application/service.rs` | 1 |
| BUG-INT-04 | MikroTik atomic queue operations | `integrations/mikrotik/adapter.rs` | 1 |
| BUG-INT-07 | Huawei traffic table parsing | `integrations/huawei/adapter.rs` | 1 |
| ISP-NET-H01 | OLT/RADIUS integration | `integrations/*/adapter.rs` | 2 |
| ISP-NET-H02 | TR-069/CWMP support | `integrations/tr069/adapter.rs` | 3 |
| ISP-NET-M01 | ZTE OLT adapter | `integrations/zte/adapter.rs` | 3 |
| ISP-NET-M03 | SNMPv3 auth/priv support | `integrations/snmp/adapter.rs` | 1 |
| ISP-INFRA-C01 | CDR ingestion pipeline | `workers/cdr_worker.rs` | 2 |
| BUG-INF-02 | Shutdown broadcast capacity | `main.rs` | 0.5 |
| BUG-INF-03 | Graceful drain period | `main.rs` | 0.5 |

**Gaps Closed:** 10
**Cumulative:** 81 / 152

---

## Phase 7: Compliance & Advanced (Days 53-60)

**Goal:** Regulatory compliance and advanced features.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| SEC-006 | Distributed rate limiting (Redis) | `middleware/rate_limit.rs` | 1 |
| SEC-008 | RADIUS response validation | `integrations/radius/adapter.rs` | (same as BUG-INT-03) | — |
| SEC-009 | Refresh token rotation | `identity/application/service.rs` | 1 |
| SEC-010 | Progressive login delays | `identity/application/service.rs` | 1 |
| SEC-011 | DPDP Act compliance | new tables, `consent/application/service.rs` | 3 |
| SEC-012 | CERT-In incident workflow | `compliance/application/service.rs` | 2 |
| SEC-013 | Aadhaar Act decision | architecture decision | 1 |
| ISP-CUST-H01 | WhatsApp two-way bot | `integrations/whatsapp/bot.rs` | 2 |
| ISP-CUST-H03 | Field tech mobile API | `routes/field_ops.rs` | 2 |
| ISP-BILL-M01 | GST E-Invoice (IRN) | `accounting/application/service.rs` | 2 |
| ISP-CUST-H04 | Notification preferences | `notification/application/service.rs` | 1 |
| ISP-CUST-H05 | Downgrade protection | `subscriptions/application/service.rs` | 1 |
| ISP-CUST-M01 | Fraud detection worker | `workers/fraud_worker.rs` | 2 |
| ISP-OPS-M01 | Reporting API | `reports/application/service.rs` | 2 |

**Gaps Closed:** 14
**Cumulative:** 95 / 152

---

## Phase 8: Quality & Production Readiness (Days 61-68)

**Goal:** Testing, documentation, hardening.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| ISP-CUST-H04 | TRAI CAF generation | `compliance/application/service.rs` | 2 |
| ISP-OPS-M02 | Device config backup | `device/application/service.rs` | 1 |
| ISP-OPS-M03 | Equipment return workflow | `inventory/application/service.rs` | 1 |
| ISP-OPS-M04 | NTP, Redis persistence, monitoring | `docker-compose.yml` | 1 |
| ISP-CUST-M01 | Referral fraud prevention | `referrals/application/service.rs` | 1 |
| All | E2E test suite | `tests/e2e/` | 5 |
| All | Integration test coverage | `tests/integration/` | 3 |
| All | OpenAPI documentation | utoipa annotations | 2 |

**Gaps Closed:** 8
**Cumulative:** 103 / 215

---

## Phase 9: Indian Finance & Tax Compliance (Days 69-78) — v3.0 NEW

**Goal:** Indian-specific GST, TDS, Ind AS compliance for billing and accounting.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| F-01 | Implement GST calculation on invoices | `billing/application/service.rs` | 1 |
| F-02 | Place-of-supply logic (intra/inter-state) | `billing/application/service.rs`, customer state | 1 |
| F-04 | GST on late fees | `billing/workers/late_fee_worker.rs` | 0.5 |
| F-05 | Credit notes / debit notes | `billing/domain/entities/credit_note.rs` | 1.5 |
| F-08 | HSN/SAC per line item | `billing/domain/entities/invoice.rs` | 0.5 |
| F-20 | Tax invoice compliance (Rule 46) | `billing/application/pdf.rs` | 1 |
| F-03 | Security deposit ledger | `accounting/domain/entities/security_deposit.rs` | 1.5 |
| F-16 | Mid-month pro-ration | `billing/domain/primitives.rs` | 1 |
| F-19 | Complete chart of accounts (15+ accounts) | `13-accounting.md` schema | 1 |
| F-25 | TRAI-compliant dunning process | `billing/workers/dunning_worker.rs` | 1 |

**New Dependencies:** `ipnetwork = "0.20"` (for F-02 state comparison)
**New Entities:** `credit_notes`, `security_deposits`
**Gaps Closed:** 10
**Cumulative:** 113 / 215

---

## Phase 10: Architecture Resilience & Missing Workers (Days 79-86) — v3.0 NEW

**Goal:** Add circuit breakers, health checks, and implement 8 missing workers.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| P-01 | Circuit breaker for external adapters | `infrastructure/resilience/circuit_breaker.rs` | 2 |
| P-14 | Deep health check endpoints | `routes/health.rs` | 1 |
| P-05 | Standardized retry policy | `infrastructure/resilience/retry.rs` | 1 |
| P-02 | Bulkhead connection pools | `shared/app_state.rs` | 1 |
| W-01 | CdrProcessingWorker | `workers/cdr_worker.rs` | 1.5 |
| W-02 | RadiusAccountingWorker | `workers/radius_accounting_worker.rs` | 1.5 |
| W-03 | UsageMeteringWorker | `workers/usage_metering_worker.rs` | 1 |
| W-04 | SlaMonitorWorker | `workers/sla_worker.rs` | 1 |

**New Dependencies:** `tokio-retry`, `dashmap`
**New Entities:** `cdr_records`, `sla_measurement`
**New Workers:** 4
**Gaps Closed:** 8
**Cumulative:** 121 / 215

---

## Phase 11: Network Ops & Fiber Plant (Days 87-100) — v3.0 NEW

**Goal:** Real SNMP polling, fiber topology, IPAM, provisioning automation.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| N-03 | SNMP polling library + real metrics | `integrations/snmp/mod.rs`, `Cargo.toml` | 3 |
| N-01 | Fiber topology entities | `migrations/network/`, entities | 2 |
| N-02 | IPAM with CIDR math | `network/domain/entities/ip_address.rs` | 2 |
| N-05 | Provisioning automation (CoA + OMCI) | `workers/provisioning_worker.rs` | 3 |
| P-10 | CDR storage schema | `migrations/network/003_create_cdr.sql` | 1 |
| D-05 | Fiber route entities | `network/domain/entities/fiber_segment.rs` | 1 |
| D-15 | ONT provisioning profile | `network/domain/entities/ont_profile.rs` | 1 |
| E-01-E-12 | All missing entities | various | 3 |

**New Dependencies:** `snmp = "0.9"`, `ipnetwork = "0.20"`
**New Entities:** 12 (ip_address, radius_accounting, provisioning_job, cdr_records, fiber_segment, olt_port, splitters, customer_equipment, mass_incident, sla_definition, sla_measurement, usage_record)
**New Workers:** CdrProcessingWorker, ProvisioningWorker
**Gaps Closed:** 12
**Cumulative:** 133 / 215

---

## Remaining Gaps (82)

These gaps are addressed incrementally or deferred:

| Category | Count | Notes |
|----------|-------|-------|
| Indian Finance & Tax (P1-P3) | 15 | Partially in Phase 9; full compliance requires legal review |
| Architecture Patterns (P2-P3) | 10 | Graceful degradation, tracing, SLOs, hot/cold data |
| SRS Design Gaps | 15 | Customer type, SLA, outage management, plan versioning |
| Network Ops (P2-P3) | 8 | Field mobile app, capacity planning, vendor mgmt |
| Medium/Low priority items | 34 | Addressed as part of module refinements |

---

## Summary

| Phase | Days | Focus | Gaps Closed | Cumulative |
|-------|------|-------|-------------|------------|
| 0 | 1-5 | Security Hardening | 13 | 13 / 215 (6%) |
| 1 | 6-12 | Data Integrity | 11 | 24 / 215 (11%) |
| 2 | 13-18 | Revenue & Billing | 10 | 34 / 215 (16%) |
| 3 | 19-28 | Network & Provisioning | 12 | 46 / 215 (21%) |
| 4 | 29-36 | Customer Portal | 10 | 56 / 215 (26%) |
| 5 | 37-44 | Operations & Monitoring | 15 | 71 / 215 (33%) |
| 6 | 45-52 | Network Ops & Integration | 10 | 81 / 215 (38%) |
| 7 | 53-60 | Compliance & Advanced | 14 | 95 / 215 (44%) |
| 8 | 61-68 | Quality & Production | 8 | 103 / 215 (48%) |
| 9 | 69-78 | Indian Finance & Tax | 10 | 113 / 215 (53%) |
| 10 | 79-86 | Architecture Resilience | 8 | 121 / 215 (56%) |
| 11 | 87-100 | Network Ops & Fiber Plant | 12 | 133 / 215 (62%) |
| **Total** | **100 days (~20 weeks)** | | **133** | **133 / 215** |

**Remaining 82 gaps** are incremental improvements addressed during normal development sprints.

---

## Resource Requirements

| Resource | Quantity | Purpose |
|----------|----------|---------|
| Rust Developer | 2 | Core implementation |
| Security Engineer | 1 | Phase 0 + compliance |
| Network Engineer | 1 | RADIUS/SNMP/MikroTik testing |
| Finance/CA Consultant | 1 | GST/TDS/Ind AS compliance (Phase 9) |
| QA Engineer | 1 | Integration + E2E testing |
| DevOps | 1 | Infrastructure setup |

---

## Success Criteria

| Metric | Target |
|--------|--------|
| Critical security vulnerabilities | 0 |
| Data integrity bugs | 0 |
| Customer onboarding time | < 2 minutes (automated) |
| NOC dashboard data accuracy | 100% (real SNMP data) |
| Invoice GST compliance | 100% |
| SLA breach detection | < 1 minute |
| Customer self-service adoption | > 80% |
| System uptime | 99.9% |
| DPDP Act compliance | Full |
| CERT-In readiness | 6-hour reporting capability |
| TRAI QoS reporting | Automated |
| Security deposit tracking | 100% |
| Revenue recognition (Ind AS 115) | Compliant |
| Fiber plant visibility | Full OLT→Splitter→ONT |
| IPAM coverage | 100% of allocated IPs tracked |

---

*Document version: 3.0 — 2026-07-21*
*Previous: v1.0 (84 gaps, 12 weeks, 6 phases)*
*v2.0 (152 gaps, 14 weeks, 9 phases including security-first Phase 0)*
*v3.0 (215 gaps, 20 weeks, 12 phases including finance compliance + architecture resilience + network ops)*
