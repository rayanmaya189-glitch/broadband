# AeroXe Backend — Database Design

> **Req Ref:** §13 Database Design

---

## 1. Overview

- **Engine:** PostgreSQL 16
- **Extensions:** PostGIS (spatial queries), pgcrypto, uuid-ossp
- **ORM:** SeaORM (no SQLx — all database access via SeaORM abstractions)
- **Partitioning:** Range-partitioned by time for high-volume tables

## 2. Migration Strategy

```
migrations/
├── 001_create_extensions.sql
├── 002_create_branches.sql
├── 003_create_rbac.sql
├── 004_create_customers.sql
├── 005_create_plans.sql
├── 006_create_subscriptions.sql
├── 007_create_billing.sql
├── 008_create_accounting.sql
├── 009_create_devices.sql
├── 010_create_network.sql
├── 011_create_tickets.sql
├── 012_create_notifications.sql
├── 013_create_audit.sql
├── 014_create_events.sql
├── 015_create_documents.sql
├── 016_seed_roles_permissions.sql
└── 017_seed_initial_plans.sql
```

- Migrations are versioned and run sequentially
- Each migration is idempotent where possible
- Rollback scripts provided for each migration
- Seed data separated from schema migrations

## 3. Table Inventory

### Core Platform (§2-§3)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `branches` | branches | No | Geographic service locations |
| `branch_working_hours` | branches | No | Operating hours per branch |
| `users` | users | No | System users (staff + customers) |
| `user_sessions` | auth | No | Active JWT sessions |
| `user_branches` | branches | No | User-branch assignments |
| `roles` | rbac | No | Role definitions |
| `permissions` | rbac | No | Permission definitions |
| `role_permissions` | rbac | No | Role-permission mappings |
| `user_roles` | rbac | No | User-role assignments |
| `permission_groups` | rbac | No | Permission group definitions |
| `permission_group_permissions` | rbac | No | Group-permission mappings |
| `approval_workflows` | rbac | No | Approval workflow configs |
| `approval_requests` | rbac | No | Pending approvals |

### Customer Management (§3)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `customers` | customers | No | Customer records |
| `customers_history` | customers | No | Customer change history |
| `customer_profiles` | customers | No | KYC, personal details |
| `kyc_documents` | customers | No | KYC document references |
| `addresses` | customers | No | Customer addresses |
| `subscriptions` | subscriptions | No | Active subscriptions |
| `subscriptions_history` | subscriptions | No | Subscription change history |
| `service_accounts` | subscriptions | No | Service type accounts |
| `installation_orders` | installations | No | Installation workflow |
| `coverage_areas` | coverage | No | Service coverage zones |
| `coverage_pincode_map` | coverage | No | Pincode-to-area mapping |
| `coverage_subnets` | coverage | No | Subnet-to-coverage mapping |
| `referral_programs` | referrals | No | Referral program configs |
| `referral_tracking` | referrals | No | Referral tracking |
| `leads` | leads | No | Sales leads |
| `lead_activities` | leads | No | Lead activity log |

### Plans & Bandwidth (§4-§5)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `plans` | plans | No | Internet plans |
| `plans_history` | plans | No | Plan change history |
| `plan_pricing` | plans | No | Multi-period pricing |
| `speed_profiles` | plans | No | Technical bandwidth config |
| `service_packages` | plans | No | Add-on packages |
| `plan_service_packages` | plans | No | Plan-package mapping |
| `bandwidth_profiles` | bandwidth | No | Bandwidth limit profiles |
| `bandwidth_profiles_history` | bandwidth | No | Profile change history |
| `bandwidth_applications` | bandwidth | No | Profile-to-subscription mapping |
| `bandwidth_usage` | bandwidth | **Yes (daily)** | Usage tracking |

### Hardware & Devices (§6)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `device_models` | devices | No | Device model catalog |
| `network_devices` | devices | No | Registered devices |
| `network_devices_history` | devices | No | Device change history |
| `device_ports` | devices | No | Physical ports |
| `device_logs` | devices | **Yes (daily)** | Device log messages |
| `device_metrics` | devices | **Yes (hourly)** | Device health metrics |
| `firmware_updates` | devices | No | Firmware update tracking |
| `discovery_scans` | discovery | No | Scan configurations |
| `discovery_results` | discovery | No | Discovered devices |
| `discovery_scan_history` | discovery | No | Scan history |
| `subnet_location_map` | discovery | No | Subnet-to-location mapping |
| `inventory_items` | inventory | No | Physical equipment |
| `inventory_movements` | inventory | No | Equipment movements |

