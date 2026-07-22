# AeroXe Broadband – Domain-Driven Design & Test-Driven Development Guide (Enterprise Edition)

**Version 2.1 – Complete DDD + TDD Folder Architecture + ISP Gap Mitigation**

This document defines the engineering standards for the **AeroXe Broadband** backend using **Domain-Driven Design** and **Test-Driven Development**. It includes a production‑ready folder architecture that enforces strict bounded‑context isolation, aggregate design, event versioning, and dedicated security/compliance contexts. Every component is designed to be tested from the ground up, with clear guidance on test placement and strategy.

> **ISP Gap Analysis:** See Section 13 for ISP operational gap mitigation rules. Full gap analysis: `docs/backend/DESIGN-GAPS-DEEP-ANALYSIS.md` (v3.0), `docs/backend/GAP-security.md`, `docs/backend/GAP-code-bugs.md`, `docs/backend/GAP-finance-compliance.md` (NEW v3.0), `docs/backend/GAP-architecture-patterns.md` (NEW v3.0).

---

## 1. Project Overview and Philosophy

AeroXe Broadband is built as a **modular monolith** in Rust that can later be split into independent microservices. We apply:

- **DDD**: Tactical patterns (Aggregates, Entities, Value Objects, Domain Events, Repositories) to model complex ISP business rules.
- **TDD**: Test-first approach applied to domain logic, application services, infrastructure, and API layers.
- **Strict Module Isolation**: Each bounded context owns its database schema, migrations, and data lifecycle. Communication between modules happens only via public service interfaces (synchronous) or versioned domain events (asynchronous, NATS).

---

## 2. Final Production Folder Architecture

The following structure incorporates all DDD patterns, TDD test placement, enterprise‑grade security, and compliance contexts.

```
aeroxe-broadband-backend/
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── docker-compose.yml
├── .env
│
├── migrations/                          # Database migrations per schema (context)
│   ├── identity/
│   │   ├── 001_create_users.sql
│   │   └── 002_create_roles.sql
│   ├── customer/
│   │   ├── 001_create_customer.sql
│   │   └── 002_create_address.sql
│   ├── billing/
│   │   ├── 001_create_invoice.sql
│   │   └── 002_create_payment.sql
│   ├── network/
│   │   ├── 001_create_device.sql
│   │   └── 002_create_vlan.sql
│   ├── ...                              # other schemas
│   └── audit/
│       └── 001_create_audit_log.sql
│
└── src/
    ├── main.rs
    ├── lib.rs
    │
    ├── config/                          # Application configuration
    │   ├── database.rs
    │   ├── redis.rs
    │   ├── nats.rs
    │   └── settings.rs
    │
    ├── modules/                         # Bounded contexts (business domains)
    │   ├── gateway/                    # NEW: API Gateway layer
    │   │   ├── auth/
    │   │   │     ├──  jwt_validator.rs
    │   │   │     └──  api_key_validator.rs
    │   │   ├── rate_limiter/
    │   │   │     └──  token_bucket.rs
    │   │   ├── request_validator/
    │   │   │     └──  schemas/
    │   │   ├── api_versioning/
    │   │   │     └──  router.rs
    │   │   └── mod.rs
    │   ├── identity/                    # Authentication & authorisation basics
    │   │   ├── domain/
    │   │   │   ├── aggregates/
    │   │   │   │   └── user/
    │   │   │   │       ├── user.rs
    │   │   │   │       └── tests/       # Domain tests for User aggregate
    │   │   │   ├── entities/
    │   │   │   │   └── session.rs
    │   │   │   ├── value_objects/
    │   │   │   │   ├── email.rs
    │   │   │   │   └── password.rs
    │   │   │   └── rules/
    │   │   │       └── auth_rules.rs
    │   │   ├── application/
    │   │   │   ├── commands/
    │   │   │   │   ├── login.rs
    │   │   │   │   └── tests/           # Command handler tests
    │   │   │   ├── queries/
    │   │   │   │   └── get_user.rs
    │   │   │   └── services/
    │   │   │       └── auth_service.rs
    │   │   ├── infrastructure/
    │   │   │   ├── repository/
    │   │   │   │   └── postgres_user_repository.rs
    │   │   │   └── security/
    │   │   │       └── jwt.rs
    │   │   └── api/
    │   │       ├── http/
    │   │       │   ├── auth_controller.rs
    │   │       │   └── tests/           # API integration tests
    │   │       └── grpc/
    │   │           └── auth_service.rs
    │   │
    │   ├── customer/                    # Customer aggregate, KYC, addresses
    │   │   ├── domain/
    │   │   │   ├── aggregates/
    │   │   │   │   └── customer/
    │   │   │   │       ├── customer.rs          # Aggregate root
    │   │   │   │       ├── profile.rs           # Entity
    │   │   │   │       ├── status.rs            # Value object / enum
    │   │   │   │       └── tests/
    │   │   │   │           └── customer_tests.rs
    │   │   │   ├── value_objects/
    │   │   │   │   ├── customer_id.rs
    │   │   │   │   ├── email.rs
    │   │   │   │   └── phone.rs
    │   │   │   └── rules/
    │   │   │       └── customer_rules.rs
    │   │   ├── application/
    │   │   │   ├── commands/
    │   │   │   │   ├── create_customer.rs
    │   │   │   │   ├── suspend_customer.rs
    │   │   │   │   ├── activate_customer.rs
    │   │   │   │   └── tests/                   # Handler tests (mock repo)
    │   │   │   ├── queries/
    │   │   │   │   └── get_customer.rs
    │   │   │   └── services/
    │   │   │       └── customer_service.rs
    │   │   ├── infrastructure/
    │   │   │   ├── repository/
    │   │   │   │   └── postgres_customer_repository.rs
    │   │   │   ├── messaging/
    │   │   │   │   ├── publishers/
    │   │   │   │   │   └── customer_event_publisher.rs
    │   │   │   │   └── subscribers/
    │   │   │   │       └── payment_event_subscriber.rs
    │   │   │   └── adapters/                    # External system adapters if any
    │   │   └── api/
    │   │       ├── http/
    │   │       │   ├── customer_controller.rs
    │   │       │   └── tests/                   # Endpoint tests
    │   │       └── grpc/
    │   │           └── customer_service.rs
    │   │
    │   ├── subscription/
    │   │   ├── domain/ ...                      # Aggregate: Subscription, Plan, SpeedProfile
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── billing/
    │   │   ├── domain/ ...                      # Aggregate: Invoice, InvoiceItem
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── payment/
    │   │   ├── domain/ ...                      # Aggregate: Payment, PaymentMethod
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── branches/
    │   │   ├── domain/ ...                      # Aggregate: Branches
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── network/
    │   │   ├── domain/                        # Aggregate: NetworkDevice, VLAN
    │   │   │   └── aggregates/
    │   │   │       ├── radius
    │   │   │       ├── pppoe
    │   │   │       ├── dhcp
    │   │   │       ├── ip_pool
    │   │   │       ├── vlan
    │   │   │       ├── olt
    │   │   │       ├── onu
    │   │   │       ├── mikrotik
    │   │   │       ├── qos
    │   │   │       └──  firewall
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── device/
    │   │   ├── domain/ ...                      # Aggregate: Device (CPE/ONT)
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── bandwidth/
    │   │   ├── domain/ ...                      # Aggregate: SpeedPlan, BandwidthProfile
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── monitoring/
    │   │   ├── domain/ ...
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── ticket/
    │   │   ├── domain/ ...                      # Aggregate: Ticket, Message
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── notification/
    │   │   ├── domain/ ...                      # Aggregate: Notification, Template
    │   │   ├── application/ ...
    │   │   ├── infrastructure/ ...
    │   │   └── api/ ...
    │   │
    │   ├── security/                    # ABAC, RBAC, policy engine, encryption
    │   │   ├── domain/
    │   │   │   ├── aggregates/
    │   │   │   │   └── policy/
    │   │   │   │       ├── policy.rs
    │   │   │   │       └── tests/
    │   │   │   ├── entities/
    │   │   │   │   └── role.rs
    │   │   │   ├── value_objects/
    │   │   │   │   └── permission.rs
    │   │   │   └── rules/
    │   │   │       └── access_rules.rs
    │   │   ├── application/
    │   │   │   ├── commands/
    │   │   │   │   └── evaluate_access.rs
    │   │   │   └── services/
    │   │   │       └── policy_engine.rs
    │   │   ├── infrastructure/
    │   │   │   ├── repository/
    │   │   │   │   └── postgres_policy_repository.rs
    │   │   │   └── adapters/
    │   │   │       └── opa_adapter.rs          # Open Policy Agent integration if needed
    │   │   └── api/ ...
    │   │
    │   ├── compliance/                  # KYC, GDPR, data retention, consent
    │   │   ├── domain/
    │   │   │   ├── aggregates/
    │   │   │   │   └── consent/
    │   │   │   │       ├── consent.rs
    │   │   │   │       └── tests/
    │   │   │   ├── value_objects/
    │   │   │   │   └── data_retention_policy.rs
    │   │   │   └── rules/
    │   │   │       └── compliance_rules.rs
    │   │   ├── application/
    │   │   │   ├── commands/
    │   │   │   │   └── verify_kyc.rs
    │   │   │   └── services/
    │   │   │       └── retention_service.rs
    │   │   ├── infrastructure/
    │   │   │   ├── repository/
    │   │   │   │   └── postgres_kyc_repository.rs
    │   │   │   └── adapters/
    │   │   │       └── kyc_provider_adapter.rs
    │   │   └── api/ ...
    │   │
    │   ├── audit/                       # Centralised audit trail (shared but owned)
    │   │   ├── domain/
    │   │   │   └── audit_event.rs
    │   │   ├── application/
    │   │   │   └── audit_service.rs
    │   │   ├── infrastructure/
    │   │   │   └── repository/
    │   │   │       └── postgres_audit_repository.rs
    │   │   └── api/ ...
    │   │
    │   ├── workflow/                    # Long‑running processes, sagas
    │   │   ├── domain/
    │   │   │   └── workflow_definition.rs
    │   │   ├── application/
    │   │   │   └── workflow_executor.rs
    │   │   └── infrastructure/
    │   │       └── nats_saga_coordinator.rs
    │   │
    │   └── integrations/               # Adapters to external ISP systems
    │       ├── mikrotik/
    │       ├── huawei/
    │       ├── radius/
    │       ├── payment_gateway/
    │       └── sms_provider/
    │           ├── adapter.rs
    │           └── tests/
    │
    ├── infrastructure/                 # Shared technical infrastructure
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
    └── shared/                          # Shared kernel (used by all modules)
        ├── errors/
        │   └── app_error.rs
        ├── event_contracts/                      # Versioned domain event contracts definitions
        │   ├── customer/
        │   │   ├── customer_created_v1.rs
        │   │   └── customer_suspended_v1.rs
        │   ├── billing/
        │   │   ├── invoice_created_v1.rs
        │   │   └── payment_completed_v1.rs
        │   ├── network/
        │   │   ├── device_online_v1.rs
        │   │   └── bandwidth_applied_v1.rs
        │   └── ...
        ├── primitives/
        │   └── ids.rs                  # Shared ID primitives (CustomerId, etc.)
        └── utils/
            └── datetime.rs
```

