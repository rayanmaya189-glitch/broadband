# ISP Design Gap — Implementation Roadmap (v2.0)

**Date:** 2026-07-21
**Total Gaps:** 152 (84 from v1.0 API/design + 68 from v2.0 code/security deep dive)
**Target:** Production-ready FTTH ISP platform for Jalgaon, India

---

## Gap Distribution

| Category | v1.0 | v2.0 | Total | Critical | High | Medium | Low |
|----------|------|------|-------|----------|------|--------|-----|
| Security & Compliance | 2 | 13 | 15 | 7 | 4 | 4 | 0 |
| ISP Network Operations | 12 | 7 | 19 | 5 | 12 | 2 | 0 |
| Billing & Revenue | 8 | 11 | 19 | 10 | 7 | 1 | 1 |
| Customer Operations | 6 | 7 | 13 | 3 | 5 | 4 | 1 |
| Infrastructure & DevOps | 5 | 5 | 10 | 4 | 0 | 6 | 0 |
| Tickets & SLA | 4 | 4 | 8 | 1 | 3 | 4 | 0 |
| Bandwidth & Monitoring | 6 | 10 | 16 | 4 | 8 | 3 | 1 |
| Integration Adapters | 7 | 8 | 15 | 2 | 9 | 4 | 0 |
| Regulatory (TRAI/GST/IT) | 12 | 0 | 12 | 2 | 3 | 5 | 2 |
| **TOTAL** | **62** | **65** | **127** | **38** | **51** | **33** | **5** |

> Note: 25 additional v1.0 gaps addressed incrementally within each phase.

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
**Cumulative:** 103 / 152

---

## Remaining Gaps (49)

These gaps are addressed incrementally or deferred:

| Category | Count | Notes |
|----------|-------|-------|
| Regulatory (TRAI/GST) | 12 | Partially addressed in Phase 7-8; full compliance requires legal review |
| Medium/Low priority items | 37 | Addressed as part of module refinements |

---

## Summary

| Phase | Days | Focus | Gaps Closed | Cumulative |
|-------|------|-------|-------------|------------|
| 0 | 1-5 | Security Hardening | 13 | 13 / 152 (9%) |
| 1 | 6-12 | Data Integrity | 11 | 24 / 152 (16%) |
| 2 | 13-18 | Revenue & Billing | 10 | 34 / 152 (22%) |
| 3 | 19-28 | Network & Provisioning | 12 | 46 / 152 (30%) |
| 4 | 29-36 | Customer Portal | 10 | 56 / 152 (37%) |
| 5 | 37-44 | Operations & Monitoring | 15 | 71 / 152 (47%) |
| 6 | 45-52 | Network Ops & Integration | 10 | 81 / 152 (53%) |
| 7 | 53-60 | Compliance & Advanced | 14 | 95 / 152 (63%) |
| 8 | 61-68 | Quality & Production | 8 | 103 / 152 (68%) |
| **Total** | **68 days (~14 weeks)** | | **103** | **103 / 152** |

**Remaining 49 gaps** are incremental improvements addressed during normal development sprints.

---

## Resource Requirements

| Resource | Quantity | Purpose |
|----------|----------|---------|
| Rust Developer | 2 | Core implementation |
| Security Engineer | 1 | Phase 0 + compliance |
| Network Engineer | 1 | RADIUS/SNMP/MikroTik testing |
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

---

*Document version: 2.0 — 2026-07-21*
*Previous: v1.0 (84 gaps, 12 weeks, 6 phases)*
*Updated: v2.0 (152 gaps, 14 weeks, 9 phases including security-first Phase 0)*
