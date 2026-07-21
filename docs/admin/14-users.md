# AeroXe Admin Portal — Users Module

> **Req Ref:** §2 User Roles and RBAC, §16 Admin Portal

---

## 1. Overview

User and role management — create/edit users, assign roles, manage permissions, view permission matrix, and handle branch assignments.

## 2. Pages

### User List (`/users`)

```
┌──────────────────────────────────────────────────────────┐
│  System Users                             [+ Add User]   │
├──────────────────────────────────────────────────────────┤
│  Search: [____] Role: [All ▼] Status: [All ▼] Branch: [All ▼] │
├──────────────────────────────────────────────────────────┤
│  Name          │ Email             │ Role         │ Branch │ Status │
│  Admin User    │ admin@aeroxe.com  │ super_admin  │ All    │ ● Active│
│  Rahul Network │ network@aeroxe.com│ network_admin│ JLG    │ ● Active│
│  Priya Support │ support@aeroxe.com│ customer_sup │ JLG    │ ● Active│
│  Amit Finance  │ finance@aeroxe.com│ finance_mgr  │ All    │ ● Active│
└──────────────────────────────────────────────────────────┘
```

### User Detail (`/users/:id`)

```
User: Rahul Network (network@aeroxe.com)
Role: Network Admin  │  Branch: Jalgaon Main  │  Status: ● Active
Last Login: Jul 8, 2026 10:30 AM

[Overview] [Roles] [Permissions] [Sessions] [Activity]

Active Sessions:
├── Chrome on Windows 10 — 10.0.1.50 — 2h ago
└── Mobile App — 10.0.1.52 — 5h ago

Recent Activity:
├── 10:30 — Restarted device OLT-01
├── 10:15 — Updated VLAN 200
└── 09:45 — Viewed device list
```

### Role Management (`/roles`)

```
┌──────────────────────────────────────────────────────────┐
│  Roles                                     [+ Add Role]  │
├──────────────────────────────────────────────────────────┤
│  Role            │ Users │ Permissions │ Parent    │ Type │
│  super_admin     │ 1     │ 150+        │ —         │ System│
│  isp_owner       │ 2     │ 120+        │ super_adm │ System│
│  network_admin   │ 3     │ 80+         │ noc_eng   │ System│
│  noc_engineer    │ 5     │ 30+         │ —         │ System│
│  customer_support│ 8     │ 20+         │ —         │ System│
│  sales_agent     │ 4     │ 15+         │ —         │ System│
│  finance_manager │ 2     │ 40+         │ billing_op│ System│
│  billing_operator│ 3     │ 15+         │ —         │ System│
│  field_tech      │ 6     │ 12+         │ —         │ System│
│  customer        │ 847   │ 4           │ —         │ System│
│  custom_role     │ 0     │ 10          │ —         │ Custom│
└──────────────────────────────────────────────────────────┘

Role Hierarchy (visual):
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

### Permission Matrix (`/permissions`)

```
┌──────────────────────────────────────────────────────────┐
│  Permission Matrix                    [Filter: ________] │
├──────────────────────────────────────────────────────────┤
│  Permission              │ Super │ ISP  │ Net  │ NOC  │ Cust│
│                          │ Admin │ Owner│ Admin│ Eng  │ Sup │
│  ────────────────────────┼───────┼──────┼──────┼──────┼─────│
│  customer.account.view   │  ✅   │  ✅  │  ✅  │  ✅  │ ✅  │
│  customer.account.create │  ✅   │  ✅  │  ❌  │  ❌  │ ❌  │
│  device.router.view      │  ✅   │  ✅  │  ✅  │  ✅  │ ❌  │
│  device.router.restart   │  ✅   │  ✅  │  ✅  │  ✅  │ ❌  │
│  device.router.configure │  ✅   │  ✅  │  ✅  │  ❌  │ ❌  │
│  billing.invoice.view    │  ✅   │  ✅  │  ❌  │  ❌  │ ❌  │
│  billing.invoice.refund  │  ✅   │  ✅  │  ❌  │  ❌  │ ❌  │
└──────────────────────────────────────────────────────────┘
```

## 3. API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/users/list` | POST | List users |
| `/api/v1/users/create` | POST | Create user |
| `/api/v1/users/get` | POST | Get user details |
| `/api/v1/users/update` | PATCH | Update user |
| `/api/v1/users/status/update` | PATCH | Change user status |
| `/api/v1/users/avatar/upload` | POST | Upload avatar |
| `/api/v1/rbac/roles/list` | POST | List roles |
| `/api/v1/rbac/roles/create` | POST | Create role |
| `/api/v1/rbac/roles/update` | PATCH | Update role |
| `/api/v1/rbac/roles/delete` | DELETE | Delete role |
| `/api/v1/rbac/roles/permissions/update` | POST | Manage permissions |
| `/api/v1/rbac/roles/permissions/delete` | DELETE | Remove permissions |
| `/api/v1/rbac/permissions/list` | POST | List all permissions |
| `/api/v1/rbac/users/roles/update` | POST | Assign roles |
| `/api/v1/rbac/users/roles/delete` | DELETE | Revoke roles |
| `/api/v1/rbac/temporary/create` | POST | Grant temporary permission |

## 4. RBAC

| Action | Required Permission |
|--------|-------------------|
| View users | `user.account.view` |
| Create user | `user.account.create` |
| Delete user | `user.account.delete` |
| Assign role | `user.role.assign` |
| Create role | `user.role.create` |
| Delete role | `user.role.delete` |
| Grant permission | `user.permission.grant` |