**Tests placement:**

- **Domain unit tests** live inside `tests/` subfolders co‑located with the aggregate/entity/value-object they test (e.g., `modules/customer/domain/aggregates/customer/tests/`).
- **Application handler tests** are placed in `tests/` next to the command handler (e.g., `modules/customer/application/commands/tests/`), using mocked repositories and publishers.
- **Infrastructure integration tests** (repository tests, messaging tests) are placed in a dedicated `tests/` folder at the crate root (or inside the module’s `tests/` if preferred). They spin up real databases/NATS via `testcontainers`.
- **API endpoint tests** reside in `tests/` next to the HTTP controller, using a full application fixture with mocked dependencies or a lightweight test server.
- **End‑to‑end tests** (scenario‑based, spanning multiple modules) live in `tests/e2e/`.

---

## 3. Database Ownership and Schema Isolation

Each bounded context **owns its database schema** inside the `aeroxe_broadband` database. Schemas are mapped one‑to‑one with modules:

| Module        | Database Schema   | Example Tables                      |
|---------------|-------------------|-------------------------------------|
| identity      | `identity`        | `users`, `sessions`, `roles`       |
| customer      | `customer`        | `customers`, `addresses`, `contacts`|
| billing       | `billing`         | `invoices`, `invoice_items`        |
| payment       | `payment`         | `payments`, `payment_methods`      |
| network       | `network`         | `devices`, `vlans`, `bandwidth_profiles`|
| device        | `device`          | `cpe_devices`, `ont_devices`       |
| ...           | ...               | ...                                 |
| audit         | `audit`           | `audit_logs`                        |
| compliance    | `compliance`      | `kyc_verifications`, `consents`    |

**Rules:**

1. **No cross‑schema queries** from other modules. A module cannot read `customer.customers` directly; it must use an application service interface from the customer module or react to a domain event like `customer.created`.
2. **Migrations are stored per module** (see `migrations/` folder above) and applied in isolation.
3. **Shared tables are not allowed** (except `audit_logs` and possibly some `system_configuration` schema owned by a platform module). Even those are accessed through dedicated service interfaces.

---

## 4. Aggregate Design

Aggregates define consistency boundaries. Every module’s domain layer explicitly models its aggregate roots and their internal entities/value objects.

**Customer Context:**

```
Customer (aggregate root)
 ├── CustomerProfile (entity)
 ├── ContactInformation (value object)
 ├── Address (value object)
 └── CustomerStatus (enum)
```

