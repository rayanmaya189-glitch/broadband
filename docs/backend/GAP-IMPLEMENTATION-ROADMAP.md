# ISP Design Gap — Implementation Roadmap

**Date:** 2026-07-21
**Total Gaps:** 84 (47 API/design + 37 ISP operational)
**Target:** Production-ready FTTH ISP platform for Jalgaon, India

---

## Phase 1: Foundation (Weeks 1-2)

**Goal:** Fix the core data layer so the system can store real ISP data.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| ISP-NET-C02 | Fix IP allocation with CIDR math | `network/application/service.rs`, `network/domain/entities/ip_address.rs` | 2 |
| ISP-BILL-C02 | Add connection pooling for MikroTik + Huawei | `integrations/mikrotik/pool.rs`, `integrations/huawei/ssh_pool.rs`, `shared/app_state.rs` | 2 |
| ISP-NET-C05 | Fix MikroTik REST API endpoints | `integrations/mikrotik/adapter.rs` | 1 |
| ISP-BILL-C01 | Implement GST calculation on invoices | `billing/application/service.rs` | 1 |
| ISP-BILL-H04 | Fix invoice number collision (use DB sequence) | `billing/application/service.rs`, migration | 1 |
| ISP-BILL-H05 | Add partial payment support | `billing/application/service.rs` | 1 |
| ISP-BILL-H01 | Implement pro-rata billing | `billing/application/service.rs`, `billing/domain/primitives.rs` | 1 |
| ISP-NET-H04 | Fix Huawei CLI parsing (firmware-aware) | `integrations/huawei/adapter.rs` | 1 |

**New Dependencies:**
```toml
ipnetwork = "0.20"
```

**New Entities:**
- `ip_address` (individual IP allocation records)

**New Migrations:**
- Create `ip_address` table
- Add `invoice_number_seq` sequence
- Add `payment_status` enum to payments

---

## Phase 2: Data Path (Weeks 3-4)

**Goal:** Collect real data from network devices.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| ISP-NET-C04 | Add SNMP polling library | `Cargo.toml`, `integrations/snmp/mod.rs` | 2 |
| ISP-NET-C04 | Implement SNMPv2c/v3 polling | `integrations/snmp/adapter.rs` | 3 |
| ISP-NET-C01 | Implement RADIUS Accounting listener | `integrations/radius/listener.rs`, `workers/radius_worker.rs` | 3 |
| ISP-NET-C01 | Correlate RADIUS sessions to customers | `network/application/service.rs` | 1 |
| ISP-INFRA-C01 | Create CdrIngestionWorker | `workers/cdr_worker.rs` | 2 |

**New Dependencies:**
```toml
snmp = "0.9"
```

**New Entities:**
- `radius_accounting` (RADIUS accounting packet logs)
- `cdr_record` (CDR ingestion records)

**New Workers:**
- `RadiusAccountingWorker` (UDP listener on port 1813)
- `CdrIngestionWorker` (CDR file parser)

---

## Phase 3: Provisioning (Weeks 5-6)

**Goal:** Automate customer onboarding from 30-60 min to 2 minutes.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| ISP-NET-C03 | Implement ProvisioningWorker | `workers/provisioning_worker.rs` | 3 |
| ISP-NET-C03 | Auto-create PPPoE on RADIUS | `integrations/radius/adapter.rs` | 1 |
| ISP-NET-C03 | Auto-push bandwidth to BNG | `integrations/mikrotik/adapter.rs`, `integrations/huawei/adapter.rs` | 2 |
| ISP-NET-C03 | Auto-configure ONT on OLT | `integrations/huawei/adapter.rs` | 1 |
| ISP-NET-C03 | Implement provisioning verification | `workers/provisioning_worker.rs` | 1 |
| ISP-NET-C03 | Implement provisioning rollback | `workers/provisioning_worker.rs` | 1 |

**New Entities:**
- `provisioning_job` (provisioning task tracking)
- `provisioning_step` (individual step status)

**New Worker:**
- `ProvisioningWorker`

---

## Phase 4: Customer Portal (Weeks 7-8)

