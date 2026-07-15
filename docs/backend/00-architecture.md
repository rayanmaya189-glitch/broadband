# AeroXe Backend — Architecture Overview (DDD + TDD Edition)

> **Req Ref:** §11 Backend Architecture, §21 System Architecture Diagram  
> **Version:** 3.0 – Domain‑Driven Design & Test‑Driven Development

---

## 1. Technology Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Language | **Rust** | Performance, memory safety, type safety |
| Web Framework | **Axum** | Async, tower middleware, ergonomic handlers |
| Database | **PostgreSQL 16** | JSONB, partitioning, PostGIS, schema isolation |
| ORM / Query | **SeaORM** (per context) | Type-safe queries, migrations, async, schema‑aware |
| Cache | **Redis 7** | Sessions, rate limiting, pub/sub, real‑time |
| Message Bus | **NATS JetStream** | Event sourcing, durable messaging, exactly‑once, versioned events |
| Object Storage | **MinIO** | S3‑compatible, self‑hosted document storage |
| WebSocket | **axum::ws** | Real‑time NOC dashboard, customer status |
| Auth | **JWT (RS256)** + **TOTP** | Stateless auth + 2FA |
| Templating | **Handlebars** | Notification templates |
| PDF Generation | **printpdf** or **wkhtmltopdf** | Invoice PDFs |
| Testing | **testcontainers**, **mockall** | Real infrastructure integration, domain‑level mocking |

---

## 2. Project Structure (DDD Monolith)

```
aeroxe-broadband-backend/
├── Cargo.toml
├── .env
├── docker-compose.yml
├── migrations/                          # Per‑schema migrations
│   ├── identity/
│   ├── customer/
│   ├── billing/
│   ├── payment/
│   ├── subscription/
│   ├── network/
│   ├── device/
│   ├── ticket/
│   ├── audit/
│   └── ...
└── src/
    ├── main.rs                          # Entry point, server bootstrap
    ├── lib.rs                           # Module declarations
    ├── config/                          # Environment configuration
    │   ├── database.rs
    │   ├── redis.rs
    │   ├── nats.rs
    │   └── settings.rs
    │
    ├── modules/                         # Bounded contexts (business domains)
    │   ├── identity/                    # Authentication & session basics
    │   ├── customer/                    # Customer, KYC, addresses
    │   ├── subscription/                # Plans, speed profiles
    │   ├── billing/                     # Invoices, line items
    │   ├── payment/                     # Payment methods, transactions
    │   ├── branches/                    # Branch management
    │   ├── network/                     # Devices, VLANs, IP pools
    │   ├── device/                      # CPE/ONT provisioning
    │   ├── bandwidth/                   # QoS, bandwidth profiles
    │   ├── ticket/                      # Support tickets
    │   ├── notification/                # Multi‑channel notifications
    │   ├── security/                    # RBAC/ABAC, policies, MFA
    │   ├── compliance/                  # KYC, GDPR, consent, data retention
    │   ├── audit/                       # Append‑only audit trail
    │   ├── workflow/                    # Sagas, long‑running processes
    │   ├── scheduler/                   # Recurring/ delayed jobs
    │   ├── event_catalog/               # Event schema registry (governance)
    │   └── integrations/                # External ISP system adapters
    │       ├── mikrotik/
    │       ├── huawei/
    │       ├── radius/
    │       ├── payment_gateway/
    │       └── sms_provider/
    │
    ├── infrastructure/                  # Shared technical capabilities
    │   ├── database/
    │   │   ├── postgres.rs
    │   │   └── transaction.rs
    │   ├── cache/
    │   │   └── redis.rs
    │   ├── messaging/
    │   │   ├── nats_client.rs
    │   │   ├── event_bus.rs
    │   │   └── subjects.rs
    │   ├── websocket/
    │   │   └── websocket_server.rs
    │   └── observability/
    │       ├── logging.rs
    │       ├── metrics.rs
    │       └── tracing.rs
    │
    ├── workers/                         # Background job processors
    │   ├── device_sync_worker.rs
    │   ├── bandwidth_worker.rs
    │   ├── billing_worker.rs
    │   └── notification_worker.rs
    │
    └── shared/                          # Shared kernel
        ├── errors/
        │   └── app_error.rs
        ├── event_contracts/             # Versioned domain event payloads
        │   ├── customer/
        │   │   ├── customer_created_v1.rs
        │   │   └── customer_suspended_v1.rs
        │   ├── billing/
        │   ├── network/
        │   └── ...
        ├── primitives/
        │   └── ids.rs                   # CustomerId, SubscriptionId, etc.
        └── utils/
            └── datetime.rs
```