Invariants enforced:
- Customer cannot be activated without KYC verification.
- Customer cannot be deleted while active subscriptions exist.
- Any status change is recorded via domain event and audit log.

**Subscription Context:**

```
Subscription (aggregate root)
 ├── Plan (value object)
 ├── SpeedProfile (value object)
 ├── BillingCycle (value object)
 └── SubscriptionStatus (enum)
```

**Billing Context:**

```
Invoice (aggregate root)
 ├── InvoiceItem (entity)
 ├── Tax (value object)
 └── PaymentStatus (enum)
```

**Network Context:**

```
NetworkDevice (aggregate root)
 ├── Interface (entity)
 ├── IPAssignment (value object)
 └── Configuration (value object)
```

Aggregates are loaded and saved through repository traits defined in the domain layer.

---

## 5. Domain Events (Versioned and Owned)

All cross‑module communication happens over **versioned** domain events published to NATS.

### Event Structure

Every event implements the `DomainEvent` trait (or similar) and carries a standard envelope:

```rust
#[derive(Debug, Clone, prost::Message)]
pub struct EventEnvelope<T: prost::Message> {
    #[prost(string, tag = "1")]
    pub event_id: String,
    #[prost(string, tag = "2")]
    pub event_type: String,
    #[prost(uint32, tag = "3")]
    pub version: u32,
    #[prost(string, tag = "4")]
    pub occurred_at: String,
    #[prost(string, tag = "5")]
    pub producer: String,
    #[prost(message, tag = "6")]
    pub payload: T,
}
```

Example concrete event (`shared/events/customer/customer_created_v1.rs`):

```rust
pub struct CustomerCreatedV1 {
    pub customer_id: Uuid,
    pub email: String,
    pub name: String,
    // ...
}
```

### NATS Subject Naming Convention

Format: `company.context.entity.action.version`

Examples:
- `aeroxe.customer.created.v1`
- `aeroxe.billing.invoice.created.v1`
- `aeroxe.network.device.online.v1`

### Event Ownership and Versioning

- Each module **publishes** events that it owns (e.g., customer module publishes `customer.created.v1`).
- Events are **immutable** after production release. If the payload must change, create a new version (e.g., `customer.created.v2`) and handle both in subscribers until old version is retired.
- All events are stored in `shared/events/<context>/` so they are accessible to publishers and subscribers alike.

---

## 6. Security and Compliance as First‑Class Domains

### Identity Context

Manages “who you are” – users, sessions, API keys. Does **not** enforce what you can do; that belongs to `security`.

### Security Context

Implements access control: RBAC/ABAC, policy evaluation, MFA, IP restrictions. Example ABAC policy rule:

```text
ALLOW support_agent TO change_bandwidth
  WHEN customer.region == agent.assigned_region
  AND customer.status == ACTIVE
```

### Compliance Context

Handles KYC verification, GDPR consent, data retention policies, and privacy rules. It listens to relevant events (e.g., `customer.registered`) and triggers compliance checks.

### Audit Context

All significant actions are audited. The audit module subscribes to a wildcard set of events or explicitly called to record audit entries. Example event:

```json
{
  "event_id": "uuid",
  "event_type": "audit.action.v1",
  "payload": {
    "user": "admin",
    "action": "CHANGE_BANDWIDTH",
    "resource": "customer:123",
    "old_value": "100Mbps",
    "new_value": "200Mbps"
  }
}
```

---

## 7. TDD Integration with Folder Architecture

TDD is embedded into the development workflow at every level. The following table maps the layer to the test location, scope, and tools.

| Layer                   | Test Location (examples)                                      | Scope                          | Mocks/Infrastructure                |
|-------------------------|---------------------------------------------------------------|--------------------------------|-------------------------------------|
| **Domain** (Aggregate, Entity, Value Object, Domain Service) | `modules/<context>/domain/aggregates/<aggregate>/tests/` or `value_objects/tests/` | Business rules, invariants, validations | No external dependencies – pure unit tests |
| **Application** (Command/Query handlers) | `modules/<context>/application/commands/tests/` | Orchestration, repository calls, event publishing | Mock `CustomerRepository`, `EventPublisher` |
| **Infrastructure – Repository** | Top‑level `tests/integration/customer_repository.rs` or same module’s `tests/` | SQL queries, mapping, transactional behaviour | Real Postgres via `testcontainers`, test migrations |
| **Infrastructure – Messaging** | `tests/integration/event_subscriber.rs` | Event publishing and consumption | Embedded NATS or `testcontainers` with NATS |
| **API (HTTP/gRPC)** | `modules/<context>/api/http/tests/` | Routing, serialisation, status codes, auth | Full app fixture (Axum/Actix) with mocked services or test DB |
| **End‑to‑End (Scenario)** | `tests/e2e/` | Multi‑module workflow (e.g., register → create subscription → bill) | Real Postgres, NATS, Redis via `docker-compose` or `testcontainers` |

### Example: Customer Domain Test

File: `modules/customer/domain/aggregates/customer/tests/customer_tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activate_customer_without_kyc_fails() {
        let mut customer = Customer::new(/* without KYC */);
        let result = customer.activate();
        assert!(matches!(result, Err(DomainError::KycRequired)));
    }

    #[test]
    fn suspend_active_customer_succeeds() {
        let mut customer = Customer::new_with_kyc_verified(/* ... */);
        customer.activate().unwrap();
        let result = customer.suspend("non-payment");
        assert!(result.is_ok());
        assert_eq!(customer.status(), CustomerStatus::Suspended);
    }
}
```

### Example: Application Command Handler Test

File: `modules/customer/application/commands/tests/create_customer_tests.rs`

```rust
#[tokio::test]
async fn handler_persists_customer_and_publishes_event() {
    let mut repo = MockCustomerRepository::new();
    let mut publisher = MockEventPublisher::new();

    repo.expect_find_by_email()
        .return_once(|_| Ok(None));
    repo.expect_save()
        .return_once(|_| Ok(()));
    publisher.expect_publish()
        .withf(|event| event.event_type == "CustomerCreated")
        .return_once(|_| Ok(()));

    let handler = CreateCustomerHandler::new(repo, publisher);
    let cmd = CreateCustomerCommand { email: "test@test.com".into(), name: "Test".into() };
    let result = handler.handle(cmd).await;

    assert!(result.is_ok());
}
```

### Example: Repository Integration Test

File: `tests/integration/customer_repository_tests.rs`

```rust
#[tokio::test]
async fn save_and_retrieve_customer() {
    let db = setup_test_db().await;  // spins up postgres testcontainer, runs migrations
    let repo = PostgresCustomerRepository::new(db.clone());

    let customer = Customer::new(/* ... */);
    repo.save(&customer).await.unwrap();

    let found = repo.find_by_id(&customer.id).await.unwrap();
    assert_eq!(found, Some(customer));
}
```

All tests are run as part of the CI pipeline:

- `cargo test --lib` for unit tests (fast)
- `cargo test --test '*'` for integration tests (requires Docker)

---

## 8. Event‑Driven Integration Testing

