# AeroXe Broadband — API Design Conventions (Protobuf-First)

**Version:** 1.0
**Date:** 2026-07-21
**Scope:** All API endpoints across backend, admin, mobile, customer portal

---

## 1. Core Principles

| Rule | Description |
|------|-------------|
| **PF-001** | ALL request and response bodies MUST be encoded as Protocol Buffers (protobuf). JSON is not accepted. |
| **PF-002** | NO GET endpoints. All reads use `POST /resource/get` or `POST /resource/list`. |
| **PF-003** | NO PUT endpoints. All updates use `PATCH /resource/update`. |
| **PF-004** | NO path variables. All identifiers (`:id`) MUST be fields in the protobuf request body. |
| **PF-005** | NO query strings. All filters, pagination, sorting MUST be fields in the protobuf request body. |
| **PF-006** | DELETE endpoints use `DELETE /resource/delete` with protobuf body containing the identifier. |
| **PF-007** | All responses MUST use a standard protobuf envelope: `Response<T> { status, data, error, meta }`. |

---

## 2. HTTP Method Mapping

| Old Pattern | New Pattern | HTTP Method | Path |
|-------------|-------------|-------------|------|
| `GET /api/v1/customers` | List customers | `POST` | `/api/v1/customers/list` |
| `GET /api/v1/customers/:id` | Get customer | `POST` | `/api/v1/customers/get` |
| `POST /api/v1/customers` | Create customer | `POST` | `/api/v1/customers/create` |
| `PUT /api/v1/customers/:id` | Full update | `PATCH` | `/api/v1/customers/update` |
| `PATCH /api/v1/customers/:id` | Partial update | `PATCH` | `/api/v1/customers/update` |
| `DELETE /api/v1/customers/:id` | Delete customer | `DELETE` | `/api/v1/customers/delete` |
| `GET /api/v1/customers/:id/invoices` | Sub-resource list | `POST` | `/api/v1/customers/invoices/list` |
| `POST /api/v1/customers/:id/suspend` | Custom action | `POST` | `/api/v1/customers/suspend` |

---

## 3. Protobuf Message Design

### 3.1 Standard Response Envelope

```protobuf
syntax = "proto3";
package aeroxe.v1;

message Response {
  ResponseStatus status = 1;
  bytes data = 2;           // Serialized inner message
  ResponseError error = 3;
  ResponseMeta meta = 4;
}

enum ResponseStatus {
  RESPONSE_STATUS_UNSPECIFIED = 0;
  RESPONSE_STATUS_OK = 1;
  RESPONSE_STATUS_ERROR = 2;
  RESPONSE_STATUS_VALIDATION_ERROR = 3;
  RESPONSE_STATUS_NOT_FOUND = 4;
  RESPONSE_STATUS_UNAUTHORIZED = 5;
  RESPONSE_STATUS_FORBIDDEN = 6;
  RESPONSE_STATUS_CONFLICT = 7;
}

message ResponseError {
  string code = 1;
  string message = 2;
  repeated FieldError details = 3;
}

message FieldError {
  string field = 1;
  string message = 2;
}

message ResponseMeta {
  uint64 total_count = 1;
  uint32 page = 2;
  uint32 page_size = 3;
  string next_cursor = 4;
}
```

### 3.2 Example: Customer Module

```protobuf
syntax = "proto3";
package aeroxe.v1.customer;

// ─── Requests ───

message CreateCustomerRequest {
  string first_name = 1;
  string last_name = 2;
  string email = 3;
  string phone = 4;
  string customer_type = 5;    // "residential" | "enterprise"
  string branch_id = 6;
}

message GetCustomerRequest {
  string customer_id = 1;
}

message ListCustomersRequest {
  uint32 page = 1;
  uint32 page_size = 2;
  string status = 3;           // "active" | "suspended" | "terminated"
  string customer_type = 4;
  string search = 5;           // name, email, phone, ID
  string branch_id = 6;
  string sort_by = 7;
  string sort_order = 8;       // "asc" | "desc"
}

message UpdateCustomerRequest {
  string customer_id = 1;
  optional string first_name = 2;
  optional string last_name = 3;
  optional string email = 4;
  optional string phone = 5;
  optional string status = 6;
}

message DeleteCustomerRequest {
  string customer_id = 1;
}

message SuspendCustomerRequest {
  string customer_id = 1;
  string reason = 2;
}

// ─── Responses ───

message Customer {
  string customer_id = 1;
  string first_name = 2;
  string last_name = 3;
  string email = 4;
  string phone = 5;
  string customer_type = 6;
  string status = 7;
  string branch_id = 8;
  string created_at = 9;
  string updated_at = 10;
}

message ListCustomersResponse {
  repeated Customer customers = 1;
  uint64 total_count = 2;
  uint32 page = 3;
  uint32 page_size = 4;
}

// ─── Service ───

service CustomerService {
  rpc CreateCustomer(CreateCustomerRequest) returns (Response);
  rpc GetCustomer(GetCustomerRequest) returns (Response);
  rpc ListCustomers(ListCustomersRequest) returns (Response);
  rpc UpdateCustomer(UpdateCustomerRequest) returns (Response);
  rpc DeleteCustomer(DeleteCustomerRequest) returns (Response);
  rpc SuspendCustomer(SuspendCustomerRequest) returns (Response);
  rpc ActivateCustomer(ActivateCustomerRequest) returns (Response);
}
```