---

## 3. Bounded Contexts Overview

| Context | Responsibility | Database Schema |
|---------|---------------|-----------------|
| **identity** | User registration, login, sessions, API keys | `identity` |
| **customer** | Customer aggregate, KYC status, contacts | `customer` |
| **subscription** | Subscription lifecycle, plans, speed profiles | `subscription` |
| **billing** | Invoices, invoice items, tax calculation | `billing` |
| **payment** | Payment methods, gateway transactions | `payment` |
| **branches** | Branch and franchise management | `branches` |
| **network** | VLANs, IP assignments, QoS, device configurations | `network` |
| **device** | CPE/ONT inventory, provisioning | `device` |
| **bandwidth** | Bandwidth profiles, traffic shaping | `bandwidth` |
| **ticket** | Support tickets, SLA tracking | `ticket` |
| **notification** | Multi‑channel messaging, templates | `notification` |
| **security** | RBAC/ABAC policies, MFA, access evaluation | `security` |
| **compliance** | KYC verification, GDPR consent, data retention | `compliance` |
| **audit** | Immutable audit trail | `audit` |
| **workflow** | Sagas, multi‑step coordination | `workflow` |
| **scheduler** | Job definitions, cron triggers | `scheduler` |
| **event_catalog** | Event schema governance, version registry | `event_catalog` |
| **integrations** | External ISP adapters (MikroTik, RADIUS, payment gateway) | (no dedicated schema) |

---

## 4. Module Internal Structure (DDD Layers)

Every business context follows a consistent four‑layer architecture:

```
modules/<context>/
├── domain/                      # Pure business logic
│   ├── aggregates/              # Aggregate roots (e.g., Customer)
│   │   └── <aggregate>/
│   │       ├── <aggregate>.rs
│   │       └── tests/           # Domain unit tests
│   ├── entities/                # Entities within aggregates
│   ├── value_objects/           # Immutable value types
│   └── rules/                   # Domain‑specific business rules
│
├── application/                 # Use‑case orchestration
│   ├── commands/                # Command handlers
│   │   ├── <handler>.rs
│   │   └── tests/               # Handler tests (mocked dependencies)
│   ├── queries/                 # Query handlers
│   └── services/                # Application services (facades)
│
├── infrastructure/              # Technical implementations
│   ├── repository/              # SeaORM/Postgres repositories
│   ├── messaging/               # NATS publishers & subscribers
│   │   ├── publishers/
│   │   └── subscribers/
│   └── adapters/                # External API integrations
│
└── api/                         # Exposed interfaces
    ├── http/                    # Axum controllers + DTOs
    │   ├── <controller>.rs
    │   └── tests/               # API integration tests
    └── grpc/                    # (future) gRPC service definitions
```

**Cross‑module communication rules:**
- Synchronous: via application service traits (e.g., `CustomerQueryService`) — never direct DB access.
- Asynchronous: via versioned domain events on NATS (e.g., `customer.activated.v1`).

---

## 5. Database Schema Isolation

Each bounded context **owns** its database schema. Cross‑schema queries are forbidden. Modules communicate only through service interfaces or events.

| Context | Schema |
|---------|--------|
| identity | `identity` |
| customer | `customer` |
| subscription | `subscription` |
| billing | `billing` |
| payment | `payment` |
| network | `network` |
| device | `device` |
| ticket | `ticket` |
| audit | `audit` |
| compliance | `compliance` |

Migrations are stored per context under `migrations/<schema>/` and applied in isolation.

---

## 6. Event‑Driven Communication

All cross‑module state changes are published as **versioned** domain events to NATS.

- **Subject format:** `aeroxe.<context>.<entity>.<action>.<version>`  
  Example: `aeroxe.customer.activated.v1`
- **Envelope:** Every event carries a standard envelope with `event_id`, `event_type`, `version`, `occurred_at`, `producer`.
- **Versioning:** Event payloads are immutable. New versions are added alongside old ones (e.g., `v2`), and subscribers handle both until deprecation.
- **Contracts** live in `shared/event_contracts/` to ensure publisher and subscriber agreement.

