# AeroXe Backend — Architecture Overview

> **Req Ref:** §11 Backend Architecture, §21 System Architecture Diagram

---

## 1. Technology Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Language | **Rust** | Performance, memory safety, type safety |
| Web Framework | **Axum** | Async, tower middleware, ergonomic handlers |
| Database | **PostgreSQL 16** | JSONB, partitioning, PostGIS, mature ecosystem |
| ORM / Query | **SeaORM** | Type-safe queries, migrations, async, no raw SQLx |
| Cache | **Redis 7** | Sessions, rate limiting, pub/sub, real-time |
| Message Bus | **NATS JetStream** | Event sourcing, durable messaging, exactly-once |
| Object Storage | **MinIO** | S3-compatible, self-hosted document storage |
| WebSocket | **axum::ws** | Real-time NOC dashboard, customer status |
| Auth | **JWT (RS256)** + **TOTP** | Stateless auth + 2FA |
| Templating | **Handlebars** | Notification templates |
| PDF Generation | **printpdf** or **wkhtmltopdf** | Invoice PDFs |

## 2. Project Structure

```
backend/
├── Cargo.toml
├── .env
├── docker-compose.yml
├── migrations/
│   ├── 001_initial_schema/
│   ├── 002_seed_data/
│   └── 003_add_partitions/
├── src/
│   ├── main.rs                    # Entry point, server bootstrap
│   ├── lib.rs                     # Module declarations
│   ├── config.rs                  # Environment config (dotenv)
│   ├── error.rs                   # Unified error types
│   ├── app.rs                     # App state, router assembly
│   │
│   ├── db/                        # Database layer
│   │   ├── mod.rs
│   │   ├── connection.rs          # Pool setup
│   │   └── migrations.rs          # Migration runner
│   │
│   ├── middleware/                 # Tower middleware
│   │   ├── mod.rs
│   │   ├── auth.rs                # JWT extraction & validation
│   │   ├── rbac.rs                # Permission checking
│   │   ├── branch_scope.rs        # Branch filtering
│   │   ├── rate_limit.rs          # Redis-backed rate limiting
│   │   ├── audit.rs               # Automatic audit logging
│   │   └── cors.rs                # CORS configuration
│   │
│   ├── modules/                   # Feature modules
│   │   ├── auth/                  # §3-auth.md
│   │   ├── users/                 # §6-users.md
│   │   ├── rbac/                  # §4-rbac.md
│   │   ├── branches/              # §5-branches.md
│   │   ├── customers/             # §7-customers.md
│   │   ├── coverage/              # §8-coverage.md
│   │   ├── plans/                 # §9-plans.md
│   │   ├── subscriptions/         # §10-subscriptions.md
│   │   ├── installations/         # §11-installations.md
│   │   ├── billing/               # §12-billing.md
│   │   ├── accounting/            # §13-accounting.md
│   │   ├── payment_gateway/       # §14-payment-gateway.md
│   │   ├── bandwidth/             # §15-bandwidth.md
│   │   ├── devices/               # §16-devices.md
│   │   ├── discovery/             # §17-discovery.md
│   │   ├── inventory/             # §18-inventory.md
│   │   ├── network/               # §19-network.md
│   │   ├── tickets/               # §20-tickets.md
│   │   ├── leads/                 # §21-leads.md
│   │   ├── referrals/             # §22-referrals.md
│   │   ├── notifications/         # §23-notifications.md
│   │   ├── events/                # §24-events.md
│   │   ├── realtime/              # §25-realtime.md
│   │   ├── documents/             # §26-documents.md
│   │   └── audit/                 # §27-audit.md
│   │
│   ├── services/                  # Cross-cutting services
│   │   ├── nats.rs                # NATS connection & publishing
│   │   ├── redis.rs               # Redis connection
│   │   ├── storage.rs             # MinIO client
│   │   └── email.rs               # Email sender
│   │
│   └── utils/
│       ├── crypto.rs              # Hashing, encryption
│       ├── validators.rs          # Input validation helpers
│       ├── id.rs                  # ID generation (ULID/UUID)
│       └── pdf.rs                 # Invoice PDF generation
```

## 3. Module Structure Pattern

Each module follows a consistent internal structure:

```
modules/customers/
├── mod.rs              # Module declaration, route registration
├── handlers.rs         # Axum request handlers (controllers)
├── service.rs          # Business logic layer
├── repository.rs       # Database queries (data access layer)
├── model.rs            # SeaORM models (ActiveModel, Entity)
├── dto.rs              # Request/Response DTOs (serialization)
├── events.rs           # NATS event publishers
├── errors.rs           # Module-specific error types
├── tests/
│   ├── service_tests.rs
│   └── handler_tests.rs
└── routes.rs           # Route definitions
```

## 4. Request Lifecycle

```
HTTP Request
    │
    ▼
Axum Router
    │
    ├── CORS Middleware
    ├── Rate Limiter (Redis)
    ├── Request ID Generator
    │
    ▼
Route Handler
    │
    ├── Auth Middleware (JWT extract → user context)
    ├── RBAC Middleware (permission check)
    ├── Branch Scope Middleware (query filtering)
    ├── Audit Middleware (log action)
    │
    ▼
Handler Function
    │
    ├── Parse & validate request (DTO)
    ├── Call Service Layer
    │       │
    │       ├── Business logic
    │       ├── Validation rules
    │       ├── Repository calls (DB)
    │       ├── Event publishing (NATS)
    │       └── External API calls
    │
    ▼
Response (JSON)
    │
    ▼
Audit Log Entry (async)
```

## 5. Error Handling

Unified error type across all modules:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),

    #[error("Database error")]
    Database(#[from] sea_orm::DbErr),

    #[error("External service error: {0}")]
    External(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Forbidden(m) => (StatusCode::FORBIDDEN, m.clone()),
            AppError::Validation(m) => (StatusCode::BAD_REQUEST, m.clone()),
            AppError::Conflict(m) => (StatusCode::CONFLICT, m.clone()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

## 6. Dependency Injection via App State

```rust
pub struct AppState {
    pub db: DatabaseConnection,        // SeaORM pool
    pub redis: redis::Client,          // Redis pool
    pub nats: async_nats::Client,      // NATS connection
    pub storage: StorageClient,        // MinIO client
    pub config: Config,                // App configuration
    pub email: EmailService,           // Email sender
}

// Passed to all handlers via axum::extract::State
pub type SharedState = Arc<AppState>;
```

## 7. Configuration

```rust
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub nats_url: String,
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub refresh_expiry_days: i64,
    pub rate_limit_requests: u64,
    pub rate_limit_window_seconds: u64,
    pub cors_origins: Vec<String>,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
}
```

## 8. API Versioning

All API routes are versioned: `/api/v1/...`

```
/api/v1/auth/login
/api/v1/auth/register
/api/v1/auth/refresh
/api/v1/users
/api/v1/customers
/api/v1/customers/:id/subscriptions
/api/v1/plans
/api/v1/billing/invoices
/api/v1/billing/payments
/api/v1/devices
/api/v1/network/vlans
/api/v1/tickets
/api/v1/admin/dashboard
...
```

## 9. Checker/Maker Workflow

Critical entities use a two-step approval process:

1. **Maker** creates or updates an entity → status = `pending`
2. **Checker** reviews and approves/rejects → status = `approved` or `rejected`
3. Only `approved` entities are active in the system

Applies to: plans, bandwidth profiles, network devices, invoices, refunds, discounts.

## 10. History Table Pattern

Every critical entity has a `_history` table:

```sql
CREATE TABLE {entity}_history (
    id BIGSERIAL PRIMARY KEY,
    {entity}_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,        -- 'created', 'updated', 'deleted'
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);
```

## 11. Phase Implementation Order

| Phase | Modules | Priority |
|-------|---------|----------|
| **Phase 1** | architecture, database, auth, rbac, users, branches | Foundation |
| **Phase 2** | customers, coverage, plans, subscriptions, installations | Core business |
| **Phase 3** | billing, accounting, payment-gateway | Revenue |
| **Phase 4** | devices, discovery, inventory, network, bandwidth | Network ops |
| **Phase 5** | tickets, leads, referrals | Operations |
| **Phase 6** | notifications, events, realtime, documents | Infrastructure |
| **Phase 7** | audit, security, devops | Hardening |
