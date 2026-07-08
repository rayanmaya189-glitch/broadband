# AeroXe Backend — Coverage & Service Area Module

> **Req Ref:** §3.5 Coverage & Service Area Management

---

## 1. Overview

Manages geographic service coverage areas to determine whether a customer address is within the ISP's service footprint. Uses PostGIS for spatial queries and pincode-based lookups.

## 2. Database Tables

```sql
CREATE TABLE coverage_areas (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    area_type VARCHAR(30) DEFAULT 'polygon'
        CHECK (area_type IN ('polygon', 'circle', 'pincode')),
    boundary polygon,
    center_point point,
    radius_meters INTEGER,
    pincodes TEXT[],
    is_active BOOLEAN DEFAULT TRUE,
    max_customers INTEGER,
    current_customers INTEGER DEFAULT 0,
    fiber_available BOOLEAN DEFAULT TRUE,
    estimated_installation_days INTEGER DEFAULT 3,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE coverage_pincode_map (
    id BIGSERIAL PRIMARY KEY,
    coverage_area_id BIGINT NOT NULL REFERENCES coverage_areas(id) ON DELETE CASCADE,
    pincode VARCHAR(10) NOT NULL,
    city VARCHAR(100) NOT NULL,
    district VARCHAR(100),
    state VARCHAR(100) DEFAULT 'Maharashtra',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(coverage_area_id, pincode)
);

CREATE TABLE coverage_subnets (
    id BIGSERIAL PRIMARY KEY,
    coverage_area_id BIGINT NOT NULL REFERENCES coverage_areas(id) ON DELETE CASCADE,
    ip_pool_id BIGINT REFERENCES ip_pools(id),
    vlan_id BIGINT REFERENCES vlans(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Indexes:**
```sql
CREATE INDEX idx_coverage_areas_branch ON coverage_areas(branch_id);
CREATE INDEX idx_coverage_areas_boundary ON coverage_areas USING GIST(boundary);
CREATE INDEX idx_coverage_areas_center ON coverage_areas USING GIST(center_point);
CREATE INDEX idx_coverage_pincode_pincode ON coverage_pincode_map(pincode);
```

## 3. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/coverage/areas` | branch-scoped | List coverage areas |
| POST | `/api/v1/coverage/areas` | network_admin+ | Create coverage area |
| GET | `/api/v1/coverage/areas/:id` | branch-scoped | Get area details |
| PUT | `/api/v1/coverage/areas/:id` | network_admin+ | Update area |
| DELETE | `/api/v1/coverage/areas/:id` | network_admin+ | Deactivate area |
| POST | `/api/v1/coverage/check` | No (public) | Check pincode availability |
| POST | `/api/v1/coverage/check/address` | No (public) | Check address availability |
| GET | `/api/v1/coverage/areas/:id/stats` | network_admin+ | Area statistics |
| POST | `/api/v1/coverage/areas/:id/pincodes` | network_admin+ | Add pincodes |
| DELETE | `/api/v1/coverage/areas/:id/pincodes/:pincode` | network_admin+ | Remove pincode |

## 4. Availability Check Flow

```
1. Customer enters pincode on landing page
2. POST /coverage/check { pincode: "425001" }
3. Query coverage_pincode_map for pincode
4. If match found → check coverage_areas.fiber_available
5. Return: { available: true, estimated_days: 3, area_name: "City Center" }
6. If no match → return: { available: false, message: "Service not yet available" }
7. Log query for demand analytics (NATS event)
```

## 5. Spatial Query Example

```sql
-- Check if a point is within any coverage area
SELECT ca.name, ca.fiber_available, ca.estimated_installation_days
FROM coverage_areas ca
WHERE ca.is_active = true
  AND ca.branch_id = $1
  AND (
    ca.boundary @> POINT($2, $3)
    OR ST_DWithin(ca.center_point, POINT($2, $3), ca.radius_meters)
    OR $4 = ANY(ca.pincodes)
  )
LIMIT 1;
```

## 6. RBAC Permissions

```
coverage.area.view
coverage.area.create
coverage.area.update
coverage.area.delete
coverage.check.view
```
