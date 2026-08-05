import type {
  AccountingAccount,
  ApprovalRequest,
  AuditLog,
  Branch,
  CoverageArea,
  DeviceMetricRow,
  Installation,
  Invoice,
  JournalEntry,
  Lead,
  MonitoringAlert,
  NetworkDevice,
  Notification,
  Paginated,
  Payment,
  Permission,
  Plan,
  Role,
  Subscription,
  Ticket,
  UserAccount,
  IpPool,
  Vlan,
} from '../../types/admin';
import { apiGet, apiPost, apiSend, apiProtoPost } from './client';
import * as proto from './proto';
import { decodeEnvelope } from './proto';

// ─── Dashboard ──────────────────────────────────────────────────────────────

export interface DashboardSummary {
  total_customers: number;
  active_subscriptions: number;
  monthly_revenue: number;
  overdue_invoices: number;
  open_tickets: number;
  open_leads: number;
  devices_online: number;
  devices_total: number;
  active_alerts: number;
}

export async function getDashboardSummary(): Promise<DashboardSummary> {
  const [customers, subscriptions, invoices, tickets, leads, devices, alerts] = await Promise.allSettled([
    listCustomers({ page: 1, page_size: 1 }),
    apiGet<unknown[]>('/subscriptions'),
    apiGet<unknown[]>('/billing/invoices'),
    apiGet<unknown[]>('/tickets'),
    apiGet<unknown[]>('/leads'),
    apiGet<unknown[]>('/devices'),
    apiGet<unknown[]>('/monitoring/alerts'),
  ]);
  const count = (r: PromiseSettledResult<unknown[]>) => (r.status === 'fulfilled' ? r.value.length : 0);
  const customerCount =
    customers.status === 'fulfilled' ? customers.value.total_count : count(customers);
  const summary: DashboardSummary = {
    total_customers: customerCount,
    active_subscriptions: 0,
    monthly_revenue: 0,
    overdue_invoices: 0,
    open_tickets: 0,
    open_leads: 0,
    devices_online: 0,
    devices_total: count(devices),
    active_alerts: count(alerts),
  };
  if (subscriptions.status === 'fulfilled') {
    summary.active_subscriptions = subscriptions.value.filter(
      (s) => (s as Subscription).status === 'active'
    ).length;
  }
  if (invoices.status === 'fulfilled') {
    const items = invoices.value as Invoice[];
    summary.overdue_invoices = items.filter((i) => i.status === 'overdue').length;
    summary.monthly_revenue = items
      .filter((i) => i.status === 'paid')
      .reduce((sum, i) => sum + (Number(i.total_amount ?? i.amount) || 0), 0);
  }
  if (tickets.status === 'fulfilled') {
    summary.open_tickets = (tickets.value as Ticket[]).filter(
      (t) => t.status !== 'closed' && t.status !== 'resolved'
    ).length;
  }
  if (leads.status === 'fulfilled') {
    summary.open_leads = (leads.value as Lead[]).filter((l) => !['converted', 'lost'].includes(l.status)).length;
  }
  if (devices.status === 'fulfilled') {
    summary.devices_online = (devices.value as NetworkDevice[]).filter((d) => d.status === 'online').length;
  }
  if (alerts.status === 'fulfilled') {
    summary.active_alerts = (alerts.value as MonitoringAlert[]).filter((a) => a.status === 'firing').length;
  }
  return summary;
}

// ─── Customers (protobuf module) ────────────────────────────────────────────

export interface CustomerListItem extends proto.ProtoCustomer {}

export interface ListCustomersOptions {
  page?: number;
  page_size?: number;
  status?: string;
  branch_id?: number;
  query?: string;
}

export async function listCustomers(opts: ListCustomersOptions = {}) {
  let data: Uint8Array;
  if (opts.query) {
    data = await apiProtoPost('/customers/search', proto.encodeSearchCustomersRequest({ query: opts.query, status: opts.status }));
    const parsed = proto.decodeListCustomersResponse(parseEnvelope(data));
    return parsed;
  }
  data = await apiProtoPost('/customers/list', proto.encodeListCustomersRequest(opts));
  return proto.decodeListCustomersResponse(parseEnvelope(data));
}

export async function getCustomer(id: number) {
  const data = await apiProtoPost('/customers/get', proto.encodeGetCustomerRequest(id));
  return proto.decodeCustomer(parseEnvelope(data));
}

export async function createCustomer(payload: {
  branch_id: number;
  name: string;
  email?: string;
  phone: string;
  alternate_phone?: string;
}) {
  const data = await apiProtoPost('/customers/create', proto.encodeCreateCustomerRequest(payload));
  return proto.decodeCustomer(parseEnvelope(data));
}

