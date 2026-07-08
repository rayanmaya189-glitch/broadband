# AeroXe Backend — Branch Management Module

> **Req Ref:** §2.11 Branch Management

---

## 1. Overview

AeroXe operates as a single-tenant platform with multiple geographic branches. Branch scoping is enforced at the middleware level — all data queries are filtered by the user's assigned branch(es) unless the user has company-wide access.

## 2. Database Tables

```sql
CREATE TABLE branches (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    code VARCHAR(20) NOT NULL UNIQUE,
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL DEFAULT 'Maharashtra',
    address TEXT,
    phone VARCHAR(20),
    email VARCHAR(255),
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    timezone VARCHAR(50) DEFAULT 'Asia/Kolkata',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE branch_working_hours (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    day_of_week INTEGER NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
    open_time TIME NOT NULL,
    close_time TIME NOT NULL,
    is_closed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(branch_id, day_of_week)
);

CREATE TABLE user_branches (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    branch_id BIGINT NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, branch_id)
);
```

## 3. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/branches` | isp_owner+ | List all branches |
| POST | `/api/v1/branches` | isp_owner+ | Create branch |
| GET | `/api/v1/branches/:id` | branch-scoped | Get branch details |
| PUT | `/api/v1/branches/:id` | isp_owner+ | Update branch |
| DELETE | `/api/v1/branches/:id` | super_admin | Deactivate branch |
| GET | `/api/v1/branches/:id/working-hours` | branch-scoped | Get working hours |
| PUT | `/api/v1/branches/:id/working-hours` | isp_owner+ | Update working hours |
| GET | `/api/v1/branches/:id/stats` | isp_owner+ | Branch statistics |
| POST | `/api/v1/branches/:id/users` | isp_owner+ | Assign user to branch |
| DELETE | `/api/v1/branches/:id/users/:uid` | isp_owner+ | Remove user from branch |

## 4. Branch Scoping Middleware

```rust
pub struct BranchScope;

impl<S> Middleware<S> for BranchScope {
    fn call(&self, req: Request, next: Next<S>) -> Response {
        let user = req.extensions().get::<UserContext>();

        // Company-wide roles bypass branch filtering
        if user.is_company_wide {
            return next.run(req).await;
        }

        // Set branch_id in request context for query filtering
        req.extensions().insert(BranchFilter {
            branch_ids: user.branch_ids.clone(),
            is_company_wide: false,
        });

        next.run(req).await
    }
}
```

## 5. Branch Scoping Rules

| Resource | Branch-Scoped | Company-Wide |
|----------|--------------|--------------|
| Customers | ✅ | ❌ |
| Subscriptions | ✅ | ❌ |
| Network devices | ✅ | ❌ |
| IP pools, VLANs | ✅ | ❌ |
| Tickets | ✅ | ❌ |
| Plans & speed profiles | ❌ | ✅ |
| Users & roles | ❌ | ✅ |
| Billing (invoices) | ✅ | Read-only |
| Reports | ✅ | ✅ (aggregated) |

## 6. Seed Data

```sql
INSERT INTO branches (name, slug, code, city, state) VALUES
('Jalgaon Main', 'jalgaon-main', 'JLG', 'Jalgaon', 'Maharashtra'),
('Bhusawal', 'bhusawal', 'BHL', 'Bhusawal', 'Maharashtra'),
('Mumbai', 'mumbai', 'MUM', 'Mumbai', 'Maharashtra'),
('Navi Mumbai', 'navi-mumbai', 'NNM', 'Navi Mumbai', 'Maharashtra');
```

## 7. RBAC Permissions

```
branch.view
branch.create
branch.update
branch.manage_working_hours
branch.view_reports
branch.manage_staff
```