### 3.3 Example: Billing Module

```protobuf
syntax = "proto3";
package aeroxe.v1.billing;

message CreateInvoiceRequest {
  string subscription_id = 1;
  string billing_period_start = 2;  // ISO 8601
  string billing_period_end = 3;
}

message ListInvoicesRequest {
  uint32 page = 1;
  uint32 page_size = 2;
  string customer_id = 3;
  string status = 4;           // "pending" | "paid" | "overdue" | "cancelled"
  string date_from = 5;
  string date_to = 6;
}

message GetInvoiceRequest {
  string invoice_id = 1;
}

message RecordPaymentRequest {
  string invoice_id = 1;
  string payment_method = 2;
  string transaction_id = 3;
  string amount = 4;           // String to avoid float precision
  string payment_date = 5;
}

message ProcessPartialPaymentRequest {
  string invoice_id = 1;
  string amount = 2;
  string payment_method = 3;
  string transaction_id = 4;
}
```

---

## 4. Path Convention

| Operation | Path Pattern | HTTP Method |
|-----------|-------------|-------------|
| Create | `POST /api/v1/{module}/create` | POST |
| Get one | `POST /api/v1/{module}/get` | POST |
| List | `POST /api/v1/{module}/list` | POST |
| Update | `PATCH /api/v1/{module}/update` | PATCH |
| Delete | `DELETE /api/v1/{module}/delete` | DELETE |
| Custom action | `POST /api/v1/{module}/{action}` | POST |

**Module paths:**

| Module | Path Prefix |
|--------|-------------|
| Auth | `/api/v1/auth/` |
| Customers | `/api/v1/customers/` |
| Subscriptions | `/api/v1/subscriptions/` |
| Billing | `/api/v1/billing/` |
| Invoices | `/api/v1/invoices/` |
| Payments | `/api/v1/payments/` |
| Plans | `/api/v1/plans/` |
| Network | `/api/v1/network/` |
| Devices | `/api/v1/devices/` |
| Bandwidth | `/api/v1/bandwidth/` |
| Tickets | `/api/v1/tickets/` |
| Notifications | `/api/v1/notifications/` |
| Users | `/api/v1/users/` |
| Branches | `/api/v1/branches/` |
| Accounting | `/api/v1/accounting/` |
| Audit | `/api/v1/audit/` |
| Leads | `/api/v1/leads/` |
| Referrals | `/api/v1/referrals/` |
| Coverage | `/api/v1/coverage/` |
| Installations | `/api/v1/installations/` |
| Inventory | `/api/v1/inventory/` |
| Documents | `/api/v1/documents/` |
| Discovery | `/api/v1/discovery/` |

---

## 5. Pagination (Protobuf Body)

```protobuf
message PaginationRequest {
  uint32 page = 1;             // 1-indexed, default 1
  uint32 page_size = 2;        // Default 20, max 100
  string sort_by = 3;          // Field name
  string sort_order = 4;       // "asc" | "desc"
  string cursor = 5;           // For cursor-based pagination (optional)
}

message PaginationResponse {
  uint64 total_count = 1;
  uint32 page = 2;
  uint32 page_size = 3;
  string next_cursor = 4;
  bool has_more = 5;
}
```

**No query strings like `?page=1&limit=20&sort=name`** — all pagination fields go in the protobuf body.

---

## 6. Filtering (Protobuf Body)

```protobuf
// NO: GET /api/v1/customers?status=active&branch_id=123
// YES: POST /api/v1/customers/list with:

message ListCustomersRequest {
  PaginationRequest pagination = 1;
  string status = 2;
  string branch_id = 3;
  string customer_type = 4;
  string search = 5;
  string created_from = 6;
  string created_to = 7;
}
```