export async function updateCustomer(payload: {
  customer_id: number;
  name?: string;
  email?: string;
  phone?: string;
  alternate_phone?: string;
}) {
  const data = await apiProtoPost('/customers/update', proto.encodeUpdateCustomerRequest(payload), 'PATCH');
  return proto.decodeCustomer(parseEnvelope(data));
}

export async function updateCustomerStatus(customerId: number, status: string) {
  await apiProtoPost('/customers/update-status', proto.encodeUpdateCustomerStatusRequest(customerId, status));
}

export async function deleteCustomer(customerId: number) {
  await apiProtoPost('/customers/delete', proto.encodeDeleteCustomerRequest(customerId), 'DELETE');
}

export async function listCustomerAddresses(customerId: number) {
  const data = await apiProtoPost('/customers/addresses/list', proto.encodeListAddressesRequest(customerId));
  return proto.decodeListAddressesResponse(parseEnvelope(data));
}

export async function getCustomerHistory(customerId: number, page?: number, page_size?: number) {
  const data = await apiProtoPost('/customers/history', proto.encodeGetCustomerHistoryRequest(customerId, page, page_size));
  return proto.decodeCustomerHistoryResponse(parseEnvelope(data));
}

function parseEnvelope(bytes: Uint8Array): Uint8Array {
  const envelope = decodeEnvelope(bytes);
  if (envelope.status !== 1 && envelope.status !== 0) {
    throw new Error(envelope.errorMessage ?? 'Request failed');
  }
  if (!envelope.data) throw new Error('Empty response');
  return envelope.data;
}

// ─── Plans ──────────────────────────────────────────────────────────────────

export async function listPlans(): Promise<Plan[]> {
  const data = await apiGet<Plan[] | { plans: Plan[] }>('/plans');
  return Array.isArray(data) ? data : data.plans ?? [];
}

export async function getPlan(id: number): Promise<Plan> {
  return apiGet<Plan>(`/plans/${id}`);
}

export async function createPlan(payload: Partial<Plan>): Promise<Plan> {
  return apiPost<Plan>('/admin/plans', payload);
}

export async function updatePlan(id: number, payload: Partial<Plan>): Promise<Plan> {
  return apiSend<Plan>('PUT', `/admin/plans/${id}`, payload);
}

export async function updatePlanPricing(id: number, pricing: unknown): Promise<unknown> {
  return apiSend('PUT', `/admin/plans/${id}/pricing`, pricing);
}

export async function approvePlan(id: number): Promise<unknown> {
  return apiPost(`/admin/plans/${id}/approve`, {});
}

export async function publishPlan(id: number, publish: boolean): Promise<unknown> {
  return apiPost(`/admin/plans/${id}/${publish ? 'publish' : 'unpublish'}`, {});
}

export async function deactivatePlan(id: number): Promise<unknown> {
  return apiSend('DELETE', `/admin/plans/${id}`);
}

// ─── Subscriptions ──────────────────────────────────────────────────────────