Example flow: `customer.activated.v1` → Billing creates first invoice, Network provisions VLAN, Notification sends welcome SMS — all without direct DB access.

---

## 7. Domain Patterns (Checker/Maker & History)

**Checker/Maker Workflow:**  
Critical entities (plans, bandwidth profiles, network devices, invoices, refunds) use a two‑step approval process.  
- **Maker** creates/updates → status = `pending`  
- **Checker** reviews → `approved` or `rejected`  
This is implemented as a domain rule within the respective aggregate and, where necessary, orchestrated by the **workflow** context.

**History / Audit Trail:**  
Every significant state change is recorded. The **audit** context subscribes to `audit.action.v1` events (published by any module) and stores an immutable log in `audit.audit_logs`. Critical entities may also retain an `_history` table within their own schema for quick rollback visibility.

---

## 8. Security & Compliance as First‑Class Domains

- **identity**: Manages “who you are” (users, sessions, API keys).  
- **security**: Evaluates “what you are allowed to do” via ABAC/RBAC policies, MFA, IP restrictions.  
- **compliance**: Handles KYC verification, GDPR consent, data retention policies — listens to events like `customer.registered` and triggers checks.  

These are separate bounded contexts with their own schemas and APIs, enforcing branch isolation even for security rules.

---

## 9. Request Lifecycle

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
Auth Middleware (JWT → user context)
RBAC Middleware (security context check)
Branch Scope Middleware (query filtering)
    │
    ▼
API Controller (module/api/http)
    │
    ├── Parse DTO & validate
    ├── Call Application Service (command/query)
    │       │
    │       ├── Domain aggregate logic (rules, invariants)
    │       ├── Repository (module’s own schema)
    │       └── Publish domain event (NATS)
    │
    ▼
Response (JSON)
    │
    ▼
Audit Event (async to audit context)
```

---

## 10. Error Handling

Unified `AppError` enum across contexts:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    NotFound(String),
    Unauthorized,
    Forbidden(String),
    Validation(String),
    Conflict(String),
    Internal(anyhow::Error),
    Database(sea_orm::DbErr),
    External(String),
}
```

Converted into HTTP responses via `IntoResponse`.

---

## 11. Dependency Injection (App State)

```rust
pub struct AppState {
    pub db: DatabaseConnection,         // SeaORM pool
    pub redis: redis::Client,           // Redis pool
    pub nats: async_nats::Client,       // NATS connection
    pub storage: StorageClient,         // MinIO client
    pub config: Config,                 // App configuration
    pub email: EmailService,            // Email sender
}
```

Each module wires its dependencies (repository, event publisher) from the shared state and through constructor injection in the application layer.

---

## 12. Configuration (Environment‑Based)

Standard 12‑factor configuration loaded from environment variables / `.env`:

- `DATABASE_URL`
- `REDIS_URL`
- `NATS_URL`
- `MINIO_*`
- `JWT_SECRET`, expiry durations
- Rate limit settings
- CORS origins
- SMTP credentials

---

## 13. API Versioning

All public HTTP endpoints are versioned: `/api/v1/...`  
gRPC services (future) will follow semantic versioning in package names.

---

## 14. Test Strategy (TDD)

Tests are placed **co‑located** with the code they verify, following the Red‑Green‑Refactor cycle at every layer:

| Layer | Test Location | Scope | Tools |
|-------|--------------|-------|-------|
| **Domain** | `modules/<context>/domain/.../tests/` | Pure business rules | No external deps |
| **Application** | `modules/<context>/application/.../tests/` | Command/query handler logic | Mocked repo, publisher |
| **Infrastructure** | `tests/integration/` (crate‑level) or module‑local | Repository, messaging | `testcontainers` (Postgres, NATS) |
| **API** | `modules/<context>/api/http/tests/` | Routing, serialisation, auth | Lightweight test server |
| **End‑to‑End** | `tests/e2e/` | Multi‑module scenarios | Full `docker-compose` stack |

All tests run in CI:  
- `cargo test --lib` (unit tests, fast)  
- `cargo test --test '*'` (integration tests, require Docker)

---

## 15. Future Microservice Extraction

The architecture is a **modular monolith** by design. Every bounded context can be extracted into a standalone service without business logic changes:
- Domain & application layers remain identical.
- In‑process trait calls are replaced with HTTP/gRPC clients (same trait interface).
- NATS subjects persist unchanged.
- Each service gets its own database with the same isolated schema.

---
