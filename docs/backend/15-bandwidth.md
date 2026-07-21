# AeroXe Backend — Bandwidth Control Module

> **Req Ref:** §5 Bandwidth Control System

---

## 1. Overview

Manages bandwidth profiles, speed limiting, QoS configuration, and applies rate limits to network devices (MikroTik, Huawei OLT, ZTE OLT, Cisco). Communicates with devices via NATS events and the Network Device Controller.

## 2. Architecture

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

## 3. Database Tables

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

CREATE TABLE bandwidth_profiles_history (
    id BIGSERIAL PRIMARY KEY,
    profile_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
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

## 4. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/bandwidth/profiles/list` | network_admin+ | List profiles |
| POST | `/api/v1/bandwidth/profiles/create` | network_admin+ | Create profile |
| POST | `/api/v1/bandwidth/profiles/get` | network_admin+ | Get profile |
| PATCH | `/api/v1/bandwidth/profiles/update` | network_admin+ | Update profile |
| DELETE | `/api/v1/bandwidth/profiles/delete` | network_admin+ | Delete profile |
| POST | `/api/v1/bandwidth/profiles/apply` | network_admin+ | Apply to all subscribers |
| POST | `/api/v1/bandwidth/apply` | network_admin+ | Apply to specific subscription |
| POST | `/api/v1/bandwidth/applications/list` | network_admin+ | List application statuses |
| POST | `/api/v1/bandwidth/usage/list` | customer_ops | Get usage data |

## 5. Speed Profile Application Flow

```
1. Admin creates/updates bandwidth profile
2. Backend validates and persists
3. Publishes bandwidth.profile.updated to NATS
4. Bandwidth Engine subscribes to event
5. Engine resolves affected devices:
   subscription → customer → device mapping
6. Sends commands to Network Device Controller
7. Controller translates to vendor-specific commands:

   MikroTik:
   /queue/simple add name="customer-rahul" \
       target=192.168.1.100/32 \
       max-limit=100M/50M \
       burst-limit=150M/75M \
       burst-threshold=80M/40M \
       burst-time=30s/30s \
       priority=2

   Huawei OLT:
   traffic-profile name "profile-100m" dba-index 1
     type4 max-bandwidth 102400
   ont traffic-profile 0 1 profile-100m

8. Controller verifies configuration applied
9. Publishes bandwidth.profile.applied event
10. Backend updates application status
```

## 6. Failure Handling

| Failure Type | Strategy |
|-------------|----------|
| Device unreachable | Retry 3x with exponential backoff (10s, 30s, 90s) |
| Command rejected | Log error, alert NOC, manual intervention |
| Partial application | Rollback to previous profile, alert NOC |
| Device reboot needed | Queue reboot with 5-minute grace period |
| Profile conflict | Resolve by priority (highest wins) |

## 7. Events Published

```yaml
bandwidth.profile.created:
  payload: { profile_id, plan_id, download_kbps, upload_kbps }
bandwidth.profile.updated:
  payload: { profile_id, changes, affected_subscriptions }
bandwidth.profile.applied:
  payload: { profile_id, subscription_id, device_id, applied_at }
bandwidth.profile.failed:
  payload: { profile_id, subscription_id, device_id, error, retry_count }
```

## 8. RBAC Permissions

```
bandwidth.profile.view
bandwidth.profile.create
bandwidth.profile.update
bandwidth.profile.delete
bandwidth.profile.apply
bandwidth.qos.view
bandwidth.qos.create
bandwidth.qos.update
bandwidth.qos.delete
bandwidth.rate_limit.view
bandwidth.rate_limit.create
bandwidth.rate_limit.update
bandwidth.rate_limit.delete
```

---

## Known Issues & Gap Reference (v2.0)

> **Cross-reference:** `GAP-code-bugs.md` §5, `DESIGN-GAPS-DEEP-ANALYSIS.md` §9.5

| Bug ID | Severity | Issue | Location |
|--------|----------|-------|----------|
| BUG-BW-01 | CRITICAL | `apply_profile` only flips DB flag — no device push, speed limits unenforced | `service.rs:167-189` |
| BUG-BW-02 | CRITICAL | `device_id` never set on profile application — worker can't find target device | `service.rs:191-211` |
| BUG-BW-03 | HIGH | `verify_applied_profiles` is a no-op — returns `Ok(())` without checking | `bandwidth_worker.rs:148-174` |
| BUG-BW-04 | HIGH | `get_usage` returns profile config, not actual usage statistics | `service.rs:232-257` |
| BUG-BW-05 | MEDIUM | Worker processes only 20 items per cycle — hours to propagate at scale | `bandwidth_worker.rs:43` |

**Priority:** Fix BW-01, 02 first (speed limits completely non-functional). See `GAP-IMPLEMENTATION-ROADMAP.md` Phase 5.