---

## 7. IDs in Request Body (No Path Variables)

```protobuf
// NO: GET /api/v1/customers/cust_abc123
// YES: POST /api/v1/customers/get with:

message GetCustomerRequest {
  string customer_id = 1;    // ID in body, not in path
}

// NO: PATCH /api/v1/customers/cust_abc123
// YES: PATCH /api/v1/customers/update with:

message UpdateCustomerRequest {
  string customer_id = 1;    // ID in body
  optional string email = 2;
}
```

---

## 8. Protobuf Compilation

### 8.1 Directory Structure

```
proto/
├── buf.yaml                    # Buf configuration
├── buf.gen.yaml                # Code generation config
├── aeroxe/
│   ├── v1/
│   │   ├── common.proto        # Response, Pagination, Error messages
│   │   ├── customer.proto
│   │   ├── billing.proto
│   │   ├── subscription.proto
│   │   ├── network.proto
│   │   ├── device.proto
│   │   ├── ticket.proto
│   │   ├── notification.proto
│   │   └── ...
│   └── ...
└── generated/                  # Generated Rust code (pb-build)
    └── aeroxe.v1/
```

### 8.2 Cargo Dependencies

```toml
[dependencies]
prost = "0.12"
prost-types = "0.12"
tonic = "0.11"          # gRPC + HTTP transcoding
tonic-build = "0.11"    # Build script
```

### 8.3 Build Script

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &[
                "proto/aeroxe/v1/common.proto",
                "proto/aeroxe/v1/customer.proto",
                "proto/aeroxe/v1/billing.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}
```

---

## 9. Axum Integration

```rust
use axum::{routing::{post, patch, delete}, Router};
use prost::Message;

// Protobuf content type
const PROTOBUF_CONTENT_TYPE: &str = "application/protobuf";

// Extractor for protobuf body
async fn extract_protobuf<T: Message + Default>(
    body: axum::body::Bytes,
) -> Result<T, AppError> {
    T::decode(&body[..])
        .map_err(|e| AppError::Validation(format!("Invalid protobuf: {}", e)))
}

// Proto response helper
fn proto_response<T: Message>(data: T) -> axum::response::Response {
    let mut buf = bytes::BytesMut::with_capacity(data.encoded_len());
    data.encode(&mut buf).unwrap();
    axum::response::Response::builder()
        .header("content-type", PROTOBUF_CONTENT_TYPE)
        .body(axum::body::Body::from(buf))
        .unwrap()
}

// Router (no path variables!)
let app = Router::new()
    .route("/api/v1/customers/create", post(create_customer))
    .route("/api/v1/customers/get", post(get_customer))
    .route("/api/v1/customers/list", post(list_customers))
    .route("/api/v1/customers/update", patch(update_customer))
    .route("/api/v1/customers/delete", delete(delete_customer))
    .route("/api/v1/customers/suspend", post(suspend_customer))
    .route("/api/v1/customers/activate", post(activate_customer));
```

---

## 10. Migration from Current REST API

### Phase 1: Define Proto Files (Week 1)
- Create `proto/` directory
- Define `common.proto` (Response, Pagination, Error)
- Define proto files for all 28 modules
- Set up `buf.yaml` and `tonic-build`

### Phase 2: Axum Router Migration (Week 2)
- Replace all Axum routes with POST/PATCH/DELETE
- Add protobuf extractor middleware
- Remove all path variable extractors
- Add content-type validation middleware

### Phase 3: Module Handler Migration (Week 3-4)
- Migrate each module's handlers to accept protobuf
- Update all handler signatures
- Remove query string parsing
- Add protobuf response encoding

### Phase 4: Frontend Migration (Week 5-8)
- Update all HTTP clients to use protobuf encoding
- Replace query string construction with protobuf message construction
- Update all path variable usage to body fields
- Add protobuf serialization/deserialization

---

## 11. Breaking Changes Policy

| Change Type | Action | Required |
|-------------|--------|----------|
| Adding new field to proto message | Add with new field number | No version bump |
| Removing field | Mark as `reserved` | Major version bump |
| Changing field number | NOT ALLOWED | — |
| Adding new RPC method | Add to service | Minor version bump |
| Changing HTTP path | NOT ALLOWED without version bump | Major version bump |
| Changing protobuf wire format | NOT ALLOWED | — |

---

*Document version: 1.0 — 2026-07-21*
*Supersedes: All previous REST API conventions*
