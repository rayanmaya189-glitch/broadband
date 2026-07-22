# AeroXe Admin Portal — Architecture Overview

> **Req Ref:** §16 Admin Portal Requirements  
> **API Convention:** Protobuf-first. See `docs/backend/API-CONVENTIONS.md`.

---

## 1. Technology Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Framework | **React 19** + **TypeScript** | Type safety, ecosystem |
| Build Tool | **Vite** | Fast dev, HMR, optimized builds |
| Styling | **Tailwind CSS 4** | Utility-first, consistent design |
| Component Library | **shadcn/ui** | Accessible, customizable components |
| State Management | **Zustand** | Lightweight, TypeScript-first |
| Data Fetching | **TanStack Query (React Query)** | Caching, mutations, optimistic updates |
| Forms | **React Hook Form** + **Zod** | Validation, performance |
| Charts | **Recharts** or **Tremor** | Dashboard visualizations |
| Tables | **TanStack Table** | Sorting, filtering, pagination |
| Auth | **JWT** stored in httpOnly cookie | Secure session management |
| Routing | **React Router v7** | File-based routing |
| WebSocket | Native WebSocket + React hooks | Real-time dashboard updates |

## 2. Project Structure

```
admin/
├── package.json
├── vite.config.ts
├── tailwind.config.ts
├── tsconfig.json
├── index.html
├── public/
│   └── favicon.ico
└── src/
    ├── main.tsx
    ├── App.tsx
    ├── index.css
    │
    ├── api/                        # API client layer
    │   ├── client.ts               # Axios instance with interceptors
    │   ├── auth.ts                 # Auth API calls
    │   ├── customers.ts            # Customer API
    │   ├── subscriptions.ts        # Subscription API
    │   ├── plans.ts                # Plan API
    │   ├── billing.ts              # Billing API
    │   ├── devices.ts              # Device API
    │   ├── network.ts              # Network API
    │   ├── tickets.ts              # Ticket API
    │   ├── leads.ts                # Lead API
    │   ├── notifications.ts        # Notification API
    │   ├── users.ts                # User API
    │   ├── audit.ts                # Audit API
    │   └── reports.ts              # Reports API
    │
    ├── hooks/                      # Custom React hooks
    │   ├── useAuth.ts              # Authentication state
    │   ├── useWebSocket.ts         # WebSocket connection
    │   ├── useDebounce.ts          # Input debouncing
    │   ├── usePagination.ts        # Table pagination
    │   └── useExport.ts            # CSV/PDF export
    │
    ├── store/                      # Zustand stores
    │   ├── authStore.ts            # Auth state & actions
    │   ├── uiStore.ts              # UI state (sidebar, theme)
    │   └── filterStore.ts          # Global filter state
    │
    ├── components/                 # Shared components
    │   ├── layout/
    │   │   ├── AdminLayout.tsx     # Main layout with sidebar
    │   │   ├── Sidebar.tsx         # Navigation sidebar
    │   │   ├── Header.tsx          # Top header bar
    │   │   ├── Breadcrumb.tsx      # Breadcrumb navigation
    │   │   └── BranchSelector.tsx  # Branch switcher
    │   ├── ui/                     # shadcn/ui components
    │   │   ├── DataTable.tsx       # Reusable data table
    │   │   ├── PageHeader.tsx      # Page title + actions
    │   │   ├── StatsCard.tsx       # KPI stat card
    │   │   ├── StatusBadge.tsx     # Status indicator
    │   │   ├── ConfirmDialog.tsx   # Confirmation modal
    │   │   ├── EmptyState.tsx      # Empty state placeholder
    │   │   ├── LoadingSpinner.tsx  # Loading indicator
    │   │   ├── ErrorAlert.tsx      # Error display
    │   │   └── DateRangePicker.tsx # Date range filter
    │   ├── charts/
    │   │   ├── LineChart.tsx       # Time series charts
    │   │   ├── BarChart.tsx        # Bar charts
    │   │   ├── PieChart.tsx        # Pie/donut charts
    │   │   └── AreaChart.tsx       # Area charts
    │   └── forms/
    │       ├── CustomerForm.tsx    # Customer create/edit
    │       ├── PlanForm.tsx        # Plan create/edit
    │       ├── InvoiceForm.tsx     # Invoice create/edit
    │       ├── TicketForm.tsx      # Ticket create/edit
    │       └── UserForm.tsx        # User create/edit
    │
    ├── pages/                      # Route pages
    │   ├── LoginPage.tsx
    │   ├── DashboardPage.tsx       # §01-dashboard.md
    │   ├── customers/              # §02-customers.md
    │   ├── subscriptions/          # §03-subscriptions.md
    │   ├── plans/                  # §04-plans.md
    │   ├── billing/                # §05-billing.md
    │   ├── accounting/             # §06-accounting.md
    │   ├── devices/                # §07-devices.md
    │   ├── discovery/              # §08-discovery.md
    │   ├── network/                # §09-network.md
    │   ├── bandwidth/              # §10-bandwidth.md
    │   ├── tickets/                # §11-tickets.md
    │   ├── leads/                  # §12-leads.md
    │   ├── notifications/          # §13-notifications.md
    │   ├── users/                  # §14-users.md
    │   ├── audit/                  # §15-audit.md
    │   ├── settings/               # §16-settings.md
    │   └── reports/                # §17-reports.md
    │
    ├── lib/
    │   ├── utils.ts                # General utilities
    │   ├── formatters.ts           # Currency, date, phone formatters
    │   ├── validators.ts           # Zod schemas
    │   └── constants.ts            # Status enums, colors, labels
    │
    └── types/
        ├── api.ts                  # API response types
        ├── models.ts               # Domain model types
        └── forms.ts                # Form input types
```

