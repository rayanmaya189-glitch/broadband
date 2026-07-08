# AeroXe Admin Portal — Customers Module

> **Req Ref:** §3 Customer Management Module, §16 Admin Portal

---

## 1. Overview

Full customer lifecycle management UI — list, search, create, view, edit, KYC verification, and status transitions. Branch-scoped for branch users, company-wide for admins.

## 2. Pages

### 2.1 Customer List Page (`/customers`)

```
┌──────────────────────────────────────────────────────────┐
│  Customers                                    [+ Add] [Export] │
├──────────────────────────────────────────────────────────┤
│  Search: [____________] Status: [All ▼] Branch: [All ▼]  │
│  Sort: [Newest ▼]  Filter: [More filters]                │
├──────────────────────────────────────────────────────────┤
│  ☐  │ Code      │ Name         │ Phone      │ Status    │ Plan    │ Created  │
│  ☐  │ AX-JLG..  │ Rahul Sharma │ +91987...  │ ● Active  │ Std 100 │ Jul 1    │
│  ☐  │ AX-JLG..  │ Priya Patil  │ +91986...  │ ● KYC Pending │ —    │ Jul 2    │
│  ☐  │ AX-JLG..  │ Amit Deshmukh│ +91985...  │ ● Active  │ Pro 200 │ Jul 3    │
├──────────────────────────────────────────────────────────┤
│  Showing 1-25 of 847  [< 1 2 3 ... 34 >]  Per page: [25▼] │
└──────────────────────────────────────────────────────────┘
```

**Filters:**
- Status (registered, kyc_pending, kyc_verified, installation_scheduled, active, suspended, terminated)
- Branch
- Plan
- Date range (created_at)
- Search (name, phone, email, customer_code)

**Bulk Actions:**
- Export to CSV
- Send bulk notification
- Change status (selected customers)

### 2.2 Customer Detail Page (`/customers/:id`)

```
┌──────────────────────────────────────────────────────────┐
│  ← Back  │  Customer: Rahul Sharma (AX-JLG-202607-0001) │
│           │  Status: ● Active  │  Branch: Jalgaon Main   │
├───────────┴──────────────────────────────────────────────┤
│  [Overview] [Subscription] [Billing] [KYC] [Tickets]     │
│  [Installation] [Activity] [History]                     │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Personal Info          │  Subscription                  │
│  ─────────────          │  ──────────────                │
│  Name: Rahul Sharma     │  Plan: Standard 100 Mbps       │
│  Phone: +919876543210   │  Status: Active                │
│  Email: rahul@example   │  Billing: Monthly              │
│  DOB: 1994-05-15        │  Next Bill: Aug 1, 2026        │
│  Occupation: Software    │  Start Date: Jul 1, 2026       │
│                          │                                │
│  Address                │  Network                       │
│  ───────                │  ───────                       │
│  42, Shivaji Nagar      │  PPPoE: rahul@aeroxe           │
│  City Center, Jalgaon   │  IP: 10.10.1.100               │
│  425001                 │  VLAN: 200                     │
│                          │  MAC: AA:BB:CC:DD:EE:FF        │
│                          │  Status: ● Online (2h 15m)     │
│                          │                                │
│  Referral Code: RAHUL24 │  Bandwidth Usage (This Month)  │
│  Referred by: —          │  ↓ 45.2 GB  ↑ 12.8 GB         │
│                          │  [████████░░░░░░] 58%           │
└──────────────────────────────────────────────────────────┘
```

### 2.3 Customer Create/Edit Modal

```typescript
interface CustomerFormData {
  name: string;           // Required, 2-255 chars
  email?: string;         // Optional, valid email
  phone: string;          // Required, +91XXXXXXXXXX
  alternate_phone?: string;
  branch_id: number;      // Required, from branch selector
  // Profile fields
  gender?: 'male' | 'female' | 'other';
  date_of_birth?: string;
  occupation?: string;
  // Address
  address_line1: string;
  address_line2?: string;
  area?: string;
  city: string;
  state: string;
  pincode: string;
  landmark?: string;
  latitude?: number;
  longitude?: number;
}
```

## 3. Customer Status Badges

| Status | Badge Color | Icon |
|--------|------------|------|
| registered | Gray | — |
| kyc_pending | Yellow | ⏳ |
| kyc_verified | Blue | ✓ |
| installation_scheduled | Purple | 📅 |
| installation_in_progress | Orange | 🔧 |
| active | Green | ● |
| suspended | Red | ⏸ |
| terminated | Gray (strikethrough) | ✕ |

## 4. Status Transition Actions

From the detail page, action buttons change based on current status:

| Current Status | Available Actions |
|---------------|-------------------|
| registered | Submit KYC, Edit, Delete |
| kyc_pending | Verify KYC, Reject KYC |
| kyc_verified | Schedule Installation |
| installation_scheduled | Start Installation, Reschedule, Cancel |
| installation_in_progress | Complete Installation |
| active | Suspend, Edit, Create Ticket |
| suspended | Reactivate, Terminate |
| terminated | — (read-only) |

## 5. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/customers` | GET | List customers (paginated, filtered) |
| `/api/v1/customers` | POST | Create customer |
| `/api/v1/customers/:id` | GET | Get customer details |
| `/api/v1/customers/:id` | PUT | Update customer |
| `/api/v1/customers/:id/status` | PUT | Change status |
| `/api/v1/customers/:id/profile` | GET/PUT | Get/update profile |
| `/api/v1/customers/:id/kyc/submit` | POST | Submit KYC documents |
| `/api/v1/customers/:id/kyc/verify` | POST | Verify KYC |
| `/api/v1/customers/:id/kyc/reject` | POST | Reject KYC |
| `/api/v1/customers/:id/addresses` | GET/POST | List/add addresses |
| `/api/v1/customers/:id/history` | GET | Change history |
| `/api/v1/customers/:id/subscriptions` | GET | List subscriptions |
| `/api/v1/customers/:id/tickets` | GET | List tickets |
| `/api/v1/customers/export` | GET | Export to CSV |

## 6. KYC Verification Flow

```
1. Customer submits KYC documents (Aadhaar, PAN, address proof)
2. Documents uploaded to MinIO via presigned URLs
3. Customer status → kyc_pending
4. Staff reviews documents on customer detail page
5. Staff clicks "Verify KYC" or "Reject KYC"
6. If verified → status → kyc_verified, customer notified
7. If rejected → status stays kyc_pending, rejection reason shown
```

## 7. Customer Code Display

Format: `AX-{BRANCH}-{YYYYMM}-{SEQ}` (e.g., `AX-JLG-202607-0001`)
- Clickable → copies to clipboard
- Used as primary identifier across all views

## 8. RBAC

| Action | Required Permission |
|--------|-------------------|
| View customer list | `customer.account.view` |
| Create customer | `customer.account.create` |
| Edit customer | `customer.account.update` |
| Delete customer | `customer.account.delete` |
| Suspend customer | `customer.account.suspend` |
| Verify KYC | `customer.profile.verify_kyc` |
