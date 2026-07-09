For your ISP backend, use the project name:

# **AeroXe Broadband**

Recommended naming:

```text
aeroxe-broadband/
│
├── backend/
│   └── Rust ISP Platform
│
├── frontend/
│   └── Admin Portal + Customer Portal
│
├── mobile/
│   ├── android/
│   └── ios/
│
├── infrastructure/
│   ├── docker/
│   ├── kubernetes/
│   └── terraform/
│
└── docs/
    ├── architecture/
    ├── api/
    ├── database/
    └── operations/
```

For the Rust backend:

```text
aeroxe-broadband-backend/
```

or simply:

```text
aeroxe-broadband/
```

Cargo package:

```toml
[package]
name = "aeroxe-broadband"
version = "0.1.0"
edition = "2024"
```

---

Suggested module namespace:

```text
src/

├── modules/

│   ├── auth/
│   ├── customer/
│   ├── subscription/
│   ├── billing/
│   ├── payment/
│   ├── network/
│   ├── device/
│   ├── bandwidth/
│   ├── ip_management/
│   ├── monitoring/
│   ├── ticket/
│   ├── notification/
│   └── audit/


├── infrastructure/

│   ├── postgres/
│   ├── redis/
│   ├── nats/
│   ├── websocket/
│   └── observability/


├── workers/

│   ├── device_sync_worker.rs
│   ├── bandwidth_worker.rs
│   ├── billing_worker.rs
│   └── notification_worker.rs


└── shared/

    ├── events/
    ├── errors/
    ├── types/
    └── utils/
```

---

For internal service/event naming:

NATS subjects:

```text
aeroxe.customer.created

aeroxe.customer.updated

aeroxe.subscription.created

aeroxe.payment.completed

aeroxe.device.online

aeroxe.device.offline

aeroxe.bandwidth.updated
```

---

Database naming:

```text
aeroxe_broadband
```

Schemas:

```text
auth

customer

billing

network

device

monitoring

audit
```

---

Docker/Kubernetes naming:

```text
namespace:

aeroxe-broadband


containers:

aeroxe-api

aeroxe-worker

aeroxe-postgres

aeroxe-redis

aeroxe-nats
```

---

Rust crate naming convention:

```
aeroxe_broadband
```

Rust uses underscore internally:

```rust
use aeroxe_broadband::modules::customer;
```

while repositories and deployments use:

```
aeroxe-broadband
```

This naming is clean for evolving from a modular monolith into future AeroXe Broadband microservices.


Below is the recommended production structure example strickly follow this structure.