## 3. Routing Structure

```tsx
<Routes>
  <Route path="/login" element={<LoginPage />} />
  <Route path="/" element={<AdminLayout />}>
    <Route index element={<DashboardPage />} />

    {/* Customers */}
    <Route path="customers" element={<CustomerListPage />} />
    <Route path="customers/:id" element={<CustomerDetailPage />} />

    {/* Subscriptions */}
    <Route path="subscriptions" element={<SubscriptionListPage />} />
    <Route path="subscriptions/:id" element={<SubscriptionDetailPage />} />

    {/* Plans */}
    <Route path="plans" element={<PlanListPage />} />
    <Route path="plans/:id" element={<PlanDetailPage />} />
    <Route path="plans/:id/pricing" element={<PlanPricingPage />} />

    {/* Billing */}
    <Route path="billing/invoices" element={<InvoiceListPage />} />
    <Route path="billing/invoices/:id" element={<InvoiceDetailPage />} />
    <Route path="billing/payments" element={<PaymentListPage />} />
    <Route path="billing/refunds" element={<RefundListPage />} />
    <Route path="billing/discounts" element={<DiscountListPage />} />

    {/* Accounting */}
    <Route path="accounting/journal" element={<JournalListPage />} />
    <Route path="accounting/trial-balance" element={<TrialBalancePage />} />
    <Route path="accounting/statements" element={<FinancialStatementsPage />} />
    <Route path="accounting/gst" element={<GSTReturnsPage />} />

    {/* Devices */}
    <Route path="devices" element={<DeviceListPage />} />
    <Route path="devices/:id" element={<DeviceDetailPage />} />
    <Route path="devices/models" element={<DeviceModelListPage />} />

    {/* Discovery */}
    <Route path="discovery" element={<DiscoveryDashboardPage />} />
    <Route path="discovery/scans" element={<ScanConfigListPage />} />
    <Route path="discovery/results" element={<DiscoveryResultsPage />} />

    {/* Network */}
    <Route path="network/vlans" element={<VLANListPage />} />
    <Route path="network/ip-pools" element={<IPPoolListPage />} />
    <Route path="network/pppoe" element={<PPPoESessionListPage />} />
    <Route path="network/dhcp" element={<DHCPLeaseListPage />} />
    <Route path="network/mac-bindings" element={<MACBindingListPage />} />
    <Route path="network/topology" element={<NetworkTopologyPage />} />

    {/* Bandwidth */}
    <Route path="bandwidth/profiles" element={<BandwidthProfileListPage />} />
    <Route path="bandwidth/applications" element={<BandwidthApplicationListPage />} />

    {/* Tickets */}
    <Route path="tickets" element={<TicketListPage />} />
    <Route path="tickets/:id" element={<TicketDetailPage />} />

    {/* Leads */}
    <Route path="leads" element={<LeadListPage />} />
    <Route path="leads/:id" element={<LeadDetailPage />} />
    <Route path="leads/pipeline" element={<LeadPipelinePage />} />

    {/* Notifications */}
    <Route path="notifications" element={<NotificationListPage />} />
    <Route path="notifications/templates" element={<TemplateListPage />} />
    <Route path="notifications/channels" element={<ChannelConfigPage />} />

    {/* Users */}
    <Route path="users" element={<UserListPage />} />
    <Route path="users/:id" element={<UserDetailPage />} />
    <Route path="roles" element={<RoleListPage />} />
    <Route path="permissions" element={<PermissionMatrixPage />} />

    {/* Audit */}
    <Route path="audit" element={<AuditLogPage />} />

    {/* Settings */}
    <Route path="settings" element={<SettingsPage />} />
    <Route path="settings/billing" element={<BillingSettingsPage />} />
    <Route path="settings/notifications" element={<NotificationSettingsPage />} />

    {/* Reports */}
    <Route path="reports" element={<ReportsDashboardPage />} />
    <Route path="reports/revenue" element={<RevenueReportPage />} />
    <Route path="reports/customers" element={<CustomerReportPage />} />
    <Route path="reports/network" element={<NetworkReportPage />} />
  </Route>
</Routes>
```