**Goal:** Enable customer self-service and mobile app.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| ISP-CUST-C01 | Create `/customer/me/*` route group | `routes/customer_portal.rs` | 2 |
| ISP-CUST-C01 | Customer usage dashboard API | `customer/application/service.rs` | 2 |
| ISP-CUST-C01 | Customer bill view & pay API | `billing/application/service.rs` | 2 |
| ISP-CUST-C01 | Customer ticket creation API | `tickets/application/service.rs` | 1 |
| ISP-CUST-C01 | Customer plan change request API | `subscriptions/application/service.rs` | 1 |
| ISP-BILL-H02 | Invoice PDF generation | `billing/application/pdf.rs` | 2 |
| ISP-CUST-C01 | Customer auth separation | `middleware/customer_auth.rs` | 1 |

**New Dependencies:**
```toml
printpdf = "0.7"
```

**New Route Groups:**
- `/api/v1/customer/me/*` (15+ endpoints)

---

## Phase 5: Operations (Weeks 9-10)

**Goal:** Enable real-time operations monitoring and enforcement.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| ISP-CUST-H02 | Implement SLA monitoring worker | `workers/sla_worker.rs` | 2 |
| ISP-NET-H03 | Implement mass incident management | `tickets/application/service.rs`, `network/application/service.rs` | 2 |
| ISP-BILL-H03 | Implement late fee application worker | `workers/late_fee_worker.rs` | 1 |
| ISP-BILL-M02 | Implement payment reconciliation | `billing/application/service.rs` | 2 |
| ISP-CUST-M02 | Implement data retention enforcement | `workers/retention_worker.rs` | 1 |
| ISP-CUST-H04 | Implement notification preferences | `notification/application/service.rs` | 1 |
| ISP-CUST-H05 | Implement downgrade protection | `subscriptions/application/service.rs` | 1 |

**New Entities:**
- `sla_definition`, `sla_measurement`
- `mass_incident`
- `notification_preference`

**New Workers:**
- `SlaMonitorWorker`
- `LateFeeWorker`
- `RetentionWorker`

---

## Phase 6: Advanced (Weeks 11-12)

**Goal:** Complete the ISP operational platform.

| Gap ID | Task | Files | Est. Days |
|--------|------|-------|-----------|
| ISP-CUST-M01 | Implement fraud detection | `workers/fraud_worker.rs` | 2 |
| ISP-NET-H01 | Complete CDR ingestion pipeline | `workers/cdr_worker.rs` | 2 |
| ISP-NET-M01 | Add ZTE OLT adapter | `integrations/zte/adapter.rs` | 3 |
| ISP-NET-M02 | Add TR-069/CWMP support | `integrations/tr069/adapter.rs` | 3 |
| ISP-BILL-M01 | Add GST E-Invoice (IRN) | `accounting/application/service.rs` | 2 |
| ISP-OPS-M01 | Implement reporting API | `reports/application/service.rs` | 2 |
| ISP-CUST-H01 | Implement WhatsApp two-way bot | `integrations/whatsapp/bot.rs` | 2 |
| ISP-CUST-H03 | Implement field tech mobile API | `routes/field_ops.rs` | 2 |

**New Integrations:**
- ZTE OLT adapter
- TR-069/CWMP client
- GSTN API client

**New Route Groups:**
- `/api/v1/field-ops/*`
- `/api/v1/reports/*`

---

## Summary

| Phase | Weeks | Focus | Gaps Closed |
|-------|-------|-------|-------------|
| 1 | 1-2 | Foundation | 8 |
| 2 | 3-4 | Data Path | 5 |
| 3 | 5-6 | Provisioning | 6 |
| 4 | 7-8 | Customer Portal | 7 |
| 5 | 9-10 | Operations | 7 |
| 6 | 11-12 | Advanced | 8 |
| **Total** | **12** | | **41** |

**Remaining gaps (43)** are addressed incrementally within each phase as part of the module implementations.

---

## Resource Requirements

| Resource | Quantity | Purpose |
|----------|----------|---------|
| Rust Developer | 2 | Core implementation |
| Network Engineer | 1 | RADIUS/SNMP/MikroTik testing |
| QA Engineer | 1 | Integration testing |
| DevOps | 1 | Infrastructure setup |

---

## Success Criteria

| Metric | Target |
|--------|--------|
| Customer onboarding time | < 2 minutes (automated) |
| NOC dashboard data accuracy | 100% (real SNMP data) |
| Invoice GST compliance | 100% |
| SLA breach detection | < 1 minute |
| Customer self-service adoption | > 80% |
| System uptime | 99.9% |