Subscriber tests verify that when an event is published, the correct business action happens. These tests use a real NATS server (via `testcontainers`) and a test database.

File: `tests/integration/event_subscriber_tests.rs` or within the module’s test folder.

```rust
#[tokio::test]
async fn customer_created_event_creates_first_invoice() {
    let nats = connect_nats().await;
    let billing_db = setup_billing_db().await;
    let subscriber = BillingEventSubscriber::new(billing_db.clone(), nats.clone());

    // Start subscriber in background
    tokio::spawn(async move { subscriber.run().await });

    // Publish customer.created.v1
    let event = CustomerCreatedV1 { customer_id: i64, email: "a@b.com".into() };
    let envelope = EventEnvelope::new(event, "customer-service".to_string());
    nats.publish("aeroxe.customer.created.v1", envelope.encode_to_vec()).await.unwrap();

    // Wait for processing
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Assert invoice exists
    let invoices = billing_db.find_invoices_for_customer(&event.customer_id).await;
    assert!(!invoices.is_empty());
}
```

---

## 9. Continuous Integration and Delivery

**Pipeline Steps:**

1. **Checkout code**
2. **Setup Docker** (for testcontainers)
3. **Run migrations** (tests will handle their own)
4. **Unit tests** – `cargo test --lib -- --test-threads=4`
5. **Integration tests** – `cargo test --test '*' -- --test-threads=1` (serial due to shared containers)
6. **Lint & format** – `cargo clippy` and `cargo fmt --check`
7. **Build release binary** (for production)

---

Below is the complete **Data Flow Examples** section, which illustrates every major business process in AeroXe Broadband while strictly respecting branch isolation. Each flow shows how bounded contexts communicate only through public service interfaces (synchronous) or versioned domain events (asynchronous). No module ever touches another module’s database tables directly.

---

## 10. Data Flow Examples with Branch Isolation

All flows follow these rules:
- **Branch isolation**: each bounded context owns its database schema. Data is never read directly across schemas.
- **Synchronous queries**: when a module needs data from another, it calls a defined query interface (e.g., `CustomerQueryService`), not the database.
- **Asynchronous events**: side‑effects across modules are triggered by domain events published to NATS. Subscribers react in their own context.
- **Audit**: every significant action emits an audit event (`audit.action.v1`) that the audit module consumes.
- **Protobuf encoding**: all API request/response bodies and NATS event payloads are Protocol Buffers (protobuf). JSON is not used for API communication.

### 10.1 Customer Registration (with KYC Compliance)

```
Mobile App / Portal
      │
      ▼
Customer HTTP API
      │
      ▼
Customer Controller
      │
      ▼
CreateCustomerHandler (application)
      │
      ▼
Customer Domain (validate rules, create Customer aggregate)
      │
      ├─► CustomerRepository.save()  ──► customer.customers table
      │
      ├─► Publish customer.registered.v1 ──► NATS
      │
      └─► (sync) Request KYC check ──► Compliance Module
                                        │
                                        ▼
                                      ComplianceService.verifyKYC()
                                        │
                                        ├─► Save to compliance.kyc_verifications
                                        │
                                        └─► Publish kyc.verification.completed.v1 ──► NATS
                                                                                      │
                                        ┌─────────────────────────────────────────────┘
                                        ▼
                                  Customer Module (subscriber) → updates customer status to ACTIVE
                                  (if KYC passed) → publishes customer.activated.v1
                                        │
                    ┌───────────────────┼────────────────────┐
                    ▼                   ▼                    ▼
              Billing Module      Network Module      Notification Module
         (subscribes to         (subscribes to        (subscribes to
          customer.activated)    customer.activated)   customer.activated)
                    │                   │                    │
                    ▼                   ▼                    ▼
            Create first invoice   Assign VLAN,          Send welcome SMS/email
            (billing.invoices)    apply default speed    (via SMS provider adapter)
                                  (network.devices,
                                   network.vlans)
```

**Branch isolation**:
- Customer module does **not** read KYC tables; it calls a compliance service synchronously and later reacts to the KYC completion event.
- Billing and Network modules never access customer tables; they only use data from the `customer.activated` event payload (customer ID, email, plan, etc.).
- Notification module uses the same event payload plus its own templates.

### 10.2 Customer Suspension

```
Admin Portal
      │
      ▼
Customer HTTP API
      │
      ▼
SuspendCustomerHandler
      │
      ▼
Customer Domain (validate not already suspended)
      │
      ├─► CustomerRepository.save()  ──► update status to SUSPENDED
      │
      └─► Publish customer.suspended.v1 ──► NATS
                                                │
                    ┌───────────────────────────┤
                    ▼                           ▼
              Billing Module              Network Module
         (subscribes to                 (subscribes to
          customer.suspended)            customer.suspended)
                    │                           │
                    ▼                           ▼
            Stop recurring billing       Disable bandwidth,
            (mark invoices as            remove active sessions
             on‑hold)                    (network.devices)
                                         │
                                         ▼
                                    Publish bandwidth.disabled.v1
                                                │
                                                ▼
                                        Notification Module → Send SMS
```

**Branch isolation**:
- Billing does **not** directly change network state; it only marks billing records.
- Network module independently disables the customer’s connectivity by acting on its own `network.devices` and `bandwidth_profiles` tables.

### 10.3 Customer Reactivation

Similar to suspension but reversed. After payment of outstanding balance (see §8.6), billing publishes `invoice.paid.v1`. Customer module subscribes and reactivates the customer, publishing `customer.reactivated.v1`. Network then re‑applies the speed plan and enables the connection.

### 10.4 Subscription Plan Change

```
Customer Portal / Admin
      │
      ▼
Subscription HTTP API
      │
      ▼
ChangeSubscriptionHandler
      │
      ▼
Subscription Domain (validate plan compatibility, billing cycle)
      │
      ├─► SubscriptionRepository.save() → subscription.subscriptions
      │
      └─► Publish subscription.plan_changed.v1 ──► NATS
                                                       │
                    ┌──────────────────────────────────┤
                    ▼                                  ▼
              Billing Module                     Network Module
         (subscribes to                        (subscribes to
          subscription.plan_changed)            subscription.plan_changed)
                    │                                  │
                    ▼                                  ▼
            Recalculate prorated              Update speed profile,
            invoice items                    reconfigure VLAN/QoS
            (billing.invoices)               (network.bandwidth_profiles)
```

**Branch isolation**:
- Subscription module owns plan definitions; it sends the new plan ID and effective date in the event. Billing and Network act on their own tables without knowing the subscription internals.

### 10.5 Scheduled Invoice Generation