## 4. Layout Design

```
┌─────────────────────────────────────────────────────┐
│  Header: Logo  |  Branch Selector  |  User Menu    │
├──────────┬──────────────────────────────────────────┤
│          │  Breadcrumb: Customers > Customer #123   │
│ Sidebar  │──────────────────────────────────────────│
│          │                                          │
│ Dashboard│         Page Content Area                │
│ Customers│                                          │
│ Subscript│                                          │
│ Plans    │                                          │
│ Billing  │                                          │
│ Accounti │                                          │
│ Devices  │                                          │
│ Network  │                                          │
│ Tickets  │                                          │
│ Leads    │                                          │
│ Notific. │                                          │
│ Users    │                                          │
│ Audit    │                                          │
│ Settings │                                          │
│ Reports  │                                          │
│          │                                          │
├──────────┴──────────────────────────────────────────┤
│  Footer: AeroXe Admin v1.0 | Environment: Production│
└─────────────────────────────────────────────────────┘
```

### Sidebar Navigation

```tsx
const navItems = [
  { label: 'Dashboard', icon: 'LayoutDashboard', href: '/' },
  { label: 'Customers', icon: 'Users', href: '/customers', badge: activeCount },
  { label: 'Subscriptions', icon: 'RefreshCw', href: '/subscriptions' },
  { label: 'Plans', icon: 'Package', href: '/plans' },
  { divider: true },
  { label: 'Billing', icon: 'CreditCard', href: '/billing/invoices' },
  { label: 'Accounting', icon: 'BookOpen', href: '/accounting/journal' },
  { divider: true },
  { label: 'Devices', icon: 'Server', href: '/devices' },
  { label: 'Discovery', icon: 'Radar', href: '/discovery' },
  { label: 'Network', icon: 'Network', href: '/network/vlans' },
  { label: 'Bandwidth', icon: 'Gauge', href: '/bandwidth/profiles' },
  { divider: true },
  { label: 'Tickets', icon: 'Ticket', href: '/tickets', badge: openCount },
  { label: 'Leads', icon: 'Target', href: '/leads' },
  { divider: true },
  { label: 'Notifications', icon: 'Bell', href: '/notifications' },
  { label: 'Users', icon: 'UserCog', href: '/users' },
  { label: 'Audit Logs', icon: 'FileText', href: '/audit' },
  { label: 'Settings', icon: 'Settings', href: '/settings' },
  { label: 'Reports', icon: 'BarChart3', href: '/reports' },
];
```

