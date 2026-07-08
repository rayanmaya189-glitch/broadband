# AeroXe Backend — Users Module

> **Req Ref:** §2.12 RBAC Database Tables, §6-users

---

## 1. Overview

Manages system users (staff and customer-facing users), their profiles, and account lifecycle. Users are distinct from customers — users are the people operating the platform.

## 2. Database Tables

(Uses `users` table from §3-auth.md)

## 3. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/users` | isp_owner+ | List all users |
| POST | `/api/v1/users` | isp_owner+ | Create user account |
| GET | `/api/v1/users/:id` | isp_owner+ | Get user details |
| PUT | `/api/v1/users/:id` | isp_owner+ | Update user |
| DELETE | `/api/v1/users/:id` | super_admin | Soft-delete user |
| PUT | `/api/v1/users/:id/status` | isp_owner+ | Activate/suspend/lock |
| GET | `/api/v1/users/me` | Yes | Get current user profile |
| PUT | `/api/v1/users/me` | Yes | Update own profile |
| POST | `/api/v1/users/:id/avatar` | isp_owner+ | Upload avatar |

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
