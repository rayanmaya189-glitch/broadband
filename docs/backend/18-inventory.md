# AeroXe Backend — Inventory Module

> **Req Ref:** §6.7 Hardware Inventory Management

---

## 1. Overview

Tracks all physical equipment from procurement to disposal. Manages stock levels, assignments to technicians/branches, and movement history (received → assigned → installed → returned → scrapped).

## 2. Database Tables

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

## 3. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/inventory` | inventory.view | List inventory items |
| POST | `/api/v1/inventory` | inventory.receive | Add item to inventory |
| GET | `/api/v1/inventory/:id` | inventory.view | Get item details |
| PUT | `/api/v1/inventory/:id` | inventory.view | Update item |
| POST | `/api/v1/inventory/:id/assign` | inventory.assign | Assign to technician |
| POST | `/api/v1/inventory/:id/install` | inventory.assign | Mark as installed |
| POST | `/api/v1/inventory/:id/return` | inventory.assign | Return item |
| POST | `/api/v1/inventory/:id/transfer` | inventory.transfer | Transfer between branches |
| POST | `/api/v1/inventory/:id/scrap` | inventory.scrapp | Scrap item |
| GET | `/api/v1/inventory/:id/movements` | inventory.view | Movement history |
| GET | `/api/v1/inventory/reports` | inventory.report | Inventory reports |
| GET | `/api/v1/inventory/alerts` | inventory.view | Low stock / warranty alerts |

## 4. Inventory Lifecycle

```
received → assigned → installed
                   → returned → in_stock
                   → damaged → scrapped
         → in_transit (between branches)
```

## 5. RBAC Permissions

```
inventory.view
inventory.receive
inventory.assign
inventory.transfer
inventory.scrapp
inventory.report
```