```
Billing Worker (cron)
      │
      ▼
GenerateInvoicesHandler
      │
      ▼
Billing Domain (for each active subscription)
      │
      ├─► Fetch subscription data via SubscriptionQueryService (sync API call)
      │      └── Subscription module returns plan details, customer ID
      │
      ├─► Fetch customer email via CustomerQueryService (sync API call)
      │
      ├─► Create Invoice aggregate
      ├─► InvoiceRepository.save() → billing.invoices
      │
      └─► Publish invoice.created.v1 ──► NATS
                                              │
                    ┌─────────────────────────┼──────────────┐
                    ▼                         ▼              ▼
              Payment Module           Notification    Audit Module
         (subscribes to               (subscribes to
          invoice.created)             invoice.created)
                    │                         │
                    ▼                         ▼
            Store payment             Send invoice email
            reference, await          to customer
            payment gateway
            webhook
```

**Branch isolation**:
- Billing uses **query interfaces** (not direct SQL) to get necessary data from Subscription and Customer modules.
- Invoice data stays in `billing` schema. Payment module only receives the invoice ID and amount.

### 10.6 Payment Completion (Webhook)

```
Payment Gateway Webhook
      │
      ▼
Payment HTTP API
      │
      ▼
ProcessPaymentHandler
      │
      ▼
Payment Domain (validate transaction, mark completed)
      │
      ├─► PaymentRepository.save() → payment.payments
      │
      └─► Publish payment.completed.v1 ──► NATS
                                                │
                    ┌───────────────────────────┼──────────────┐
                    ▼                           ▼              ▼
              Billing Module              Customer Module   Notification
         (subscribes to                 (subscribes to      (subscribes to
          payment.completed)             payment.completed)  payment.completed)
                    │                           │              │
                    ▼                           ▼              ▼
            Mark invoice as PAID          Reactivate if       Send payment
            (billing.invoices)            previously          confirmation SMS
                                          suspended
                                          (calls SuspendHandler)
```

**Branch isolation**:
- Payment module doesn’t know about suspension logic. It only broadcasts a `payment.completed` event containing the invoice ID and customer ID. Customer module checks its own status and reactivates if needed.

### 10.7 Payment Failure

Analogous to completion, but publishes `payment.failed.v1`. Billing may mark the invoice overdue; Customer module may schedule suspension after grace period (via a workflow/saga).

### 10.8 New Device Provisioning

```
Admin / Technician Portal
      │
      ▼
Device HTTP API
      │
      ▼
AddDeviceHandler
      │
      ▼
Device Domain (validate device type, authenticate)
      │
      ├─► DeviceRepository.save() → device.devices
      │
      └─► Publish device.provisioned.v1 ──► NATS
                                                 │
                    ┌────────────────────────────┤
                    ▼                            ▼
              Network Module               Monitoring Module
         (subscribes to                   (subscribes to
          device.provisioned)              device.provisioned)
                    │                            │
                    ▼                            ▼
            Assign IP, VLAN,               Start health checks
            push configuration             add to dashboard
            (network.devices,
             network.ip_assignments)
```

### 10.9 Device Online / Offline (from monitoring)

```
Monitoring Worker detects device heartbeat
      │
      ▼
Device Status Evaluation
      │
      ├── online:  publish device.online.v1
      └── offline: publish device.offline.v1
                              │
                              ▼
                    Notification Module (SMS alert)
                    Audit Module (log status change)
```

### 10.10 Bandwidth Policy Update (Manual or Automated)

```
Network Admin Portal / Automation Worker
      │
      ▼
Bandwidth HTTP API
      │
      ▼
UpdateBandwidthPolicyHandler
      │
      ▼
Bandwidth Domain (validate new profile, apply to affected devices)
      │
      ├─► BandwidthRepository.save() → bandwidth.bandwidth_profiles
      │
      ├─► (sync) call Network Module’s configurator to push changes to routers
      │
      └─► Publish bandwidth.profile.applied.v1 ──► NATS
                                                          │
                                                          ▼
                                                  Notification Module → inform customer
                                                  Audit Module
```

**Branch isolation**:
- Bandwidth module owns speed profiles but delegates actual network device configuration via a defined service interface (`NetworkConfigurator`). It does **not** touch `network.devices` directly.

### 10.11 Support Ticket Creation

```
Customer Portal / Call Centre
      │
      ▼
Ticket HTTP API
      │
      ▼
CreateTicketHandler
      │
      ▼
Ticket Domain (create Ticket aggregate)
      │
      ├─► TicketRepository.save() → ticket.tickets
      │
      └─► Publish ticket.created.v1 ──► NATS
                                              │
                    ┌─────────────────────────┤
                    ▼                         ▼
              Workflow Module           Notification Module
         (subscribes to                (subscribes to
          ticket.created)               ticket.created)
                    │                         │
                    ▼                         ▼
            Start SLA timer,            Notify assigned agent
            escalation saga             (push/email)
```

### 10.12 Ticket Resolution

Similar: `ticket.resolved.v1` triggers workflow closure and customer satisfaction survey (notification).

### 10.13 Cross‑Cutting Audit Flow

Every command handler can optionally publish an `audit.action.v1` event (or the audit module subscribes to all domain events). The Audit module stores a tamper‑proof log in the `audit` schema.

```
Any Module
      │
      └── Publish audit.action.v1 ──► NATS
                                          │
                                          ▼
                                    Audit Module
                                          │
                                          ▼
                                    audit.audit_logs (append only)
```

This ensures a complete, isolated audit trail without any direct coupling.

I see you've highlighted the `subscription.created` event and the corresponding network provisioning flow. That's an important scenario—especially when an existing customer purchases an additional plan or upgrades. I’ve added a dedicated flow for this below, which fits right into the “Data Flow Examples with Branch Isolation” section of the document. It demonstrates how the Network module reacts to a new subscription without ever touching the Subscription module’s database.

## 10.14 Subscription Created (New Plan Provisioning)

This flow occurs when a **new subscription** is added to an existing customer (e.g., adding a second connection or upgrading). The Network module listens to `subscription.created.v1` and creates the required network profile, VLAN, and bandwidth settings.

```
Admin/Customer Portal
      │
      ▼
Subscription HTTP API
      │
      ▼
CreateSubscriptionHandler
      │
      ▼
Subscription Domain
      │
      ├─► Validate plan availability, customer status (via CustomerQueryService)
      ├─► SubscriptionRepository.save() → subscription.subscriptions
      │
      └─► Publish subscription.created.v1 ──► NATS
                                                 │
        ┌────────────────────────────────────────┤
        │                                        │
        ▼                                        ▼
  Billing Module                          Network Module
  (subscribes to                         (subscribes to
   subscription.created)                  subscription.created)
        │                                        │
        ▼                                        ▼
  Create prorated invoice               • Create network profile
  (billing.invoices)                    • Assign VLAN from pool
                                        • Apply bandwidth/speed plan
                                        • Generate IP assignment
                                        → network.devices, network.vlans,
                                          network.bandwidth_profiles
                                        │
                                        └─► Publish network.provisioned.v1
                                                 │
                                                 ▼
                                         Notification Module
                                         (subscribes to network.provisioned)
                                                 │
                                                 ▼
                                         Send "Your new connection is ready" SMS
```

**Key isolation points:**

