# AeroXe Backend — Plans Module

> **Req Ref:** §4 Product and Plan Management

---

## 1. Overview

Manages internet plans, their pricing across billing periods, speed profiles, and optional service packages. Plans are company-wide (not branch-scoped). Includes a checker/maker approval workflow.

## 2. Database Tables

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

## 3. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/plans/list` | No (public) | List active plans |
| POST | `/api/v1/plans/get` | No (public) | Get plan details |
| POST | `/api/v1/admin/plans/create` | finance_manager+ | Create plan |
| PATCH | `/api/v1/admin/plans/update` | finance_manager+ | Update plan |
| DELETE | `/api/v1/admin/plans/delete` | finance_manager+ | Soft-delete plan |
| POST | `/api/v1/admin/plans/publish` | finance_manager+ | Publish plan |
| POST | `/api/v1/admin/plans/unpublish` | finance_manager+ | Unpublish plan |
| POST | `/api/v1/admin/plans/clone` | finance_manager+ | Clone plan |
| PATCH | `/api/v1/admin/plans/pricing` | finance_manager+ | Update pricing |
| POST | `/api/v1/admin/plans/speed-profile/list` | network_admin+ | Get speed profile |
| POST | `/api/v1/admin/plans/speed-profile` | network_admin+ | Create/update speed profile |
| POST | `/api/v1/admin/plans/history/list` | finance_manager+ | View change history |

## 4. Plan Data (Seed)

```json
[
  { "slug": "basic-50", "name": "Basic", "download_mbps": 50, "upload_mbps": 25, "burst_mbps": 75, "pricing": { "1": 400, "3": 1150, "6": 2250, "12": 4300 } },
  { "slug": "standard-100", "name": "Standard", "download_mbps": 100, "upload_mbps": 50, "burst_mbps": 150, "pricing": { "1": 600, "3": 1700, "6": 3350, "12": 6400 } },
  { "slug": "premium-150", "name": "Premium", "download_mbps": 150, "upload_mbps": 75, "burst_mbps": 200, "pricing": { "1": 800, "3": 2300, "6": 4550, "12": 8700 } },
  { "slug": "pro-200", "name": "Pro", "download_mbps": 200, "upload_mbps": 100, "burst_mbps": 250, "pricing": { "1": 1000, "3": 2850, "6": 5650, "12": 10800 } },
  { "slug": "ultimate-300", "name": "Ultimate", "download_mbps": 300, "upload_mbps": 150, "burst_mbps": 400, "pricing": { "1": 1300, "3": 3700, "6": 7350, "12": 14000 } }
]
```

## 5. Speed Profile Structure

```rust
pub struct SpeedProfile {
    pub plan_id: i64,
    pub download_limit_kbps: i32,
    pub upload_limit_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: i32,
    pub priority_queue: i32,
    pub qos_marking: Option<String>,
    pub htb_parent_queue: Option<String>,
    pub fq_codel_enabled: bool,
    pub device_type: String,
}
```

## 6. Events Published

```yaml
plan.created:
  payload: { plan_id, name, slug }
plan.updated:
  payload: { plan_id, changes }
plan.published:
  payload: { plan_id, name }
plan.unpublished:
  payload: { plan_id, name }
```

## 7. RBAC Permissions

```
plan.view
plan.create
plan.update
plan.delete
plan.publish
plan.unpublish
plan.clone
plan.speed_profile.view
plan.speed_profile.create
plan.speed_profile.update
plan.speed_profile.delete
plan.package.view
plan.package.create
plan.package.update
plan.package.delete
```
