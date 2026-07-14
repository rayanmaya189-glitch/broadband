# AeroXe Broadband – Domain-Driven Design & Test-Driven Development Guide (Enterprise Edition)

**Version 2.0 – Complete DDD + TDD Folder Architecture**

This document defines the engineering standards for the **AeroXe Broadband** backend using **Domain-Driven Design** and **Test-Driven Development**. It includes a production‑ready folder architecture that enforces strict bounded‑context isolation, aggregate design, event versioning, and dedicated security/compliance contexts. Every component is designed to be tested from the ground up, with clear guidance on test placement and strategy.

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
    │   ├── network/
    │   │   ├── domain/ ...                      # Aggregate: NetworkDevice, VLAN
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
        ├── events/                      # Versioned domain event definitions
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
        ├── types/
        │   └── ids.rs                  # Shared ID types (CustomerId, etc.)
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub event_id: Uuid,
    pub event_type: String,
    pub version: u32,
    pub occurred_at: DateTime<Utc>,
    pub producer: String,         // "customer-service"
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
    let event = CustomerCreatedV1 { customer_id: Uuid::new_v4(), email: "a@b.com".into() };
    let envelope = EventEnvelope::new(event, "customer-service".to_string());
    nats.publish("aeroxe.customer.created.v1", serde_json::to_vec(&envelope).unwrap()).await.unwrap();

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

## 10. Future Evolution to Microservices

The architecture already treats each module as an independent bounded context with its own database schema and event contracts. To extract a service:

1. Extract the module’s `domain`, `application`, `infrastructure`, and `api` folders into a new Rust crate.
2. Give it its own database (clone the schema).
3. Replace in‑process calls in other modules with HTTP/gRPC clients (using the same application service interfaces).
4. No changes to NATS subjects – events remain unchanged.
5. All existing tests are portable and continue to work, with integration tests adjusted for the network boundary.

---

## 11. Conclusion

This document provides a complete, enterprise‑grade blueprint for building AeroXe Broadband with DDD and TDD. The folder structure enforces strict isolation, aggregate design, versioned events, and dedicated security/compliance contexts. Testing is woven into every layer, ensuring that the system remains maintainable, scalable, and ready for future microservice extraction.

Adopting this architecture will result in a codebase that accurately reflects the ISP domain, is resilient to change, and can be developed with confidence through test‑first practices.

---

*Document maintained by the AeroXe Engineering Team. Version 2.0 – July 2026.*