- **Subscription module** owns the plan details and subscription status. It emits an event containing the customer ID, plan ID, and subscription ID—nothing more.
- **Network module** does **not** read from `subscription` tables. It uses the event payload to perform its own provisioning. The VLAN assignment and IP management are entirely within the `network` schema.
- **Billing module** independently creates an invoice based on the same event, also without direct access to subscription internals.
- The **Notification module** waits for the actual network provisioning to complete (`network.provisioned.v1`), ensuring the customer is only notified when the connection is ready.

---
Below is the expanded **Platform Plane** section, designed to complete the AeroXe Broadband architecture. Each platform module is treated as a bounded context (where appropriate) or a dedicated infrastructure capability, with strict branch isolation. The flows demonstrate how they coordinate business processes without touching other modules’ databases.

---

## 11. Platform Plane – Supporting Contexts

The Platform Plane provides cross‑cutting capabilities that are essential for the system’s operation but do not belong to any single business domain. They are implemented as separate modules with well‑defined contracts.

### 11.1 Audit Context

**Responsibility:** Record every significant business event in an append‑only, immutable log for traceability, compliance, and debugging.

- **Database schema:** `audit` (tables: `audit_logs`).
- **Owns:** Audit event storage, retention policies.
- **Does NOT own:** The events themselves (they are published by other modules) – the audit module simply consumes and persists them.

**Implementation:** The audit module subscribes to a NATS subject like `audit.action.>` or (better) each module explicitly publishes an `audit.action.v1` event when a noteworthy action occurs. This allows the audit module to remain completely decoupled.

**Data flow – Audit recording:**

```
Any Module (e.g., Customer, Billing)
      │
      ├── After successful command: publish audit.action.v1 ──► NATS
      │
      └── Continue with business event (e.g., customer.activated.v1)
              │
              ▼
        NATS routes to Audit subscriber
              │
              ▼
        AuditService.record(event)
              │
              ▼
        Save to audit.audit_logs (immutable, with full event envelope)
```

**Branch isolation:** The audit module never reads from other schemas. All information needed (who, what, when, old/new value) is included in the event payload.

**TDD example (audit subscriber):**

```rust
#[tokio::test]
async fn audit_subscriber_persists_event() {
    let db = setup_audit_db().await;
    let subscriber = AuditEventSubscriber::new(db.clone(), nats_client());

    // Publish a sample audit event
    let event = AuditActionV1 {
        user_id: "admin".into(),
        action: "SUSPEND_CUSTOMER",
        resource_id: "customer:123",
        old_value: Some("ACTIVE".into()),
        new_value: Some("SUSPENDED".into()),
    };
    nats.publish("aeroxe.audit.action.v1", envelope.encode_to_vec()).await?;

    // Let subscriber process
    sleep(Duration::from_secs(1)).await;

    let logs = db.get_logs_for_resource("customer:123").await;
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].action, "SUSPEND_CUSTOMER");
}
```

### 11.2 Event Context (Event Catalogue & Governance)

While NATS is the transport infrastructure, we need a way to **manage event schemas, versioning, and discovery** – this is the responsibility of the Event context.

**Responsibility:**
- Maintain a registry of all domain event types (schema definitions, versions).
- Validate event payloads against schemas (optional, during development/testing).
- Provide a self‑service UI (in Admin Portal) to explore events.
- Enforce event ownership and deprecation policies.

**Design note:** The event definitions themselves live in `shared/events/` so they are available to all modules. The Event context is a **passive governance module** – it does not interfere with runtime publishing. It simply stores metadata about events.

**Database schema:** `event_catalog` (tables: `event_definitions`, `event_versions`).

**Data flow – Registering a new event version:**

```
Developer / CI pipeline
      │
      ▼
Event Catalog API (or a build step)
      │
      ▼
EventCatalogService.register_event("customer.created.v2", schema)
      │
      ▼
Save to event_catalog.event_versions
```

This is not a runtime flow, but an administrative one. The catalogue is used during integration testing to verify that subscribers can handle the promised event schemas.

**TDD:** Unit tests can check that a published event conforms to the version schema recorded in the catalogue. This ensures that the shared kernel stays consistent.

### 11.3 Scheduler Context

**Responsibility:** Manage recurring or delayed tasks (cron‑like jobs) that trigger business processes.

**Examples:**
- Generate invoices on the 1st of each month.
- Check for expired grace periods and suspend customers.
- Daily network health reports.

**Owns:** Job definitions, schedules, execution history.
**Does NOT own:** The actual business logic – it merely triggers a command or publishes a request event.

**Database schema:** `scheduler` (tables: `job_definitions`, `job_executions`).

**Data flow – Monthly invoice generation:**

```
Scheduler Worker (checks job definitions)
      │
      ▼
Job: "generate_monthly_invoices"
  → publishes scheduler.job.triggered.v1 {
      job_id, job_type: "GENERATE_INVOICES"
    }
      │
      ▼
Billing Module (subscriber to job.triggered with type filter)
      │
      ▼
Billing Worker processes the trigger:
  1. Query active subscriptions (via SubscriptionQueryService)
  2. Generate invoices (billing schema)
  3. Publish invoice.created.v1 events
```

**Branch isolation:** The Scheduler knows nothing about billing logic. It just emits a generic trigger event. The Billing module decides whether to act on it. This keeps the scheduler reusable for any future scheduled task.

**TDD example (scheduler trigger → billing action):**

```rust
#[tokio::test]
async fn monthly_invoice_job_trigger_generates_invoices() {
    // Set up billing db with some active subscriptions (via test fixtures)
    let billing_db = setup_billing_db().await;
    let billing_handler = BillingJobHandler::new(billing_db.clone(), subscription_query_mock());

    // Simulate scheduler event
    let trigger = SchedulerJobTriggeredV1 { job_type: "GENERATE_INVOICES".into(), ... };
    billing_handler.handle(trigger).await.unwrap();

    // Assert invoices were created in billing db
    let invoices = billing_db.get_all_invoices().await;
    assert!(!invoices.is_empty());
}
```

### 11.4 Workflow Context (Sagas / Long‑Running Processes)

**Responsibility:** Coordinate complex, multi‑step transactions that span multiple bounded contexts. A workflow orchestrates a sequence of steps with compensating actions in case of failure.

**Examples:**
- New customer activation (KYC → Account creation → Network provisioning → Welcome notification).
- Subscription upgrade with prorated billing and network reconfiguration.
- Payment failure → grace period → suspension → eventual termination.

The workflow module implements the **saga pattern**: each step is a transaction in one service, and the workflow reacts to success/failure events to proceed or rollback.

**Owns:** Workflow definitions, running instances, state transitions.
**Database schema:** `workflow` (tables: `workflow_instances`, `activity_tasks`).

**Data flow – Customer Activation Workflow (Saga):**