export async function listSubscriptions(): Promise<Subscription[]> {
  const data = await apiGet<Subscription[] | { items: Subscription[] }>('/subscriptions');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createSubscription(payload: Partial<Subscription>): Promise<Subscription> {
  return apiPost<Subscription>('/subscriptions', payload);
}

export async function subscriptionAction(id: number, action: 'renew' | 'cancel' | 'suspend' | 'reactivate' | 'upgrade' | 'downgrade', payload?: unknown): Promise<unknown> {
  return apiPost(`/subscriptions/${id}/${action}`, payload ?? {});
}

// ─── Billing ────────────────────────────────────────────────────────────────

export async function listInvoices(): Promise<Invoice[]> {
  const data = await apiGet<Invoice[] | { items: Invoice[] }>('/billing/invoices');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function listOverdueInvoices(): Promise<Invoice[]> {
  const data = await apiGet<Invoice[] | { items: Invoice[] }>('/billing/invoices/overdue');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createInvoice(payload: Partial<Invoice>): Promise<Invoice> {
  return apiPost<Invoice>('/billing/invoices', payload);
}

export async function getInvoice(id: number): Promise<Invoice> {
  return apiGet<Invoice>(`/billing/invoices/${id}`);
}

export async function sendInvoice(id: number): Promise<unknown> {
  return apiPost(`/billing/invoices/${id}/send`, {});
}

export async function voidInvoice(id: number): Promise<unknown> {
  return apiPost(`/billing/invoices/${id}/void`, {});
}

export async function listPayments(): Promise<Payment[]> {
  const data = await apiGet<Payment[] | { items: Payment[] }>('/billing/payments');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function recordPayment(payload: Partial<Payment>): Promise<Payment> {
  return apiPost<Payment>('/billing/payments', payload);
}

export async function listDiscounts(): Promise<unknown[]> {
  const data = await apiGet<unknown[] | { items: unknown[] }>('/billing/discounts');
  return Array.isArray(data) ? data : data.items ?? [];
}

// ─── Tickets ────────────────────────────────────────────────────────────────

export async function listTickets(): Promise<Ticket[]> {
  const data = await apiGet<Ticket[] | { items: Ticket[] }>('/tickets');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function getTicket(id: number): Promise<Ticket> {
  return apiGet<Ticket>(`/tickets/${id}`);
}

export async function createTicket(payload: Partial<Ticket>): Promise<Ticket> {
  return apiPost<Ticket>('/tickets', payload);
}

export async function updateTicket(id: number, payload: Partial<Ticket>): Promise<Ticket> {
  return apiSend<Ticket>('PUT', `/tickets/${id}`, payload);
}

export async function ticketAction(id: number, action: 'assign' | 'resolve' | 'escalate' | 'close' | 'reopen', payload?: unknown): Promise<unknown> {
  return apiPost(`/tickets/${id}/${action}`, payload ?? {});
}

export async function getTicketMetrics(): Promise<unknown> {
  return apiGet('/tickets/metrics');
}

// ─── Leads ──────────────────────────────────────────────────────────────────

export async function listLeads(): Promise<Lead[]> {
  const data = await apiGet<Lead[] | { items: Lead[] }>('/leads');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createLead(payload: Partial<Lead>): Promise<Lead> {
  return apiPost<Lead>('/leads', payload);
}

export async function updateLeadStatus(id: number, status: string): Promise<unknown> {
  return apiSend('PUT', `/leads/${id}/status`, { status });
}

export async function assignLead(id: number, assigneeId: number): Promise<unknown> {
  return apiPost(`/leads/${id}/assign`, { assignee_id: assigneeId });
}

export async function convertLead(id: number): Promise<unknown> {
  return apiPost(`/leads/${id}/convert`, {});
}

// ─── Network ────────────────────────────────────────────────────────────────

export async function getTopology(): Promise<unknown> {
  return apiGet('/network/topology');
}

export async function listVlans(): Promise<Vlan[]> {
  const data = await apiGet<Vlan[] | { items: Vlan[] }>('/network/vlans');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function listIpPools(): Promise<IpPool[]> {
  const data = await apiGet<IpPool[] | { items: IpPool[] }>('/network/ip-pools');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createIpPool(payload: Partial<IpPool>): Promise<IpPool> {
  return apiPost<IpPool>('/network/ip-pools', payload);
}

export async function listDhcpLeases(): Promise<unknown[]> {
  const data = await apiGet<unknown[] | { items: unknown[] }>('/network/dhcp/leases');
  return Array.isArray(data) ? data : data.items ?? [];
}

// ─── Devices ────────────────────────────────────────────────────────────────

export async function listDevices(): Promise<NetworkDevice[]> {
  const data = await apiGet<NetworkDevice[] | { items: NetworkDevice[] }>('/devices');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function registerDevice(payload: Partial<NetworkDevice>): Promise<NetworkDevice> {
  return apiPost<NetworkDevice>('/devices', payload);
}

export async function updateDeviceStatus(id: number, status: string): Promise<unknown> {
  return apiSend('PUT', `/devices/${id}/status`, { status });
}

export async function deviceAction(id: number, action: 'restart' | 'shutdown'): Promise<unknown> {
  return apiPost(`/devices/${id}/${action}`, {});
}

export async function listDeviceMetrics(id: number): Promise<DeviceMetricRow[]> {
  const data = await apiGet<DeviceMetricRow[] | { items: DeviceMetricRow[] }>(`/devices/${id}/metrics`);
  return Array.isArray(data) ? data : data.items ?? [];
}

// ─── Monitoring ─────────────────────────────────────────────────────────────

export async function listAlerts(): Promise<MonitoringAlert[]> {
  const data = await apiGet<MonitoringAlert[] | { items: MonitoringAlert[] }>('/monitoring/alerts');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function alertAction(id: number, action: 'acknowledge' | 'resolve'): Promise<unknown> {
  return apiPost(`/monitoring/alerts/${id}/${action}`, {});
}

// ─── Installations ──────────────────────────────────────────────────────────

export async function listInstallations(): Promise<Installation[]> {
  const data = await apiGet<Installation[] | { items: Installation[] }>('/installations');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createInstallation(payload: Partial<Installation>): Promise<Installation> {
  return apiPost<Installation>('/installations', payload);
}

export async function installationAction(id: number, action: 'schedule' | 'reschedule' | 'start' | 'complete' | 'cancel', payload?: unknown): Promise<unknown> {
  return apiPost(`/installations/${id}/${action}`, payload ?? {});
}

// ─── Approvals / Workflow ───────────────────────────────────────────────────

export async function listPendingApprovals(): Promise<ApprovalRequest[]> {
  const data = await apiGet<ApprovalRequest[] | { items: ApprovalRequest[] }>('/approvals/pending');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function listAllApprovals(): Promise<ApprovalRequest[]> {
  const pending = await listPendingApprovals();
  return pending;
}

export async function approvalAction(id: number, action: 'approve' | 'reject', reason?: string): Promise<unknown> {
  return apiPost(`/approvals/${id}/${action}`, reason ? { reason } : {});
}

// ─── Notifications ──────────────────────────────────────────────────────────

export async function listNotifications(): Promise<Notification[]> {
  const data = await apiGet<Notification[] | { items: Notification[] }>('/notifications/list');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function sendNotification(payload: unknown): Promise<unknown> {
  return apiPost('/notifications/send', payload);
}

export async function retryNotification(id: number): Promise<unknown> {
  return apiPost(`/notifications/${id}/retry`, {});
}

// ─── Users & RBAC ───────────────────────────────────────────────────────────

export async function listUsers(): Promise<UserAccount[]> {
  const data = await apiGet<UserAccount[] | { items: UserAccount[] }>('/users');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function listRoles(): Promise<Role[]> {
  const data = await apiGet<Role[] | { items: Role[] }>('/rbac/roles');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createRole(payload: Partial<Role>): Promise<Role> {
  return apiPost<Role>('/rbac/roles', payload);
}

export async function updateRole(id: number, payload: Partial<Role>): Promise<Role> {
  return apiSend<Role>('PUT', `/rbac/roles/${id}`, payload);
}

export async function deleteRole(id: number): Promise<unknown> {
  return apiSend('DELETE', `/rbac/roles/${id}`);
}

export async function listPermissions(): Promise<Permission[]> {
  const data = await apiGet<Permission[] | { items: Permission[] }>('/rbac/permissions');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function assignRoleToUser(userId: number, roleId: number): Promise<unknown> {
  return apiPost(`/rbac/users/${userId}/roles`, { role_id: roleId });
}

export async function revokeRoleFromUser(userId: number, roleId: number): Promise<unknown> {
  return apiSend('DELETE', `/rbac/users/${userId}/roles/${roleId}`);
}

// ─── Branches ───────────────────────────────────────────────────────────────

export async function listBranches(): Promise<Branch[]> {
  const data = await apiGet<Branch[] | { items: Branch[] }>('/branches');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createBranch(payload: Partial<Branch>): Promise<Branch> {
  return apiPost<Branch>('/branches', payload);
}

export async function updateBranch(id: number, payload: Partial<Branch>): Promise<Branch> {
  return apiSend<Branch>('PUT', `/branches/${id}`, payload);
}

export async function deleteBranch(id: number): Promise<unknown> {
  return apiSend('DELETE', `/branches/${id}`);
}

// ─── Audit ──────────────────────────────────────────────────────────────────

export async function listAuditLogs(): Promise<AuditLog[]> {
  const data = await apiGet<AuditLog[] | { items: AuditLog[] }>('/audit/logs');
  return Array.isArray(data) ? data : data.items ?? [];
}

// ─── Accounting ─────────────────────────────────────────────────────────────

export async function listAccounts(): Promise<AccountingAccount[]> {
  const data = await apiGet<AccountingAccount[] | { items: AccountingAccount[] }>('/accounting/accounts');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function listJournalEntries(): Promise<JournalEntry[]> {
  const data = await apiGet<JournalEntry[] | { items: JournalEntry[] }>('/accounting/journal');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function getTrialBalance(): Promise<unknown> {
  return apiGet('/accounting/trial-balance');
}

// ─── Coverage ───────────────────────────────────────────────────────────────

export async function listCoverageAreas(): Promise<CoverageArea[]> {
  const data = await apiGet<CoverageArea[] | { items: CoverageArea[] }>('/coverage/areas');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createCoverageArea(payload: Partial<CoverageArea>): Promise<CoverageArea> {
  return apiPost<CoverageArea>('/coverage/areas', payload);
}
