# AeroXe Backend — RBAC Module

> **Req Ref:** §2 User Roles and RBAC System

---

## 1. Overview

Role-Based Access Control with hierarchical role inheritance, resource-level permissions, and branch-scoped access. Every API action is checked against the user's assigned role and permissions.

## 2. Database Tables

```sql
CREATE TABLE roles (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    parent_role_id BIGINT REFERENCES roles(id),
    is_system BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE permissions (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    module VARCHAR(100) NOT NULL,
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(module, resource, action)
);

CREATE TABLE role_permissions (
    id BIGSERIAL PRIMARY KEY,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    conditions JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(role_id, permission_id)
);

CREATE TABLE user_roles (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_by BIGINT REFERENCES users(id),
    is_active BOOLEAN DEFAULT TRUE,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, role_id)
);

CREATE TABLE permission_groups (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE permission_group_permissions (
    id BIGSERIAL PRIMARY KEY,
    group_id BIGINT NOT NULL REFERENCES permission_groups(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    UNIQUE(group_id, permission_id)
);
```

## 3. Role Hierarchy

```
super_admin
├── isp_owner
│   ├── network_admin
│   │   └── noc_engineer
│   └── finance_manager
│       └── billing_operator
├── field_technician
├── customer_support
├── sales_agent
└── customer
```

Inheritance is additive: a child role gets all parent permissions + its own.

## 4. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/rbac/roles/list` | isp_owner+ | List all roles |
| POST | `/api/v1/rbac/roles/create` | super_admin | Create custom role |
| PATCH | `/api/v1/rbac/roles/update` | super_admin | Update role |
| DELETE | `/api/v1/rbac/roles/delete` | super_admin | Delete custom role |
| POST | `/api/v1/rbac/permissions/list` | isp_owner+ | List all permissions |
| POST | `/api/v1/rbac/roles/permissions` | super_admin | Assign permissions to role |
| DELETE | `/api/v1/rbac/roles/permissions/delete` | super_admin | Remove permission from role |
| POST | `/api/v1/rbac/users/roles/list` | isp_owner+ | List user's roles |
| POST | `/api/v1/rbac/users/roles` | super_admin | Assign role to user |
| DELETE | `/api/v1/rbac/users/roles/delete` | super_admin | Revoke role from user |
| POST | `/api/v1/rbac/temporary/create` | isp_owner+ | Grant temporary permission |
| POST | `/api/v1/rbac/approval-workflows/list` | isp_owner+ | List approval workflows |
| POST | `/api/v1/rbac/approval-requests/approve` | Approver role | Approve request |
| POST | `/api/v1/rbac/approval-requests/reject` | Approver role | Reject request |

## 5. Permission Resolution Algorithm

```
fn resolve_permissions(user_id) -> Vec<String>:
    1. Fetch user_roles (active, not expired)
    2. For each role:
       a. Fetch role's direct permissions
       b. Fetch parent role's permissions (recursively)
       c. Merge all permissions (deduplicated)
    3. Add group permissions if role is in a permission group
    4. Cache result in Redis: SET rbac:permissions:{user_id} {json} EX 1800
    5. Return merged permission list
```

## 6. RBAC Middleware

```rust
// Tower middleware that extracts user context and checks permissions
pub struct RequirePermission {
    permission: String,  // e.g., "device.router.restart"
}

impl<S> Middleware<S> for RequirePermission {
    fn call(&self, req: Request, next: Next<S>) -> Response {
        let user = req.extensions().get::<UserContext>();
        let permissions = resolve_permissions(user.id);

        if !permissions.contains(&self.permission) {
            // Check wildcard: "device.*.restart"
            if !matches_wildcard(&permissions, &self.permission) {
                return StatusCode::FORBIDDEN.into_response();
            }
        }

        next.run(req).await
    }
}
```

## 7. Usage in Handlers

```rust
// Route definition with permission guard
Router::new()
    .route("/devices/:id/restart", post(restart_device))
    .layer(RequirePermission::new("device.router.restart"))

// Or per-handler check
async fn restart_device(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Response, AppError> {
    require_permission(&user, "device.router.restart")?;
    // ... business logic
}
```

## 8. Checker/Maker Approval Workflow

For critical operations requiring multi-step approval:

```rust
pub struct ApprovalWorkflow {
    pub operation: String,
    pub required_approver_roles: Vec<String>,
    pub timeout_hours: i64,
}

// Approval flow:
// 1. User initiates action → create approval_request
// 2. Request sent to approvers via notification
// 3. Approvers review and approve/reject
// 4. On approval → execute the action
// 5. On timeout → auto-reject
```

**Operations requiring approval:**
- OLT firmware update → network_admin + isp_owner
- Bulk customer suspension → finance_manager + isp_owner
- Network-wide config change → network_admin + noc_engineer
- Refund > ₹5,000 → billing_operator + finance_manager
- Device removal → noc_engineer + network_admin

## 9. Seed Data

### Default Roles (migration)
```sql
INSERT INTO roles (name, slug, description, parent_role_id, is_system) VALUES
('Super Admin', 'super_admin', 'Platform-wide control', NULL, TRUE),
('ISP Owner', 'isp_owner', 'Business owner', 1, TRUE),
('Network Admin', 'network_admin', 'Network infrastructure', 2, TRUE),
('NOC Engineer', 'noc_engineer', 'Network monitoring', 3, TRUE),
('Field Technician', 'field_technician', 'Field operations', NULL, TRUE),
('Customer Support', 'customer_support', 'Support handling', NULL, TRUE),
('Sales Agent', 'sales_agent', 'Lead management', NULL, TRUE),
('Finance Manager', 'finance_manager', 'Financial oversight', 2, TRUE),
('Billing Operator', 'billing_operator', 'Billing operations', 8, TRUE),
('Customer', 'customer', 'End-user self-service', NULL, TRUE);
```

## 10. Temporary Permissions

```sql
-- Stored in user_roles with expires_at
INSERT INTO user_roles (user_id, role_id, granted_by, expires_at)
VALUES (10, 3, 2, '2026-07-15T18:00:00Z');

-- Cleanup job runs every hour to revoke expired temporary permissions
DELETE FROM user_roles WHERE expires_at < NOW() AND expires_at IS NOT NULL;
```

## 11. RBAC Permissions

```
rbac.role.view
rbac.role.create
rbac.role.update
rbac.role.delete
rbac.permission.view
rbac.permission.grant
rbac.permission.revoke
rbac.user.role.assign
rbac.user.role.revoke
rbac.temporary.grant
rbac.approval.view
rbac.approval.approve
rbac.approval.reject
```

## Known Issues & Gap Reference (v3.0)

> **Full details:** `GAP-finance-compliance.md`, `GAP-architecture-patterns.md`, `DESIGN-GAPS-DEEP-ANALYSIS.md` §11
