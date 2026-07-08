# AeroXe Broadband — Production-Ready ISP Platform Requirement Document

> **Version:** 1.0.0  
> **Date:** July 2026  
> **Company:** Aeroxe Enterprises Pvt. Ltd. (AeroXe Broadband)  
> **Location:** Jalgaon, Maharashtra, India  
> **Domain:** aeroxebroadband.com  
> **Status:** Authoritative requirement specification

---

## Table of Contents

1. [Business Analysis](#1-business-analysis)
2. [User Roles and RBAC System](#2-user-roles-and-rbac-system)
3. [Customer Management Module](#3-customer-management-module)
4. [Product and Plan Management](#4-product-and-plan-management)
5. [Bandwidth Control System](#5-bandwidth-control-system)
6. [Hardware Device Management](#6-hardware-device-management)
7. [Network Management Module](#7-network-management-module)
8. [Billing System](#8-billing-system)
8A. [General Ledger & Double-Entry Accounting](#8a-general-ledger--double-entry-accounting)
8B. [Payment Gateway Integration](#8b-payment-gateway-integration)
8C. [Manual Payment & Top-Up Flow](#8c-manual-payment--top-up-flow)
8D. [Entity History & Rollback](#8d-entity-history--rollback)
9. [Notification Platform](#9-notification-platform)
10. [Realtime System](#10-realtime-system)
11. [Backend Architecture](#11-backend-architecture)
12. [Event Sourcing Design](#12-event-sourcing-design)
13. [Database Design](#13-database-design)
14. [Redis Design](#14-redis-design)
15. [NATS Event Architecture](#15-nats-event-architecture)
16. [Admin Portal Requirements](#16-admin-portal-requirements)
17. [Customer Android Application](#17-customer-android-application)
18. [Customer iOS Application](#18-customer-ios-application)
19. [Security Design](#19-security-design)
20. [DevOps and Production Deployment](#20-devops-and-production-deployment)
21. [System Architecture Diagram](#21-system-architecture-diagram)

---

## 1. Business Analysis

### 1.1 Business Overview

AeroXe Broadband is a **Fiber-to-the-Home (FTTH) Internet Service Provider** operating in Jalgaon, Maharashtra, India, with planned expansion to Bhusawal, Mumbai, Navi Mumbai, and Barhanpur. The company operates under the legal entity **Aeroxe Enterprises Pvt. Ltd.**

The business model is a **local ISP** that deploys, manages, and monetizes fiber optic infrastructure to deliver high-speed broadband internet to residential and business customers within a defined geographic footprint.

**Core value proposition:** Affordable, reliable, unlimited fiber internet with local support — competing against national telcos on price and responsiveness.

### 1.2 Target Customers

| Segment | Description | Typical Usage | Price Sensitivity |
|---------|-------------|---------------|-------------------|
| **Residential — Budget** | Students, remote workers in tier-2/3 cities | Streaming, browsing, online classes | High — ₹400–600/mo |
| **Residential — Standard** | Families, WFH professionals | Multi-device streaming, video calls | Medium — ₹600–1000/mo |
| **Residential — Premium** | Power users, gamers, streamers | 4K streaming, gaming, large downloads | Low — ₹800–1300/mo |
| **Small Business** | Shops, cafes, co-working spaces | POS systems, video conferencing, cloud | Medium — ₹1000–1300/mo |
| **Enterprise** | Offices, warehouses, industrial | Critical uptime, guaranteed SLA | Low — ₹1300+/mo |

### 1.3 Customer Personas

**Persona 1: College Student (Priya)**
- Age: 20, Jalgaon resident
- Needs: Affordable internet for online classes, Netflix, social media
- Pain: Buffering, hidden FUP limits, expensive plans
- Plan: Basic 50 Mbps @ ₹400/mo

**Persona 2: Remote Professional (Rahul)**
- Age: 32, works from home
- Needs: Reliable 100+ Mbps for video calls, large file transfers
- Pain: Downtime during work hours, poor support response
- Plan: Standard 100 Mbps @ ₹600/mo or Premium 150 Mbps @ ₹800/mo

**Persona 3: Small Business Owner (Amit)**
- Age: 45, runs a shop
- Needs: 200+ Mbps for POS, multiple staff devices, cloud backup
- Pain: Business-critical downtime, need priority support
- Plan: Pro 200 Mbps @ ₹1000/mo

**Persona 4: Gamer/Streamer (Vikram)**
- Age: 24, competitive gamer
- Needs: Low latency, consistent high speeds, no lag
- Pain: Speed drops during peak hours, router quality
- Plan: Premium 150 Mbps @ ₹800/mo or Pro 200 Mbps @ ₹1000/mo

### 1.4 Revenue Streams

| Stream | Description | Recurring? |
|--------|-------------|------------|
| **Subscription Revenue** | Monthly/quarterly/annual plan fees | ✅ Primary |
| **Installation Fees** | One-time installation (currently free promo) | ❌ |
| **Hardware Sales** | Routers, ONTs, switches (upsell) | ❌ |
| **Priority Support Tier** | Premium support packages | ✅ |
| **Late Payment Fees** | Penalty for overdue invoices | ❌ |
| **Plan Upgrades** | Mid-cycle speed upgrades | ✅ |

### 1.5 Products & Services

**Internet Plans (5 tiers):**

| Plan | Speed | Monthly Price | 3-Month | 6-Month | 12-Month |
|------|-------|---------------|---------|---------|----------|
| Basic | 50 Mbps | ₹400 | ₹1,150 | ₹2,250 | ₹4,300 |
| Standard | 100 Mbps | ₹600 | ₹1,700 | ₹3,350 | ₹6,400 |
| Premium | 150 Mbps | ₹800 | ₹2,300 | ₹4,550 | ₹8,700 |
| Pro | 200 Mbps | ₹1,000 | ₹2,850 | ₹5,650 | ₹10,800 |
| Ultimate | 300 Mbps | ₹1,300 | ₹3,700 | ₹7,350 | ₹14,000 |

**Common features across all plans:**
- Unlimited data (no FUP)
- Free installation (promotional)
- 24/7 support
- Free dual-band WiFi router (12-month plans)

### 1.6 Competitive Advantages

1. **Truly Unlimited Data** — No FUP caps, no throttling
2. **Free Dual-Band Router** — With annual plans
3. **Free Installation** — Professional setup at no cost
4. **Local Support** — Jalgaon-based team, 24/7 via phone/WhatsApp
5. **Aggressive Pricing** — Starting ₹400/mo, undercutting national ISPs
6. **Fiber-Only** — Modern infrastructure, not legacy copper
7. **Rapid Activation** — 24–48 hour installation turnaround

### 1.7 Operational Challenges

| Challenge | Impact | Platform Solution |
|-----------|--------|-------------------|
| Network downtime | Customer churn, SLA breach | Real-time monitoring, auto-alerting |
| Billing disputes | Revenue loss, customer friction | Transparent invoicing, event audit trail |
| Device failures | Service disruption | SNMP monitoring, auto-restart, alerting |
| Scaling to new cities | High operational complexity | Branch-level isolation, centralized management |
| Manual bandwidth management | Inconsistent QoS | Centralized bandwidth engine with device controller |
| Field technician coordination | Slow installation | Mobile app with workflow management |
| Customer onboarding friction | High CAC | Self-service portal, WhatsApp integration |

### 1.8 How the Platform Supports the Business

The software platform is the **operational nervous system** of AeroXe Broadband. It must:

- **Acquire customers** → Landing page, availability checker, WhatsApp integration
- **Onboard customers** → Registration, KYC, installation workflow, service activation
- **Deliver service** → Bandwidth control, device management, network orchestration
- **Bill customers** → Subscription management, invoicing, payments, dunning
- **Support customers** → Ticketing, real-time chat, status notifications
- **Monitor operations** → NOC dashboard, device health, network topology
- **Enable growth** → Multi-city support, scalable architecture, analytics

---

## 2. User Roles and RBAC System

### 2.1 Role Definitions

| Role | Description | Scope |
|------|-------------|-------|
| **super_admin** | Platform-wide control, manages all branches and system configuration | Global |
| **isp_owner** | Business owner with full control over all branches | Company-wide |
| **network_admin** | Manages network infrastructure, OLT/ONT, VLAN, IP pools | Network-scoped |
| **noc_engineer** | Monitors network health, handles alerts, first-response | Monitoring |
| **field_technician** | Performs installations, repairs, hardware swaps | Field ops |
| **customer_support** | Handles tickets, customer queries, plan changes | Support |
| **sales_agent** | Registers new customers, manages leads, conversions | Sales |
| **finance_manager** | Manages billing, payments, refunds, financial reports | Finance |
| **billing_operator** | Generates invoices, processes payments, handles refunds | Billing |
| **customer** | End-user who manages their subscription, views invoices, raises tickets | Self-service |

### 2.2 Permission Naming Convention

Format: `module.resource.action`

Examples:

```
customer.account.view
customer.account.disable
customer.subscription.suspend
customer.subscription.upgrade
bandwidth.profile.view
bandwidth.profile.update
bandwidth.profile.delete
device.router.view
device.router.restart
device.router.configure
olt.configuration.view
olt.configuration.change
olt.configuration.deploy
billing.invoice.view
billing.invoice.generate
billing.invoice.refund
billing.payment.process
network.vlan.create
network.vlan.delete
network.ippool.view
network.ippool.allocate
plan.view
plan.create
plan.update
plan.delete
plan.publish
plan.unpublish
ticket.view
ticket.create
ticket.assign
ticket.resolve
audit.log.view
report.generate
report.export
user.role.assign
user.role.revoke
notification.send
notification.template.manage
```

### 2.3 Complete Permission Matrix

```yaml
modules:
  customer:
    resources:
      account:
        actions: [view, create, update, delete, disable, enable, suspend, reactivate]
      subscription:
        actions: [view, create, upgrade, downgrade, cancel, suspend, reactivate]
      installation:
        actions: [view, create, schedule, complete, cancel, reschedule]
      profile:
        actions: [view, update, verify_kyc]
      address:
        actions: [view, create, update, delete]

  bandwidth:
    resources:
      profile:
        actions: [view, create, update, delete, apply]
      qos:
        actions: [view, create, update, delete]
      rate_limit:
        actions: [view, create, update, delete]
      traffic_shaping:
        actions: [view, create, update, delete]

  device:
    resources:
      router:
        actions: [view, register, configure, restart, shutdown, update_firmware, remove]
      switch:
        actions: [view, register, configure, restart, shutdown, update_firmware, remove]
      olt:
        actions: [view, register, configure, restart, update_firmware, remove, deploy_config]
      ont:
        actions: [view, register, configure, restart, update_firmware, remove, provision]
      access_point:
        actions: [view, register, configure, restart, update_firmware, remove]
      port:
        actions: [view, enable, disable, configure]

  network:
    resources:
      vlan:
        actions: [view, create, update, delete, assign, unassign]
      ippool:
        actions: [view, create, update, delete, allocate, release]
      pppoe:
        actions: [view, create, update, delete, authenticate, terminate]
      dhcp:
        actions: [view, create, update, delete, lease]
      mac_binding:
        actions: [view, create, update, delete]

  billing:
    resources:
      invoice:
        actions: [view, generate, send, void, refund, export]
      payment:
        actions: [view, process, refund, reconcile]
      discount:
        actions: [view, create, update, delete, apply]
      tax:
        actions: [view, configure]
      dunning:
        actions: [view, configure, execute]

  plan:
    resources:
      plan:
        actions: [view, create, update, delete, publish, unpublish, clone]
      speed_profile:
        actions: [view, create, update, delete]
      package:
        actions: [view, create, update, delete]

  ticket:
    resources:
      ticket:
        actions: [view, create, assign, update, resolve, close, escalate, reopen]
      comment:
        actions: [view, create, update, delete]

  notification:
    resources:
      template:
        actions: [view, create, update, delete]
      channel:
        actions: [view, configure]
      send:
        actions: [view, send, retry]

  audit:
    resources:
      log:
        actions: [view, export, search]

  report:
    resources:
      report:
        actions: [view, generate, export, schedule]

  user:
    resources:
      account:
        actions: [view, create, update, delete, disable, enable]
      role:
        actions: [view, assign, revoke, create, update, delete]
      permission:
        actions: [view, grant, revoke]

  network_topology:
    resources:
      topology:
        actions: [view, update]
```

### 2.4 Role-Permission Mapping

```yaml
super_admin:
  inherits: [isp_owner]
  additional: [user.role.assign, user.role.create, user.role.delete, audit.log.export]

isp_owner:
  inherits: [network_admin, finance_manager]
  additional: [user.account.create, user.account.delete, notification.template.manage]

network_admin:
  inherits: [noc_engineer]
  additional:
    - device.*.configure
    - device.*.update_firmware
    - device.*.remove
    - network.*.create
    - network.*.delete
    - network.*.update
    - bandwidth.*.create
    - bandwidth.*.update
    - bandwidth.*.delete
    - olt.configuration.deploy

noc_engineer:
  inherits: []
  additional:
    - device.*.view
    - device.*.restart
    - device.*.shutdown
    - device.*.register
    - network.*.view
    - bandwidth.profile.view
    - ticket.view
    - ticket.assign
    - ticket.update

field_technician:
  inherits: []
  additional:
    - customer.installation.view
    - customer.installation.schedule
    - customer.installation.complete
    - customer.installation.reschedule
    - device.ont.view
    - device.ont.provision
    - device.router.view
    - device.router.configure
    - ticket.view
    - ticket.update
    - customer.profile.view

customer_support:
  inherits: []
  additional:
    - customer.account.view
    - customer.account.update
    - customer.subscription.view
    - customer.subscription.upgrade
    - customer.subscription.downgrade
    - customer.subscription.suspend
    - customer.subscription.reactivate
    - ticket.*.all
    - notification.send

sales_agent:
  inherits: []
  additional:
    - customer.account.create
    - customer.account.view
    - customer.profile.create
    - customer.subscription.create
    - plan.view
    - customer.installation.create

finance_manager:
  inherits: [billing_operator]
  additional:
    - billing.invoice.refund
    - billing.payment.reconcile
    - report.generate
    - report.export
    - billing.discount.create
    - billing.discount.delete
    - billing.dunning.configure

billing_operator:
  inherits: []
  additional:
    - billing.invoice.view
    - billing.invoice.generate
    - billing.invoice.send
    - billing.payment.view
    - billing.payment.process
    - customer.subscription.view

customer:
  additional:
    - customer.subscription.view (own)
    - customer.profile.update (own)
    - ticket.create (own)
    - ticket.view (own)
```

### 2.5 Permission Groups

```yaml
permission_groups:
  network_ops:
    description: "Combined network and device operations"
    permissions:
      - device.*.view
      - device.*.restart
      - network.*.view
      - bandwidth.profile.view

  customer_ops:
    description: "Full customer lifecycle management"
    permissions:
      - customer.account.view
      - customer.account.update
      - customer.subscription.view
      - customer.subscription.suspend
      - customer.installation.view

  billing_ops:
    description: "Billing and payment operations"
    permissions:
      - billing.invoice.view
      - billing.invoice.generate
      - billing.payment.view
      - billing.payment.process

  support_ops:
    description: "Customer support operations"
    permissions:
      - ticket.*.all
      - customer.account.view
      - customer.profile.view
      - notification.send
```

### 2.6 Role Inheritance

Roles inherit permissions from their parent roles. Inheritance is additive — a role receives all permissions of its parent plus its own additional permissions.

```
super_admin
├── isp_owner
│   ├── network_admin
│   │   └── noc_engineer
│   └── finance_manager
│       └── billing_operator
├── field_technician
├── customer_support
├── sales_agent
└── customer
```

### 2.7 Resource-Level Permissions

Permissions can be scoped to specific resources via conditions, including **branch-level scoping**:

```json
{
  "permission": "device.router.restart",
  "conditions": {
    "resource_type": "device",
    "resource_id": "device-uuid",
    "scope": {
      "branch_id": 1,
      "city": "jalgaon",
      "area": "city-center"
    }
  }
}
```

**Branch-scoping enforcement:**
- Branch-level users can only access resources within their assigned branch(es)
- The `user_branches` table defines which branches a user can access
- API middleware extracts `branch_id` from JWT and filters all queries automatically
- Company-wide roles (`isp_owner`, `finance_manager`) bypass branch filtering
- Cross-branch operations (e.g., customer migration) require elevated permissions

### 2.8 Temporary Permissions

Time-bound permissions for special scenarios (e.g., a NOC engineer temporarily elevated to network_admin during an emergency):

```json
{
  "user_id": "user-uuid",
  "role_id": "network_admin",
  "granted_by": "isp-owner-uuid",
  "expires_at": "2026-07-15T18:00:00Z",
  "reason": "Emergency OLT firmware upgrade in Jalgaon City Center"
}
```

### 2.9 Approval Workflow

Critical operations require multi-step approval:

| Operation | Required Approver | Timeout |
|-----------|-------------------|---------|
| OLT firmware update | network_admin + isp_owner | 24h |
| Bulk customer suspension | finance_manager + isp_owner | 12h |
| Network-wide configuration change | network_admin + noc_engineer | 6h |
| Refund > ₹5,000 | billing_operator + finance_manager | 24h |
| Device removal | noc_engineer + network_admin | 12h |

### 2.10 Audit Tracking

Every permission check and action is logged:

```json
{
  "audit_id": "uuid",
  "timestamp": "2026-07-08T14:30:00Z",
  "user_id": "user-uuid",
  "user_email": "admin@aeroxe.com",
  "role": "network_admin",
  "action": "device.router.restart",
  "resource_type": "device",
  "resource_id": "router-uuid",
  "ip_address": "10.0.1.50",
  "user_agent": "Mozilla/5.0...",
  "result": "granted",
  "metadata": {
    "device_name": "Jalgaon-CityCenter-R01",
    "reason": "Customer reported connectivity issue"
  }
}
```

### 2.11 Branch Management

AeroXe operates as a **single-tenant** platform (one ISP company) with **multiple branches/locations**. Each branch represents a geographic service area (e.g., Jalgaon City, Bhusawal, Mumbai) with its own:
- Network infrastructure (OLTs, routers, switches)
- Customer base
- Field technicians and NOC engineers
- IP pools, VLANs, and subnets
- Revenue and billing data

**Branch-scoping rules:**
- Branch-level users (noc_engineer, field_technician, customer_support, sales_agent) are scoped to their branch
- Company-wide users (isp_owner, finance_manager, super_admin) can access all branches
- Plans and speed profiles are company-wide (shared across branches)
- Network devices, IP pools, VLANs, and customers are branch-scoped
- Reports can be filtered by branch or show consolidated company-wide data

```sql
CREATE TABLE branches (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    code VARCHAR(20) NOT NULL UNIQUE,
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL DEFAULT 'Maharashtra',
    address TEXT,
    phone VARCHAR(20),
    email VARCHAR(255),
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    timezone VARCHAR(50) DEFAULT 'Asia/Kolkata',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE branch_working_hours (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    day_of_week INTEGER NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
    open_time TIME NOT NULL,
    close_time TIME NOT NULL,
    is_closed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, day_of_week)
);

CREATE TABLE user_branches (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    branch_id BIGINT NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, branch_id)
);
```

**Branch-scoped permissions:**

```
branch.view
branch.create
branch.update
branch.manage_working_hours
branch.view_reports
branch.manage_staff
```

**Example branch data:**

| Code | Name | City | Status |
|------|------|------|--------|
| JLG | Jalgaon Main | Jalgaon | Active |
| BHL | Bhusawal | Bhusawal | Active |
| MUM | Mumbai | Mumbai | Planned |
| NNM | Navi Mumbai | Navi Mumbai | Planned |

### 2.12 RBAC Database Tables

```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone VARCHAR(20) NOT NULL UNIQUE,
    password_hash VARCHAR(255),
    name VARCHAR(255) NOT NULL,
    avatar_url TEXT,
    branch_id BIGINT REFERENCES branches(id),
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'inactive', 'suspended', 'locked')),
    last_login_at TIMESTAMPTZ,
    failed_login_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMPTZ,
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    two_factor_secret VARCHAR(255),
    phone_verified BOOLEAN DEFAULT FALSE,
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE user_sessions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE roles (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    parent_role_id BIGINT REFERENCES roles(id),
    is_system BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE permissions (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    module VARCHAR(100) NOT NULL,
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(module, resource, action)
);

CREATE TABLE role_permissions (
    id BIGSERIAL PRIMARY KEY,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    conditions JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(role_id, permission_id)
);

CREATE TABLE user_roles (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_by BIGINT REFERENCES users(id),
    is_active BOOLEAN DEFAULT TRUE,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, role_id)
);

CREATE TABLE permission_groups (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE permission_group_permissions (
    id BIGSERIAL PRIMARY KEY,
    group_id BIGINT NOT NULL REFERENCES permission_groups(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    UNIQUE(group_id, permission_id)
);

CREATE TABLE approval_workflows (
    id BIGSERIAL PRIMARY KEY,
    operation VARCHAR(255) NOT NULL,
    required_approver_roles BIGINT[] NOT NULL,
    timeout_hours INTEGER DEFAULT 24,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE approval_requests (
    id BIGSERIAL PRIMARY KEY,
    workflow_id BIGINT NOT NULL REFERENCES approval_workflows(id),
    requested_by BIGINT NOT NULL REFERENCES users(id),
    operation VARCHAR(255) NOT NULL,
    resource_type VARCHAR(100),
    resource_id UUID,
    payload JSONB NOT NULL,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected', 'expired')),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    rejection_reason TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE audit_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    action VARCHAR(255) NOT NULL,
    resource_type VARCHAR(100),
    resource_id UUID,
    ip_address INET,
    user_agent TEXT,
    result VARCHAR(20) NOT NULL CHECK (result IN ('granted', 'denied', 'expired')),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);
```

**Indexes:**

```sql
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
```


### 2.13 Lead Management

Sales agents track potential customers through a pipeline:

**Database:**
```sql
CREATE TABLE leads (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    assigned_to BIGINT REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    email VARCHAR(255),
    source VARCHAR(50) NOT NULL
        CHECK (source IN ('landing_page', 'whatsapp', 'referral', 'walk_in',
                          'cold_call', 'social_media', 'field_visit')),
    status VARCHAR(30) DEFAULT 'new'
        CHECK (status IN ('new', 'contacted', 'interested', 'surveyed',
                          'quoted', 'converted', 'lost')),
    interested_plan_id BIGINT REFERENCES plans(id),
    estimated_install_date DATE,
    address TEXT,
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    lost_reason TEXT,
    notes TEXT,
    converted_customer_id BIGINT REFERENCES customers(id),
    converted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE lead_activities (
    id BIGSERIAL PRIMARY KEY,
    lead_id BIGINT NOT NULL REFERENCES leads(id) ON DELETE CASCADE,
    activity_type VARCHAR(30) NOT NULL
        CHECK (activity_type IN ('call', 'whatsapp', 'visit', 'email', 'note', 'status_change')),
    description TEXT NOT NULL,
    performed_by BIGINT NOT NULL REFERENCES users(id),
    scheduled_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Lead Pipeline Stages:**
```
new → contacted → interested → surveyed → quoted → converted
                                          ↘ lost
```

---

## 3. Customer Management Module

### 3.1 Customer Lifecycle States

```
prospect → registered → kyc_pending → kyc_verified → installation_scheduled
→ installation_in_progress → active → suspended → terminated
```

**State transitions:**

| From | To | Trigger | Event |
|------|----|---------|-------|
| prospect | registered | Customer fills registration form | `customer.created` |
| registered | kyc_pending | KYC documents uploaded | `customer.kyc.submitted` |
| kyc_pending | kyc_verified | KYC approved by operator | `customer.kyc.verified` |
| kyc_verified | installation_scheduled | Field tech assigned | `customer.installation.scheduled` |
| installation_scheduled | installation_in_progress | Tech arrives on-site | `customer.installation.started` |
| installation_in_progress | active | Installation complete, ONT provisioned | `customer.activated` |
| active | suspended | Payment overdue / manual suspension | `customer.suspended` |
| suspended | active | Payment received / manual reactivation | `customer.reactivated` |
| active | terminated | Customer requests cancellation | `customer.terminated` |
| suspended | terminated | Exceeded suspension period | `customer.terminated` |

### 3.2 Customer Registration Flow

1. Customer enters phone number on landing page → WhatsApp message generated
2. Sales agent or self-service portal creates account
3. KYC documents uploaded (Aadhaar, PAN, address proof)
4. KYC verification (manual or automated)
5. Address verification against coverage area database
6. Installation order created and assigned to field technician
7. Technician installs ONT, provisions service
8. Customer account activated, first invoice generated

### 3.3 Customer Profile Data

```json
{
  "customer_id": "uuid",
  "name": "Rahul Sharma",
  "email": "rahul@example.com",
  "phone": "+919876543210",
  "alternate_phone": "+919876543211",
  "aadhaar_number": "hashed_aadhaar",
  "pan_number": "hashed_pan",
  "kyc_status": "verified",
  "kyc_documents": [...],
  "gender": "male",
  "date_of_birth": "1994-05-15",
  "occupation": "software_engineer",
  "referral_code": "RAHUL2024",
  "referred_by": "uuid-of-referrer"
}
```

### 3.4 Installation Workflow

```json
{
  "installation_order_id": "uuid",
  "customer_id": "uuid",
  "status": "scheduled",
  "assigned_technician_id": "uuid",
  "scheduled_date": "2026-07-10",
  "scheduled_time_slot": "10:00-12:00",
  "address": {
    "line1": "42, Shivaji Nagar",
    "area": "City Center",
    "city": "Jalgaon",
    "state": "Maharashtra",
    "pincode": "425001",
    "latitude": 21.0077,
    "longitude": 75.5626
  },
  "installation_type": "new",
  "equipment_issued": [
    {"type": "ont", "model": "Huawei HG8245H", "serial": "ONT-001"},
    {"type": "router", "model": "TP-Link Archer C6", "serial": "RTR-001"}
  ],
  "fiber_drop_length_meters": 45,
  "onu_power_dbm": -18.5,
  "speed_profile_id": "uuid",
  "pppoe_username": "rahul@aeroxe",
  "pppoe_password": "generated_password",
  "notes": "Customer premises, 2nd floor, existing conduit available",
  "photos": [...],
  "completed_at": "2026-07-10T11:30:00Z"
}
```

### 3.5 Coverage & Service Area Management

The platform maintains a database of service coverage areas to validate whether a customer address is within the ISP's service footprint before scheduling installation.

```sql
-- Requires PostGIS extension for spatial queries
-- CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE coverage_areas (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    area_type VARCHAR(30) DEFAULT 'polygon'
        CHECK (area_type IN ('polygon', 'circle', 'pincode')),
    boundary polygon,
    center_point point,
    radius_meters INTEGER,
    pincodes TEXT[],
    is_active BOOLEAN DEFAULT TRUE,
    max_customers INTEGER,
    current_customers INTEGER DEFAULT 0,
    fiber_available BOOLEAN DEFAULT TRUE,
    estimated_installation_days INTEGER DEFAULT 3,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE coverage_pincode_map (
    id BIGSERIAL PRIMARY KEY,
    coverage_area_id BIGINT NOT NULL REFERENCES coverage_areas(id) ON DELETE CASCADE,
    pincode VARCHAR(10) NOT NULL,
    city VARCHAR(100) NOT NULL,
    district VARCHAR(100),
    state VARCHAR(100) DEFAULT 'Maharashtra',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(coverage_area_id, pincode)
);

CREATE TABLE coverage_subnets (
    id BIGSERIAL PRIMARY KEY,
    coverage_area_id BIGINT NOT NULL REFERENCES coverage_areas(id) ON DELETE CASCADE,
    ip_pool_id BIGINT REFERENCES ip_pools(id),
    vlan_id BIGINT REFERENCES vlans(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_coverage_areas_branch ON coverage_areas(branch_id);
CREATE INDEX idx_coverage_areas_boundary ON coverage_areas USING GIST(boundary);
CREATE INDEX idx_coverage_areas_center ON coverage_areas USING GIST(center_point);
CREATE INDEX idx_coverage_pincode_pincode ON coverage_pincode_map(pincode);
```

**Availability Check API Flow:**

```
1. Customer enters pincode on landing page
2. Backend queries coverage_pincode_map for pincode
3. If match found → check coverage_areas for fiber_available
4. Return: { available: true, estimated_days: 3, area_name: "City Center" }
5. If no match → return: { available: false, message: "Service not yet available" }
6. Log query for demand analytics
```

### 3.6 Database Tables

```sql
CREATE TABLE customers (
    id BIGSERIAL PRIMARY KEY,
    customer_code VARCHAR(20) NOT NULL UNIQUE,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(20) NOT NULL,
    alternate_phone VARCHAR(20),
    status VARCHAR(30) NOT NULL DEFAULT 'registered'
        CHECK (status IN ('registered', 'kyc_pending', 'kyc_verified',
                          'installation_scheduled', 'installation_in_progress',
                          'active', 'suspended', 'terminated')),
    referral_code VARCHAR(20) UNIQUE,
    referred_by BIGINT REFERENCES customers(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Customers history table
CREATE TABLE customers_history (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE customer_profiles (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    aadhaar_hash VARCHAR(255),
    pan_hash VARCHAR(255),
    gender VARCHAR(10),
    date_of_birth DATE,
    occupation VARCHAR(100),
    kyc_status VARCHAR(20) DEFAULT 'pending'
        CHECK (kyc_status IN ('pending', 'submitted', 'verified', 'rejected')),
    kyc_verified_at TIMESTAMPTZ,
    kyc_verified_by BIGINT REFERENCES users(id),
    kyc_rejection_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE kyc_documents (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    document_type VARCHAR(50) NOT NULL,
    file_url TEXT NOT NULL,
    file_hash VARCHAR(255),
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected')),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE addresses (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    address_type VARCHAR(20) DEFAULT 'installation'
        CHECK (address_type IN ('installation', 'billing', 'correspondence')),
    line1 VARCHAR(255) NOT NULL,
    line2 VARCHAR(255),
    area VARCHAR(100),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    pincode VARCHAR(10) NOT NULL,
    country VARCHAR(50) DEFAULT 'India',
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    landmark VARCHAR(255),
    is_primary BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE subscriptions (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    plan_id BIGINT NOT NULL REFERENCES plans(id),
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'suspended', 'cancelled', 'expired')),
    billing_period_months INTEGER NOT NULL DEFAULT 1,
    start_date DATE NOT NULL,
    end_date DATE,
    next_billing_date DATE,
    auto_renew BOOLEAN DEFAULT TRUE,
    -- pppoe_username managed via pppoe_sessions table
    pppoe_session_id BIGINT REFERENCES pppoe_sessions(id),
    mac_address MACADDR,
    ip_address INET,
    vlan_id INTEGER,
    -- Checker/Maker workflow
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Subscriptions history table
CREATE TABLE subscriptions_history (
    id BIGSERIAL PRIMARY KEY,
    subscription_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE service_accounts (
    id BIGSERIAL PRIMARY KEY,
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    service_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'active',
    config JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE installation_orders (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    assigned_technician_id BIGINT REFERENCES users(id),
    status VARCHAR(30) DEFAULT 'pending'
        CHECK (status IN ('pending', 'scheduled', 'in_progress', 'completed', 'cancelled', 'rescheduled')),
    scheduled_date DATE,
    scheduled_time_slot VARCHAR(20),
    completed_at TIMESTAMPTZ,
    installation_type VARCHAR(20) DEFAULT 'new',
    equipment_issued JSONB,
    fiber_drop_length_meters INTEGER,
    onu_power_dbm DECIMAL(5,2),
    notes TEXT,
    photos TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 3.6 Events

```yaml
customer.created:
  payload:
    customer_id: uuid
    name: string
    phone: string
    email: string
    referred_by: uuid | null
    source: "landing_page" | "whatsapp" | "portal" | "agent"

customer.activated:
  payload:
    customer_id: uuid
    subscription_id: uuid
    plan_id: uuid
    pppoe_username: string
    vlan_id: integer
    ip_address: string

customer.suspended:
  payload:
    customer_id: uuid
    subscription_id: uuid
    reason: "payment_overdue" | "manual" | "violation"
    suspended_by: uuid
    suspension_note: string

customer.reactivated:
  payload:
    customer_id: uuid
    subscription_id: uuid
    reactivated_by: uuid

customer.terminated:
  payload:
    customer_id: uuid
    subscription_id: uuid
    reason: string
    terminated_by: uuid
    final_invoice_id: uuid

customer.kyc.submitted:
  payload:
    customer_id: uuid
    document_types: string[]

customer.kyc.verified:
  payload:
    customer_id: uuid
    verified_by: uuid

customer.installation.scheduled:
  payload:
    customer_id: uuid
    installation_order_id: uuid
    technician_id: uuid
    scheduled_date: date
    scheduled_time_slot: string
```


### 3.7 Referral Program

Customers earn rewards for referring new subscribers:

**Database:**
```sql
CREATE TABLE referral_programs (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    referrer_reward_type VARCHAR(20) NOT NULL CHECK (referrer_reward_type IN ('credit', 'free_days', 'plan_upgrade')),
    referrer_reward_value DECIMAL(10,2) NOT NULL,
    referee_reward_type VARCHAR(20) NOT NULL CHECK (referee_reward_type IN ('credit', 'free_days', 'discount')),
    referee_reward_value DECIMAL(10,2) NOT NULL,
    max_referrals_per_customer INTEGER,
    valid_from DATE NOT NULL,
    valid_until DATE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE referral_tracking (
    id BIGSERIAL PRIMARY KEY,
    program_id BIGINT NOT NULL REFERENCES referral_programs(id),
    referrer_id BIGINT NOT NULL REFERENCES customers(id),
    referee_id BIGINT REFERENCES customers(id),
    referral_code VARCHAR(20) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'registered', 'activated', 'rewarded')),
    referrer_reward_amount DECIMAL(10,2),
    referee_reward_amount DECIMAL(10,2),
    referrer_reward_applied BOOLEAN DEFAULT FALSE,
    referee_reward_applied BOOLEAN DEFAULT FALSE,
    registered_at TIMESTAMPTZ,
    activated_at TIMESTAMPTZ,
    rewarded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Flow:**
1. Referrer shares unique referral code
2. New customer registers with code → `status: 'registered'`
3. New customer activates service → `status: 'activated'`
4. System credits referrer wallet + applies referee discount
5. `status: 'rewarded'`
6. Journal entry: Dr. Marketing Expense, Cr. Prepaid Expenses

---

## 4. Product and Plan Management

### 4.1 Plan Structure

Plans are composed of:

1. **Plan** — The customer-facing product (e.g., "Standard 100 Mbps")
2. **Speed Profile** — The technical bandwidth configuration
3. **Service Package** — Optional add-ons (priority support)

### 4.2 Plan Definitions

```json
{
  "plans": [
    {
      "id": "basic-50",
      "name": "Basic",
      "speed": "50 Mbps",
      "download_mbps": 50,
      "upload_mbps": 25,
      "burst_mbps": 75,
      "data_quota": "unlimited",
      "fair_usage_policy": null,
      "qos_priority": "standard",
      "sla_uptime_percent": 99.5,
      "is_popular": false,
      "is_business": false,
      "billing_cycles": {
        "1": {"price": 400, "savings": null},
        "3": {"price": 1150, "savings": 50},
        "6": {"price": 2250, "savings": 150},
        "12": {"price": 4300, "savings": 500}
      },
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "Reliable Connection"
      ]
    },
    {
      "id": "standard-100",
      "name": "Standard",
      "speed": "100 Mbps",
      "download_mbps": 100,
      "upload_mbps": 50,
      "burst_mbps": 150,
      "data_quota": "unlimited",
      "fair_usage_policy": null,
      "qos_priority": "standard",
      "sla_uptime_percent": 99.5,
      "is_popular": true,
      "is_business": false,
      "billing_cycles": {
        "1": {"price": 600, "savings": null},
        "3": {"price": 1700, "savings": 100},
        "6": {"price": 3350, "savings": 250},
        "12": {"price": 6400, "savings": 800}
      },
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "Reliable Connection",
        "Dual Band WiFi Router Free*"
      ]
    },
    {
      "id": "premium-150",
      "name": "Premium",
      "speed": "150 Mbps",
      "download_mbps": 150,
      "upload_mbps": 75,
      "burst_mbps": 200,
      "data_quota": "unlimited",
      "fair_usage_policy": null,
      "qos_priority": "high",
      "sla_uptime_percent": 99.9,
      "is_popular": false,
      "is_business": false,
      "billing_cycles": {
        "1": {"price": 800, "savings": null},
        "3": {"price": 2300, "savings": 100},
        "6": {"price": 4550, "savings": 250},
        "12": {"price": 8700, "savings": 900}
      },
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "Reliable Connection",
        "Dual Band WiFi Router Free*",
        "Priority Support"
      ]
    },
    {
      "id": "pro-200",
      "name": "Pro",
      "speed": "200 Mbps",
      "download_mbps": 200,
      "upload_mbps": 100,
      "burst_mbps": 250,
      "data_quota": "unlimited",
      "fair_usage_policy": null,
      "qos_priority": "high",
      "sla_uptime_percent": 99.9,
      "is_popular": false,
      "is_business": true,
      "billing_cycles": {
        "1": {"price": 1000, "savings": null},
        "3": {"price": 2850, "savings": 150},
        "6": {"price": 5650, "savings": 350},
        "12": {"price": 10800, "savings": 1200}
      },
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "Reliable Connection",
        "Dual Band WiFi Router Free*",
        "Priority Support"
      ]
    },
    {
      "id": "ultimate-300",
      "name": "Ultimate",
      "speed": "300 Mbps",
      "download_mbps": 300,
      "upload_mbps": 150,
      "burst_mbps": 400,
      "data_quota": "unlimited",
      "fair_usage_policy": null,
      "qos_priority": "critical",
      "sla_uptime_percent": 99.99,
      "is_popular": false,
      "is_business": true,
      "billing_cycles": {
        "1": {"price": 1300, "savings": null},
        "3": {"price": 3700, "savings": 200},
        "6": {"price": 7350, "savings": 450},
        "12": {"price": 14000, "savings": 1600}
      },
      "features": [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "Reliable Connection",
        "Dual Band WiFi Router Free*",
        "Priority Support",
        "Business Grade"
      ]
    }
  ]
}
```

### 4.3 Speed Profile Structure

```json
{
  "speed_profile_id": "uuid",
  "plan_id": "standard-100",
  "download_limit_kbps": 102400,
  "upload_limit_kbps": 51200,
  "burst_download_kbps": 153600,
  "burst_upload_kbps": 76800,
  "burst_duration_seconds": 30,
  "priority_queue": 2,
  "qos_marking": "af21",
  "htb_parent_queue": "1:1",
  "fq_codel_enabled": true,
  "device_type": "mikrotik",
  "applied_at": "2026-07-08T14:30:00Z"
}
```

### 4.4 Database Tables

```sql
CREATE TABLE plans (
    id BIGSERIAL PRIMARY KEY,
    slug VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    speed_label VARCHAR(20) NOT NULL,
    download_mbps INTEGER NOT NULL,
    upload_mbps INTEGER NOT NULL,
    burst_mbps INTEGER,
    data_quota VARCHAR(50) DEFAULT 'unlimited',
    fair_usage_policy JSONB,
    qos_priority VARCHAR(20) DEFAULT 'standard',
    sla_uptime_percent DECIMAL(5,2) DEFAULT 99.5,
    is_popular BOOLEAN DEFAULT FALSE,
    is_business BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    sort_order INTEGER DEFAULT 0,
    -- Checker/Maker workflow
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Plans history table
CREATE TABLE plans_history (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE plan_pricing (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    billing_period_months INTEGER NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    savings DECIMAL(10,2),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(plan_id, billing_period_months)
);

CREATE TABLE speed_profiles (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES plans(id),
    name VARCHAR(100) NOT NULL,
    download_limit_kbps INTEGER NOT NULL,
    upload_limit_kbps INTEGER NOT NULL,
    burst_download_kbps INTEGER,
    burst_upload_kbps INTEGER,
    burst_duration_seconds INTEGER DEFAULT 30,
    priority_queue INTEGER DEFAULT 1,
    qos_marking VARCHAR(10),
    htb_parent_queue VARCHAR(20),
    fq_codel_enabled BOOLEAN DEFAULT TRUE,
    device_type VARCHAR(50) NOT NULL DEFAULT 'mikrotik',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE service_packages (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    type VARCHAR(50) NOT NULL,
    monthly_price DECIMAL(10,2),
    config JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE plan_service_packages (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    package_id BIGINT NOT NULL REFERENCES service_packages(id) ON DELETE CASCADE,
    is_included BOOLEAN DEFAULT FALSE,
    additional_price DECIMAL(10,2),
    UNIQUE(plan_id, package_id)
);
```

---

## 5. Bandwidth Control System

### 5.1 Architecture

```
Admin Portal
    ↓ (REST API)
Rust Axum Backend
    ↓ (Internal Service)
Bandwidth Engine
    ↓ (NATS Event)
Network Device Controller
    ↓ (SNMP / SSH / API)
MikroTik RouterOS / Cisco IOS / Huawei ONT / ZTE OLT
```

### 5.2 Speed Profile Application Flow

1. Admin creates/updates a speed profile in the portal
2. Backend validates the profile and persists to database
3. Backend publishes `bandwidth.profile.updated` event to NATS
4. Bandwidth Engine subscribes to the event
5. Bandwidth Engine resolves which devices need the update (via customer → subscription → device mapping)
6. Bandwidth Engine sends commands to Network Device Controller
7. Network Device Controller translates to vendor-specific commands:
   - **MikroTik:** `/ip/firewall/mangle`, `/queue/simple`, HTB trees
   - **Cisco:** `policy-map`, `class-map`, MQC
   - **Huawei OLT:** `traffic-profile`, `ont-mutual-auth`
   - **ZTE OLT:** `ont-traffic-profile`
8. Controller verifies the configuration was applied
9. Controller publishes `bandwidth.profile.applied` event
10. Backend updates subscription status

### 5.3 MikroTik Integration Example

```
# Rate limiting via simple queue
/queue/simple add name="customer-rahul" \
    target=192.168.1.100/32 \
    max-limit=100M/50M \
    burst-limit=150M/75M \
    burst-threshold=80M/40M \
    burst-time=30s/30s \
    priority=2

# HTB tree for fine-grained control
/queue tree add name="download-rahul" \
    parent=global-out \
    packet-mark="" \
    queue=default \
    priority=2 \
    max-limit=102400k

# FQ-CoDel for latency
/queue type add name=fq-codel-kind fq-codel \
    target=15ms \
    quantum=1514 \
    limit=10240
```

### 5.4 Huawei OLT Integration Example

```
# Create traffic profile
traffic-profile name "profile-100m" dba-index 1
  type4 max-bandwidth 102400

# Apply to ONT
ont traffic-profile 0 1 profile-100m
```

### 5.5 Database Tables

```sql
CREATE TABLE bandwidth_profiles (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    plan_id BIGINT REFERENCES plans(id),
    download_kbps INTEGER NOT NULL,
    upload_kbps INTEGER NOT NULL,
    burst_download_kbps INTEGER,
    burst_upload_kbps INTEGER,
    burst_duration_seconds INTEGER DEFAULT 30,
    priority INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT TRUE,
    -- Checker/Maker workflow
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Bandwidth profiles history table
CREATE TABLE bandwidth_profiles_history (
    id BIGSERIAL PRIMARY KEY,
    profile_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE bandwidth_applications (
    id BIGSERIAL PRIMARY KEY,
    profile_id BIGINT NOT NULL REFERENCES bandwidth_profiles(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'applying', 'applied', 'failed')),
    applied_at TIMESTAMPTZ,
    failed_reason TEXT,
    retry_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE bandwidth_usage (
    id BIGSERIAL PRIMARY KEY,
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    download_bytes BIGINT DEFAULT 0,
    upload_bytes BIGINT DEFAULT 0,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (recorded_at);
```

### 5.6 Events

```yaml
bandwidth.profile.created:
  payload:
    profile_id: uuid
    plan_id: uuid
    download_kbps: integer
    upload_kbps: integer

bandwidth.profile.updated:
  payload:
    profile_id: uuid
    changes: object
    affected_subscriptions: integer

bandwidth.profile.applied:
  payload:
    profile_id: uuid
    subscription_id: uuid
    device_id: uuid
    applied_at: timestamp

bandwidth.profile.failed:
  payload:
    profile_id: uuid
    subscription_id: uuid
    device_id: uuid
    error: string
    retry_count: integer
```

### 5.7 Failure Handling

| Failure Type | Strategy |
|-------------|----------|
| Device unreachable | Retry 3x with exponential backoff (10s, 30s, 90s) |
| Command rejected | Log error, alert NOC, manual intervention required |
| Partial application | Rollback to previous profile, alert NOC |
| Device reboot needed | Queue reboot with 5-minute grace period |
| Profile conflict | Resolve by priority (highest priority wins) |

---

## 6. Hardware Device Management

### 6.1 Supported Device Types

| Type | Vendor Examples | Management Protocol | Use Case |
|------|-----------------|---------------------|----------|
| OLT | Huawei MA5683T, ZTE C300 | Telnet/SSH, SNMP, NETCONF | Fiber aggregation |
| ONT | Huawei HG8245H, ZTE F670L | TR-069, OMCI, SSH | Customer premises |
| Router | MikroTik RB760iGS, Cisco ISR | RouterOS API, SSH, SNMP | Distribution/core |
| Switch | MikroTik CRS, Cisco Catalyst | SNMP, SSH, API | Distribution/access |
| Access Point | Ubiquiti, TP-Link | SNMP, HTTP API | WiFi coverage |

### 6.2 Device Registration

```json
{
  "device_id": "uuid",
  "name": "Jalgaon-CityCenter-OLT-01",
  "type": "olt",
  "vendor": "huawei",
  "model": "MA5683T",
  "serial_number": "HW-2100-OLT-001",
  "firmware_version": "V800R017C10",
  "management_ip": "10.0.0.1",
  "management_port": 23,
  "snmp_community": "encrypted_community",
  "location": {
    "city": "jalgaon",
    "area": "city-center",
    "latitude": 21.0077,
    "longitude": 75.5626,
    "address": "Jalgaon City Center Data Center"
  },
  "status": "online",
  "health_score": 95,
  "last_seen_at": "2026-07-08T14:30:00Z"
}
```

### 6.3 Health Monitoring

Metrics collected per device:

| Metric | Threshold (Critical) | Threshold (Warning) | Collection Method |
|--------|---------------------|---------------------|-------------------|
| CPU Usage | > 90% | > 70% | SNMP |
| Memory Usage | > 90% | > 80% | SNMP |
| Uplink Status | Down | Flapping | SNMP/ICMP |
| Temperature | > 70°C | > 60°C | SNMP |
| ONT Optical Power | < -28 dBm | < -25 dBm | OMCI |
| Packet Loss | > 5% | > 1% | SNMP counters |
| Latency | > 50ms | > 20ms | ICMP probe |
| Bandwidth Utilization | > 95% | > 80% | SNMP counters |

### 6.4 Database Tables

```sql
CREATE TABLE device_models (
    id BIGSERIAL PRIMARY KEY,
    vendor VARCHAR(100) NOT NULL,
    model VARCHAR(100) NOT NULL,
    device_type VARCHAR(50) NOT NULL,
    management_protocol VARCHAR(50) NOT NULL,
    default_port INTEGER,
    firmware_versions TEXT[],
    specs JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(vendor, model)
);

CREATE TABLE network_devices (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    device_model_id BIGINT NOT NULL REFERENCES device_models(id),
    serial_number VARCHAR(255) NOT NULL UNIQUE,
    management_ip INET NOT NULL,
    management_port INTEGER DEFAULT 22,
    snmp_community_encrypted TEXT,
    ssh_key_id UUID,
    firmware_version VARCHAR(50),
    firmware_update_available VARCHAR(50),
    status VARCHAR(20) DEFAULT 'offline'
        CHECK (status IN ('online', 'offline', 'degraded', 'maintenance', 'decommissioned')),
    health_score INTEGER DEFAULT 0,
    location_city VARCHAR(100),
    location_area VARCHAR(100),
    location_address TEXT,
    location_latitude DECIMAL(10, 7),
    location_longitude DECIMAL(10, 7),
    parent_device_id BIGINT REFERENCES network_devices(id),
    -- Checker/Maker workflow
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Network devices history table
CREATE TABLE network_devices_history (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE device_ports (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id) ON DELETE CASCADE,
    port_number INTEGER NOT NULL,
    port_name VARCHAR(50),
    port_type VARCHAR(50),
    speed_mbps INTEGER,
    status VARCHAR(20) DEFAULT 'down'
        CHECK (status IN ('up', 'down', 'disabled')),
    connected_device_id BIGINT REFERENCES network_devices(id),
    connected_port_id UUID,
    customer_id BIGINT REFERENCES customers(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(device_id, port_number)
);

CREATE TABLE device_logs (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    level VARCHAR(10) NOT NULL CHECK (level IN ('info', 'warning', 'error', 'critical')),
    message TEXT NOT NULL,
    source VARCHAR(50),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

CREATE TABLE device_metrics (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(15,4) NOT NULL,
    unit VARCHAR(20),
    recorded_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (recorded_at);

CREATE TABLE firmware_updates (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES network_devices(id),
    from_version VARCHAR(50),
    to_version VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'downloading', 'installing', 'completed', 'failed', 'rolled_back')),
    initiated_by BIGINT REFERENCES users(id),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    failure_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 6.5 Device Control Permission Model

| Operation | Required Role | Approval Required |
|-----------|---------------|-------------------|
| View device | noc_engineer+ | No |
| Restart device | noc_engineer+ | No |
| Shutdown device | network_admin+ | Yes |
| Configure device | network_admin+ | Yes |
| Update firmware | network_admin+ | Yes (isp_owner) |
| Register device | network_admin+ | No |
| Decommission device | network_admin+ | Yes (isp_owner) |
| Enable/disable port | noc_engineer+ | No |

### 6.6 Plug-and-Play Device Detection

The platform supports **automatic network device discovery** — when a new OLT, ONT, router, switch, or access point is connected to the network, it is automatically detected, fingerprinted, and registered without manual configuration.

#### 6.6.1 Discovery Architecture

```
Network Device Connected to Port
    ↓
Discovery Engine (Background Service)
    ├── SNMP Walk (sysDescr, sysObjectID, sysName, sysContact)
    ├── LLDP Neighbor Discovery
    ├── CDP Neighbor Discovery (Cisco)
    ├── ARP Table Scanning
    ├── MAC Address Table Learning
    ├── PON Port Scanning (OLT → ONT discovery)
    ├── DHCP Lease Table Scanning
    └── IP Range ICMP Sweep
    ↓
Device Fingerprinting
    ├── Vendor Identification (OUI lookup + sysObjectID)
    ├── Model Detection (sysDescr parsing)
    ├── Firmware Version Extraction
    ├── Port Count & Speed Detection
    ├── Capability Detection (routing, switching, wireless)
    └── Management Protocol Detection (SNMP, SSH, API)
    ↓
Auto-Registration
    ├── Match against known device_models table
    ├── Create network_devices entry
    ├── Assign management credentials (from pool)
    ├── Map to parent device (via LLDP/CDP)
    ├── Assign to city/area (via subnet mapping)
    ├── Publish device.discovered event
    └── Alert NOC engineer for approval
```

#### 6.6.2 Discovery Protocols

| Protocol | Method | Data Collected | Frequency |
|----------|--------|----------------|-----------|
| **SNMP Walk** | Walk OID tree (IF-MIB, ENTITY-MIF, BRIDGE-MIB) | sysDescr, sysObjectID, sysName, interfaces, MAC tables, uptime | Every 15 minutes |
| **LLDP** | IEEE 802.1AB neighbor discovery | Neighbor device ID, port, chassis ID, management IP | Every 60 seconds |
| **CDP** | Cisco proprietary neighbor discovery | Neighbor device ID, port, platform, management IP | Every 60 seconds |
| **ARP Scan** | Send ARP requests across subnet | IP-to-MAC mappings, host discovery | Every 5 minutes |
| **MAC Table** | Read bridge/MAC address tables | Port-to-MAC mappings, device connectivity | Every 5 minutes |
| **PON Scan** | Query OLT for ONT list via OMCI/SNMP | ONT serial, distance, optical power, status | Every 2 minutes |
| **DHCP Scan** | Read DHCP lease tables | Client hostname, MAC, IP, lease time | Every 5 minutes |
| **ICMP Sweep** | Ping sweep of IP ranges | Live hosts, response time, TTL | Every 10 minutes |

#### 6.6.3 Device Fingerprinting

**SNMP OID-based identification:**

| OID | Description | Used For |
|-----|-------------|----------|
| `1.3.6.1.2.1.1.1.0` | sysDescr | Full device description, model parsing |
| `1.3.6.1.2.1.1.2.0` | sysObjectID | Vendor/model identification (IANA private enterprise) |
| `1.3.6.1.2.1.1.5.0` | sysName | Device hostname |
| `1.3.6.1.2.1.2.1.0` | ifNumber | Port count |
| `1.3.6.1.2.1.2.2.1.1` | ifIndex | Interface index |
| `1.3.6.1.2.1.2.2.1.2` | ifDescr | Interface description |
| `1.3.6.1.2.1.2.2.1.6` | ifPhysAddress | MAC address |
| `1.3.6.1.2.1.2.2.1.10` | ifHCInOctets | Bytes received |
| `1.3.6.1.2.1.2.2.1.16` | ifHCOutOctets | Bytes sent |

**Vendor identification via IANA Private Enterprise Numbers:**

| Enterprise Number | Vendor | Common Models |
|------------------|--------|---------------|
| 2011 | Huawei | MA5683T, HG8245H, EG8247H |
| 4881 | ZTE | C300, F670L, ZXHN H108N |
| 14988 | MikroTik | RB760iGS, CCR1036, CRS326 |
| 9 | Cisco | ISR, Catalyst, Meraki |
| 4370 | TP-Link | Archer C6, EAP245 |
| 13014 | Ubiquiti | UniFi AP, EdgeRouter |
| 14823 | D-Link | DAP-2610, DGS-1100 |
| 1186 | Netgear | GS308, WAX220 |

#### 6.6.4 Auto-Registration Flow

```
1. Device connected to network port
2. Discovery engine detects new MAC address
3. SNMP walk performed on new IP
4. sysDescr + sysObjectID extracted
5. Vendor OUI lookup → vendor identified
6. sysDescr regex parsing → model identified
7. Match against device_models table
   ├── Match found → use existing model definition
   └── No match → create new device_model entry (pending approval)
8. Check if device already exists (by serial_number)
   ├── Yes → update last_seen_at, check for changes
   └── No → proceed to registration
9. Create network_devices entry with:
   - name: sysName (or auto-generated)
   - device_model_id: matched model
   - serial_number: from SNMP or MAC-based
   - management_ip: discovered IP
   - status: 'online'
   - health_score: calculated from initial SNMP data
10. Discover connected neighbors (LLDP/CDP)
11. Map to parent device (if found)
12. Assign to city/area (via IP subnet → location mapping)
13. Publish device.discovered event
14. Send notification to NOC engineer
15. NOC engineer reviews and approves/rejects
```

#### 6.6.5 Discovery Scan Configuration

```sql
CREATE TABLE discovery_scans (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    scan_type VARCHAR(30) NOT NULL
        CHECK (scan_type IN ('snmp_walk', 'lldp', 'cdp', 'arp_scan',
                            'mac_table', 'pon_scan', 'dhcp_scan', 'icmp_sweep')),
    target_subnets CIDR[],
    target_devices BIGINT[],
    snmp_community_id BIGINT,
    scan_interval_seconds INTEGER DEFAULT 900,
    is_active BOOLEAN DEFAULT TRUE,
    last_scan_at TIMESTAMPTZ,
    next_scan_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE discovery_results (
    id BIGSERIAL PRIMARY KEY,
    scan_id BIGINT NOT NULL REFERENCES discovery_scans(id),
    discovered_ip INET NOT NULL,
    discovered_mac MACADDR,
    sys_descr TEXT,
    sys_object_id VARCHAR(255),
    sys_name VARCHAR(255),
    sys_uptime INTERVAL,
    vendor VARCHAR(100),
    vendor_enterprise_id INTEGER,
    model VARCHAR(100),
    firmware_version VARCHAR(50),
    port_count INTEGER,
    management_protocol VARCHAR(50),
    capabilities VARCHAR(100)[],
    lldp_neighbors JSONB,
    cdp_neighbors JSONB,
    arp_entries JSONB,
    mac_table_entries JSONB,
    matched_model_id BIGINT REFERENCES device_models(id),
    matched_device_id BIGINT REFERENCES network_devices(id),
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'auto_registered', 'manual_review',
                          'approved', 'rejected', 'duplicate')),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    rejection_reason TEXT,
    raw_snmp_data JSONB,
    discovered_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE discovery_scan_history (
    id BIGSERIAL PRIMARY KEY,
    scan_id BIGINT NOT NULL REFERENCES discovery_scans(id),
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);
```

#### 6.6.6 Location Mapping

Subnets are mapped to physical locations for automatic area assignment:

```sql
CREATE TABLE subnet_location_map (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    subnet CIDR NOT NULL UNIQUE,
    city VARCHAR(100) NOT NULL,
    area VARCHAR(100),
    location_latitude DECIMAL(10, 7),
    location_longitude DECIMAL(10, 7),
    location_address TEXT,
    vlan_id INTEGER,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Example mapping:**
| Subnet | City | Area | VLAN |
|--------|------|------|------|
| 10.0.0.0/24 | Jalgaon | Data Center | 100 |
| 10.10.0.0/16 | Jalgaon | City Center | 200 |
| 10.20.0.0/16 | Jalgaon | MIDC Area | 300 |
| 10.30.0.0/16 | Bhusawal | — | 200 |

#### 6.6.7 PON Port Auto-Discovery

For OLT devices, the system automatically discovers ONTs connected to each PON port:

```
1. OLT registered in platform
2. Discovery engine queries OLT PON ports via SNMP/CLI
3. For each PON port:
   a. Query ONT list (Huawei: display ont info 0 all)
   b. For each ONT:
      - Serial number (from OMCI)
      - Distance from OLT (round-trip time)
      - Optical power (RX/TX dBm)
      - Status (online/offline/unknown)
      - Model (if reported)
      - Connected MAC address
4. Match ONT serial against existing network_devices
5. If new ONT → auto-register with parent_device_id = OLT
6. If existing ONT → update metrics and status
7. Map ONT to customer (via PPPoE username or MAC binding)
8. Publish device.ont.discovered event
```

**OLT-specific SNMP OIDs (Huawei):**
| OID | Description |
|-----|-------------|
| `1.3.6.1.4.1.2011.5.25.24.1.1.1.1.3` | ONT Index |
| `1.3.6.1.4.1.2011.5.25.24.1.1.1.1.5` | ONT SN |
| `1.3.6.1.4.1.2011.5.25.24.1.1.1.1.7` | ONT Distance |
| `1.3.6.1.4.1.2011.5.25.24.1.1.1.1.11` | ONT RX Power |
| `1.3.6.1.4.1.2011.5.25.24.1.1.1.1.12` | ONT TX Power |
| `1.3.6.1.4.1.2011.5.25.24.1.1.1.1.8` | ONT Status |

#### 6.6.8 Auto-Configuration on Discovery

When a device is auto-discovered, the platform can automatically apply initial configuration:

| Device Type | Auto-Configuration |
|-------------|-------------------|
| **ONT** | Apply speed profile from customer subscription, configure VLAN, set QoS |
| **Router** | Apply bandwidth limits via RouterOS API, configure customer queues |
| **Switch** | Configure VLAN ports, enable LLDP, set port security |
| **Access Point** | Configure SSID, set power limits, enable monitoring |
| **OLT** | Register PON ports, start ONT discovery scan, apply traffic profiles |

#### 6.6.9 Device Discovery Events

```yaml
device.discovered:
  payload:
    discovery_result_id: bigint
    discovered_ip: string
    discovered_mac: string
    vendor: string
    model: string
    sys_name: string
    matched_model_id: bigint | null
    matched_device_id: bigint | null
    auto_registered: boolean
    lldp_neighbors: array
    discovered_at: timestamp

device.auto_registered:
  payload:
    device_id: bigint
    device_name: string
    device_type: string
    vendor: string
    model: string
    management_ip: string
    parent_device_id: bigint | null
    location_city: string
    location_area: string
    discovered_at: timestamp

device.rejected:
  payload:
    discovery_result_id: bigint
    discovered_ip: string
    reason: string
    rejected_by: bigint

device.ont.discovered:
  payload:
    olt_device_id: bigint
    ont_serial: string
    pon_port: integer
    ont_distance_meters: integer
    ont_rx_power_dbm: decimal
    ont_tx_power_dbm: decimal
    ont_status: string
    customer_id: bigint | null
    subscription_id: bigint | null
```

#### 6.6.10 Discovery Permissions

```
discovery.scan.view
discovery.scan.create
discovery.scan.start
discovery.scan.stop
discovery.result.view
discovery.result.approve
discovery.result.reject
discovery.config.view
discovery.config.update
```

#### 6.6.11 Discovery Dashboard

The NOC dashboard shows a real-time view of device discovery:

- **New Devices Detected** (pending approval count)
- **Auto-Registered Today** (count)
- **Rejected Today** (count)
- **Discovery Scan Status** (last scan time, next scan time, active scans)
- **Device Map** (geographic view of all discovered devices)
- **Recent Discoveries** (list with approve/reject actions)
- **Duplicate Detection** (alerts for devices seen on multiple subnets)
- **Anomaly Detection** (unknown devices, MAC spoofing alerts)

#### 6.6.12 Security Considerations

| Threat | Mitigation |
|--------|------------|
| Rogue device connected | Unknown devices flagged for manual review |
| MAC spoofing | Cross-reference MAC with known OUI database |
| SNMP brute force | Rate limit SNMP community attempts, use SNMPv3 |
| Unauthorized management access | Restrict management VLAN, use ACLs |
| Device impersonation | Validate sysObjectID against known signatures |
| ARP spoofing | Enable DHCP snooping + dynamic ARP inspection |


### 6.7 Hardware Inventory Management

Tracks all physical equipment from procurement to disposal:

**Database:**
```sql
CREATE TABLE inventory_items (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    item_type VARCHAR(50) NOT NULL,
    device_model_id BIGINT REFERENCES device_models(id),
    serial_number VARCHAR(255) UNIQUE,
    barcode VARCHAR(100) UNIQUE,
    purchase_date DATE,
    purchase_price DECIMAL(10,2),
    warranty_expiry DATE,
    supplier VARCHAR(255),
    status VARCHAR(30) DEFAULT 'in_stock'
        CHECK (status IN ('in_stock', 'assigned', 'installed', 'returned',
                          'damaged', 'scrapped', 'in_transit')),
    assigned_to BIGINT REFERENCES users(id),
    assigned_to_branch_id BIGINT REFERENCES branches(id),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE inventory_movements (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGINT NOT NULL REFERENCES inventory_items(id),
    movement_type VARCHAR(30) NOT NULL
        CHECK (movement_type IN ('received', 'assigned', 'installed',
                                 'returned', 'transferred', 'scrapped')),
    from_branch_id BIGINT REFERENCES branches(id),
    to_branch_id BIGINT REFERENCES branches(id),
    reference_type VARCHAR(50),
    reference_id BIGINT,
    performed_by BIGINT NOT NULL REFERENCES users(id),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Permissions:**
```
inventory.view
inventory.receive
inventory.assign
inventory.transfer
inventory.scrapp
inventory.report
```

---

## 7. Network Management Module

### 7.1 Network Topology

```
Internet Upstream (Tier 1/2 ISP)
    ↓
Core Router (MikroTik CCR / Cisco ASR)
    ↓
Distribution Switch (MikroTik CRS / Cisco Catalyst)
    ↓
OLT (Huawei MA5683T / ZTE C300)
    ↓ (Fiber split 1:32 / 1:64)
Splitter
    ↓
ONT (Huawei HG8245H / ZTE F670L)
    ↓ (Ethernet / WiFi)
Customer Premises
```

### 7.2 VLAN Management

| VLAN ID Range | Purpose | Example |
|---------------|---------|---------|
| 100–199 | Management | VLAN 100 — OLT management |
| 200–299 | Customer Data (Residential) | VLAN 200 — Jalgaon City Center |
| 300–399 | Customer Data (Business) | VLAN 300 — MIDC Area |
| 400–499 | IPTV/Multicast | VLAN 400 — IPTV |
| 500–599 | VoIP | VLAN 500 — SIP Trunk |
| 900–999 | Monitoring/SNMP | VLAN 900 — SNMP |

### 7.3 IP Pool Management

```json
{
  "pool_id": "uuid",
  "name": "Jalgaon-CityCenter-Pool",
  "cidr": "10.10.0.0/16",
  "gateway": "10.10.0.1",
  "dns_primary": "1.1.1.1",
  "dns_secondary": "8.8.8.8",
  "dhcp_range_start": "10.10.1.1",
  "dhcp_range_end": "10.10.254.254",
  "vlan_id": 200,
  "allocated_count": 1250,
  "total_count": 65534,
  "utilization_percent": 1.9,
  "status": "healthy"
}
```

### 7.4 PPPoE Management

```json
{
  "session_id": "uuid",
  "customer_id": "uuid",
  "subscription_id": "uuid",
  "username": "rahul@aeroxe",
  "password": "encrypted_password",
  "pppoe_server_ip": "10.10.0.1",
  "assigned_ip": "10.10.1.100",
  "session_start": "2026-07-08T06:00:00Z",
  "session_duration_seconds": 28800,
  "bytes_in": 1073741824,
  "bytes_out": 536870912,
  "status": "active",
  "device_id": "uuid-of-mikrotik",
  "nas_port_id": "pppoe-0/1/2"
}
```

### 7.5 DHCP Management

```json
{
  "lease_id": "uuid",
  "mac_address": "AA:BB:CC:DD:EE:FF",
  "ip_address": "10.10.1.100",
  "hostname": "customer-pc",
  "vlan_id": 200,
  "pool_id": "uuid",
  "lease_start": "2026-07-08T06:00:00Z",
  "lease_end": "2026-07-08T18:00:00Z",
  "lease_type": "dynamic",
  "client_id": "uuid"
}
```

### 7.6 MAC Binding

```json
{
  "binding_id": "uuid",
  "customer_id": "uuid",
  "subscription_id": "uuid",
  "mac_address": "AA:BB:CC:DD:EE:FF",
  "assigned_ip": "10.10.1.100",
  "vlan_id": 200,
  "bound_at": "2026-07-08T11:30:00Z",
  "bound_by": "technician-uuid",
  "is_active": true
}
```

### 7.7 Customer Session Tracking

```json
{
  "session_id": "uuid",
  "customer_id": "uuid",
  "subscription_id": "uuid",
  "pppoe_session_id": "uuid",
  "dhcp_lease_id": "uuid",
  "mac_address": "AA:BB:CC:DD:EE:FF",
  "ip_address": "10.10.1.100",
  "device_id": "uuid-of-ont",
  "port_id": "uuid-of-port",
  "vlan_id": 200,
  "connected_at": "2026-07-08T06:00:00Z",
  "last_activity_at": "2026-07-08T14:30:00Z",
  "bytes_in": 1073741824,
  "bytes_out": 536870912,
  "is_online": true,
  "latency_ms": 8.5,
  "packet_loss_percent": 0.1
}
```

### 7.8 Online/Offline Status

Status is determined by:

1. **PPPoE session active** → Online
2. **DHCP lease active** → Online
3. **SNMP heartbeat within 5 minutes** → Online
4. **ONT OMCI link up** → Online
5. **None of above** → Offline

Status updates are published in real-time via WebSocket to the NOC dashboard.

### 7.9 Database Tables

```sql
CREATE TABLE vlans (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    vlan_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    vlan_type VARCHAR(30) NOT NULL
        CHECK (vlan_type IN ('management', 'customer_residential', 'customer_business',
                             'iptv', 'voip', 'monitoring')),
    is_active BOOLEAN DEFAULT TRUE,
    created_by BIGINT REFERENCES users(id),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, vlan_id)
);

CREATE TABLE ip_pools (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(100) NOT NULL,
    cidr CIDR NOT NULL,
    gateway INET NOT NULL,
    dns_primary INET DEFAULT '1.1.1.1',
    dns_secondary INET DEFAULT '8.8.8.8',
    dhcp_range_start INET,
    dhcp_range_end INET,
    vlan_id BIGINT REFERENCES vlans(id),
    pool_type VARCHAR(30) DEFAULT 'customer'
        CHECK (pool_type IN ('customer', 'management', 'shared_services')),
    allocated_count INTEGER DEFAULT 0,
    total_count INTEGER NOT NULL,
    utilization_percent DECIMAL(5,2) GENERATED ALWAYS AS
        (CASE WHEN total_count > 0 THEN (allocated_count::DECIMAL / total_count) * 100 ELSE 0 END) STORED,
    status VARCHAR(20) DEFAULT 'healthy'
        CHECK (status IN ('healthy', 'warning', 'critical', 'exhausted')),
    warning_threshold_percent DECIMAL(5,2) DEFAULT 80.0,
    critical_threshold_percent DECIMAL(5,2) DEFAULT 95.0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, cidr)
);

CREATE TABLE ip_addresses (
    id BIGSERIAL PRIMARY KEY,
    ip_pool_id BIGINT NOT NULL REFERENCES ip_pools(id),
    ip_address INET NOT NULL UNIQUE,
    status VARCHAR(20) DEFAULT 'available'
        CHECK (status IN ('available', 'allocated', 'reserved', 'excluded')),
    allocated_to_type VARCHAR(50),
    allocated_to_id BIGINT,
    allocated_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE pppoe_sessions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    username VARCHAR(100) NOT NULL UNIQUE,
    password_encrypted VARCHAR(255) NOT NULL,
    pppoe_server_ip INET,
    assigned_ip INET,
    nas_port_id VARCHAR(100),
    nas_ip_address INET,
    nas_session_id VARCHAR(100),
    session_start TIMESTAMPTZ,
    session_duration_seconds BIGINT DEFAULT 0,
    bytes_in BIGINT DEFAULT 0,
    bytes_out BIGINT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'inactive'
        CHECK (status IN ('active', 'inactive', 'terminated')),
    device_id BIGINT REFERENCES network_devices(id),
    last_activity_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE dhcp_leases (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    mac_address MACADDR NOT NULL,
    ip_address INET NOT NULL,
    hostname VARCHAR(255),
    vlan_id BIGINT REFERENCES vlans(id),
    ip_pool_id BIGINT NOT NULL REFERENCES ip_pools(id),
    lease_start TIMESTAMPTZ NOT NULL,
    lease_end TIMESTAMPTZ NOT NULL,
    lease_type VARCHAR(20) DEFAULT 'dynamic'
        CHECK (lease_type IN ('dynamic', 'static', 'reserved')),
    client_id VARCHAR(255),
    customer_id BIGINT REFERENCES customers(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    device_id BIGINT REFERENCES network_devices(id),
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'expired', 'released')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE mac_bindings (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    mac_address MACADDR NOT NULL,
    assigned_ip INET NOT NULL,
    vlan_id BIGINT REFERENCES vlans(id),
    bound_at TIMESTAMPTZ DEFAULT NOW(),
    bound_by BIGINT REFERENCES users(id),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, mac_address)
);

CREATE TABLE customer_sessions (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    pppoe_session_id BIGINT REFERENCES pppoe_sessions(id),
    dhcp_lease_id BIGINT REFERENCES dhcp_leases(id),
    mac_address MACADDR NOT NULL,
    ip_address INET NOT NULL,
    device_id BIGINT REFERENCES network_devices(id),
    port_id BIGINT REFERENCES device_ports(id),
    vlan_id BIGINT REFERENCES vlans(id),
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    disconnected_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    bytes_in BIGINT DEFAULT 0,
    bytes_out BIGINT DEFAULT 0,
    is_online BOOLEAN DEFAULT TRUE,
    latency_ms DECIMAL(7,2),
    packet_loss_percent DECIMAL(5,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Indexes
CREATE INDEX idx_ip_pools_branch ON ip_pools(branch_id);
CREATE INDEX idx_ip_addresses_pool ON ip_addresses(ip_pool_id);
CREATE INDEX idx_ip_addresses_status ON ip_addresses(status);
CREATE INDEX idx_pppoe_sessions_branch ON pppoe_sessions(branch_id);
CREATE INDEX idx_pppoe_sessions_customer ON pppoe_sessions(customer_id);
CREATE INDEX idx_pppoe_sessions_status ON pppoe_sessions(status);
CREATE INDEX idx_dhcp_leases_branch ON dhcp_leases(branch_id);
CREATE INDEX idx_dhcp_leases_mac ON dhcp_leases(mac_address);
CREATE INDEX idx_dhcp_leases_ip ON dhcp_leases(ip_address);
CREATE INDEX idx_mac_bindings_branch ON mac_bindings(branch_id);
CREATE INDEX idx_mac_bindings_customer ON mac_bindings(customer_id);
CREATE INDEX idx_mac_bindings_mac ON mac_bindings(mac_address);
CREATE INDEX idx_customer_sessions_branch ON customer_sessions(branch_id);
CREATE INDEX idx_customer_sessions_customer ON customer_sessions(customer_id);
CREATE INDEX idx_customer_sessions_online ON customer_sessions(is_online);
```

CREATE TABLE vlans_history (
    id BIGSERIAL PRIMARY KEY,
    vlan_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE ip_pools_history (
    id BIGSERIAL PRIMARY KEY,
    pool_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE pppoe_sessions_history (
    id BIGSERIAL PRIMARY KEY,
    session_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

### 7.10 Events

```yaml
vlan.created:
  payload:
    vlan_id: bigint
    branch_id: bigint
    vlan_tag: integer
    vlan_type: string

vlan.deleted:
  payload:
    vlan_id: bigint
    branch_id: bigint

ippool.exhausted:
  payload:
    pool_id: bigint
    branch_id: bigint
    utilization_percent: decimal

ippool.warning:
  payload:
    pool_id: bigint
    branch_id: bigint
    utilization_percent: decimal

pppoe.session.started:
  payload:
    session_id: bigint
    customer_id: bigint
    assigned_ip: string
    nas_port_id: string

pppoe.session.ended:
  payload:
    session_id: bigint
    customer_id: bigint
    duration_seconds: bigint
    bytes_in: bigint
    bytes_out: bigint

customer.session.connected:
  payload:
    session_id: bigint
    customer_id: bigint
    ip_address: string
    mac_address: string

customer.session.disconnected:
  payload:
    session_id: bigint
    customer_id: bigint
    reason: string
```

---

## 7A. Support Ticketing System

### 7A.1 Ticket Lifecycle

```
open → assigned → in_progress → waiting_customer → resolved → closed
                                                          ↗
                                   escalated ↗
```

**State transitions:**

| From | To | Trigger | Event |
|------|----|---------|-------|
| — | open | Customer creates ticket | `ticket.created` |
| open | assigned | Support agent assigned | `ticket.assigned` |
| assigned | in_progress | Agent starts working | `ticket.in_progress` |
| in_progress | waiting_customer | Agent needs customer info | `ticket.waiting_customer` |
| waiting_customer | in_progress | Customer responds | `ticket.customer_responded` |
| in_progress | resolved | Issue fixed | `ticket.resolved` |
| resolved | closed | Auto-closed after 7 days or customer confirms | `ticket.closed` |
| any | escalated | Escalated to higher tier | `ticket.escalated` |
| closed | open | Customer reopens | `ticket.reopened` |

### 7A.2 Ticket Priority & SLA

| Priority | Response SLA | Resolution SLA | Example |
|----------|-------------|----------------|---------|
| **critical** | 15 minutes | 2 hours | Total outage, all customers affected |
| **high** | 30 minutes | 4 hours | Partial outage, business customer down |
| **medium** | 2 hours | 24 hours | Intermittent issues, speed complaints |
| **low** | 8 hours | 72 hours | General queries, billing questions |

### 7A.3 Ticket Categories

| Category | Subcategories |
|----------|---------------|
| **connectivity** | No internet, slow speed, intermittent, dns_resolution |
| **installation** | New installation, relocation, disconnection, router_issue |
| **billing** | Payment issue, invoice_query, refund_request, plan_change |
| **hardware** | Router_replacement, ONT_issue, cable_damage, fiber_cut |
| **account** | KYC_update, password_reset, profile_change, suspension_query |
| **other** | General inquiry, feedback, complaint |

### 7A.4 Database Tables

```sql
CREATE TABLE tickets (
    id BIGSERIAL PRIMARY KEY,
    ticket_number VARCHAR(20) NOT NULL UNIQUE,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT REFERENCES customers(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    created_by BIGINT NOT NULL REFERENCES users(id),
    assigned_to BIGINT REFERENCES users(id),
    escalated_to BIGINT REFERENCES users(id),
    category VARCHAR(50) NOT NULL,
    subcategory VARCHAR(50),
    priority VARCHAR(10) DEFAULT 'medium'
        CHECK (priority IN ('critical', 'high', 'medium', 'low')),
    status VARCHAR(30) DEFAULT 'open'
        CHECK (status IN ('open', 'assigned', 'in_progress', 'waiting_customer',
                          'escalated', 'resolved', 'closed', 'reopened')),
    subject VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    source VARCHAR(20) DEFAULT 'customer'
        CHECK (source IN ('customer', 'phone', 'email', 'whatsapp', 'agent', 'system')),
    resolution_notes TEXT,
    sla_response_at TIMESTAMPTZ,
    sla_resolution_at TIMESTAMPTZ,
    first_response_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    reopen_count INTEGER DEFAULT 0,
    satisfaction_rating INTEGER CHECK (satisfaction_rating BETWEEN 1 AND 5),
    satisfaction_feedback TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_comments (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id),
    is_customer BOOLEAN DEFAULT FALSE,
    comment TEXT NOT NULL,
    is_internal BOOLEAN DEFAULT FALSE,
    attachments JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_escalations (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id),
    from_user_id BIGINT NOT NULL REFERENCES users(id),
    to_user_id BIGINT NOT NULL REFERENCES users(id),
    from_priority VARCHAR(10),
    to_priority VARCHAR(10),
    reason TEXT NOT NULL,
    escalated_at TIMESTAMPTZ DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_attachments (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    comment_id BIGINT REFERENCES ticket_comments(id) ON DELETE SET NULL,
    document_id BIGINT NOT NULL REFERENCES document_files(id),
    uploaded_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_status_history (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    old_status VARCHAR(30),
    new_status VARCHAR(30) NOT NULL,
    changed_by BIGINT NOT NULL REFERENCES users(id),
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE tickets_history (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

-- Indexes
CREATE INDEX idx_tickets_branch ON tickets(branch_id);
CREATE INDEX idx_tickets_customer ON tickets(customer_id);
CREATE INDEX idx_tickets_assigned ON tickets(assigned_to);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_priority ON tickets(priority);
CREATE INDEX idx_tickets_category ON tickets(category);
CREATE INDEX idx_tickets_created ON tickets(created_at);
CREATE INDEX idx_ticket_comments_ticket ON ticket_comments(ticket_id);
CREATE INDEX idx_ticket_escalations_ticket ON ticket_escalations(ticket_id);
```

### 7A.5 Events

```yaml
ticket.created:
  payload:
    ticket_id: bigint
    ticket_number: string
    branch_id: bigint
    customer_id: bigint | null
    category: string
    priority: string
    subject: string
    source: string

ticket.assigned:
  payload:
    ticket_id: bigint
    assigned_to: bigint
    assigned_by: bigint

ticket.escalated:
  payload:
    ticket_id: bigint
    from_user_id: bigint
    to_user_id: bigint
    reason: string
    new_priority: string

ticket.resolved:
  payload:
    ticket_id: bigint
    resolved_by: bigint
    resolution_notes: string
    resolution_time_minutes: integer

ticket.reopened:
  payload:
    ticket_id: bigint
    reopen_count: integer
    reopened_by: bigint

sla.breach.warning:
  payload:
    ticket_id: bigint
    breach_type: "response" | "resolution"
    sla_at: timestamp
    current_time: timestamp
```

---

## 8. Billing System

### 8.1 Billing Lifecycle

```
subscription.created → invoice.generated → invoice.sent → payment.pending
→ payment.received → payment.processed → payment.completed
→ invoice.paid → subscription.renewed
```

**Dunning flow:**

```
invoice.paid (on due date)
→ payment.overdue (day 1)
→ payment.reminder.sent (day 3)
→ payment.reminder.sent (day 7)
→ subscription.suspended (day 10)
→ payment.received → subscription.reactivated
→ OR customer.terminated (day 30)
```

### 8.2 Invoice Structure

```json
{
  "invoice_id": "INV-2026-07-0001",
  "customer_id": "uuid",
  "subscription_id": "uuid",
  "plan_name": "Standard 100 Mbps",
  "billing_period": {
    "start": "2026-07-01",
    "end": "2026-07-31"
  },
  "line_items": [
    {
      "description": "Standard 100 Mbps — July 2026",
      "quantity": 1,
      "unit_price": 600.00,
      "amount": 600.00
    }
  ],
  "subtotal": 600.00,
  "discount": 0.00,
  "tax": {
    "cgst": { "rate": 9, "amount": 54.00 },
    "sgst": { "rate": 9, "amount": 54.00 }
  },
  "total_tax": 108.00,
  "total": 708.00,
  "currency": "INR",
  "status": "pending",
  "due_date": "2026-07-10",
  "paid_at": null,
  "payment_method": null,
  "payment_reference": null
}
```

### 8.3 Payment Methods

| Method | Gateway | Processing |
|--------|---------|------------|
| UPI (GPay, PhonePe, Paytm) | Razorpay / PayU | Instant |
| Net Banking | Razorpay / PayU | 1-2 hours |
| Cash | Manual | Immediate (manual entry) |
| Card (Debit/Credit) | Razorpay | Instant |
| Auto-debit | NACH / UPI AutoPay | Scheduled |

### 8.4 Discount System

```json
{
  "discount_id": "uuid",
  "name": "Annual Plan Discount",
  "type": "percentage",
  "value": 5,
  "applicable_to": "plan",
  "applicable_plan_ids": ["*"],
  "applicable_billing_periods": [12],
  "max_uses": null,
  "current_uses": 0,
  "valid_from": "2026-01-01",
  "valid_until": "2026-12-31",
  "is_stackable": false
}
```

### 8.5 Tax Configuration

```json
{
  "tax_config": {
    "cgst_rate": 9.0,
    "sgst_rate": 9.0,
    "igst_rate": 18.0,
    "applicable_state": "Maharashtra",
    "hsn_code": "998421",
    "sac_code": "998421",
    "tax_name": "GST on Internet Services"
  }
}
```

### 8.6 Database Tables

```sql
CREATE TABLE invoices (
    id BIGSERIAL PRIMARY KEY,
    invoice_number VARCHAR(20) NOT NULL UNIQUE,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    billing_period_start DATE NOT NULL,
    billing_period_end DATE NOT NULL,
    subtotal DECIMAL(10,2) NOT NULL,
    discount_amount DECIMAL(10,2) DEFAULT 0,
    tax_amount DECIMAL(10,2) DEFAULT 0,
    total_amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'INR',
    status VARCHAR(20) DEFAULT 'draft'
        CHECK (status IN ('draft', 'pending', 'sent', 'paid', 'partial',
                          'overdue', 'void', 'refunded')),
    due_date DATE NOT NULL,
    paid_at TIMESTAMPTZ,
    payment_method VARCHAR(50),
    payment_reference VARCHAR(255),
    -- Checker/Maker workflow
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE invoice_line_items (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity DECIMAL(10,2) DEFAULT 1,
    unit_price DECIMAL(10,2) NOT NULL,
    amount DECIMAL(10,2) NOT NULL,
    tax_rate DECIMAL(5,2) DEFAULT 0,
    tax_amount DECIMAL(10,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE payments (
    id BIGSERIAL PRIMARY KEY,
    payment_number VARCHAR(20) NOT NULL UNIQUE,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'INR',
    payment_method VARCHAR(50) NOT NULL,
    payment_gateway VARCHAR(50),
    gateway_transaction_id VARCHAR(255),
    gateway_response JSONB,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'refunded')),
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE refunds (
    id BIGSERIAL PRIMARY KEY,
    refund_number VARCHAR(20) NOT NULL UNIQUE,
    payment_id BIGINT NOT NULL REFERENCES payments(id),
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    amount DECIMAL(10,2) NOT NULL,
    reason TEXT NOT NULL,
    approved_by BIGINT REFERENCES users(id),
    -- Checker/Maker workflow
    requested_by BIGINT REFERENCES users(id),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    review_notes TEXT,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'processed', 'rejected')),
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE invoices_history (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE approval_requests_history (
    id BIGSERIAL PRIMARY KEY,
    approval_request_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

-- Refunds history table
CREATE TABLE refunds_history (
    id BIGSERIAL PRIMARY KEY,
    refund_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE discounts (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) UNIQUE,
    type VARCHAR(20) NOT NULL CHECK (type IN ('percentage', 'fixed')),
    value DECIMAL(10,2) NOT NULL,
    max_uses INTEGER,
    current_uses INTEGER DEFAULT 0,
    valid_from DATE NOT NULL,
    valid_until DATE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    -- Checker/Maker workflow
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Discounts history table
CREATE TABLE discounts_history (
    id BIGSERIAL PRIMARY KEY,
    discount_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE payment_reminders (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    reminder_type VARCHAR(20) NOT NULL,
    channel VARCHAR(20) NOT NULL,
    sent_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) DEFAULT 'sent'
        CHECK (status IN ('sent', 'delivered', 'failed')),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 8.7 Events

```yaml
invoice.generated:
  payload:
    invoice_id: uuid
    invoice_number: string
    customer_id: uuid
    total_amount: decimal
    due_date: date

invoice.paid:
  payload:
    invoice_id: uuid
    payment_id: uuid
    amount: decimal
    payment_method: string

invoice.overdue:
  payload:
    invoice_id: uuid
    days_overdue: integer
    total_amount: decimal

payment.failed:
  payload:
    invoice_id: uuid
    payment_id: uuid
    reason: string

subscription.suspended:
  payload:
    customer_id: uuid
    subscription_id: uuid
    reason: "payment_overdue"

subscription.reactivated:
  payload:
    customer_id: uuid
    subscription_id: uuid
```


### 8.8 Pro-Rata Billing

When a customer upgrades or downgrades mid-cycle, the system calculates pro-rata charges:

**Upgrade (e.g., Standard → Premium on day 15 of 30-day cycle):**
```
Remaining days = 30 - 15 = 15
Old plan daily rate = ₹600 / 30 = ₹20/day
New plan daily rate = ₹800 / 30 = ₹26.67/day
Credit for remaining old plan = 15 × ₹20 = ₹300
Charge for remaining new plan = 15 × ₹26.67 = ₹400
Pro-rata adjustment = ₹400 - ₹300 = ₹100 (additional charge)
```

**Downgrade (e.g., Premium → Standard on day 15):**
```
Credit for remaining old plan = 15 × ₹26.67 = ₹400
Charge for remaining new plan = 15 × ₹20 = ₹300
Pro-rata adjustment = ₹300 - ₹400 = -₹100 (credit to next invoice)
```

**Database:**
```sql
CREATE TABLE prorata_adjustments (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    invoice_id BIGINT REFERENCES invoices(id),
    old_plan_id BIGINT NOT NULL REFERENCES plans(id),
    new_plan_id BIGINT NOT NULL REFERENCES plans(id),
    change_date DATE NOT NULL,
    cycle_start DATE NOT NULL,
    cycle_end DATE NOT NULL,
    old_daily_rate DECIMAL(10,2) NOT NULL,
    new_daily_rate DECIMAL(10,2) NOT NULL,
    remaining_days INTEGER NOT NULL,
    credit_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    charge_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    net_adjustment DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'applied', 'refunded')),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Rules:**
- Pro-rata applies only to monthly billing cycles (not 3/6/12-month prepaid)
- Upgrades take effect immediately; pro-rata charged on next invoice
- Downgrades take effect at cycle end; credit applied to next invoice
- Minimum charge: ₹1 (no negative invoices)
- Pro-rata adjustments generate journal entries automatically


### 8.9 Late Fee Engine

Automatic penalty charges for overdue invoices:

**Configuration:**
```sql
CREATE TABLE late_fee_rules (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    grace_period_days INTEGER NOT NULL DEFAULT 3,
    fee_type VARCHAR(20) NOT NULL CHECK (fee_type IN ('percentage', 'fixed')),
    fee_value DECIMAL(10,2) NOT NULL,
    max_fee DECIMAL(10,2),
    compounding BOOLEAN DEFAULT FALSE,
    applies_to_plan_ids BIGINT[],
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE late_fee_applications (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    rule_id BIGINT NOT NULL REFERENCES late_fee_rules(id),
    amount DECIMAL(10,2) NOT NULL,
    days_overdue INTEGER NOT NULL,
    applied_at TIMESTAMPTZ DEFAULT NOW(),
    invoice_line_item_id BIGINT REFERENCES invoice_line_items(id)
);
```

**Example rules:**
| Rule | Grace Period | Fee | Max Fee | Compounding |
|------|-------------|-----|---------|-------------|
| Standard Late Fee | 3 days | 2% of invoice | ₹200 | No |
| Business Premium | 5 days | 1% of invoice | ₹500 | No |

**Flow:**
1. Dunning engine runs daily (cron job at 00:00 IST)
2. Finds invoices overdue beyond grace period
3. Applies late fee → creates invoice line item
4. Generates journal entry: Dr. Accounts Receivable, Cr. Late Payment Fees
5. Sends notification to customer
6. Total late fees capped at `max_fee` per invoice

---

## 8A. General Ledger & Double-Entry Accounting

### 8A.1 Accounting Overview

Every financial transaction in the platform is recorded using **double-entry bookkeeping**. Every debit has a corresponding credit. This ensures:
- Complete financial audit trail
- Accurate trial balance at any point
- GST-compliant records
- Easy reconciliation with bank statements
- Regulatory compliance

### 8A.2 Chart of Accounts

```sql
CREATE TABLE chart_of_accounts (
    id BIGSERIAL PRIMARY KEY,
    account_code VARCHAR(20) NOT NULL UNIQUE,
    account_name VARCHAR(255) NOT NULL,
    account_type VARCHAR(30) NOT NULL
        CHECK (account_type IN ('asset', 'liability', 'equity', 'revenue', 'expense')),
    parent_account_id BIGINT REFERENCES chart_of_accounts(id),
    is_group BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Default Chart of Accounts for ISP:**

| Code | Account Name | Type | Description |
|------|-------------|------|-------------|
| 1000 | Cash | Asset | Cash on hand |
| 1100 | Bank Account | Asset | Primary bank account |
| 1200 | Accounts Receivable | Asset | Customer invoices pending |
| 1300 | Prepaid Expenses | Asset | Advance payments received |
| 1400 | GST Input Credit | Asset | GST input tax credit |
| 2000 | Accounts Payable | Liability | Vendor payments due |
| 2100 | GST Output Tax | Liability | GST collected |
| 2200 | TDS Payable | Liability | Tax deducted at source |
| 3000 | Owner's Equity | Equity | Capital invested |
| 3100 | Retained Earnings | Equity | Accumulated profits |
| 4000 | Subscription Revenue | Revenue | Monthly plan revenue |
| 4100 | Installation Revenue | Revenue | One-time installation fees |
| 4200 | Hardware Revenue | Revenue | Router/ONT sales |
| 4300 | Late Payment Fees | Revenue | Penalty charges |
| 5000 | Bandwidth Cost | Expense | Upstream ISP charges |
| 5100 | Equipment Cost | Expense | ONT, router procurement |
| 5200 | Salary Expense | Expense | Employee salaries |
| 5300 | Rent Expense | Expense | Office/server room rent |
| 5400 | Electricity Expense | Expense | Power costs |
| 5500 | Maintenance Expense | Expense | Network maintenance |
| 5600 | Marketing Expense | Expense | Advertising costs |
| 5700 | Refund Expense | Expense | Customer refunds |

### 8A.3 Journal Entries

```sql
CREATE TABLE journal_entries (
    id BIGSERIAL PRIMARY KEY,
    entry_number VARCHAR(30) NOT NULL UNIQUE,
    entry_date DATE NOT NULL DEFAULT CURRENT_DATE,
    description TEXT NOT NULL,
    reference_type VARCHAR(50),
    reference_id BIGINT,
    total_debit DECIMAL(15,2) NOT NULL,
    total_credit DECIMAL(15,2) NOT NULL,
    status VARCHAR(20) DEFAULT 'draft'
        CHECK (status IN ('draft', 'posted', 'void')),
    -- Checker/Maker workflow
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    posted_by BIGINT REFERENCES users(id),
    posted_at TIMESTAMPTZ,
    voided_by BIGINT REFERENCES users(id),
    voided_at TIMESTAMPTZ,
    void_reason TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE journal_entry_lines (
    id BIGSERIAL PRIMARY KEY,
    journal_entry_id BIGINT NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id BIGINT NOT NULL REFERENCES chart_of_accounts(id),
    debit DECIMAL(15,2) DEFAULT 0,
    credit DECIMAL(15,2) DEFAULT 0,
    description TEXT,
    customer_id BIGINT REFERENCES customers(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    CHECK (debit >= 0 AND credit >= 0),
    CHECK (debit > 0 OR credit > 0)
);

-- Journal entries history
CREATE TABLE journal_entries_history (
    id BIGSERIAL PRIMARY KEY,
    journal_entry_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);
```

### 8A.4 Double-Entry Examples

**Invoice Generated (Subscription Revenue):**
```
Debit:  1200 Accounts Receivable    ₹708.00
Credit: 4000 Subscription Revenue   ₹600.00
Credit: 2100 GST Output Tax         ₹108.00
```

**Payment Received (UPI):**
```
Debit:  1100 Bank Account           ₹708.00
Credit: 1200 Accounts Receivable   ₹708.00
```

**Cash Payment:**
```
Debit:  1000 Cash                   ₹708.00
Credit: 1200 Accounts Receivable   ₹708.00
```

**Refund Issued:**
```
Debit:  5700 Refund Expense         ₹708.00
Credit: 1100 Bank Account          ₹708.00
```

**Upstream Bandwidth Cost (Monthly):**
```
Debit:  5000 Bandwidth Cost         ₹50,000.00
Credit: 1100 Bank Account          ₹50,000.00
```

### 8A.5 Trial Balance

```sql
CREATE TABLE trial_balance (
    id BIGSERIAL PRIMARY KEY,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    account_id BIGINT NOT NULL REFERENCES chart_of_accounts(id),
    opening_balance DECIMAL(15,2) DEFAULT 0,
    total_debit DECIMAL(15,2) DEFAULT 0,
    total_credit DECIMAL(15,2) DEFAULT 0,
    closing_balance DECIMAL(15,2) DEFAULT 0,
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(period_start, period_end, account_id)
);
```

### 8A.6 Accounting Integration with Billing

Every billing event automatically generates journal entries:

| Event | Journal Entry |
|-------|---------------|
| `invoice.generated` | Dr. Accounts Receivable, Cr. Subscription Revenue + GST Output |
| `invoice.paid` | Dr. Bank/Cash, Cr. Accounts Receivable |
| `invoice.overdue` | No entry (status update only) |
| `refund.approved` | Dr. Refund Expense, Cr. Bank Account |
| `payment.failed` | No entry (retry flow) |
| `subscription.upgraded` | Dr. Accounts Receivable (new), Cr. Subscription Revenue (new); Dr. Subscription Revenue (old), Cr. Accounts Receivable (old) |
| `late_fee.applied` | Dr. Accounts Receivable, Cr. Late Payment Fees |
| `pro_rata.upgrade` | Dr. Accounts Receivable, Cr. Subscription Revenue (pro-rata amount) |
| `pro_rata.downgrade` | Dr. Subscription Revenue (pro-rata credit), Cr. Accounts Receivable |
| `wallet.credited` | Dr. Bank Account, Cr. Prepaid Expenses |
| `wallet.applied` | Dr. Prepaid Expenses, Cr. Accounts Receivable |
| `referral.rewarded` | Dr. Marketing Expense, Cr. Prepaid Expenses |

### 8A.7 Permissions

```
accounting.journal_entry.view
accounting.journal_entry.create
accounting.journal_entry.post
accounting.journal_entry.void
accounting.journal_entry.approve
accounting.chart_of_accounts.view
accounting.chart_of_accounts.create
accounting.chart_of_accounts.update
accounting.trial_balance.view
accounting.trial_balance.generate
accounting.reports.view
accounting.reports.export
```


### 8A.8 GST Filing Data Generation

The system generates GST-compliant reports for monthly filing:

**GSTR-1 Data (Outward Supplies):**
```sql
CREATE TABLE gst_returns (
    id BIGSERIAL PRIMARY KEY,
    return_type VARCHAR(10) NOT NULL CHECK (return_type IN ('GSTR1', 'GSTR3B', 'GSTR9')),
    filing_period VARCHAR(7) NOT NULL,  -- YYYY-MM
    branch_id BIGINT REFERENCES branches(id),
    gstin VARCHAR(15) NOT NULL,
    total_taxable_value DECIMAL(15,2) DEFAULT 0,
    total_igst DECIMAL(15,2) DEFAULT 0,
    total_cgst DECIMAL(15,2) DEFAULT 0,
    total_sgst DECIMAL(15,2) DEFAULT 0,
    total_cess DECIMAL(15,2) DEFAULT 0,
    invoice_count INTEGER DEFAULT 0,
    hsn_summary JSONB,
    status VARCHAR(20) DEFAULT 'draft'
        CHECK (status IN ('draft', 'generated', 'filed')),
    filed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Auto-generated reports:**
| Report | Frequency | Contents |
|--------|-----------|----------|
| GSTR-1 | Monthly | B2B invoices, HSN summary, document-wise tax |
| GSTR-3B | Monthly | Summary of outward/inward supplies, tax liability |
| HSN-wise Summary | Monthly | Service accounting code (998421), qty, value, tax |
| Cash/Payment Register | Monthly | All payments received with tax breakup |
| Input Tax Credit Register | Monthly | GST paid on purchases/expenses |


### 8A.9 Financial Statements

Automated generation of standard financial statements:

**Profit & Loss Statement:**
```sql
CREATE TABLE financial_statements (
    id BIGSERIAL PRIMARY KEY,
    statement_type VARCHAR(30) NOT NULL
        CHECK (statement_type IN ('profit_loss', 'balance_sheet', 'cash_flow')),
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    branch_id BIGINT REFERENCES branches(id),  -- NULL = consolidated
    generated_data JSONB NOT NULL,
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(statement_type, period_start, period_end, branch_id)
);
```

**Statement Types:**

1. **Profit & Loss (Monthly/Quarterly/Annual):**
```
Revenue:
  Subscription Revenue          ₹X,XXX,XXX
  Installation Revenue          ₹XX,XXX
  Hardware Revenue              ₹XX,XXX
  Late Payment Fees             ₹X,XXX
  ─────────────────────────────────────
  Total Revenue                 ₹X,XXX,XXX

Expenses:
  Bandwidth Cost                ₹XX,XXX
  Equipment Cost                ₹XX,XXX
  Salary Expense                ₹X,XXX,XXX
  Rent Expense                  ₹XX,XXX
  Electricity Expense           ₹XX,XXX
  Maintenance Expense           ₹XX,XXX
  Marketing Expense             ₹XX,XXX
  Refund Expense                ₹X,XXX
  ─────────────────────────────────────
  Total Expenses                ₹X,XXX,XXX

Net Profit/Loss                 ₹X,XXX,XXX
```

2. **Balance Sheet (Point-in-Time):**
```
Assets:
  Cash                          ₹X,XXX,XXX
  Bank Account                  ₹XX,XXX,XXX
  Accounts Receivable           ₹X,XXX,XXX
  GST Input Credit              ₹XX,XXX
  Prepaid Expenses              ₹XX,XXX
  ─────────────────────────────
  Total Assets                  ₹XX,XXX,XXX

Liabilities:
  Accounts Payable              ₹XX,XXX
  GST Output Tax                ₹X,XXX,XXX
  TDS Payable                   ₹XX,XXX
  ─────────────────────────────
  Total Liabilities             ₹X,XXX,XXX

Equity:
  Owner's Equity                ₹XX,XXX,XXX
  Retained Earnings             ₹XX,XXX,XXX
  ─────────────────────────────
  Total Equity                  ₹XX,XXX,XXX
```

3. **Cash Flow Statement (Monthly):**
```
Operating Activities:
  Cash from customers           ₹X,XXX,XXX
  Cash paid to vendors          (₹XX,XXX)
  Cash paid to employees        (₹X,XXX,XXX)
  ─────────────────────────────
  Net Operating Cash Flow       ₹X,XXX,XXX

Investing Activities:
  Equipment purchases           (₹XX,XXX)
  ─────────────────────────────
  Net Investing Cash Flow       (₹XX,XXX)

Financing Activities:
  Owner investment              ₹XX,XXX
  ─────────────────────────────
  Net Financing Cash Flow       ₹XX,XXX

Net Change in Cash              ₹X,XXX,XXX
```

**Permissions:**
```
financial.statement.profit_loss.view
financial.statement.profit_loss.generate
financial.statement.balance_sheet.view
financial.statement.balance_sheet.generate
financial.statement.cash_flow.view
financial.statement.cash_flow.generate
financial.statement.export
```

---

## 8B. Payment Gateway Integration

### 8B.1 Supported Payment Gateways

| Gateway | Link Payment | UPI | Cards | Net Banking | Auto-debit |
|---------|-------------|-----|-------|-------------|------------|
| **RazorPay** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **InstaMojo** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **CCAvenue** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **PayTm** | ✅ | ✅ | ✅ | ✅ | ✅ |

### 8B.2 Link Payment Flow

**Customer does NOT select gateway.** The system generates a single payment link that works across all supported gateways:

1. Customer receives invoice with a **universal payment link**
2. Clicking the link opens a hosted payment page
3. Page shows all available payment methods (UPI, cards, net banking)
4. Customer selects their preferred method — gateway is selected automatically
5. Payment is processed via the best available gateway
6. Webhook confirms payment to the platform
7. Invoice is marked as paid, journal entry is created

```
Invoice Generated
    ↓
Payment Link Created
    ↓
Hosted Payment Page (gateway-agnostic)
    ├── UPI (any UPI app)
    ├── Credit/Debit Card
    ├── Net Banking
    └── Wallet
    ↓
Gateway Selection (automatic based on method)
    ├── RazorPay (UPI, Cards)
    ├── InstaMojo (UPI, Cards)
    ├── CCAvenue (Cards, Net Banking)
    └── PayTm (UPI, Wallet)
    ↓
Payment Processed
    ↓
Webhook → Platform
    ↓
Invoice Updated + Journal Entry Created
```

### 8B.3 Payment Gateway Configuration

```sql
CREATE TABLE payment_gateways (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(50) NOT NULL UNIQUE,
    is_active BOOLEAN DEFAULT TRUE,
    priority INTEGER DEFAULT 0,
    supported_methods JSONB NOT NULL,
    -- Encrypted credentials
    api_key_encrypted TEXT,
    api_secret_encrypted TEXT,
    webhook_secret_encrypted TEXT,
    merchant_id_encrypted TEXT,
    config JSONB,
    -- Fee structure
    transaction_fee_percent DECIMAL(5,2) DEFAULT 0,
    transaction_fee_fixed DECIMAL(10,2) DEFAULT 0,
    gst_on_fee DECIMAL(5,2) DEFAULT 18.0,
    -- Checker/Maker
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Payment gateway history
CREATE TABLE payment_gateways_history (
    id BIGSERIAL PRIMARY KEY,
    gateway_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE payment_links (
    id BIGSERIAL PRIMARY KEY,
    link_id VARCHAR(50) NOT NULL UNIQUE,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'INR',
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'used', 'expired', 'cancelled')),
    expires_at TIMESTAMPTZ,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 8B.4 Gateway Credentials (Encrypted)

```yaml
razorpay:
  api_key: ${RAZORPAY_KEY_ID}
  api_secret: ${RAZORPAY_KEY_SECRET}
  webhook_secret: ${RAZORPAY_WEBHOOK_SECRET}

instamojo:
  api_key: ${INSTAMOJO_API_KEY}
  api_secret: ${INSTAMOJO_API_SECRET}
  auth_token: ${INSTAMOJO_AUTH_TOKEN}

ccavenue:
  merchant_id: ${CCAVENUE_MERCHANT_ID}
  access_code: ${CCAVENUE_ACCESS_CODE}
  working_key: ${CCAVENUE_WORKING_KEY}

paytm:
  merchant_id: ${PAYTM_MERCHANT_ID}
  merchant_key: ${PAYTM_MERCHANT_KEY}
  merchant_website: ${PAYTM_MERCHANT_WEBSITE}
  channel: ${PAYTM_CHANNEL}
  industry_type: ${PAYTM_INDUSTRY_TYPE}
```

### 8B.5 Webhook Handling

```yaml
webhook_events:
  razorpay:
    - payment.captured
    - payment.failed
    - refund.created
    - refund.processed
  instamojo:
    - payment.successful
    - payment.failed
  ccavenue:
    - transaction.success
    - transaction.failure
  paytm:
    - TXN_SUCCESS
    - TXN_FAILURE
```

### 8B.6 Gateway Failover

If primary gateway fails:
1. Retry on same gateway (1 attempt, 5s delay)
2. Try next gateway by priority
3. If all gateways fail → show manual payment option
4. Alert finance team

### 8B.7 Permissions

```
gateway.configuration.view
gateway.configuration.update
gateway.webhook.view
gateway.reconciliation.view
gateway.reconciliation.process
```


### 8B.8 Idempotency

All payment operations use idempotency keys to prevent double-processing:

```sql
CREATE TABLE idempotency_keys (
    id BIGSERIAL PRIMARY KEY,
    idempotency_key VARCHAR(255) NOT NULL UNIQUE,
    user_id BIGINT REFERENCES users(id),
    endpoint VARCHAR(255) NOT NULL,
    request_hash VARCHAR(64) NOT NULL,
    response_status INTEGER,
    response_body JSONB,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Rules:**
- Client generates idempotency key (UUID v4) for every POST request
- Key is valid for 24 hours
- If duplicate key received → return cached response (no reprocessing)
- Webhook handlers use `gateway_transaction_id` as natural idempotency key
- Journal entries use `reference_type + reference_id` as natural idempotency key

---

## 8C. Manual Payment & Top-Up Flow

### 8C.1 Manual Payment (Cash/Transfer)

Customers can pay via cash, bank transfer, or cheque. Staff records the payment manually:

1. Customer contacts support or visits office
2. Staff creates manual payment record
3. Staff uploads proof (photo of cash receipt, bank transfer screenshot)
4. Payment goes through **checker/maker approval**
5. Maker enters payment details → status: `pending_approval`
6. Checker reviews and approves → status: `approved`
7. Invoice is marked as paid
8. Journal entry is created automatically

```sql
CREATE TABLE manual_payments (
    id BIGSERIAL PRIMARY KEY,
    payment_id BIGINT REFERENCES payments(id),
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    amount DECIMAL(10,2) NOT NULL,
    payment_mode VARCHAR(30) NOT NULL
        CHECK (payment_mode IN ('cash', 'bank_transfer', 'cheque', 'upi_direct', 'other')),
    reference_number VARCHAR(100),
    bank_name VARCHAR(100),
    transaction_date DATE,
    proof_urls TEXT[],
    notes TEXT,
    -- Checker/Maker workflow
    recorded_by BIGINT REFERENCES users(id),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    rejection_reason TEXT,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Manual payments history
CREATE TABLE manual_payments_history (
    id BIGSERIAL PRIMARY KEY,
    manual_payment_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);
```

### 8C.2 Customer Top-Up Flow

Customers can top-up their subscription balance to extend service or prepay:

1. Customer opens app → selects "Top Up" / "Add Balance"
2. Enters top-up amount (minimum ₹100)
3. System generates payment link
4. Customer pays via any method (UPI, card, etc.)
5. Payment confirmed → balance credited to customer wallet
6. Balance is auto-applied to next invoice
7. If balance exceeds invoice amount, excess carries forward

```sql
CREATE TABLE customer_wallets (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id) UNIQUE,
    balance DECIMAL(10,2) DEFAULT 0.00,
    total_credited DECIMAL(10,2) DEFAULT 0.00,
    total_used DECIMAL(10,2) DEFAULT 0.00,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE wallet_transactions (
    id BIGSERIAL PRIMARY KEY,
    wallet_id BIGINT NOT NULL REFERENCES customer_wallets(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    type VARCHAR(20) NOT NULL
        CHECK (type IN ('credit', 'debit', 'refund', 'adjustment')),
    amount DECIMAL(10,2) NOT NULL,
    balance_after DECIMAL(10,2) NOT NULL,
    reference_type VARCHAR(50),
    reference_id BIGINT,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Top-Up Journal Entry:**
```
Customer tops up ₹500:
Dr. 1100 Bank Account    ₹500.00
Cr. 1300 Prepaid Expenses ₹500.00

Auto-applied to invoice:
Dr. 1300 Prepaid Expenses ₹500.00
Cr. 1200 Accounts Receivable ₹500.00
```

### 8C.3 Permissions

```
manual_payment.view
manual_payment.create
manual_payment.approve
manual_payment.reject
wallet.view
wallet.credit
wallet.debit
wallet.adjust
```

---

## 8D. Entity History & Rollback

### 8D.1 History Table Pattern

Every critical entity has a companion `_history` table that tracks all changes:

```sql
-- Generic history table pattern
CREATE TABLE {entity}_history (
    id BIGSERIAL PRIMARY KEY,
    entity_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL
        CHECK (action IN ('created', 'updated', 'deleted', 'status_changed')),
    old_data JSONB,
    new_data JSONB,
    changed_fields TEXT[],
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    reason TEXT,
    rollback_reference BIGINT REFERENCES {entity}_history(id)
);
```

### 8D.2 Complete History Tables List

| History Table | Tracks Changes For | Key Fields |
|--------------|-------------------|------------|
| `customers_history` | Customer records | Status, profile, KYC |
| `subscriptions_history` | Subscription changes | Plan, status, billing period |
| `plans_history` | Plan modifications | Price, speed, features |
| `bandwidth_profiles_history` | Speed profile changes | Speeds, QoS, priority |
| `network_devices_history` | Device config changes | Status, firmware, config |
| `invoices_history` | Invoice modifications | Status, amounts, payments |
| `refunds_history` | Refund processing | Amount, status, approval |
| `discounts_history` | Discount changes | Value, validity, status |
| `payment_gateways_history` | Gateway config changes | Credentials, status |
| `manual_payments_history` | Manual payment records | Amount, approval status |
| `journal_entries_history` | Accounting entries | Status, amounts, approval |
| `approval_requests_history` | Approval workflow | Status, approver |

### 8D.3 Rollback Mechanism

For any change that needs to be undone:

1. Query the `_history` table for the last `old_data` snapshot
2. Validate the rollback is safe (no downstream dependencies)
3. Apply the `old_data` back to the main table
4. Create a new history entry with `action = 'rollback'`
5. Link to original history entry via `rollback_reference`

```sql
-- Example: Rollback a plan change
-- 1. Get last change
SELECT old_data FROM plans_history
WHERE plan_id = 1 AND action = 'updated'
ORDER BY performed_at DESC LIMIT 1;

-- 2. Restore
UPDATE plans SET
    price = (old_data->>'price')::DECIMAL,
    speed_label = old_data->>'speed_label',
    updated_at = NOW()
WHERE id = 1;

-- 3. Log rollback
INSERT INTO plans_history (plan_id, action, old_data, new_data, performed_by, reason)
SELECT plan_id, 'rollback', new_data, old_data, $performer_id, 'Reverted due to error'
FROM plans_history WHERE id = $original_change_id;
```

### 8D.4 History Retention Policy

| Table | Retention | Archive Strategy |
|-------|-----------|------------------|
| customers_history | 7 years | Compress after 1 year |
| subscriptions_history | 7 years | Compress after 1 year |
| plans_history | 7 years | Compress after 1 year |
| invoices_history | 7 years | Compress after 1 year |
| refunds_history | 7 years | Compress after 1 year |
| network_devices_history | 3 years | Compress after 6 months |
| bandwidth_profiles_history | 2 years | Compress after 6 months |
| journal_entries_history | 7 years | Compress after 1 year |
| manual_payments_history | 7 years | Compress after 1 year |
| payment_gateways_history | 3 years | Compress after 6 months |
| discounts_history | 3 years | Compress after 6 months |
| approval_requests_history | 3 years | Compress after 6 months |

---

## 9. Notification Platform

### 9.1 Notification Channels

| Channel | Provider | Use Cases |
|---------|----------|-----------|
| Email | AWS SES / SendGrid | Invoices, account updates, marketing |
| SMS | Twilio / MSG91 | OTP, payment reminders, alerts |
| Push (Android) | Firebase Cloud Messaging | Real-time alerts, status updates |
| Push (iOS) | Apple Push Notification Service | Real-time alerts, status updates |
| In-App | WebSocket | Real-time dashboard updates |
| WhatsApp | WhatsApp Business API | Customer onboarding, support |

### 9.2 Notification Events

| Event | Channel(s) | Template |
|-------|------------|----------|
| `customer.created` | Email, SMS | Welcome message with account details |
| `customer.activated` | Email, SMS, Push | Service activation confirmation |
| `customer.suspended` | Email, SMS, Push | Suspension notice with reactivation instructions |
| `invoice.generated` | Email, SMS | Monthly invoice with payment link |
| `invoice.paid` | Email, SMS, Push | Payment confirmation receipt |
| `invoice.overdue` | Email, SMS | Overdue reminder with payment link |
| `payment.failed` | Email, SMS, Push | Payment failure alert |
| `service.expiring` | Email, SMS, Push | Subscription expiry warning |
| `device.offline` | Email, Push | Network device alert |
| `bandwidth.changed` | Push | Speed plan change confirmation |
| `ticket.created` | Email, Push | Support ticket acknowledgment |
| `ticket.updated` | Email, Push | Ticket status update |
| `ticket.resolved` | Email, Push | Ticket resolution notification |
| `installation.scheduled` | Email, SMS, Push | Installation appointment confirmation |
| `installation.completed` | Email, SMS, Push | Installation completion with next steps |

### 9.3 Template Engine

Templates use Handlebars syntax with dynamic variables:

```handlebars
<!-- Email: Invoice Generated -->
<h2>Invoice #{{invoice_number}}</h2>
<p>Dear {{customer_name}},</p>
<p>Your monthly invoice for <strong>{{plan_name}}</strong> is ready.</p>
<table>
  <tr><td>Plan</td><td>{{plan_name}}</td></tr>
  <tr><td>Period</td><td>{{billing_period_start}} to {{billing_period_end}}</td></tr>
  <tr><td>Amount</td><td>₹{{total_amount}}</td></tr>
</table>
<a href="{{payment_url}}">Pay Now</a>
<p>Due date: {{due_date}}</p>
```

### 9.4 Queue System

```
Event Published → NATS → Notification Service
    ↓
Notification Service
    ├── Validate channel availability
    ├── Render template
    ├── Check rate limits (per customer, per channel)
    ├── Enqueue to channel-specific queue
    │   ├── Email Queue (Redis List)
    │   ├── SMS Queue (Redis List)
    │   ├── Push Queue (Redis List)
    │   └── In-App Queue (WebSocket)
    ├── Worker picks up from queue
    ├── Send via provider API
    ├── Update delivery status
    └── Retry on failure (3 attempts, exponential backoff)
```

### 9.5 Retry Mechanism

| Attempt | Delay | Action on Failure |
|---------|-------|-------------------|
| 1 | Immediate | Retry |
| 2 | 30 seconds | Retry |
| 3 | 5 minutes | Retry |
| 4 | 30 minutes | Retry |
| 5+ | 1 hour (max 24h) | Move to dead letter queue |

Dead letter queue entries are reviewed manually by operators.

### 9.6 Database Tables

```sql
CREATE TABLE notification_templates (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    channel VARCHAR(20) NOT NULL,
    subject VARCHAR(255),
    body_template TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(event_type, channel)
);

CREATE TABLE notifications (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT REFERENCES customers(id),
    event_type VARCHAR(100) NOT NULL,
    channel VARCHAR(20) NOT NULL,
    subject VARCHAR(255),
    body TEXT NOT NULL,
    status VARCHAR(20) DEFAULT 'queued'
        CHECK (status IN ('queued', 'sending', 'sent', 'delivered', 'failed', 'bounced')),
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 5,
    last_attempt_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    failed_reason TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

CREATE TABLE notification_preferences (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    channel VARCHAR(20) NOT NULL,
    event_category VARCHAR(50) NOT NULL,
    is_enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(customer_id, channel, event_category)
);
```

---

## 10. Realtime System

### 10.1 WebSocket Architecture

```
Client (Browser/Mobile)
    ↓ WebSocket Connection
API Gateway (Reverse Proxy)
    ↓
Axum WebSocket Handler
    ↓
Connection Manager (HashMap<client_id, connection>)
    ↓
Redis Pub/Sub Channel
    ↓
NATS Event Stream
    ↓
Backend Services (publishers)
```

### 10.2 Realtime Data Streams

| Stream | Data | Update Frequency | Consumers |
|--------|------|------------------|-----------|
| `noc.dashboard` | Online customers, bandwidth, alerts | Every 5s | NOC dashboard |
| `customer.status` | Online/offline, current speed, usage | Every 10s | Customer app |
| `device.health` | CPU, memory, ports, optical power | Every 30s | Device management page |
| `network.alerts` | Device down, threshold breach, errors | Real-time | NOC dashboard |
| `bandwidth.usage` | Real-time bandwidth per customer | Every 5s | NOC dashboard |
| `ticket.updates` | New tickets, status changes | Real-time | Support dashboard |
| `invoice.updates` | Payment received, overdue alerts | Real-time | Finance dashboard |

### 10.3 Connection Management

```rust
struct ConnectionManager {
    connections: HashMap<String, ClientConnection>,
    user_subscriptions: HashMap<String, Vec<String>>,  // user_id -> stream_ids
}

struct ClientConnection {
    client_id: String,
    user_id: String,
    role: String,
    connected_at: DateTime<Utc>,
    last_heartbeat: DateTime<Utc>,
    streams: Vec<String>,
}
```

### 10.4 Redis Pub/Sub Integration

```yaml
channels:
  - noc:dashboard          # NOC dashboard updates
  - customer:{id}:status   # Per-customer status
  - device:{id}:health     # Per-device health
  - alerts:critical        # Critical alerts
  - alerts:warning         # Warning alerts
  - tickets:updates        # Ticket update stream
```

### 10.5 Authentication

WebSocket connections are authenticated via:
1. Initial HTTP request includes JWT token in query parameter or header
2. Token is validated on connection establishment
3. Connection is associated with user role
4. Role determines which streams the client can subscribe to

### 10.6 Rate Limiting

| Client Type | Max Connections | Max Subscriptions | Heartbeat Interval |
|-------------|-----------------|-------------------|-------------------|
| Admin Portal | 1 per user | 10 | 30s |
| Customer App | 1 per user | 3 | 60s |
| NOC Dashboard | 1 per user | 20 | 15s |
| Mobile Background | 1 per user | 2 | 120s |

---

## 11. Backend Architecture

### 11.1 Technology Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Language | Rust | 1.75+ |
| Web Framework | Axum | 0.7+ |
| Database | PostgreSQL | 16+ |
| Cache | Redis | 7+ |
| Message Broker | NATS | 2.10+ |
| ORM | SQLx (async, compile-time checked) | 0.7+ |
| Auth | JWT (jsonwebtoken crate) | latest |
| Serialization | serde + serde_json | latest |
| Validation | validator | latest |
| Tracing | tracing + tracing-subscriber | latest |
| Error Handling | thiserror + anyhow | latest |
| Config | config + dotenvy | latest |
| Time | chrono | latest |
| UUID | uuid (v4) | latest |
| Password Hashing | argon2 | latest |

### 11.2 Folder Structure

```
backend/
├── Cargo.toml
├── .env
├── .env.example
├── migrations/
│   ├── 001_initial_schema/
│   ├── 002_add_indexes/
│   └── 003_add_partitions/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── database.rs
│   │   ├── redis.rs
│   │   ├── nats.rs
│   │   └── app.rs
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── rbac.rs
│   │   ├── rate_limit.rs
│   │   ├── audit.rs
│   │   ├── request_id.rs
│   │   └── error_handler.rs
│   ├── modules/
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── rbac/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── customers/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── plans/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── bandwidth/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── engine.rs
│   │   │   ├── device_controller.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── devices/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── snmp_client.rs
│   │   │   ├── ssh_client.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── network/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── vlan_manager.rs
│   │   │   ├── ippool_manager.rs
│   │   │   ├── pppoe_manager.rs
│   │   │   ├── dhcp_manager.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── billing/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── invoice_generator.rs
│   │   │   ├── payment_processor.rs
│   │   │   ├── dunning_engine.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── notifications/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── template_engine.rs
│   │   │   ├── channels/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── email.rs
│   │   │   │   ├── sms.rs
│   │   │   │   ├── push.rs
│   │   │   │   ├── whatsapp.rs
│   │   │   │   └── websocket.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── audit/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   └── routes.rs
│   │   ├── reports/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   └── routes.rs
│   │   ├── installation/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   ├── support/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   ├── service.rs
│   │   │   ├── repository.rs
│   │   │   ├── models.rs
│   │   │   ├── events.rs
│   │   │   └── routes.rs
│   │   └── coverage/
│   │       ├── mod.rs
│   │       ├── handler.rs
│   │       ├── service.rs
│   │       ├── repository.rs
│   │       ├── models.rs
│   │       └── routes.rs
│   ├── shared/
│   │   ├── mod.rs
│   │   ├── errors.rs
│   │   ├── types.rs
│   │   ├── pagination.rs
│   │   ├── validation.rs
│   │   └── response.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── pool.rs
│   │   └── migrations.rs
│   ├── cache/
│   │   ├── mod.rs
│   │   └── redis.rs
│   ├── events/
│   │   ├── mod.rs
│   │   ├── nats.rs
│   │   ├── publisher.rs
│   │   └── subscriber.rs
│   ├── realtime/
│   │   ├── mod.rs
│   │   ├── websocket.rs
│   │   ├── connection_manager.rs
│   │   └── channels.rs
│   └── jobs/
│       ├── mod.rs
│       ├── invoice_generator.rs
│       ├── dunning_checker.rs
│       ├── device_monitor.rs
│       ├── bandwidth_monitor.rs
│       └── cleanup.rs
└── tests/
    ├── integration/
    └── unit/
```

### 11.3 Module Communication

Modules communicate through:

1. **Direct function calls** — For synchronous operations within a request
2. **Events (NATS)** — For asynchronous cross-module communication
3. **Shared database** — Via SQLx queries
4. **Redis cache** — For shared state (sessions, permissions, online status)

### 11.4 Event Publishing Pattern

```rust
// Within a service method
async fn activate_customer(
    &self,
    customer_id: Uuid,
    subscription_id: Uuid,
) -> Result<Customer, AppError> {
    // 1. Update database
    let customer = self.repository.activate(customer_id).await?;

    // 2. Apply bandwidth profile
    self.bandwidth_service.apply_profile(subscription_id).await?;

    // 3. Publish event
    self.event_publisher.publish("customer.activated", &CustomerActivated {
        customer_id,
        subscription_id,
        plan_id: customer.plan_id,
        activated_at: Utc::now(),
    }).await?;

    Ok(customer)
}
```

### 11.5 Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("NATS error: {0}")]
    Nats(#[from] async_nats::Error),

    #[error("Authentication required")]
    Unauthorized,

    #[error("Insufficient permissions: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
```

---


### 11.6 CQRS Pattern (Where Beneficial)

For high-read, low-write paths, the backend uses Command Query Responsibility Segregation:

**Write Side (Commands):**
- Handles all state mutations (create, update, delete)
- Writes to PostgreSQL primary tables
- Publishes events to NATS after successful write
- Validates business rules and permissions

**Read Side (Queries):**
- Optimized for fast reads (dashboards, reports, lists)
- Uses Redis cache with materialized views
- Event subscribers update read models asynchronously
- No business logic — pure data access

**Applicable Modules:**

| Module | Write Model | Read Model | Update Strategy |
|--------|------------|------------|-----------------|
| Customer Dashboard | `customers` + `subscriptions` | Redis hash `customer:{id}:dashboard` | Event-driven (real-time) |
| NOC Dashboard | `network_devices` + `customer_sessions` | Redis sorted set `noc:online` | Event-driven (real-time) |
| Billing Reports | `invoices` + `payments` | PostgreSQL materialized views | Periodic refresh (5 min) |
| Plan Listings | `plans` + `plan_pricing` | Redis hash `plans:active` | Event-driven (on plan change) |

**Event Subscriber Pattern:**
```rust
// Each read model has a dedicated NATS subscriber
struct CustomerDashboardSubscriber {
    redis: RedisPool,
}

impl CustomerDashboardSubscriber {
    async fn handle(&self, event: &DomainEvent) -> Result<()> {
        match event {
            DomainEvent::CustomerActivated(e) => {
                // Update read model
                self.redis.hset(
                    format!("customer:{}:dashboard", e.customer_id),
                    "status", "active"
                ).await?;
            }
            DomainEvent::InvoicePaid(e) => {
                self.redis.hset(
                    format!("customer:{}:dashboard", e.customer_id),
                    "last_payment", &e.amount.to_string()
                ).await?;
            }
            _ => {}
        }
        Ok(())
    }
}
```

## 12. Event Sourcing Design

### 12.1 Where Event Sourcing Is Required

| Aggregate | Reason | Retention |
|-----------|--------|-----------|
| Customer Lifecycle | Full audit trail of state changes, regulatory compliance | 7 years |
| Billing & Payments | Financial audit, dispute resolution | 7 years |
| Bandwidth Changes | Debugging QoS issues, SLA compliance | 2 years |
| Device Configuration | Network change management, rollback capability | 2 years |
| RBAC Changes | Security audit, compliance | 7 years |

### 12.2 Event Store Schema

```sql
CREATE TABLE event_store (
    id BIGSERIAL PRIMARY KEY,
    aggregate_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    event_version INTEGER NOT NULL DEFAULT 1,
    payload JSONB NOT NULL,
    metadata JSONB,
    causation_id BIGINT REFERENCES event_store(id),
    correlation_id UUID,
    actor_id UUID,
    actor_type VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

CREATE INDEX idx_event_store_aggregate ON event_store(aggregate_type, aggregate_id);
CREATE INDEX idx_event_store_event_type ON event_store(event_type);
CREATE INDEX idx_event_store_created_at ON event_store(created_at);
CREATE INDEX idx_event_store_correlation ON event_store(correlation_id);
```

### 12.3 Event Definitions

```yaml
Customer Events:
  CustomerCreated:
    aggregate: customer
    payload: { customer_id, name, phone, email, referred_by, source }

  CustomerKYCSubmitted:
    aggregate: customer
    payload: { customer_id, document_types[] }

  CustomerKYCVerified:
    aggregate: customer
    payload: { customer_id, verified_by }

  CustomerActivated:
    aggregate: customer
    payload: { customer_id, subscription_id, plan_id, pppoe_username }

  CustomerSuspended:
    aggregate: customer
    payload: { customer_id, reason, suspended_by }

  CustomerReactivated:
    aggregate: customer
    payload: { customer_id, reactivated_by }

  CustomerTerminated:
    aggregate: customer
    payload: { customer_id, reason, terminated_by }

Billing Events:
  InvoiceGenerated:
    aggregate: billing
    payload: { invoice_id, customer_id, total_amount, due_date }

  InvoicePaid:
    aggregate: billing
    payload: { invoice_id, payment_id, amount, payment_method }

  InvoiceOverdue:
    aggregate: billing
    payload: { invoice_id, days_overdue }

  PaymentFailed:
    aggregate: billing
    payload: { payment_id, invoice_id, reason }

  RefundProcessed:
    aggregate: billing
    payload: { refund_id, payment_id, amount, reason }

Bandwidth Events:
  BandwidthProfileCreated:
    aggregate: bandwidth
    payload: { profile_id, plan_id, download_kbps, upload_kbps }

  BandwidthProfileApplied:
    aggregate: bandwidth
    payload: { profile_id, subscription_id, device_id }

  BandwidthProfileFailed:
    aggregate: bandwidth
    payload: { profile_id, subscription_id, device_id, error }

Device Events:
  DeviceRegistered:
    aggregate: device
    payload: { device_id, name, type, vendor, model, serial }

  DeviceStatusChanged:
    aggregate: device
    payload: { device_id, old_status, new_status }

  DeviceConfigurationChanged:
    aggregate: device
    payload: { device_id, config_diff, changed_by }

  FirmwareUpdateStarted:
    aggregate: device
    payload: { device_id, from_version, to_version, initiated_by }

  FirmwareUpdateCompleted:
    aggregate: device
    payload: { device_id, version, duration_seconds }

RBAC Events:
  RoleCreated:
    aggregate: rbac
    payload: { role_id, name, created_by }

  PermissionGranted:
    aggregate: rbac
    payload: { user_id, role_id, permission, granted_by }

  PermissionRevoked:
    aggregate: rbac
    payload: { user_id, role_id, permission, revoked_by }
```

---

## 13. Database Design

### 13.1 Schema Overview

The database follows a modular design with clear domain boundaries:

- **auth** — Users, sessions, tokens
- **rbac** — Roles, permissions, assignments
- **customers** — Customer lifecycle, profiles, addresses
- **subscriptions** — Active service subscriptions
- **plans** — Products, pricing, speed profiles
- **bandwidth** — Bandwidth profiles and applications
- **devices** — Network device management
- **network** — VLAN, IP pools, PPPoE, DHCP, MAC bindings
- **billing** — Invoices, payments, refunds, discounts
- **installation** — Installation orders and workflows
- **support** — Tickets, comments, escalations
- **notifications** — Notification templates, delivery tracking
- **audit** — Audit logs (partitioned by month)
- **events** — Event store (partitioned by month)
- **coverage** — Coverage areas, pincode mapping

### 13.2 Core Tables Summary

| Table | Domain | Partitioned | Estimated Rows (Year 1) |
|-------|--------|-------------|-------------------------|
| customers | customers | No | 5,000 |
| customer_profiles | customers | No | 5,000 |
| addresses | customers | No | 7,500 |
| subscriptions | customers | No | 5,000 |
| service_accounts | customers | No | 5,000 |
| installation_orders | installation | No | 5,000 |
| plans | plans | No | 5 |
| plan_pricing | plans | No | 20 |
| speed_profiles | plans | No | 5 |
| service_packages | plans | No | 10 |
| bandwidth_profiles | bandwidth | No | 10 |
| bandwidth_applications | bandwidth | No | 5,000 |
| bandwidth_usage | bandwidth | Yes (daily) | 180,000 |
| network_devices | devices | No | 50 |
| device_ports | devices | No | 500 |
| device_logs | devices | Yes (monthly) | 500,000 |
| device_metrics | devices | Yes (hourly) | 4,000,000 |
| firmware_updates | devices | No | 200 |
| invoices | billing | No | 60,000 |
| invoice_line_items | billing | No | 60,000 |
| payments | billing | No | 60,000 |
| refunds | billing | No | 500 |
| discounts | billing | No | 20 |
| payment_reminders | billing | No | 30,000 |
| pppoe_sessions | network | No | 5,000 |
| dhcp_leases | network | No | 5,000 |
| mac_bindings | network | No | 5,000 |
| ip_pools | network | No | 20 |
| vlans | network | No | 30 |
| tickets | support | No | 10,000 |
| ticket_comments | support | No | 50,000 |
| notifications | notifications | Yes (monthly) | 500,000 |
| notification_preferences | notifications | No | 5,000 |
| notification_templates | notifications | No | 30 |
| audit_logs | audit | Yes (monthly) | 10,000,000 |
| event_store | events | Yes (monthly) | 5,000,000 |
| coverage_areas | coverage | No | 100 |
| approval_requests | rbac | No | 500 |

### 13.3 Partition Strategy

**Audit Logs & Events:** Monthly range partitions, auto-created via pg_partman.

```sql
-- Create partition for current month
CREATE TABLE audit_logs_2026_07 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');

-- Automatic partition management
SELECT partman.create_parent(
    'public.audit_logs',
    'created_at',
    'native',
    'monthly'
);
```

**Bandwidth Usage:** Daily range partitions for high-volume metrics.

**Device Metrics:** Hourly range partitions for real-time monitoring data.

**Notifications:** Monthly range partitions.

### 13.4 Indexes

```sql
-- Customers
CREATE INDEX idx_customers_phone ON customers(phone);
CREATE INDEX idx_customers_status ON customers(status);
CREATE INDEX idx_customers_customer_code ON customers(customer_code);

-- Subscriptions
CREATE INDEX idx_subscriptions_customer_id ON subscriptions(customer_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);
CREATE INDEX idx_subscriptions_next_billing ON subscriptions(next_billing_date);

-- Invoices
CREATE INDEX idx_invoices_customer_id ON invoices(customer_id);
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_invoices_due_date ON invoices(due_date);
CREATE INDEX idx_invoices_invoice_number ON invoices(invoice_number);

-- Devices
CREATE INDEX idx_network_devices_status ON network_devices(status);
CREATE INDEX idx_network_devices_ip ON network_devices(management_ip);
CREATE INDEX idx_network_devices_type ON network_devices(device_model_id);

-- Network
CREATE INDEX idx_pppoe_sessions_customer ON pppoe_sessions(customer_id);
CREATE INDEX idx_pppoe_sessions_status ON pppoe_sessions(status);
CREATE INDEX idx_dhcp_leases_mac ON dhcp_leases(mac_address);
CREATE INDEX idx_mac_bindings_customer ON mac_bindings(customer_id);

-- Support
CREATE INDEX idx_tickets_customer_id ON tickets(customer_id);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_assigned_to ON tickets(assigned_to);
```

---

## 14. Redis Design

### 14.1 Usage Patterns

| Pattern | Data Type | TTL | Use Case |
|---------|-----------|-----|----------|
| Session Cache | Hash | 24h | User session data |
| Permission Cache | Hash | 1h | Cached role permissions |
| Online Status | String (bit) | 5m | Customer online/offline |
| Rate Limiting | Sorted Set | 1m | API rate limiting |
| Device Health Cache | Hash | 30s | Latest device metrics |
| Bandwidth Usage Cache | Hash | 5s | Real-time bandwidth counters |
| WebSocket Subscriptions | Set | On disconnect | Active WS subscriptions |
| Notification Queue | List | On process | Queued notifications |
| Temp Token | String | 15m | Password reset, OTP |
| Cache Layer | String | 5m | General API response cache |

### 14.2 Key Naming Strategy

```
{module}:{entity}:{id}:{field}

Examples:
auth:session:{session_id}
auth:session:user:{user_id}
rbac:permissions:{user_id}
rbac:role:{role_id}:permissions
customer:status:{customer_id}
customer:online:{customer_id}
device:health:{device_id}
device:metrics:{device_id}:cpu
device:metrics:{device_id}:memory
bandwidth:usage:{subscription_id}:download
bandwidth:usage:{subscription_id}:upload
ratelimit:{ip}:{endpoint}
notification:queue:{channel}
ws:subscriptions:{client_id}
temp:token:{token}
cache:plans:list
```

### 14.3 Redis Configuration

```yaml
redis:
  host: localhost
  port: 6379
  password: ${REDIS_PASSWORD}
  database: 0
  pool_size: 20
  max_connections: 100
  timeout_ms: 5000
  cluster_mode: false

  # Separate databases for different concerns
  databases:
    sessions: 0
    cache: 1
    pubsub: 2
    queues: 3
```

---

## 15. NATS Event Architecture

### 15.1 Subject Hierarchy

```
{domain}.{entity}.{action}

customer.created
customer.activated
customer.suspended
customer.terminated
customer.kyc.submitted
customer.kyc.verified
customer.installation.scheduled
customer.installation.completed

billing.invoice.generated
billing.invoice.paid
billing.invoice.overdue
billing.payment.received
billing.payment.failed
billing.refund.processed

bandwidth.profile.created
bandwidth.profile.updated
bandwidth.profile.applied
bandwidth.profile.failed
bandwidth.usage.alert

device.registered
device.status.changed
device.configuration.changed
device.offline
device.online
device.firmware.update.started
device.firmware.update.completed
device.firmware.update.failed

network.vlan.created
network.vlan.deleted
network.ippool.exhausted
network.pppoe.session.started
network.pppoe.session.ended
network.dhcp.lease.expired

support.ticket.created
support.ticket.assigned
support.ticket.updated
support.ticket.resolved

notification.send
notification.retry

audit.action.logged
```

### 15.2 Publishers

| Publisher | Module | Events Published |
|-----------|--------|------------------|
| CustomerService | customers | customer.* |
| BillingService | billing | billing.invoice.*, billing.payment.* |
| BandwidthService | bandwidth | bandwidth.profile.*, bandwidth.usage.* |
| DeviceService | devices | device.* |
| NetworkService | network | network.* |
| SupportService | support | support.ticket.* |
| NotificationService | notifications | notification.* |
| AuditService | audit | audit.* |
| InstallationService | installation | customer.installation.* |

### 15.3 Subscribers

| Subscriber | Module | Events Subscribed | Action |
|------------|--------|-------------------|--------|
| NotificationService | notifications | customer.*, billing.*, support.ticket.*, device.offline | Send notifications |
| AuditService | audit | * (all) | Log audit trail |
| BandwidthEngine | bandwidth | customer.activated, customer.suspended | Apply/remove bandwidth profiles |
| DeviceMonitor | devices | device.* | Update device status cache |
| DunningEngine | billing | billing.invoice.overdue | Trigger payment reminders |
| RealtimeBroadcaster | realtime | device.*, bandwidth.*, customer.* | Push to WebSocket clients |

### 15.4 Event Payload Example

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "customer.activated",
  "aggregate_type": "customer",
  "aggregate_id": "customer-uuid",
  "payload": {
    "customer_id": "customer-uuid",
    "subscription_id": "subscription-uuid",
    "plan_id": "standard-100",
    "pppoe_username": "rahul@aeroxe",
    "vlan_id": 200,
    "ip_address": "10.10.1.100",
    "activated_at": "2026-07-08T14:30:00Z"
  },
  "metadata": {
    "actor_id": "technician-uuid",
    "actor_type": "field_technician",
    "correlation_id": "installation-order-uuid",
    "causation_id": "previous-event-id"
  },
  "timestamp": "2026-07-08T14:30:00Z"
}
```

---

## 16. Admin Portal Requirements

### 16.1 Technology Stack

| Component | Technology |
|-----------|-----------|
| Framework | React 18+ |
| Language | TypeScript 5+ |
| Styling | TailwindCSS 3+ |
| State Management | Zustand |
| Server State | React Query (TanStack Query) |
| Forms | React Hook Form + Zod |
| Charts | Recharts / Tremor |
| Tables | TanStack Table |
| Real-time | WebSocket (native) |
| Routing | React Router v6 |
| HTTP Client | Axios |
| Icons | Lucide React |

### 16.2 Page Requirements

#### Dashboard

**Purpose:** Real-time overview of ISP operations

**Features:**
- Total active customers (live count)
- Online customers (real-time)
- Monthly revenue (current month vs previous)
- New customers this month
- Pending installations
- Open support tickets
- Device health summary (online/degraded/offline counts)
- Bandwidth utilization graph (per OLT/area)
- Recent activity feed
- Revenue trend (12-month chart)
- Customer churn rate
- Quick actions: Register customer, Generate invoice, View alerts

**API Requirements:**
- `GET /api/v1/dashboard/stats`
- `GET /api/v1/dashboard/revenue-trend`
- `GET /api/v1/dashboard/customer-trend`
- `GET /api/v1/dashboard/device-summary`
- `GET /api/v1/dashboard/bandwidth-utilization`
- `GET /api/v1/dashboard/activity-feed`

**Permissions Required:** `dashboard.view`

---

#### Customers

**Purpose:** Manage customer lifecycle

**Features:**
- Customer list with search, filter, sort
- Filter by status (active, suspended, terminated, etc.)
- Filter by plan, area, registration date
- Customer detail view with full lifecycle timeline
- Edit customer profile
- KYC document upload/review
- View subscription details
- View billing history
- Suspend/reactivate/terminate customer
- View installation history
- View support tickets
- Customer communication log

**API Requirements:**
- `GET /api/v1/customers` (list with pagination)
- `GET /api/v1/customers/:id`
- `POST /api/v1/customers`
- `PATCH /api/v1/customers/:id`
- `DELETE /api/v1/customers/:id`
- `POST /api/v1/customers/:id/suspend`
- `POST /api/v1/customers/:id/reactivate`
- `POST /api/v1/customers/:id/terminate`
- `GET /api/v1/customers/:id/subscriptions`
- `GET /api/v1/customers/:id/invoices`
- `GET /api/v1/customers/:id/installations`
- `GET /api/v1/customers/:id/tickets`

**Permissions Required:** `customer.account.view`, `customer.account.update`, `customer.account.disable`

---

#### Subscriptions

**Purpose:** Manage active subscriptions

**Features:**
- Active subscriptions list
- Subscription detail view
- Upgrade/downgrade plan
- Change billing period
- Cancel subscription
- View PPPoE session details
- View bandwidth profile
- View real-time usage

**API Requirements:**
- `GET /api/v1/subscriptions`
- `GET /api/v1/subscriptions/:id`
- `POST /api/v1/subscriptions/:id/upgrade`
- `POST /api/v1/subscriptions/:id/downgrade`
- `POST /api/v1/subscriptions/:id/cancel`
- `GET /api/v1/subscriptions/:id/pppoe-session`
- `GET /api/v1/subscriptions/:id/bandwidth-usage`

**Permissions Required:** `customer.subscription.view`, `customer.subscription.upgrade`, `customer.subscription.suspend`

---

#### Plans

**Purpose:** Manage internet plans and pricing

**Features:**
- Plan list with status indicators
- Create/edit/delete plans
- Manage pricing tiers
- Manage speed profiles
- Publish/unpublish plans
- Clone existing plans
- View plan subscriber count
- View plan revenue

**API Requirements:**
- `GET /api/v1/plans`
- `GET /api/v1/plans/:id`
- `POST /api/v1/plans`
- `PATCH /api/v1/plans/:id`
- `DELETE /api/v1/plans/:id`
- `POST /api/v1/plans/:id/publish`
- `POST /api/v1/plans/:id/unpublish`
- `POST /api/v1/plans/:id/clone`
- `GET /api/v1/plans/:id/subscribers`
- `GET /api/v1/plans/:id/revenue`

**Permissions Required:** `plan.view`, `plan.create`, `plan.update`, `plan.delete`, `plan.publish`

---

#### Bandwidth Management

**Purpose:** Manage bandwidth profiles and QoS

**Features:**
- Speed profile list
- Create/edit/delete speed profiles
- Apply profiles to subscriptions
- View applied profiles per customer
- Bandwidth utilization per profile
- QoS policy management
- Rate limiting configuration

**API Requirements:**
- `GET /api/v1/bandwidth/profiles`
- `POST /api/v1/bandwidth/profiles`
- `PATCH /api/v1/bandwidth/profiles/:id`
- `DELETE /api/v1/bandwidth/profiles/:id`
- `POST /api/v1/bandwidth/profiles/:id/apply`
- `GET /api/v1/bandwidth/utilization`

**Permissions Required:** `bandwidth.profile.view`, `bandwidth.profile.update`

---

#### Network Devices

**Purpose:** Manage all network hardware

**Features:**
- Device list with status indicators (online/offline/degraded)
- Device detail view with health metrics
- Register new device
- Configure device
- Restart/shutdown device
- Firmware update management
- Device port status
- Device logs viewer
- Device metrics graphs
- Map view (geographic device locations)

**API Requirements:**
- `GET /api/v1/devices`
- `GET /api/v1/devices/:id`
- `POST /api/v1/devices`
- `PATCH /api/v1/devices/:id`
- `POST /api/v1/devices/:id/restart`
- `POST /api/v1/devices/:id/shutdown`
- `GET /api/v1/devices/:id/ports`
- `GET /api/v1/devices/:id/logs`
- `GET /api/v1/devices/:id/metrics`
- `POST /api/v1/devices/:id/firmware/update`

**Permissions Required:** `device.*.view`, `device.*.configure`, `device.*.restart`

---

#### OLT Management

**Purpose:** Specialized OLT management

**Features:**
- OLT port status (PON ports)
- ONT list per OLT port
- ONT provisioning
- OLT traffic profile management
- Optical power monitoring
- ONT distance from OLT
- ONT online/offline status

**API Requirements:**
- `GET /api/v1/olt/:id/ports`
- `GET /api/v1/olt/:id/onts`
- `POST /api/v1/olt/:id/onts/:ont_id/provision`
- `GET /api/v1/olt/:id/traffic-profiles`
- `GET /api/v1/olt/:id/optical-monitoring`

**Permissions Required:** `olt.configuration.view`, `olt.configuration.change`, `olt.configuration.deploy`

---

#### Billing

**Purpose:** Financial management

**Features:**
- Invoice list with status filters
- Invoice detail view
- Generate manual invoices
- Process payments
- Issue refunds
- Discount management
- Tax configuration
- Payment reconciliation
- Dunning configuration
- Revenue reports

**API Requirements:**
- `GET /api/v1/billing/invoices`
- `GET /api/v1/billing/invoices/:id`
- `POST /api/v1/billing/invoices`
- `POST /api/v1/billing/invoices/:id/send`
- `POST /api/v1/billing/invoices/:id/void`
- `POST /api/v1/billing/payments`
- `POST /api/v1/billing/refunds`
- `GET /api/v1/billing/discounts`
- `POST /api/v1/billing/discounts`

**Permissions Required:** `billing.invoice.view`, `billing.invoice.generate`, `billing.payment.process`, `billing.invoice.refund`

---

#### Reports

**Purpose:** Business intelligence and analytics

**Features:**
- Revenue reports (daily, weekly, monthly, yearly)
- Customer acquisition reports
- Customer churn reports
- Plan popularity reports
- Area-wise performance reports
- Bandwidth utilization reports
- Device uptime reports
- Support ticket analytics
- Export to CSV/PDF

**API Requirements:**
- `GET /api/v1/reports/revenue`
- `GET /api/v1/reports/customers`
- `GET /api/v1/reports/churn`
- `GET /api/v1/reports/plans`
- `GET /api/v1/reports/area-performance`
- `GET /api/v1/reports/bandwidth`
- `GET /api/v1/reports/device-uptime`
- `GET /api/v1/reports/tickets`
- `POST /api/v1/reports/export`

**Permissions Required:** `report.view`, `report.generate`, `report.export`

---

#### Users & Roles

**Purpose:** User and role management

**Features:**
- User list with role assignments
- Create/edit/disable users
- Role management (create, edit, delete)
- Permission assignment
- Permission groups
- Temporary permission grants
- Audit log viewer

**API Requirements:**
- `GET /api/v1/users`
- `GET /api/v1/users/:id`
- `POST /api/v1/users`
- `PATCH /api/v1/users/:id`
- `DELETE /api/v1/users/:id`
- `GET /api/v1/roles`
- `POST /api/v1/roles`
- `PATCH /api/v1/roles/:id`
- `DELETE /api/v1/roles/:id`
- `POST /api/v1/roles/:id/permissions`

**Permissions Required:** `user.account.view`, `user.role.assign`, `user.role.create`

---

#### Audit Logs

**Purpose:** Security and compliance audit trail

**Features:**
- Searchable audit log list
- Filter by user, action, resource, date range
- Export audit logs
- Alert on suspicious patterns

**API Requirements:**
- `GET /api/v1/audit/logs`
- `GET /api/v1/audit/logs/:id`
- `POST /api/v1/audit/logs/export`

**Permissions Required:** `audit.log.view`, `audit.log.export`

---

#### Notifications

**Purpose:** Notification management

**Features:**
- Notification template editor
- Channel configuration (email, SMS, push)
- Notification history
- Delivery status tracking
- Manual notification send
- Notification preferences per customer

**API Requirements:**
- `GET /api/v1/notifications/templates`
- `POST /api/v1/notifications/templates`
- `PATCH /api/v1/notifications/templates/:id`
- `GET /api/v1/notifications/history`
- `POST /api/v1/notifications/send`

**Permissions Required:** `notification.template.manage`, `notification.send`

---

## 17. Customer Android Application

### 17.1 Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Kotlin |
| UI | Jetpack Compose |
| Architecture | Clean Architecture (MVVM/MVI) |
| DI | Hilt |
| Networking | Retrofit + OkHttp |
| Local DB | Room |
| Background | WorkManager |
| Push | Firebase Cloud Messaging |
| Navigation | Navigation Compose |
| State | StateFlow + MVI |
| Image Loading | Coil |
| Analytics | Firebase Analytics |

### 17.2 Features

| Feature | Description | API Endpoint |
|---------|-------------|-------------|
| **Authentication** | Phone + OTP login, biometric unlock | `POST /api/v1/auth/otp/send`, `POST /api/v1/auth/otp/verify` |
| **Dashboard** | Current plan, usage stats, online status | `GET /api/v1/customer/dashboard` |
| **Internet Usage** | Real-time bandwidth, daily/monthly usage charts | `GET /api/v1/customer/usage` |
| **Current Plan** | Plan details, speed, pricing, next billing | `GET /api/v1/customer/subscription` |
| **Invoices** | Invoice list, payment status, download PDF | `GET /api/v1/customer/invoices` |
| **Payment** | UPI, card, net banking via Razorpay SDK | `POST /api/v1/customer/payments` |
| **Support Tickets** | Create, view, update, close tickets | `GET /api/v1/customer/tickets`, `POST /api/v1/customer/tickets` |
| **Notifications** | Push notification history, read/unread | `GET /api/v1/customer/notifications` |
| **Profile** | View/edit personal details, KYC status | `GET /api/v1/customer/profile`, `PATCH /api/v1/customer/profile` |
| **Settings** | Notification preferences, theme, language | `GET /api/v1/customer/settings` |

### 17.3 Architecture

```
app/
├── data/
│   ├── remote/
│   │   ├── api/          # Retrofit API interfaces
│   │   ├── dto/          # API response/request DTOs
│   │   └── interceptor/  # Auth, logging interceptors
│   ├── local/
│   │   ├── db/           # Room database
│   │   ├── dao/          # Data access objects
│   │   └── entity/       # Room entities
│   └── repository/       # Repository implementations
├── domain/
│   ├── model/            # Domain models
│   ├── repository/       # Repository interfaces
│   ├── usecase/          # Use cases
│   └── mapper/           # DTO <-> Domain mappers
├── presentation/
│   ├── theme/            # Material 3 theme
│   ├── navigation/       # Navigation graph
│   ├── screens/
│   │   ├── auth/         # Login, OTP verification
│   │   ├── dashboard/    # Home dashboard
│   │   ├── usage/        # Internet usage
│   │   ├── plan/         # Current plan details
│   │   ├── invoices/     # Invoice list & detail
│   │   ├── payment/      # Payment flow
│   │   ├── support/      # Ticket list & detail
│   │   ├── notifications/# Notification center
│   │   ├── profile/      # User profile
│   │   └── settings/     # App settings
│   └── viewmodel/        # ViewModels (MVI pattern)
├── di/                   # Hilt modules
└── utils/                # Extensions, helpers
```

### 17.4 Offline Support

| Data | Cache Duration | Sync Strategy |
|------|---------------|---------------|
| Plan details | 24 hours | Background sync on app open |
| Invoice list | 1 hour | Pull-to-refresh |
| Usage data | 5 minutes | Polling + WebSocket |
| Profile | 24 hours | Background sync |
| Notifications | 15 minutes | FCM + background sync |

Room database stores cached data for offline viewing. WorkManager handles background sync.

---

## 18. Customer iOS Application

### 18.1 Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Swift |
| UI | SwiftUI |
| Architecture | Clean Architecture (MVVM) |
| Networking | URLSession + async/await |
| Local DB | SwiftData / CoreData |
| Background | BackgroundTasks |
| Push | APNs |
| Navigation | NavigationStack |
| State | @Observable / ObservableObject |
| Image Loading | AsyncImage |
| DI | Factory / Manual |

### 18.2 Architecture

```
AeroXeApp/
├── Data/
│   ├── Network/
│   │   ├── API/          # API client
│   │   ├── DTO/          # Response models
│   │   └── Interceptor/  # Auth middleware
│   ├── Storage/
│   │   ├── Models/       # SwiftData models
│   │   └── Repository/   # Storage implementations
│   └── Repository/       # Repository implementations
├── Domain/
│   ├── Model/            # Domain models
│   ├── Repository/       # Repository protocols
│   └── UseCase/          # Use cases
├── Presentation/
│   ├── Theme/            # Custom theme
│   ├── Navigation/       # Router
│   ├── Screens/
│   │   ├── Auth/         # Login, OTP
│   │   ├── Dashboard/    # Home
│   │   ├── Usage/        # Bandwidth usage
│   │   ├── Plan/         # Plan details
│   │   ├── Invoices/     # Invoices
│   │   ├── Payment/      # Payment
│   │   ├── Support/      # Tickets
│   │   ├── Notifications/# Notifications
│   │   ├── Profile/      # Profile
│   │   └── Settings/     # Settings
│   └── ViewModels/       # ViewModels
├── DI/                   # Dependency injection
└── Core/                 # Extensions, utilities
```

### 18.3 Feature Parity

The iOS application mirrors the Android application feature-for-feature. API endpoints and data models are identical. The difference is only in platform-specific implementation (SwiftUI vs Compose, APNs vs FCM, SwiftData vs Room).

---

## 18A. Document Storage (MinIO)

All file uploads — both staff and customer — are stored in **MinIO** (S3-compatible object storage). Files are never served directly from the backend; all access is via presigned URLs.

### 18A.1 Architecture

```
Client (Admin Portal / Mobile App)
    ↓ (Upload Request)
Rust Axum Backend
    ├── Validate file type & size
    ├── Generate unique key: {branch}/{module}/{entity}/{uuid}.{ext}
    ├── Generate presigned PUT URL (upload)
    ├── Client uploads directly to MinIO via presigned URL
    ├── Client confirms upload to backend
    ├── Backend stores metadata in document_files table
    └── Returns document_id

Client (Read)
    ↓
Rust Axum Backend
    ├── Check RBAC permissions
    ├── Validate document ownership / access scope
    ├── Generate presigned GET URL (expires in 15 minutes)
    └── Returns URL → Client fetches from MinIO directly
```

### 18A.2 MinIO Configuration

```yaml
minio:
  endpoint: ${MINIO_ENDPOINT}         # https://storage.aeroxebroadband.com
  access_key: ${MINIO_ACCESS_KEY}
  secret_key: ${MINIO_SECRET_KEY}
  region: ap-south-1
  buckets:
    staff-documents: "aeroxe-staff-docs"       # Staff uploads
    customer-documents: "aeroxe-customer-docs"  # Customer uploads
  presigned:
    upload_expiry_seconds: 300     # 5 minutes to complete upload
    download_expiry_seconds: 900   # 15 minutes for download URLs
```

### 18A.3 Staff Document Upload (Backend Users)

Staff users (noc_engineer, field_technician, customer_support, etc.) can upload files via the backend API for purposes such as:
- Installation photos (fiber drop, ONT placement, router setup)
- Equipment documentation (photos of hardware, serial numbers)
- Support ticket attachments
- KYC document review copies
- Device configuration backups
- Network diagrams

**Allowed file types (Staff):**
| Category | Extensions | Max Size |
|----------|-----------|----------|
| Images | `.jpg`, `.jpeg`, `.png`, `.webp`, `.heic` | 20 MB |
| Videos | `.mp4`, `.mov`, `.avi`, `.mkv` | 100 MB |
| Documents | `.pdf`, `.doc`, `.docx`, `.xls`, `.xlsx` | 50 MB |
| Archives | `.zip`, `.rar` | 50 MB |

**Staff API Endpoints:**

```
POST   /api/v1/documents/upload-url        → Generate presigned upload URL
POST   /api/v1/documents/confirm-upload     → Confirm upload, store metadata
GET    /api/v1/documents/:id/download-url   → Generate presigned download URL
GET    /api/v1/documents                    → List documents (filtered by module/entity)
DELETE /api/v1/documents/:id                → Soft-delete document
```

**Staff Upload Flow:**

```
1. Staff selects file in admin portal
2. Frontend calls POST /api/v1/documents/upload-url
   ├── file_name: "installation-photo-001.jpg"
   ├── file_type: "image/jpeg"
   ├── file_size: 5242880
   ├── module: "installation"
   ├── entity_type: "installation_order"
   ├── entity_id: 1234
   └── branch_id: 1
3. Backend validates:
   ├── Check user has permission for the module/entity
   ├── Check branch access (branch-scoped users)
   ├── Validate file type against allowed list
   ├── Validate file size against limits
   └── Generate unique key: installations/1234/a1b2c3d4.jpg
4. Backend creates document_files record (status: 'uploading')
5. Returns presigned PUT URL + document_id
6. Frontend uploads file directly to MinIO via PUT
7. Frontend calls POST /api/v1/documents/confirm-upload
8. Backend verifies file exists in MinIO, updates status to 'completed'
9. File is now accessible via presigned GET URL
```

### 18A.4 Customer Document Upload (Mobile App)

Customers can upload documents via the mobile app for:
- KYC documents (Aadhaar, PAN, address proof)
- Support ticket attachments
- Payment proof (manual payment screenshots)
- Installation feedback photos

**Allowed file types (Customer):**
| Category | Extensions | Max Size |
|----------|-----------|----------|
| Images | `.jpg`, `.jpeg`, `.png`, `.webp` | 10 MB |
| Documents | `.pdf` | 10 MB |

**Stricter limits for customers:**
- No video uploads
- No archive uploads
- 10 MB max per file (vs 100 MB for staff)
- Max 5 files per upload request
- Total per-entity limit: 50 MB

**Customer API Endpoints:**

```
POST   /api/v1/customer/documents/upload-url   → Generate presigned upload URL
POST   /api/v1/customer/documents/confirm-upload → Confirm upload
GET    /api/v1/customer/documents/:id/download-url → Get download URL
GET    /api/v1/customer/documents               → List own documents
```

**Customer Upload Flow:**

```
1. Customer selects file in mobile app
2. App calls POST /api/v1/customer/documents/upload-url
   ├── file_name: "aadhaar-front.jpg"
   ├── file_type: "image/jpeg"
   ├── file_size: 3145728
   ├── module: "kyc"
   ├── entity_type: "customer_profile"
   └── entity_id: (own customer_id)
3. Backend validates:
   ├── Check user is authenticated (customer role)
   ├── Check entity belongs to this customer
   ├── Validate file type (images + PDF only)
   ├── Validate file size (<= 10 MB)
   └── Generate unique key: customers/{customer_id}/kyc/a1b2c3d4.jpg
4. Backend creates document_files record (status: 'uploading')
5. Returns presigned PUT URL + document_id
6. App uploads file directly to MinIO via PUT
7. App calls POST /api/v1/customer/documents/confirm-upload
8. Backend verifies, updates status to 'completed'
9. File accessible via presigned GET URL
```

### 18A.5 Database Tables

```sql
CREATE TABLE document_files (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT REFERENCES branches(id),
    uploaded_by BIGINT NOT NULL REFERENCES users(id),
    file_key TEXT NOT NULL UNIQUE,
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(50) NOT NULL,
    file_extension VARCHAR(10) NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    bucket VARCHAR(100) NOT NULL,
    module VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id BIGINT NOT NULL,
    status VARCHAR(20) DEFAULT 'uploading'
        CHECK (status IN ('uploading', 'completed', 'failed', 'deleted')),
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT REFERENCES users(id),
    checksum_sha256 VARCHAR(64),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE document_upload_audit (
    id BIGSERIAL PRIMARY KEY,
    document_id BIGINT NOT NULL REFERENCES document_files(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    action VARCHAR(20) NOT NULL
        CHECK (action IN ('upload_url_generated', 'upload_confirmed', 'downloaded', 'deleted')),
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_document_files_entity ON document_files(entity_type, entity_id);
CREATE INDEX idx_document_files_branch ON document_files(branch_id);
CREATE INDEX idx_document_files_module ON document_files(module);
CREATE INDEX idx_document_files_uploaded_by ON document_files(uploaded_by);
```

### 18A.6 File Validation Rules

```yaml
staff_uploads:
  images:
    extensions: [".jpg", ".jpeg", ".png", ".webp", ".heic"]
    max_size_mb: 20
  videos:
    extensions: [".mp4", ".mov", ".avi", ".mkv"]
    max_size_mb: 100
  documents:
    extensions: [".pdf", ".doc", ".docx", ".xls", ".xlsx"]
    max_size_mb: 50
  archives:
    extensions: [".zip", ".rar"]
    max_size_mb: 50

customer_uploads:
  images:
    extensions: [".jpg", ".jpeg", ".png", ".webp"]
    max_size_mb: 10
  documents:
    extensions: [".pdf"]
    max_size_mb: 10
  max_files_per_request: 5
  max_total_per_entity_mb: 50

validation:
  check_magic_bytes: true     # Validate file header matches extension
  check_mime_type: true       # Validate actual MIME type
  scan_virus: true            # ClamAV integration (optional)
  strip_metadata: true        # Strip EXIF data from images for privacy
```

### 18A.7 Storage Lifecycle

| Event | Action |
|-------|--------|
| Upload confirmed | Status → 'completed', checksum computed |
| Entity deleted (customer terminated) | Soft-delete documents (status → 'deleted') |
| Retention period (7 years for KYC) | Hard-delete from MinIO + database |
| Retention period (1 year for tickets) | Hard-delete from MinIO + database |
| Orphaned documents (entity missing) | Nightly job flags for review |

### 18A.8 Permissions

```
document.upload.staff
document.upload.customer
document.download
document.delete
document.view_metadata
document.admin.cleanup
```

---

## 19. Security Design

### 19.1 Authentication

| Method | Implementation | Use Case |
|--------|---------------|----------|
| Phone + OTP | SMS-based OTP (6 digits, 5 min expiry) | Customer login |
| Email + Password | Argon2id hashing, bcrypt fallback | Admin portal login |
| Biometric | Android BiometricPrompt / iOS Face ID | App quick unlock |
| API Key | HMAC-SHA256 signed | Service-to-service |
| JWT | RS256, 15-min access + 7-day refresh | Session management |

### 19.2 JWT Structure

```json
{
  "header": {
    "alg": "RS256",
    "typ": "JWT"
  },
  "payload": {
    "sub": "user-uuid",
    "email": "admin@aeroxe.com",
    "role": "network_admin",
    "permissions": ["device.*.view", "device.*.restart"],
    "iss": "aeroxe-platform",
    "iat": 1688829000,
    "exp": 1688829900,
    "jti": "token-uuid"
  }
}
```

### 19.3 Two-Factor Authentication

For admin portal users with elevated roles (super_admin, isp_owner, network_admin):

| Method | Provider | Backup |
|--------|----------|--------|
| TOTP | Google Authenticator / Authy | Recovery codes (10 single-use) |
| SMS OTP | MSG91 | Backup phone number |
| Hardware Key | YubiKey (FIDO2/WebAuthn) | TOTP fallback |

### 19.4 Encryption

| Data | Method | Key Management |
|------|--------|----------------|
| Passwords | Argon2id (memory: 64MB, iterations: 3) | Application-level |
| API keys | AES-256-GCM | AWS KMS / HashiCorp Vault |
| PII (Aadhaar, PAN) | SHA-256 hash (one-way) | Application-level |
| Database | PostgreSQL TDE (Transparent Data Encryption) | Infrastructure-level |
| Backups | AES-256 encrypted | Offsite key storage |
| TLS | TLS 1.3 (all external traffic) | Let's Encrypt / AWS ACM |

### 19.5 API Security

| Measure | Implementation |
|---------|---------------|
| Rate Limiting | 100 req/min per IP (general), 10 req/min per IP (auth) |
| CORS | Whitelist: `aeroxebroadband.com`, `localhost:5173` |
| Input Validation | Zod schema validation on all inputs |
| SQL Injection | SQLx parameterized queries (compile-time checked) |
| XSS | Input sanitization, CSP headers |
| CSRF | SameSite cookies, CSRF tokens for form submissions |
| Request Size Limit | 10MB max body size |
| Timeout | 30s request timeout |

### 19.6 Audit Logging

Every API request is logged with:

```json
{
  "request_id": "uuid",
  "user_id": "uuid",
  "method": "POST",
  "path": "/api/v1/customers/:id/suspend",
  "ip_address": "10.0.1.50",
  "user_agent": "Mozilla/5.0...",
  "status_code": 200,
  "duration_ms": 45,
  "permissions_checked": ["customer.account.disable"],
  "permission_result": "granted",
  "created_at": "2026-07-08T14:30:00Z"
}
```

### 19.7 Device Security

| Measure | Scope |
|---------|-------|
| SNMPv3 (encrypted) | All network devices |
| SSH key-based auth | OLT, ONT management |
| Firmware integrity check | Before firmware updates |
| Configuration backup | Before any config change |
| Access control lists | Management interface restricted to admin VLAN |

---

## 20. DevOps and Production Deployment

### 20.1 Docker Configuration

```dockerfile
# Backend
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/aeroxe-platform /usr/local/bin/
EXPOSE 8080
CMD ["aeroxe-platform"]

# Frontend
FROM node:20-alpine as builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY frontend/public/.htaccess /usr/share/nginx/html/
EXPOSE 80
```

### 20.2 Docker Compose (Development)

```yaml
version: '3.8'
services:
  backend:
    build: ./backend
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgres://aeroxe:secret@postgres:5432/aeroxe
      REDIS_URL: redis://redis:6379
      NATS_URL: nats://nats:4222
    depends_on:
      - postgres
      - redis
      - nats

  frontend:
    build: ./frontend
    ports:
      - "5173:5173"
    depends_on:
      - backend

  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: aeroxe
      POSTGRES_USER: aeroxe
      POSTGRES_PASSWORD: secret
    volumes:
      - pgdata:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

  nats:
    image: nats:2.10-alpine
    ports:
      - "4222:4222"
      - "8222:8222"

volumes:
  pgdata:
```

### 20.3 Kubernetes Architecture

```yaml
# Namespace
apiVersion: v1
kind: Namespace
metadata:
  name: aeroxe-production

# Backend Deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: aeroxe-backend
  namespace: aeroxe-production
spec:
  replicas: 3
  selector:
    matchLabels:
      app: aeroxe-backend
  template:
    spec:
      containers:
        - name: backend
          image: aeroxe/backend:latest
          ports:
            - containerPort: 8080
          resources:
            requests:
              memory: "512Mi"
              cpu: "500m"
            limits:
              memory: "2Gi"
              cpu: "2000m"
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: aeroxe-secrets
                  key: database-url

# Horizontal Pod Autoscaler
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: aeroxe-backend-hpa
  namespace: aeroxe-production
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: aeroxe-backend
  minReplicas: 3
  maxReplicas: 20
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: 80
```

### 20.4 CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI/CD
on:
  push:
    branches: [main, dev]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        run: cargo test
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Check formatting
        run: cargo fmt --check

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: docker build -t aeroxe/backend:${{ github.sha }} .
      - name: Push to registry
        run: docker push aeroxe/backend:${{ github.sha }}

  deploy:
    needs: build
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to Kubernetes
        run: |
          kubectl set image deployment/aeroxe-backend \
            backend=aeroxe/backend:${{ github.sha }} \
            -n aeroxe-production
```

### 20.5 Monitoring Stack

| Tool | Purpose | Metrics |
|------|---------|---------|
| **Prometheus** | Metrics collection | CPU, memory, request rate, error rate, latency |
| **Grafana** | Dashboards & alerts | Business metrics, system metrics, network metrics |
| **OpenTelemetry** | Distributed tracing | Request traces across services |
| **Loki** | Log aggregation | Application logs, audit logs |
| **Alertmanager** | Alert routing | PagerDuty, Slack, email alerts |

### 20.6 Key Dashboards

1. **Business Dashboard** — Revenue, customers, churn, plan popularity
2. **Infrastructure Dashboard** — CPU, memory, disk, network I/O
3. **Application Dashboard** — Request rate, error rate, P50/P95/P99 latency
4. **Network Dashboard** — Device health, bandwidth utilization, online customers
5. **Billing Dashboard** — Payment success rate, dunning metrics, outstanding amount

### 20.7 Alert Rules

| Alert | Condition | Severity | Channel |
|-------|-----------|----------|---------|
| High Error Rate | 5xx > 1% for 5 min | Critical | PagerDuty |
| High Latency | P99 > 2s for 5 min | Warning | Slack |
| Database Connection Pool Exhausted | Available < 10% | Critical | PagerDuty |
| OLT Device Offline | Status = offline for 2 min | Critical | PagerDuty + SMS |
| ONT Power Low | Optical power < -28 dBm | Warning | Slack |
| Disk Usage > 80% | Disk usage > 80% | Warning | Slack |
| Certificate Expiry | < 14 days | Warning | Email |
| Daily Revenue Drop | > 30% vs previous day | Warning | Slack |

---

## 21. System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLIENT LAYER                             │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │   Android    │  │     iOS      │  │    Admin Portal      │  │
│  │   App        │  │    App       │  │    (React SPA)       │  │
│  │  (Kotlin/    │  │  (Swift/     │  │    React + TS +      │  │
│  │   Compose)   │  │   SwiftUI)   │  │    TailwindCSS       │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬───────────┘  │
│         │                  │                      │               │
│         └──────────────────┼──────────────────────┘               │
│                            │                                      │
│                     ┌──────▼───────┐                              │
│                     │   HTTPS /    │                              │
│                     │   WebSocket  │                              │
│                     └──────┬───────┘                              │
└────────────────────────────┼────────────────────────────────────┘
                             │
┌────────────────────────────┼────────────────────────────────────┐
│                     GATEWAY LAYER                                │
│                     ┌──────▼───────┐                              │
│                     │  API Gateway │                              │
│                     │  (Nginx/     │                              │
│                     │   Traefik)   │                              │
│                     │  - TLS       │                              │
│                     │  - Rate Limit│                              │
│                     │  - CORS      │                              │
│                     └──────┬───────┘                              │
└────────────────────────────┼────────────────────────────────────┘
                             │
┌────────────────────────────┼────────────────────────────────────┐
│                   APPLICATION LAYER                              │
│                     ┌──────▼───────┐                              │
│                     │  Rust Axum   │                              │
│                     │  Platform    │                              │
│                     │              │                              │
│   ┌─────────────────┤  Modules:    ├─────────────────┐           │
│   │                 │              │                 │           │
│   │  ┌─────┐ ┌─────┐│ ┌─────┐ ┌──┴──┐ ┌─────┐     │           │
│   │  │Auth │ │ RBAC││ │Cust │ │Bill │ │Plan │     │           │
│   │  └─────┘ └─────┘│ └─────┘ └─────┘ └─────┘     │           │
│   │  ┌─────┐ ┌─────┐│ ┌─────┐ ┌─────┐ ┌─────┐     │           │
│   │  │Bw   │ │Dev  ││ │Net  │ │Notif│ │Audit│     │           │
│   │  └─────┘ └─────┘│ └─────┘ └─────┘ └─────┘     │           │
│   │  ┌─────┐ ┌─────┐│ ┌─────┐                       │           │
│   │  │Rpt  │ │Inst ││ │Supp │                       │           │
│   │  └─────┘ └─────┘│ └─────┘                       │           │
│   │                 │              │                 │           │
│   └─────────────────┤  Shared:     ├─────────────────┘           │
│                     │  - Middleware │                              │
│                     │  - Error Hdl  │                              │
│                     │  - Validation │                              │
│                     └──────┬───────┘                              │
└────────────────────────────┼────────────────────────────────────┘
                             │
┌────────────────────────────┼────────────────────────────────────┐
│                     DATA LAYER                                   │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │  PostgreSQL  │  │    Redis     │  │     NATS             │  │
│  │  (Primary)   │  │  (Cache +    │  │  (Event Bus +        │  │
│  │  (16+)       │  │   Pub/Sub +  │  │   Messaging)         │  │
│  │              │  │   Sessions)  │  │  (2.10+)             │  │
│  │  - Tables    │  │              │  │                      │  │
│  │  - Partitions│  │  - Sessions  │  │  - customer.*        │  │
│  │  - Indexes   │  │  - Cache     │  │  - billing.*         │  │
│  │  - Event     │  │  - Rate Limit│  │  - bandwidth.*       │  │
│  │    Store     │  │  - Queues    │  │  - device.*          │  │
│  └──────────────┘  └──────────────┘  │  - network.*         │  │
│                                       │  - support.*         │  │
│  ┌──────────────┐                     │  - notification.*    │  │
│  │  PostgreSQL  │                     │  - audit.*           │  │
│  │  (Read       │                     └──────────┬───────────┘  │
│  │   Replica)   │                                │               │
│  └──────────────┘                                │               │
└──────────────────────────────────────────────────┼───────────────┘
                                                   │
┌──────────────────────────────────────────────────┼───────────────┐
│                   NETWORK LAYER                                  │
│                                                   │               │
│                     ┌──────────────┐              │               │
│                     │  Bandwidth   │◄─────────────┘               │
│                     │  Engine      │                              │
│                     └──────┬───────┘                              │
│                            │                                      │
│                     ┌──────▼───────┐                              │
│                     │   Network    │                              │
│                     │   Device     │                              │
│                     │   Controller │                              │
│                     └──────┬───────┘                              │
│                            │                                      │
│              ┌─────────────┼─────────────┐                        │
│              │             │             │                        │
│       ┌──────▼──────┐ ┌───▼────┐ ┌─────▼──────┐                 │
│       │  MikroTik   │ │ Huawei │ │  ZTE       │                 │
│       │  Router     │ │  OLT   │ │  OLT       │                 │
│       │  (RouterOS) │ │(MA56xx)│ │  (C300)    │                 │
│       └──────┬──────┘ └───┬────┘ └─────┬──────┘                 │
│              │            │            │                          │
│       ┌──────▼──────┐    │      ┌─────▼──────┐                  │
│       │ Distribution│    │      │ Distribution│                  │
│       │ Switch      │    │      │ Switch      │                  │
│       └──────┬──────┘    │      └─────┬──────┘                  │
│              │           │            │                          │
│              │     ┌─────▼─────┐      │                          │
│              │     │ Splitter  │      │                          │
│              │     │ (1:32/64) │      │                          │
│              │     └─────┬─────┘      │                          │
│              │           │            │                          │
│              │     ┌─────▼─────┐      │                          │
│              │     │   ONT     │      │                          │
│              │     │(Huawei/   │      │                          │
│              │     │ ZTE)      │      │                          │
│              │     └─────┬─────┘      │                          │
│              │           │            │                          │
│              │     ┌─────▼─────┐      │                          │
│              │     │ Customer  │      │                          │
│              │     │ Premises  │      │                          │
│              │     └───────────┘      │                          │
└──────────────────────────────────────────────────────────────────┘
```

---

## Appendix A: API Versioning Strategy

All APIs are versioned: `/api/v1/...`

Breaking changes require a new version (`/api/v2/...`). Non-breaking additions (new fields, new endpoints) are added to the current version.

## Appendix B: Coding Standards

- **Rust:** Follow Rust API Guidelines, use `clippy` with strict lints
- **TypeScript:** Follow ESLint recommended + Prettier
- **Kotlin:** Follow Kotlin Coding Conventions, ktlint
- **Swift:** Follow Swift API Design Guidelines

## Appendix C: Data Retention Policy

| Data Type | Retention Period | Archive Strategy |
|-----------|-----------------|------------------|
| Active customer data | Indefinite | — |
| Terminated customer data | 7 years | Compress after 1 year |
| Audit logs | 7 years | Compress after 1 year |
| Event store | 2 years | Archive to S3 after 6 months |
| Device metrics | 90 days | Delete after 90 days |
| Device logs | 30 days | Delete after 30 days |
| Bandwidth usage | 1 year | Compress after 3 months |
| Notifications | 90 days | Delete after 90 days |
| Sessions | 24 hours | Auto-expire |

## Appendix D: Scalability Considerations

| Dimension | Current (Year 1) | Year 2 | Year 3 | Year 5 |
|-----------|-------------------|--------|--------|--------|
| Customers | 5,000 | 15,000 | 40,000 | 100,000 |
| Devices | 50 | 200 | 500 | 2,000 |
| Cities | 1 | 3 | 6 | 15 |
| Daily transactions | 10,000 | 50,000 | 200,000 | 1,000,000 |
| Concurrent WebSocket | 200 | 1,000 | 5,000 | 20,000 |

**Scaling strategy:**

1. **Read replicas** for PostgreSQL (Year 2)
2. **Connection pooling** with PgBouncer (Year 1)
3. **Redis Cluster** for horizontal scaling (Year 2)
4. **NATS JetStream** for event persistence (Year 1)
5. **Kubernetes HPA** for backend auto-scaling (Year 1)
6. **CDN** for static assets (Year 1)
7. **Multi-region** deployment (Year 3+)
8. **Database sharding** by city (Year 3+)

---

*End of Document*
