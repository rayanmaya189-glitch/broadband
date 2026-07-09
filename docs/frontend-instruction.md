Got it — you want a **production-grade frontend architecture** that matches your backend discipline (DDD-style modular monolith), not just a typical React folder mess.

Below is a **clean, scalable, enterprise-level structure** for:

> **AeroXe Broadband Admin Portal (React 19 + RR v7 + Tailwind v4 + Vite 8 + React Query + Zustand + Framer Motion)**

---

# 🚀 Project Name

```bash
aeroxe-broadband-admin
```

---

# 📦 Tech Stack (Aligned with your backend)

* React 19
* React Router DOM v7 (data APIs)
* Tailwind CSS v4
* Vite 8
* React Query (server state)
* Zustand (client state)
* Framer Motion (animations)
* Axios (HTTP client)
* Zod (validation)
* Session Storage (auth persistence)

---

# 🧠 Architecture Philosophy

Same as your backend:

* **Module-first (NOT component-first)**
* **DDD-inspired frontend**
* **Feature isolation**
* **Shared core layer**
* **API layer per module**
* **State separated (server vs client)**

---

# 📁 Final Folder Structure (STRICT)

```bash
frontend/

├── index.html
├── package.json
├── vite.config.ts
├── tsconfig.json
├── .env

├── public/

└── src/

    ├── main.tsx
    ├── app.tsx

    ├── router/
    │   ├── router.tsx
    │   ├── protected_route.tsx
    │   └── route_types.ts

    ├── config/
    │   ├── env.ts
    │   ├── axios.ts
    │   └── query_client.ts

    ├── core/                      # Global shared layer
    │
    ├── ui/
    │   │
    │   ├── components/           # reusable UI (dumb)
    │   │   ├── button/
    │   │   ├── input/
    │   │   ├── modal/
    │   │   ├── table/
    │   │   └── form/
    │   │
    │   ├── layout/
    │   │   ├── sidebar/
    │   │   ├── header/
    │   │   └── dashboard_layout.tsx
    │   │
    │   ├── theme/
    │   │   └── tailwind.css
    │   │
    │   └── motion/
    │       └── animations.ts
    │
    │
    ├── modules/                  # 🔥 DOMAIN DRIVEN FRONTEND
    │
    │── auth/
    │   │
    │   ├── api/
    │   │   └── auth_api.ts
    │   │
    │   ├── hooks/
    │   │   ├── use_login.ts
    │   │   └── use_me.ts
    │   │
    │   ├── store/
    │   │   └── auth_store.ts
    │   │
    │   ├── types/
    │   │   └── auth_types.ts
    │   │
    │   ├── pages/
    │   │   └── login_page.tsx
    │   │
    │   └── components/
    │       └── login_form.tsx
    │
    │
    │── customer/
    │   │
    │   ├── api/
    │   │   └── customer_api.ts
    │   │
    │   ├── hooks/
    │   │   ├── use_customers.ts
    │   │   ├── use_create_customer.ts
    │   │   └── use_customer.ts
    │   │
    │   ├── types/
    │   │   └── customer_types.ts
    │   │
    │   ├── pages/
    │   │   ├── customer_list_page.tsx
    │   │   └── customer_detail_page.tsx
    │   │
    │   ├── components/
    │   │   ├── customer_table.tsx
    │   │   └── customer_form.tsx
    │   │
    │   └── store/
    │       └── customer_store.ts
    │
    │
    │── subscription/
    │── billing/
    │── payment/
    │── network/
    │── device/
    │── bandwidth/
    │── ip_management/
    │── monitoring/
    │── ticket/
    │── notification/
    │── audit/
    │
    │
    ├── services/                # Cross-module services
    │   ├── websocket.ts
    │   ├── event_bus.ts
    │   └── permissions.ts
    │
    │
    ├── store/                   # Global Zustand stores
    │   ├── app_store.ts
    │   └── ui_store.ts
    │
    │
    ├── hooks/                   # Shared hooks
    │   ├── use_debounce.ts
    │   └── use_pagination.ts
    │
    │
    ├── lib/                     # Utilities
    │   ├── date.ts
    │   ├── format.ts
    │   └── validator.ts
    │
    │
    ├── constants/
    │   ├── routes.ts
    │   └── roles.ts
    │
    │
    └── assets/
```

---

# 🔐 Auth + Session Storage Strategy (IMPORTANT)

### Zustand Store

```ts
auth_store.ts

persist(
  (set) => ({
    token: null,
    user: null,
    login: (data) => set(data),
    logout: () => set({ token: null, user: null }),
  }),
  {
    name: "aeroxe-auth",
    storage: createJSONStorage(() => sessionStorage),
  }
)
```

---

# 🌐 Axios Setup

```ts
axios.ts

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
})

api.interceptors.request.use((config) => {
  const token = authStore.getState().token
  if (token) config.headers.Authorization = `Bearer ${token}`
  return config
})
```

---

# ⚡ React Query Strategy

* Queries → fetch data
* Mutations → create/update/delete
* Cache per module

Example:

```ts
useQuery({
  queryKey: ["customers"],
  queryFn: getCustomers,
})
```

---

# 🧭 Router Design (RR v7 Data Router)

```ts
router.tsx

createBrowserRouter([
  {
    path: "/login",
    element: <LoginPage />,
  },
  {
    element: <ProtectedRoute />,
    children: [
      {
        path: "/dashboard",
        element: <DashboardLayout />,
        children: [
          { path: "customers", element: <CustomerListPage /> },
          { path: "customers/:id", element: <CustomerDetailPage /> },
        ],
      },
    ],
  },
])
```

---

# 🎨 UI + Tailwind v4

* Use **design tokens**
* No inline chaos
* Create reusable components:

```bash
ui/components/button/
ui/components/table/
```

---

# 🎬 Framer Motion Integration

```ts
motion/animations.ts

export const fadeIn = {
  hidden: { opacity: 0 },
  show: { opacity: 1 },
}
```

Use per page:

```tsx
<motion.div initial="hidden" animate="show" variants={fadeIn}>
```

---

# 🔌 WebSocket (Real-time ISP Monitoring)

```ts
services/websocket.ts

- device online/offline
- bandwidth updates
- alerts
```

---

# 🧩 Module Rules (CRITICAL)

Each module MUST:

✔ Own its API
✔ Own its types
✔ Own its hooks
✔ Own its UI
✔ NOT depend on other modules directly

Cross-communication → via:

```ts
services/event_bus.ts
```

---

# 🔥 Naming Convention

| Layer     | Example            |
| --------- | ------------------ |
| Hook      | `useCustomers`     |
| API       | `getCustomers`     |
| Store     | `customerStore`    |
| Page      | `CustomerListPage` |
| Component | `CustomerTable`    |

---

# 🧠 Scaling Strategy (Same as Backend)

Today:

```bash
ONE React App
```

Later:

```bash
micro-frontends (optional)

customer-admin
network-admin
billing-admin
```

---

# 🧪 Production Add-ons (Recommended)

* RBAC UI guards
* Feature flags
* Audit viewer UI
* Realtime dashboard (WebSocket)
* Error boundary system
* Toast notification system

---

# ⚡ Final Verdict

This structure gives you:

✅ Backend-aligned architecture
✅ Zero spaghetti scaling
✅ Clean domain isolation
✅ Ready for micro-frontend future
✅ High performance with React Query
✅ Stable auth via session storage
✅ Perfect for ISP complexity (network + billing + devices)

---
