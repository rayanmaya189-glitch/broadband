# AeroXe Backend — Appendices

## Appendix A: API Versioning Strategy

### Versioning Scheme

All APIs are versioned using a semantic path structure:

```
https://api.aeroxebroadband.com/api/v1/...
https://api.aeroxebroadband.com/api/v2/...
```

### Rules

| Change Type | Action | Example |
|-------------|--------|---------|
| **Breaking change** | Create new version (`v2`) | Renaming a field, changing response structure, removing an endpoint |
| **Non-breaking addition** | Add to current version | New optional field in response, new endpoint |
| **Bug fix** | Add to current version | Fixing incorrect calculation, fixing status code |
| **Deprecation** | Add `Sunset` header, support for 6 months | `Sunset: Sat, 01 Jan 2028 00:00:00 GMT` |

### Breaking Changes (Require New Version)

- Removing an endpoint
- Renaming a field in request/response
- Changing the type of a field (e.g., string → object)
- Changing the semantics of an existing field
- Removing a query parameter
- Changing authentication requirements

### Non-Breaking Changes (Same Version)

- Adding new optional fields to request/response
- Adding new endpoints
- Adding new optional query parameters
- Adding new enum values
- Adding new HTTP headers

### Version Routing (Rust Axum)

```rust
// src/api/mod.rs
pub fn api_routes() -> Router {
    Router::new()
        .nest("/api/v1", v1_routes())
        .nest("/api/v2", v2_routes())
        .fallback(redirect_to_latest)
}

fn v1_routes() -> Router {
    Router::new()
        .nest("/auth", auth::v1_routes())
        .nest("/customers", customers::v1_routes())
        .nest("/billing", billing::v1_routes())
        // ... other v1 routes
}

fn v2_routes() -> Router {
    Router::new()
        .nest("/auth", auth::v2_routes())
        // ... v2 routes (only when breaking changes occur)
}
```

### Sunset Header

When deprecating a version:

```rust
use axum::http::header::SUNSET;

fn sunset_middleware() -> Middleware {
    on_response(move |response: &mut Response| {
        if current_api_version() < latest_api_version() {
            response.headers_mut().insert(
                "Sunset",
                "Sat, 01 Jan 2028 00:00:00 GMT".parse().unwrap()
            );
            response.headers_mut().insert(
                "Deprecation",
                "true".parse().unwrap()
            );
            response.headers_mut().insert(
                "Link",
                "</api/v2>; rel=\"successor-version\"".parse().unwrap()
            );
        }
    })
}
```

### Client Versioning Headers

```
Accept-Version: v1
X-API-Version: v1
```

Or via path (preferred):
```
GET /api/v1/customers
GET /api/v2/customers
```

### Migration Guide Template

When releasing a new version:

```markdown
# API v2 Migration Guide

## Breaking Changes
1. `GET /api/v1/customers/:id` → Response field `phone` renamed to `phone_number`
2. `POST /api/v1/auth/otp/verify` → Response structure changed

## Migration Steps
1. Update base URL: `/api/v1/` → `/api/v2/`
2. Update response parsing for affected fields
3. Test with staging environment

## Timeline
- v1 Sunset: January 1, 2028
- v2 Available: July 1, 2026
```

---

## Appendix B: Coding Standards

### Rust (Backend)

Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and enforce with `clippy`:

```toml
# clippy.toml
msrv = "1.75"
type-complexity-threshold = 250
too-many-arguments-threshold = 8

# .cargo/config.toml
[target.'cfg(all())']
rustflags = [
    "-D warnings",
    "-D clippy::all",
    "-D clippy::pedantic",
    "-D clippy::nursery",
    "-W clippy::cargo",
]
```

#### Key Rules
| Rule | Description |
|------|-------------|
| `clippy::all` | All default clippy lints as errors |
| `clippy::pedantic` | Strict pedantic lints |
| `clippy::nursery` | Experimental lints (review before enabling) |
| `clippy::unwrap_used` | Disallow `unwrap()` in production code |
| `clippy::expect_used` | Disallow `expect()` — use `?` or `match` |
| `clippy::panic` | Disallow `panic!()` in library code |
| `clippy::module_name_repetitions` | No repeating module name in type names |
| `clippy::must_use_candidate` | Mark functions that return values as `#[must_use]` |

#### Format
```bash
# Check
cargo fmt --all -- --check

# Fix
cargo fmt --all
```

### TypeScript (Frontend/Admin)

Follow ESLint recommended rules and Prettier:

```json
// .eslintrc.json
{
  "extends": [
    "eslint:recommended",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    "plugin:@typescript-eslint/recommended"
  ],
  "rules": {
    "@typescript-eslint/no-unused-vars": "error",
    "@typescript-eslint/no-explicit-any": "error",
    "react-hooks/exhaustive-deps": "warn",
    "no-console": "warn"
  }
}
```