## 5. Authentication Flow

```
1. User navigates to admin.aeroxebroadband.com
2. Redirect to /login if no valid JWT
3. Login: email + password → JWT access token + refresh token
4. If 2FA enabled → redirect to /login/2fa
5. Store JWT in httpOnly cookie (not localStorage)
6. Axios interceptor attaches JWT to all requests
7. On 401 → attempt token refresh → if fails → redirect to login
8. On 403 → show "Access Denied" page
```

## 6. Role-Based UI Rendering

```tsx
// Sidebar items filtered by user permissions
const filteredNavItems = navItems.filter(item => {
  if (!item.permission) return true;
  return user.permissions.includes(item.permission);
});

// Action buttons conditionally rendered
{hasPermission('customer.account.create') && (
  <Button onClick={openCreateModal}>Add Customer</Button>
)}

// Route-level permission guard
<ProtectedRoute requiredPermission="billing.invoice.view">
  <InvoiceListPage />
</ProtectedRoute>
```

## 7. Branch Selector

```tsx
// Company-wide users see all branches
// Branch-scoped users see only their assigned branches
// Selecting a branch filters all data views

function BranchSelector() {
  const { user, branches } = useAuth();
  const [selectedBranch, setSelectedBranch] = useFilterStore(s => s.branch);

  if (user.is_company_wide) {
    return (
      <Select value={selectedBranch} onChange={setSelectedBranch}>
        <option value="all">All Branches</option>
        {branches.map(b => <option key={b.id} value={b.id}>{b.name}</option>)}
      </Select>
    );
  }

  return <span>{user.branchName}</span>;
}
```

## 8. Real-Time Updates

```tsx
// WebSocket connection for live dashboard data
function useRealtime(channel: string) {
  const { token } = useAuth();

  useEffect(() => {
    const ws = new WebSocket(`wss://api.aeroxe.com/ws?token=${token}`);

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      handleRealtimeEvent(data);
    };

    // Subscribe to channel
    ws.send(JSON.stringify({ action: 'subscribe', channel }));

    return () => ws.close();
  }, [channel, token]);
}
```

## 9. Common Patterns

### Data Table Pattern
```tsx
<DataTable
  columns={columns}
  data={data}
  isLoading={isLoading}
  pagination={pagination}
  onSort={handleSort}
  onFilter={handleFilter}
  onExport={handleExport}
  bulkActions={[
    { label: 'Export Selected', onClick: exportSelected },
    { label: 'Delete Selected', onClick: deleteSelected, requiresPermission: '...' },
  ]}
/>
```

### Form Pattern
```tsx
const form = useForm<FormData>({
  resolver: zodResolver(schema),
  defaultValues: { ... },
});

<Form onSubmit={form.handleSubmit(onSubmit)}>
  <FormField name="name" label="Name" control={form.control} />
  <FormField name="email" label="Email" control={form.control} type="email" />
  <FormActions onCancel={onCancel} isSubmitting={form.formState.isSubmitting} />
</Form>
```

### Detail Page Pattern
```tsx
function DetailPage({ id }) {
  const { data, isLoading } = useQuery(['entity', id], () => fetchEntity(id));

  return (
    <PageHeader title={data.name} actions={[...]} />
    <Tabs defaultValue="overview">
      <TabsList>
        <TabsTrigger value="overview">Overview</TabsTrigger>
        <TabsTrigger value="history">History</TabsTrigger>
        <TabsTrigger value="activity">Activity</TabsTrigger>
      </TabsList>
      <TabsContent value="overview">...</TabsContent>
      <TabsContent value="history">...</TabsContent>
    </Tabs>
  );
}
```

## 10. Performance Optimizations

- **Lazy loading** all route pages via `React.lazy()`
- **Query caching** with TanStack Query (staleTime: 5 min)
- **Optimistic updates** for mutations (create/update/delete)
- **Virtual scrolling** for large tables (TanStack Table + @tanstack/react-virtual)
- **Debounced search** (300ms delay)
- **Pagination** with 25/50/100 items per page
- **Code splitting** by route
- **Image optimization** with lazy loading
