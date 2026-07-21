# AeroXe Backend — Users Module

> **Req Ref:** §2.12 RBAC Database Tables, §6-users

---

## 1. Overview

Manages system users (staff and customer-facing users), their profiles, and account lifecycle. Users are distinct from customers — users are the people operating the platform.

## 2. Database Tables

(Uses `users` table from §3-auth.md)

## 3. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/users/list` | isp_owner+ | List all users |
| POST | `/api/v1/users/create` | isp_owner+ | Create user account |
| POST | `/api/v1/users/get` | isp_owner+ | Get user details |
| PATCH | `/api/v1/users/update` | isp_owner+ | Update user |
| DELETE | `/api/v1/users/delete` | super_admin | Soft-delete user |
| PATCH | `/api/v1/users/status/update` | isp_owner+ | Activate/suspend/lock |
| POST | `/api/v1/users/me/get` | Yes | Get current user profile |
| PATCH | `/api/v1/users/me/update` | Yes | Update own profile |
| POST | `/api/v1/users/avatar` | isp_owner+ | Upload avatar |

## 4. DTOs

```rust
pub struct CreateUserRequest {
    pub email: String,
    pub phone: String,
    pub name: String,
    pub password: String,
    pub branch_id: Option<i64>,
    pub role_ids: Vec<i64>,
}

pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub phone: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub branch_id: Option<i64>,
    pub status: String,
    pub roles: Vec<RoleResponse>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
```

## 5. Business Rules

- Users must have at least one role assigned
- A user can be assigned to multiple branches
- One branch can be marked as primary
- Company-wide roles (isp_owner, finance_manager, super_admin) bypass branch filtering
- Account lockout after 5 failed login attempts (30 min lock)
- Soft-delete with 30-day recovery window

## 6. RBAC Permissions

```
user.account.view
user.account.create
user.account.update
user.account.delete
user.account.disable
user.account.enable
user.role.assign
user.role.revoke
```

## Known Issues & Gap Reference (v3.0)

> **Full details:** `GAP-finance-compliance.md`, `GAP-architecture-patterns.md`, `DESIGN-GAPS-DEEP-ANALYSIS.md` §11