```json
// .prettierrc
{
  "semi": true,
  "trailingComma": "es5",
  "singleQuote": true,
  "printWidth": 100,
  "tabWidth": 2
}
```

#### Key Rules
| Rule | Description |
|------|-------------|
| No `any` | Disallow `any` type (use `unknown` or proper types) |
| No unused imports | Remove all unused variables and imports |
| Consistent naming | camelCase for variables/functions, PascalCase for components/types |
| Strict null checks | Always handle `null`/`undefined` explicitly |

### Kotlin (Android)

Follow [Kotlin Coding Conventions](https://kotlinlang.org/docs/coding-conventions.html) and enforce with `ktlint`:

```kotlin
// .editorconfig
[*.{kt,kts}]
indent_style = space
indent_size = 4
max_line_length = 120
```

#### Key Rules
| Rule | Description |
|------|-------------|
| Naming | camelCase for properties/functions, PascalCase for classes |
| Coroutines | Use structured concurrency, avoid `GlobalScope` |
| Null safety | Prefer `?.` and `?:` over `!!` |
| Immutability | Use `val` over `var` whenever possible |
| Data classes | Use for simple data holders |
| Sealed classes | Use for closed type hierarchies |

### Swift (iOS)

Follow [Swift API Design Guidelines](https://www.swift.org/documentation/api-design-guidelines/):

```yaml
# .swiftlint.yml
disabled_rules:
  - trailing_whitespace
opt_in_rules:
  - empty_count
  - closure_spacing
  - force_unwrapping
  - implicitly_unwrapped_optional
line_length:
  warning: 120
  error: 200
```

#### Key Rules
| Rule | Description |
|------|-------------|
| Naming | camelCase for properties/methods, PascalCase for types |
| Optionals | Prefer `guard let` over force unwrapping |
| Access control | Use `private`/`fileprivate` by default |
| Value types | Prefer `struct` over `class` unless reference semantics needed |
| Protocol-oriented | Use protocols for abstraction |

---

## Appendix C: Data Retention Policy

### Retention Schedule

| Data Type | Retention | Archive/Compression | Deletion |
|-----------|-----------|---------------------|----------|
| Active customer data | Indefinite | — | — |
| Terminated customer data | 7 years | Compress after 1 year | Delete after 7 years |
| Audit logs | 7 years | Compress after 1 year | Delete after 7 years |
| Event store (NATS JetStream) | 2 years | Archive to S3 after 6 months | Delete after 2 years |
| Device metrics (SNMP) | 90 days | — | Delete after 90 days |
| Device logs | 30 days | — | Delete after 30 days |
| Bandwidth usage records | 1 year | Compress after 3 months | Delete after 1 year |
| Push notifications | 90 days | — | Delete after 90 days |
| Sessions (Redis) | 24 hours | — | Auto-expired |
| Refresh tokens | 7 days | — | Auto-expired |
| OTP codes | 5 minutes | — | Auto-expired |
| Presigned URLs | 5 minutes | — | Auto-expired |

### Implementation

#### PostgreSQL Partitioning (Audit Logs)

```sql
-- Monthly partitioned audit logs
CREATE TABLE audit_logs (
    id BIGSERIAL,
    user_id UUID NOT NULL,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID,
    details JSONB,
    ip_address INET,
    branch_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Create monthly partitions
CREATE TABLE audit_logs_2026_01 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-01-01') TO ('2026-02-01');

-- Auto-create partitions (run monthly via cron/pg_cron)
CREATE OR REPLACE FUNCTION create_monthly_partition()
RETURNS void AS $$
DECLARE
    next_month DATE := DATE_TRUNC('month', NOW() + INTERVAL '1 month');
    partition_name TEXT := 'audit_logs_' || TO_CHAR(next_month, 'YYYY_MM');
BEGIN
    EXECUTE FORMAT(
        'CREATE TABLE IF NOT EXISTS %I PARTITION OF audit_logs FOR VALUES FROM (%L) TO (%L)',
        partition_name,
        next_month,
        next_month + INTERVAL '1 month'
    );
END;
$$ LANGUAGE plpgsql;
```

#### Data Cleanup Jobs

```rust
// src/jobs/cleanup.rs

/// Remove expired data based on retention policy
pub async fn run_cleanup(pool: &PgPool) -> Result<()> {
    // 1. Delete expired OTP codes (5 minutes)
    sqlx::query("DELETE FROM otp_codes WHERE expires_at < NOW()")
        .execute(pool).await?;

    // 2. Delete expired sessions (24 hours)
    sqlx::query("DELETE FROM sessions WHERE expires_at < NOW()")
        .execute(pool).await?;

    // 3. Delete expired refresh tokens (7 days)
    sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < NOW()")
        .execute(pool).await?;

    // 4. Delete old device metrics (90 days)
    sqlx::query("DELETE FROM device_metrics WHERE recorded_at < NOW() - INTERVAL '90 days'")
        .execute(pool).await?;

    // 5. Delete old device logs (30 days)
    sqlx::query("DELETE FROM device_logs WHERE recorded_at < NOW() - INTERVAL '30 days'")
        .execute(pool).await?;

    // 6. Delete old notifications (90 days)
    sqlx::query("DELETE FROM notifications WHERE created_at < NOW() - INTERVAL '90 days'")
        .execute(pool).await?;

    // 7. Compress old audit logs (1 year) — move to compressed table
    // Handled by partition maintenance job

    // 8. Archive old event store to S3 (6 months)
    // Handled by NATS JetStream retention policy

    Ok(())
}

/// Schedule: Run daily at 2 AM
pub fn cleanup_schedule() -> JobScheduler {
    let mut scheduler = JobScheduler::new().unwrap();
    scheduler.add(
        Job::new("0 2 * * *", move |_, _| {
            let pool = get_pool();
            tokio::spawn(run_cleanup(&pool));
        }).unwrap()
    ).unwrap();
    scheduler
}
```

#### Redis Key Expiration

```rust
// Redis keys with automatic expiration
pub struct RedisKeyPatterns;

impl RedisKeyPatterns {
    pub fn session(customer_id: &str) -> (String, Duration) {
        (format!("session:{}", customer_id), Duration::from_secs(86400)) // 24h
    }

    pub fn otp(phone: &str) -> (String, Duration) {
        (format!("otp:{}", phone), Duration::from_secs(300)) // 5 min
    }

    pub fn rate_limit(key: &str) -> (String, Duration) {
        (format!("rate:{}", key), Duration::from_secs(60)) // 1 min
    }

    pub fn websocket_session(session_id: &str) -> (String, Duration) {
        (format!("ws:session:{}", session_id), Duration::from_secs(3600)) // 1h
    }

    pub fn cache(key: &str, ttl_seconds: u64) -> (String, Duration) {
        (format!("cache:{}", key), Duration::from_secs(ttl_seconds))
    }
}
```

---

## Appendix D: Scalability Considerations

### Growth Projections (5-Year Plan)

| Dimension | Year 1 | Year 2 | Year 3 | Year 5 |
|-----------|--------|--------|--------|--------|
| Customers | 5,000 | 15,000 | 40,000 | 100,000 |
| Devices | 50 | 200 | 500 | 2,000 |
| Cities | 1 | 3 | 6 | 15 |
| Daily transactions | 10,000 | 50,000 | 200,000 | 1,000,000 |
| Concurrent WebSocket | 200 | 1,000 | 5,000 | 20,000 |

### Scaling Strategy by Year

#### Year 1 (5K customers)

| Component | Strategy | Details |
|-----------|----------|---------|
| Database | Single PostgreSQL | With PgBouncer connection pooling |
| Cache | Single Redis | With persistence |
| Events | NATS JetStream | Single node with file-backed storage |
| Compute | Kubernetes HPA | 2-4 backend pods, auto-scale on CPU |
| WebSocket | Single server | Via Axum + Tokio |
| Storage | MinIO single node | For documents |
| CDN | CloudFlare | Static assets + API caching |

**Infrastructure:**
```
┌─────────────────────────────────────────┐
│              CloudFlare CDN             │
├─────────────────────────────────────────┤
│         Kubernetes Cluster              │
│  ┌──────────┐  ┌──────────┐            │
│  │ Backend  │  │ Backend  │  (2-4 pods)│
│  │ (Axum)   │  │ (Axum)   │            │
│  └────┬─────┘  └────┬─────┘            │
│       └──────┬──────┘                   │
│              │                          │
│  ┌───────────┼───────────┐              │
│  │           │           │              │
│  ▼           ▼           ▼              │
│ PgBouncer  Redis      NATS             │
│    │                      │            │
│    ▼                      ▼            │
│ PostgreSQL           NATS JetStream    │
│ (single)             (single)          │
└─────────────────────────────────────────┘
```

#### Year 2 (15K customers)

| Component | Strategy | Details |
|-----------|----------|---------|
| Database | Read replicas | 1 primary + 2 read replicas |
| Cache | Redis Cluster | 3 nodes (1 primary + 2 replicas) |
| Compute | HPA + increased limits | 4-8 backend pods |
| WebSocket | Sticky sessions | Route to same pod via session affinity |

**Infrastructure:**
```
┌─────────────────────────────────────────┐
│              CloudFlare CDN             │
├─────────────────────────────────────────┤
│         Kubernetes Cluster              │
│  ┌──────────────────────────────┐       │
│  │    Backend Pods (4-8)        │       │
│  │    (Axum + HPA)              │       │
│  └──────────────┬───────────────┘       │
│                 │                       │
│  ┌──────────────┼───────────────┐       │
│  │              │               │       │
│  ▼              ▼               ▼       │
│ PgBouncer    Redis Cluster    NATS      │
│    │         (3 nodes)        Cluster   │
│    ▼              │            (3 nodes)│
│ PostgreSQL       ▼               │      │
│ Primary+2 Repl  Redis            ▼      │
│               Replicas      JetStream   │
└─────────────────────────────────────────┘
```

#### Year 3+ (40K+ customers)

| Component | Strategy | Details |
|-----------|----------|---------|
| Database | Multi-region + sharding | Shard by city/region |
| Cache | Multi-region Redis | Per-region Redis clusters |
| Events | Multi-region NATS | Geo-distributed JetStream |
| Compute | Multi-region K8s | Primary + secondary regions |
| Storage | Multi-region MinIO | S3-compatible multi-site replication |
| CDN | Multi-CDN | CloudFlare + regional CDN |

### Performance Benchmarks (Targets)

| Metric | Year 1 | Year 3 | Year 5 |
|--------|--------|--------|--------|
| API response time (p50) | < 50ms | < 100ms | < 150ms |
| API response time (p99) | < 200ms | < 500ms | < 1s |
| WebSocket latency | < 50ms | < 100ms | < 200ms |
| Database query time (p95) | < 10ms | < 50ms | < 100ms |
| Uptime | 99.9% | 99.95% | 99.99% |

### Horizontal Scaling Triggers

| Metric | Threshold | Action |
|--------|-----------|--------|
| CPU usage | > 70% sustained | Scale up backend pods |
| Memory usage | > 80% | Scale up backend pods |
| DB connections | > 80% pool capacity | Add PgBouncer + read replicas |
| Redis memory | > 75% | Add Redis nodes |
| WebSocket connections | > 1000/pod | Add WebSocket pods |
| Queue depth | > 10,000 messages | Scale NATS consumers |

### Database Partitioning Strategy

| Table | Partition Key | Strategy | Rationale |
|-------|---------------|----------|-----------|
| `audit_logs` | `created_at` | Monthly | High write volume, time-based queries |
| `device_metrics` | `recorded_at` | Daily | Very high volume, short retention |
| `device_logs` | `recorded_at` | Daily | High volume, short retention |
| `bandwidth_usage` | `recorded_at` | Monthly | High volume, 1-year retention |
| `notifications` | `created_at` | Monthly | Moderate volume, 90-day retention |
| `invoices` | `created_at` | Quarterly | Lower volume, long retention |

### Connection Pooling (PgBouncer)

```toml
# pgbouncer.ini
[databases]
aeroxe = host=localhost port=5432 dbname=aeroxe

[pgbouncer]
pool_mode = transaction
max_client_conn = 1000
default_pool_size = 50
min_pool_size = 10
reserve_pool_size = 5
reserve_pool_timeout = 3
server_idle_timeout = 600
```

### CDN Strategy

| Asset Type | Cache Duration | Origin |
|------------|---------------|--------|
| Static frontend (JS/CSS) | 1 year | MinIO / S3 |
| Images | 30 days | MinIO |
| API responses (cacheable) | 5 minutes | Backend |
| Invoice PDFs | 24 hours | MinIO |
| Presigned URLs | No cache | MinIO |

### Disaster Recovery

| Component | RPO | RTO | Strategy |
|-----------|-----|-----|----------|
| PostgreSQL | 5 min | 30 min | Streaming replication + WAL archiving |
| Redis | 1 min | 5 min | AOF persistence + replica |
| NATS JetStream | 0 (durable) | 1 min | File-backed + cluster |
| MinIO | 1 hour | 15 min | Cross-region replication |
| Backend state | 0 (stateless) | 2 min | Auto-scale from image |

### Cost Optimization

| Strategy | Savings | When to Apply |
|----------|---------|---------------|
| Reserved instances | 30-40% | Year 2+ for predictable workloads |
| Spot instances | 60-70% | For non-critical batch jobs |
| Right-sizing | 20-30% | Quarterly resource review |
| Auto-scaling | 30-50% | Off-peak hours |
| CDN caching | 50-80% bandwidth | Always |
| Database read replicas | 30-40% DB load | Year 2+ |