```
Workflow Instance: "CustomerActivation" started after customer.registered.v1
│
├─ Step 1: KYC Verification (call Compliance module)
│   → Publish: workflow.step.request { step: "KYC", payload... }
│   → Compliance processes, publishes kyc.verification.completed.v1
│   → Workflow listens, moves to next step
│
├─ Step 2: Create Billing Account (call Billing module)
│   → Request via sync service or request event
│   → Billing replies with account.created.v1
│
├─ Step 3: Provision Network (call Network module)
│   → Request: provision network for customer
│   → Network publishes network.provisioned.v1
│
├─ Step 4: Send Welcome Notification (call Notification module)
│   → Fire‑and‑forget event
│
└─ Completion: mark workflow instance as COMPLETED
│
If any step fails (e.g., KYC rejected):
  → Workflow executes compensation:
    • If billing account created → request cancellation
    • If network provisioned → request deprovisioning
    • Publish activation.failed.v1
```

**Branch isolation:** The workflow module never calls repositories of other modules directly. It communicates exclusively through:
- Synchronous queries (via defined interfaces) to read state (e.g., “what is the customer’s current status?”).
- Asynchronous commands/events to trigger actions and listen for outcomes.

**TDD example (workflow step execution):**

```rust
#[tokio::test]
async fn activation_workflow_succeeds_when_all_steps_pass() {
    let (workflow, mock_handlers) = setup_workflow_test();

    // Start workflow
    let instance_id = workflow.start("CustomerActivation", &customer_registered_event()).await.unwrap();

    // Simulate KYC completion
    workflow.handle_event(kyc_completed_event(instance_id)).await.unwrap();
    // Simulate billing account created
    workflow.handle_event(account_created_event(instance_id)).await.unwrap();
    // Simulate network provisioned
    workflow.handle_event(network_provisioned_event(instance_id)).await.unwrap();

    let state = workflow.get_state(instance_id).await.unwrap();
    assert_eq!(state, WorkflowState::Completed);

    // Verify that each mock handler was called exactly once
    mock_handlers.verify_kyc_called_once();
    mock_handlers.verify_billing_called_once();
    mock_handlers.verify_network_called_once();
}
```

---

## Integration with the Overall Architecture

The platform plane modules live inside `modules/` alongside business domains, but they are treated as infrastructure‑adjacent contexts:

```
modules/
 ├── ... (business contexts)
 ├── audit/               # Platform
 ├── event_catalog/       # Platform
 ├── scheduler/           # Platform
 └── workflow/            # Platform

```

## 12. Future Extraction to Microservices

The architecture is intentionally designed so that **no business logic ever needs to be rewritten** when moving from the modular monolith to independent services. The same DDD aggregates, application services, and domain events run unchanged—only the deployment boundaries change.

### 12.1 Today: Modular Monolith

```
┌───────────────────────────────────────┐
│         AeroXe Broadband (Rust)       │
│                                       │
│  ┌──────────┐  ┌──────────┐          │
│  │ Customer │  │  Billing │          │
│  │  Module  │  │  Module  │  ...     │
│  └────┬─────┘  └────┬─────┘          │
│       │              │                │
│       └──────┬───────┘                │
│              │                        │
│      ┌───────┴───────┐               │
│      │     NATS       │ (internal)    │
│      └───────────────┘               │
│              │                        │
│   All modules share one process      │
└───────────────────────────────────────┘
```

Communication between modules is:
- **Synchronous:** direct trait calls (e.g., `CustomerQueryService`).
- **Asynchronous:** NATS subjects like `aeroxe.customer.created.v1`.

### 12.2 Tomorrow: Independent Microservices

When traffic or team size demands it, individual modules are extracted into standalone services.

**Step‑by‑step extraction of the `Billing` module:**

1. **Create a new Rust crate** (`aeroxe-billing-service`).
2. **Copy the `billing/` folder** from `src/modules/billing/` into the new crate.
3. **Give it its own database** with the same `billing` schema (migrations are already isolated).
4. **Expose an HTTP/gRPC API** using the same `api/` module’s controller (e.g., convert `customer_controller` into a gRPC server).
5. **Replace in‑process calls** from other modules to the `billing` query/command interfaces with HTTP/gRPC clients. The client implements the **same trait** that the module’s application layer defines, so the calling code barely changes.
6. **NATS subjects remain identical** – `aeroxe.invoice.created.v1` is still published by the billing service and consumed by the notification and audit services, exactly as before.
7. **Remove the `billing` folder** from the monolith and deploy the new service.

The result:

```
 ┌──────────────┐        ┌──────────────┐
 │ customer-    │        │ billing-     │
 │ service      │        │ service      │
 └──────┬───────┘        └──────┬───────┘
        │                       │
        │   NATS (aeroxe.*)     │
        └───────────┬───────────┘
                    │
        ┌───────────┴───────────┐
        │ network-service       │
        └──────────────────────┘
        │ gRPC (sync)
        ▼
 ┌──────────────┐
 │ device-      │
 │ service      │
 └──────────────┘
```

**Key points:**
- **No business rule rewrite:** The domain and application layers are untouched.
- **Event contracts are stable:** Versioned events guarantee backward compatibility.
- **Sync calls become remote:** But the calling code already depends on a trait; we just swap the implementation at wiring time.
- **Testing follows the code:** Unit tests move with the domain/application layers. Integration tests that previously ran inside the monolith can be adjusted to use service boundaries, but the core logic remains verified.

### 12.3 Which Modules Extract First?

Extraction order is driven by non‑functional requirements (scalability, team ownership, independent deployability). Typical candidates:

- **Billing** (compute‑intensive, isolated).
- **Payment** (high security, separate scaling).
- **Network / Device** (interacts heavily with external hardware; separate deployment rhythm).
- **Notification** (stateless, easy to scale).

The `workflow` and `scheduler` services can also be extracted, but they remain lean orchestrators that coordinate through events and service interfaces.

### 12.4 Consistency During Transition

During a migration period, the monolith might still contain some modules while others run as services. This is seamless because:

- NATS messages reach all subscribers regardless of deployment model.
- Synchronous calls from monolith‑resident modules to external services use the same client traits, just pointed to an HTTP/gRPC endpoint instead of an in‑process service.

Thus, the system can be split incrementally, at the team’s own pace.

---

## 13. ISP Operational Gap Mitigation Rules (v3.0)

> **Cross-reference:**
> - `docs/backend/DESIGN-GAPS-DEEP-ANALYSIS.md` (v3.0) — 215 total gaps (84 v1.0 + 68 v2.0 + 76 v3.0)
> - `docs/backend/GAP-security.md` — 13 Tier 0 security vulnerabilities
> - `docs/backend/GAP-code-bugs.md` — 52 code-level bugs with file:line references
> - `docs/backend/GAP-finance-compliance.md` (NEW v3.0) — 25 Indian finance/GST/TDS/Ind AS compliance gaps
> - `docs/backend/GAP-architecture-patterns.md` (NEW v3.0) — 18 architecture pattern gaps + 8 missing workers + 10 network ops domains + 15 SRS design gaps
> - `docs/backend/GAP-IMPLEMENTATION-ROADMAP.md` (v3.0) — 20-week, 12-phase implementation plan