### Network Management (§7)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `vlans` | network | No | VLAN definitions |
| `vlans_history` | network | No | VLAN change history |
| `ip_pools` | network | No | IP address pools |
| `ip_pools_history` | network | No | Pool change history |
| `ip_addresses` | network | No | Individual IP addresses |
| `pppoe_sessions` | network | No | PPPoE sessions |
| `pppoe_sessions_history` | network | No | Session history |
| `dhcp_leases` | network | No | DHCP lease tracking |
| `mac_bindings` | network | No | MAC-IP bindings |
| `customer_sessions` | network | **Yes (daily)** | Active sessions |

### Billing & Payments (§8)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `invoices` | billing | No | Customer invoices |
| `invoice_line_items` | billing | No | Invoice line items |
| `payments` | billing | No | Payment records |
| `refunds` | billing | No | Refund records |
| `discounts` | billing | No | Discount definitions |
| `payment_reminders` | billing | No | Dunning reminders |
| `invoices_history` | billing | No | Invoice change history |
| `refunds_history` | billing | No | Refund change history |
| `discounts_history` | billing | No | Discount change history |
| `approval_requests_history` | billing | No | Approval history |

### Accounting (§8A)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `chart_of_accounts` | accounting | No | Account definitions |
| `journal_entries` | accounting | No | Double-entry journals |
| `journal_entry_lines` | accounting | No | Debit/credit lines |
| `trial_balances` | accounting | No | Periodic balances |
| `gst_returns` | accounting | No | GST filing data |
| `financial_statements` | accounting | No | Generated statements |

### Support (§7A)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `tickets` | tickets | No | Support tickets |
| `ticket_comments` | tickets | No | Ticket conversation |
| `ticket_escalations` | tickets | No | Escalation records |
| `ticket_attachments` | tickets | No | File attachments |
| `ticket_status_history` | tickets | No | Status change log |
| `tickets_history` | tickets | No | Ticket change history |

### Notifications & Events (§9, §10, §12, §15)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `notification_templates` | notifications | No | Message templates |
| `notification_channels` | notifications | No | Channel configs |
| `notifications` | notifications | **Yes (daily)** | Sent notifications |
| `notification_history` | notifications | **Yes (daily)** | Delivery tracking |
| `events` | events | **Yes (daily)** | Event store |
| `event_subscriptions` | events | No | Subscriber configs |

### Security & Audit (§19)

| Table | Module | Partitioned | Description |
|-------|--------|-------------|-------------|
| `audit_logs` | audit | **Yes (monthly)** | Action audit trail |
| `document_files` | documents | No | File metadata |
| `document_access_logs` | documents | **Yes (daily)** | Access tracking |

## 4. Partitioning Strategy

High-volume tables use range partitioning by timestamp:

```sql
-- Example: audit_logs partitioned monthly
CREATE TABLE audit_logs (...) PARTITION BY RANGE (created_at);

CREATE TABLE audit_logs_2026_01 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-01-01') TO ('2026-02-01');

-- Auto-create future partitions via pg_partman or cron job
```

**Tables requiring partitioning:**
- `audit_logs` — monthly
- `device_logs` — daily (high volume)
- `device_metrics` — hourly
- `bandwidth_usage` — daily
- `customer_sessions` — daily
- `events` — daily
- `notifications` — daily
- `notification_history` — daily
- `document_access_logs` — daily

## 5. Row-Level Security (RLS)

Branch-scoped tables use RLS policies:

```sql
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;

CREATE POLICY branch_scope ON customers
    USING (
        branch_id = current_setting('app.current_branch_id')::BIGINT
        OR current_setting('app.is_company_wide')::BOOLEAN = TRUE
    );
```

Applied via middleware that sets session variables from JWT claims.

## 6. Key Indexes

```sql
-- Customers
CREATE INDEX idx_customers_branch ON customers(branch_id);
CREATE INDEX idx_customers_phone ON customers(phone);
CREATE INDEX idx_customers_status ON customers(status);
CREATE INDEX idx_customers_referral ON customers(referral_code);

-- Subscriptions
CREATE INDEX idx_subscriptions_customer ON subscriptions(customer_id);
CREATE INDEX idx_subscriptions_plan ON subscriptions(plan_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);
CREATE INDEX idx_subscriptions_billing ON subscriptions(next_billing_date);

-- Invoices
CREATE INDEX idx_invoices_customer ON invoices(customer_id);
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_invoices_due ON invoices(due_date);

-- Tickets
CREATE INDEX idx_tickets_branch ON tickets(branch_id);
CREATE INDEX idx_tickets_customer ON tickets(customer_id);
CREATE INDEX idx_tickets_assigned ON tickets(assigned_to);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_priority ON tickets(priority);
```

## 7. Connection Pool Configuration

```toml
# SeaORM config
[database]
max_connections = 20
min_connections = 5
connect_timeout = 30
idle_timeout = 600
```

## 8. Backup Strategy

- Full backup: daily at 2 AM IST
- WAL archiving: continuous (point-in-time recovery)
- Retention: 30 days for full backups, 7 days for WAL
- Test restore: weekly