```text
backend/

├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── .env
│
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_customers.sql
│   ├── 003_create_network.sql
│   └── 004_create_billing.sql
│
└── src/

    ├── main.rs
    ├── lib.rs


    ├── config/
    │   ├── database.rs
    │   ├── redis.rs
    │   ├── nats.rs
    │   └── settings.rs


    ├── modules/
    │
    │
    │── auth/
    │   │
    │   ├── domain/
    │   │   ├── entities/
    │   │   │   └── user.rs
    │   │   ├── value_objects/
    │   │   │   └── password.rs
    │   │   └── rules/
    │   │       └── auth_rules.rs
    │   │
    │   ├── application/
    │   │   ├── commands/
    │   │   │   └── login.rs
    │   │   ├── queries/
    │   │   │   └── get_user.rs
    │   │   └── services/
    │   │       └── auth_service.rs
    │   │
    │   ├── infrastructure/
    │   │   ├── repository/
    │   │   │   └── postgres_user_repository.rs
    │   │   └── security/
    │   │       └── jwt.rs
    │   │
    │   └── api/
    │       ├── http/
    │       │   └── auth_controller.rs
    │       └── grpc/
    │           └── auth_service.rs
    │


    │── customer/
    │   │
    │   ├── domain/
    │   │   ├── entities/
    │   │   │   └── customer.rs
    │   │   ├── value_objects/
    │   │   │   ├── email.rs
    │   │   │   └── phone.rs
    │   │   └── rules/
    │   │       └── customer_rules.rs
    │   │
    │   ├── application/
    │   │   ├── commands/
    │   │   │   ├── create_customer.rs
    │   │   │   ├── suspend_customer.rs
    │   │   │   └── activate_customer.rs
    │   │   │
    │   │   ├── queries/
    │   │   │   └── get_customer.rs
    │   │   │
    │   │   └── services/
    │   │       └── customer_service.rs
    │   │
    │   ├── infrastructure/
    │   │   │
    │   │   ├── repository/
    │   │   │   └── postgres_customer_repository.rs
    │   │   │
    │   │   ├── messaging/
    │   │   │
    │   │   ├── publishers/
    │   │   │   └── customer_event_publisher.rs
    │   │   │
    │   │   └── subscribers/
    │   │       └── payment_event_subscriber.rs
    │   │
    │   └── api/
    │       ├── http/
    │       │   └── customer_controller.rs
    │       └── grpc/
    │           └── customer_service.rs
    │



    │── subscription/
    │
    │── billing/
    │
    │── payment/
    │
    │── network/
    │
    │── device/
    │
    │── bandwidth/
    │
    │── monitoring/
    │
    │── ticket/
    │
    │── notification/
    │
    └── audit/


    ├── infrastructure/
    │
    │
    ├── database/
    │   ├── postgres.rs
    │   └── transaction.rs
    │
    ├── cache/
    │   └── redis.rs
    │
    ├── messaging/
    │   │
    │   ├── nats_client.rs
    │   ├── event_bus.rs
    │   └── subjects.rs
    │
    ├── websocket/
    │   └── websocket_server.rs
    │
    └── observability/
        ├── logging.rs
        ├── metrics.rs
        └── tracing.rs



    ├── workers/

    │
    ├── device_sync_worker.rs
    │
    ├── bandwidth_worker.rs
    │
    ├── billing_worker.rs
    │
    └── notification_worker.rs



    ├── shared/

    │
    ├── errors/
    │   └── app_error.rs
    │
    ├── events/
    │   ├── customer_events.rs
    │   ├── billing_events.rs
    │   └── network_events.rs
    │
    ├── types/
    │   └── ids.rs
    │
    └── utils/
        └── datetime.rs
```

---

# How Data Flows

## Customer Registration

```text
Mobile App
    |
    |
HTTP API
    |
    |
Customer Controller
    |
    |
Customer Service
    |
    |
Customer Domain
    |
    |
Validate Rules
    |
    |
Repository
    |
    |
PostgreSQL


After success:

Customer Event Publisher

    |
    |
NATS

customer.created


        |
        |
        +----------------+
        |                |
        |                |
   Billing Module    Network Module

   Create Invoice    Create VLAN
                    Apply Speed Plan


        |
        |
 Notification Module

 Send SMS
```

---

# NATS Subjects Design Examples

Create a central event naming standard.

Example:

```text
customer.created

customer.updated

customer.suspended

customer.deleted


subscription.created

subscription.changed


payment.completed

payment.failed


device.online

device.offline


bandwidth.changed
```

---

# Example Event Ownership

## Customer Module Publishes

```text
customer.created
customer.updated
customer.suspended
```

---

## Billing Module Publishes

```text
invoice.created

payment.completed

payment.failed
```

---

## Network Module Publishes

```text
device.online

device.offline

bandwidth.applied
```

---

# Subscriber Examples

## Billing subscribes:

```text
customer.created
```

Purpose:

```
Create first invoice
```

---

## Network subscribes:

```text
subscription.created
```

Purpose:

```
Create network profile
Assign VLAN
Apply bandwidth
```

---

## Notification subscribes:

```text
customer.created

payment.completed

device.offline
```

Purpose:

```
Send SMS/email/push
```

---

# Database Ownership

Even in monolith:

```text
customer module

owns:

customer tables


billing module

owns:

invoice tables


network module

owns:

router/device tables
```

Do not create:

```text
one giant database.rs

with every module querying everything
```

---

# Future Extraction

Today:

```text
ONE Rust Application


customer
billing
network
device


        |
        |
       NATS

```

Later:

```text
customer-service

        |
       NATS

billing-service

        |
       NATS

network-service


        |
       gRPC

device-service
```

No business rewrite.

---

# For AeroXe Broadband, I would split modules like this below example:

## Business Plane

```
auth
customer
subscription
billing
payment
ticket
notification
crm
reporting
```

## Network Plane

```
network
device
bandwidth
ip_management
traffic
monitoring
automation
```

## Platform Plane

```
audit
event
scheduler
workflow
```

---