The following rules address critical ISP operational gaps identified in the deep analysis. **All new code MUST comply with these rules.**

### 13.1 Network Operations Rules

| Rule | Description |
|------|-------------|
| **NET-001** | All IP allocation MUST use CIDR math with per-address tracking. No counter-only allocation. |
| **NET-002** | All device adapter connections (SSH, REST) MUST use connection pools. No per-request connection creation. |
| **NET-003** | All device provisioning MUST be automated via `ProvisioningWorker`. Manual NOC intervention is not acceptable for standard installations. |
| **NET-004** | All network device metrics MUST be collected via SNMP. Mock/placeholder data is not acceptable in production. |
| **NET-005** | All bandwidth enforcement MUST be verified on-device after push. No DB-only speed limits. |
| **NET-006** | All RADIUS interactions MUST use proper RFC 2865/2866 compliant implementations. No raw UDP byte-packing for production. |
| **NET-007** | All device adapter parsing MUST handle firmware version differences gracefully. No hardcoded output format assumptions. |

### 13.2 Billing Operations Rules

| Rule | Description |
|------|-------------|
| **BILL-001** | All invoices MUST include GST calculation (CGST 9% + SGST 9% intra-state, IGST 18% inter-state). Tax columns must never be ₹0 for active customers. |
| **BILL-002** | All invoice numbers MUST be unique and non-colliding. Use database sequences, not timestamp-based generation. |
| **BILL-003** | All payment recording MUST support partial payments. Marking entire invoice paid for partial amount is incorrect. |
| **BILL-004** | All late fees MUST be calculated and applied via background worker. Hardcoded values are not acceptable. |
| **BILL-005** | All invoice generation MUST use proper month-end calculation. `period_start + 30 days` is incorrect for months with 28/31 days. |
| **BILL-006** | All mid-cycle plan changes MUST generate pro-rata invoices. The `prorata_adjustments` table must be populated. |

### 13.3 Customer Operations Rules

| Rule | Description |
|------|-------------|
| **CUST-001** | All customer self-service MUST be available via `/api/v1/customer/me/*` endpoints. Customers must not require staff assistance for basic operations. |
| **CUST-002** | All customer notifications MUST respect channel preferences. Customers must be able to opt-out of specific channels. |
| **CUST-003** | All ticket SLA MUST be enforced with timers and auto-escalation. Status-only escalation is not acceptable. |
| **CUST-004** | All subscription downgrades MUST validate current bandwidth usage before applying. Immediate speed reduction during active transfers is not acceptable. |

### 13.4 Infrastructure Rules

| Rule | Description |
|------|-------------|
| **INFRA-001** | All background workers MUST implement graceful shutdown, panic recovery, and restart logic. |
| **INFRA-002** | All external integrations MUST implement retry with exponential backoff, circuit breaker, and fallback. |
| **INFRA-003** | All database operations MUST use connection pooling. Per-request connection creation is not acceptable. |
| **INFRA-004** | All scheduled jobs MUST have idempotency. Re-running a job must not create duplicates. |
| **INFRA-005** | All health check endpoints MUST verify DB, Redis, NATS, and RADIUS connectivity. Returning 200 when dependencies are down is not acceptable. |
| **INFRA-006** | All external HTTP calls (webhooks, API integrations) MUST use circuit breaker + retry + DLQ pattern. Silent failures are not acceptable. |
| **INFRA-007** | All worker queues MUST implement per-job DLQ with max retry count. A single bad record must not block the entire queue. |

### 13.5 Finance & Tax Compliance Rules (v3.0 NEW)

| Rule | Description |
|------|-------------|
| **FIN-001** | All invoices MUST include GST (CGST 9% + SGST 9% intra-state, IGST 18% inter-state). Tax must never be ₹0 for active customers. |
| **FIN-002** | All invoices MUST include place-of-supply comparison between provider and customer state codes. |
| **FIN-003** | All late fees MUST include 18% GST per Circular 178/10/2022. |
| **FIN-004** | Credit notes and debit notes MUST be supported for all post-invoice corrections (Section 34 CGST Act). |
| **FIN-005** | Security deposits MUST be tracked as refundable liabilities in a separate ledger. |
| **FIN-006** | Revenue recognition MUST follow Ind AS 115 — deferred revenue for annual/quarterly prepayments. |
| **FIN-007** | HSN/SAC codes MUST be assigned per line item: broadband=998421, router_rental=998314, late_fee=997159. |
| **FIN-008** | Tax invoices MUST include all 8 mandatory fields per Rule 46 CGST Rules. |

### 13.6 Architecture Pattern Rules (v3.0 NEW)

| Rule | Description |
|------|-------------|
| **ARCH-001** | All external adapter calls (MikroTik, Huawei, RADIUS) MUST use circuit breaker pattern with failure threshold. |
| **ARCH-002** | All background workers MUST have dedicated connection pool limits, not share the main pool. |
| **ARCH-003** | All multi-step provisioning MUST implement saga compensation for rollback on partial failure. |
| **ARCH-004** | All distributed retries MUST use exponential backoff with jitter to prevent thundering herd. |
| **ARCH-005** | All CDR data MUST be persisted in a partitioned `cdr_records` table for usage dispute resolution. |
| **ARCH-006** | All fiber plant operations MUST have a physical layer model (OLT→Splitter→ONT). |

### 13.7 Testing Requirements for Gap Mitigation

| Test Type | Requirement |
|-----------|-------------|
| **IP Allocation** | Unit tests for CIDR parsing, range generation, conflict detection, allocation/release |
| **Provisioning** | Integration test for full provisioning sequence: RADIUS → BNG → OLT → verify |
| **GST Calculation** | Unit tests for intra-state (CGST+SGST), inter-state (IGST), zero-tax scenarios |
| **Bandwidth Push** | Integration test for bandwidth profile application with device verification |
| **SLA Enforcement** | Unit test for SLA timer calculation, breach detection, escalation matrix |
| **Invoice Generation** | Unit test for month-end calculation, pro-rata, line item totals |

---

## 14. Conclusion

This document provides a complete, enterprise‑grade blueprint for building AeroXe Broadband with DDD and TDD. The folder structure enforces strict isolation, aggregate design, versioned events, and dedicated security/compliance contexts. Testing is woven into every layer, ensuring that the system remains maintainable, scalable, and ready for future microservice extraction.

The ISP operational gap mitigation rules (Section 13) ensure that all new code addresses the critical gaps identified in the deep analysis. These rules are mandatory for all new development.

**Version 3.0** — Updated 2026-07-21 with v3.0 finance compliance, architecture patterns, and network ops gap analysis.

Adopting this architecture will result in a codebase that accurately reflects the ISP domain, is resilient to change, and can be developed with confidence through test‑first practices.

---

*Document maintained by the AeroXe Engineering Team. Version 3.0 – July 2026.*