import type {
  AccountingAccount,
  AlertRule,
  BalanceSheet,
  ApprovalRequest,
  AuditEvent,
  AuditLog,
  Branch,
  BranchStats,
  CoverageArea,
  CoverageCheck,
  DeviceMetricRow,
  Discount,
  GstinReturn,
  Installation,
  Invoice,
  InvoiceItem,
  JournalEntry,
  JournalEntryLine,
  KycVerification,
  Lead,
  MacBinding,
  MonitoringAlert,
  NetworkDevice,
  Notification,
  NotificationChannel,
  NotificationTemplate,
  Paginated,
  Payment,
  PppoeSession,
  Permission,
  Plan,
  ProfitAndLoss,
  Refund,
  Role,
  Subscription,
  SubscriptionHistory,
  Ticket,
  TicketComment,
  TicketMetrics,
  UserAccount,
  WorkingHours,
  IpPool,
  Vlan,
  BandwidthProfile,
  BandwidthPolicy,
  BandwidthUsage,
  BandwidthApplication,
  SchedulerJob,
  SchedulerExecution,
  SchedulerStats,
  Consent,
  RetentionPolicy,
  ReferralProgram,
  ReferralRecord,
  ReferralWallet,
  ReferralAnalytics,
  DiscoveryScan,
  DiscoveryResult,
  InventoryItem,
  DocumentRecord,
  ApiKey,
  RateLimitRule,
  RequestLog,
  GatewayStats,
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

// Single backend call — the server returns COUNT aggregates so the dashboard
// stays cheap at any data volume (no full-list fetches).
export async function getDashboardSummary(): Promise<DashboardSummary> {
  return apiGet<DashboardSummary>('/admin/dashboard/summary');
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

export interface CreatePlanPayload {
  name: string;
  slug?: string;
  description?: string;
  speed_label: string;
  download_mbps: number;
  upload_mbps: number;
  burst_mbps?: number;
  is_business?: boolean;
}

export async function createPlan(payload: CreatePlanPayload): Promise<Plan> {
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

export async function clonePlan(id: number): Promise<Plan> {
  return apiPost<Plan>(`/admin/plans/${id}/clone`, {});
}

export async function deactivatePlan(id: number): Promise<unknown> {
  return apiSend('DELETE', `/admin/plans/${id}`);
}

export async function getPlanHistory(id: number): Promise<unknown[]> {
  const data = await apiGet<unknown[] | { items: unknown[] }>(`/admin/plans/${id}/history`);
  return Array.isArray(data) ? data : (data as { items: unknown[] }).items ?? [];
}

// ─── Subscriptions ──────────────────────────────────────────────────────────

export async function listSubscriptions(): Promise<Subscription[]> {
  const data = await apiGet<Subscription[] | { items: Subscription[] }>('/subscriptions');
  return Array.isArray(data) ? data : data.items ?? [];
}

export interface CreateSubscriptionPayload {
  customer_id: number;
  plan_id: number;
  billing_period_months: number;
  branch_id?: number;
}

export async function createSubscription(payload: CreateSubscriptionPayload): Promise<Subscription> {
  return apiPost<Subscription>('/subscriptions', payload);
}

export async function subscriptionAction(id: number, action: 'renew' | 'cancel' | 'suspend' | 'reactivate' | 'upgrade' | 'downgrade', payload?: unknown): Promise<unknown> {
  return apiPost(`/subscriptions/${id}/${action}`, payload ?? {});
}

export async function getSubscriptionHistory(id: number): Promise<SubscriptionHistory[]> {
  const data = await apiGet<SubscriptionHistory[] | { items: SubscriptionHistory[] }>(`/subscriptions/${id}/history`);
  return Array.isArray(data) ? data : (data as { items: SubscriptionHistory[] }).items ?? [];
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

export interface CreateInvoicePayload {
  customer_id?: number;
  branch_id?: number;
  subscription_id: number;
  billing_period_start: string;
  billing_period_end: string;
  total_amount: string | number;
}

export async function createInvoice(payload: CreateInvoicePayload): Promise<Invoice> {
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

export async function listInvoiceItems(invoiceId: number): Promise<InvoiceItem[]> {
  const data = await apiGet<InvoiceItem[] | { items: InvoiceItem[] }>(`/billing/invoices/${invoiceId}/items`);
  return Array.isArray(data) ? data : (data as { items: InvoiceItem[] }).items ?? [];
}

export async function addInvoiceItem(invoiceId: number, payload: Partial<InvoiceItem>): Promise<InvoiceItem> {
  return apiPost<InvoiceItem>(`/billing/invoices/${invoiceId}/items`, payload);
}

export async function removeInvoiceItem(invoiceId: number, itemId: number): Promise<unknown> {
  return apiSend('DELETE', `/billing/invoices/${invoiceId}/items/${itemId}`);
}

export async function autoGenerateInvoices(): Promise<unknown> {
  return apiPost('/billing/invoices/auto-generate', {});
}

export async function listPayments(): Promise<Payment[]> {
  const data = await apiGet<Payment[] | { items: Payment[] }>('/billing/payments');
  return Array.isArray(data) ? data : data.items ?? [];
}

export interface RecordPaymentPayload {
  invoice_id: number;
  amount: string | number;
  payment_method: string;
}

export async function recordPayment(payload: RecordPaymentPayload): Promise<Payment> {
  return apiPost<Payment>('/billing/payments', payload);
}

export async function requestRefund(payload: { payment_id: number; amount: number; reason?: string }): Promise<Refund> {
  return apiPost<Refund>('/billing/refunds', payload);
}

export async function approveRefund(id: number): Promise<unknown> {
  return apiSend('PUT', `/billing/refunds/${id}/approve`);
}

export async function rejectRefund(id: number): Promise<unknown> {
  return apiSend('PUT', `/billing/refunds/${id}/reject`);
}

export async function listDiscounts(): Promise<Discount[]> {
  const data = await apiGet<Discount[] | { items: Discount[] }>('/billing/discounts');
  return Array.isArray(data) ? data : (data as { items: Discount[] }).items ?? [];
}

export async function createDiscount(payload: Partial<Discount>): Promise<Discount> {
  return apiPost<Discount>('/billing/discounts', payload);
}

// ─── Tickets ────────────────────────────────────────────────────────────────

export async function listTickets(): Promise<Ticket[]> {
  const data = await apiGet<Ticket[] | { items: Ticket[] }>('/tickets');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function getTicket(id: number): Promise<Ticket> {
  return apiGet<Ticket>(`/tickets/${id}`);
}

export interface CreateTicketPayload {
  subject: string;
  description: string;
  category: string;
  priority: string;
  source: string;
  customer_id?: number;
}

export async function createTicket(payload: CreateTicketPayload): Promise<Ticket> {
  return apiPost<Ticket>('/tickets', payload);
}

export async function updateTicket(id: number, payload: Partial<Ticket>): Promise<Ticket> {
  return apiSend<Ticket>('PUT', `/tickets/${id}`, payload);
}

export async function ticketAction(
  id: number,
  action: 'assign' | 'resolve' | 'escalate' | 'close' | 'reopen',
  payload?: { assigned_to?: number; escalated_to?: number; reason?: string; closure_notes?: string; resolution_notes?: string }
): Promise<unknown> {
  return apiPost(`/tickets/${id}/${action}`, payload ?? {});
}

export async function getTicketMetrics(): Promise<TicketMetrics> {
  return apiGet<TicketMetrics>('/tickets/metrics');
}

export async function listTicketComments(ticketId: number): Promise<TicketComment[]> {
  const data = await apiGet<TicketComment[] | { items: TicketComment[] }>(`/tickets/${ticketId}/comments`);
  return Array.isArray(data) ? data : (data as { items: TicketComment[] }).items ?? [];
}

export async function addTicketComment(ticketId: number, payload: { content: string; is_internal?: boolean }): Promise<TicketComment> {
  return apiPost<TicketComment>(`/tickets/${ticketId}/comments`, payload);
}

export async function rateTicketSatisfaction(ticketId: number, rating: number, comment?: string): Promise<unknown> {
  return apiPost(`/tickets/${ticketId}/satisfaction`, { rating, comment });
}

export async function listMyAssignments(): Promise<Ticket[]> {
  const data = await apiGet<Ticket[] | { items: Ticket[] }>('/tickets/my-assignments');
  return Array.isArray(data) ? data : (data as { items: Ticket[] }).items ?? [];
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
  return apiPost(`/leads/${id}/assign`, { assigned_to: assigneeId });
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

export interface CreateVlanPayload {
  vlan_id: number;
  name: string;
  vlan_type: string;
  description?: string;
  branch_id?: number;
}

export async function createVlan(payload: CreateVlanPayload): Promise<Vlan> {
  return apiPost<Vlan>('/network/vlans', payload);
}

export async function updateVlan(id: number, payload: Partial<Vlan>): Promise<Vlan> {
  return apiSend<Vlan>('PUT', `/network/vlans/${id}`, payload);
}

export async function deleteVlan(id: number): Promise<unknown> {
  return apiSend('DELETE', `/network/vlans/${id}`);
}

export async function listIpPools(): Promise<IpPool[]> {
  const data = await apiGet<IpPool[] | { items: IpPool[] }>('/network/ip-pools');
  return Array.isArray(data) ? data : data.items ?? [];
}

export interface CreateIpPoolPayload {
  name?: string;
  cidr: string;
  gateway?: string;
  pool_type: string;
  total_count: number;
  branch_id?: number;
}

export async function createIpPool(payload: CreateIpPoolPayload): Promise<IpPool> {
  return apiPost<IpPool>('/network/ip-pools', payload);
}

export async function allocateIp(poolId: number): Promise<unknown> {
  return apiPost(`/network/ip-pools/${poolId}/allocate`, {});
}

export async function releaseIp(poolId: number, payload: { ip_address: string }): Promise<unknown> {
  return apiPost(`/network/ip-pools/${poolId}/release`, payload);
}

export async function listDhcpLeases(): Promise<unknown[]> {
  const data = await apiGet<unknown[] | { items: unknown[] }>('/network/dhcp/leases');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function listPppoeSessions(): Promise<PppoeSession[]> {
  const data = await apiGet<PppoeSession[] | { items: PppoeSession[] }>('/network/pppoe/sessions');
  return Array.isArray(data) ? data : (data as { items: PppoeSession[] }).items ?? [];
}

export async function terminatePppoeSession(id: number): Promise<unknown> {
  return apiPost(`/network/pppoe/sessions/${id}/terminate`, {});
}

export async function listMacBindings(): Promise<MacBinding[]> {
  const data = await apiGet<MacBinding[] | { items: MacBinding[] }>('/network/mac-bindings');
  return Array.isArray(data) ? data : (data as { items: MacBinding[] }).items ?? [];
}

export interface CreateMacBindingPayload {
  mac_address: string;
  assigned_ip: string;
  customer_id: number;
  subscription_id: number;
  branch_id?: number;
}

export async function createMacBinding(payload: CreateMacBindingPayload): Promise<MacBinding> {
  return apiPost<MacBinding>('/network/mac-bindings', payload);
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

export interface CreateAlertPayload {
  device_id: number;
  branch_id?: number;
  severity: string;
  title: string;
  message: string;
}

export async function createAlert(payload: CreateAlertPayload): Promise<MonitoringAlert> {
  return apiPost<MonitoringAlert>('/monitoring/alerts', payload);
}

export async function getAlertStats(): Promise<unknown> {
  return apiGet('/monitoring/alerts/stats');
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

export interface SendNotificationPayload {
  channel: string;
  recipient_type: string;
  recipient_id: number;
  recipient_address: string;
  subject?: string;
  body: string;
}

export async function sendNotification(payload: SendNotificationPayload): Promise<unknown> {
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

export interface CreateRolePayload {
  name: string;
  slug: string;
  description?: string;
  parent_role_id?: number;
}

export async function createRole(payload: CreateRolePayload): Promise<Role> {
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

export interface CreateBranchPayload {
  name: string;
  slug: string;
  code: string;
  city: string;
  state?: string;
}

export async function createBranch(payload: CreateBranchPayload): Promise<Branch> {
  return apiPost<Branch>('/branches', payload);
}

export async function updateBranch(id: number, payload: Partial<Branch>): Promise<Branch> {
  return apiSend<Branch>('PUT', `/branches/${id}`, payload);
}

export async function deleteBranch(id: number): Promise<unknown> {
  return apiSend('DELETE', `/branches/${id}`);
}

export async function getBranchHierarchy(): Promise<unknown> {
  return apiGet('/branches/hierarchy');
}

export async function getBranchStats(id: number): Promise<BranchStats> {
  return apiGet<BranchStats>(`/branches/${id}/stats`);
}

export async function getBranchWorkingHours(id: number): Promise<WorkingHours[]> {
  const data = await apiGet<WorkingHours[] | { items: WorkingHours[] }>(`/branches/${id}/working-hours`);
  return Array.isArray(data) ? data : (data as { items: WorkingHours[] }).items ?? [];
}

export async function updateBranchWorkingHours(id: number, hours: WorkingHours[]): Promise<unknown> {
  return apiSend('PUT', `/branches/${id}/working-hours`, { hours });
}

// ─── Audit ──────────────────────────────────────────────────────────────────

export async function listAuditLogs(): Promise<AuditLog[]> {
  const data = await apiGet<AuditLog[] | { items: AuditLog[] }>('/audit/logs');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function searchAuditLogs(params: { action?: string; entity_type?: string; user_id?: number; start_date?: string; end_date?: string }): Promise<AuditLog[]> {
  const data = await apiGet<AuditLog[] | { items: AuditLog[] }>('/audit/logs', params);
  return Array.isArray(data) ? data : (data as { items: AuditLog[] }).items ?? [];
}

export async function exportAuditLogs(params?: { format?: string }): Promise<unknown> {
  return apiGet('/audit/export', params);
}

export async function listAuditEvents(): Promise<AuditEvent[]> {
  const data = await apiGet<AuditEvent[] | { items: AuditEvent[] }>('/audit/events');
  return Array.isArray(data) ? data : (data as { items: AuditEvent[] }).items ?? [];
}

export async function replayAuditEvent(id: number): Promise<unknown> {
  return apiPost(`/audit/events/${id}/replay`, {});
}

// ─── Accounting ─────────────────────────────────────────────────────────────

export async function listAccounts(): Promise<AccountingAccount[]> {
  const data = await apiGet<AccountingAccount[] | { items: AccountingAccount[] }>('/accounting/accounts');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createAccount(payload: Partial<AccountingAccount>): Promise<AccountingAccount> {
  return apiPost<AccountingAccount>('/accounting/accounts', payload);
}

export async function listJournalEntries(): Promise<JournalEntry[]> {
  const data = await apiGet<JournalEntry[] | { items: JournalEntry[] }>('/accounting/journal');
  return Array.isArray(data) ? data : data.items ?? [];
}

export async function createJournalEntry(payload: { description: string; lines: JournalEntryLine[]; entry_type?: string }): Promise<JournalEntry> {
  return apiPost<JournalEntry>('/accounting/journal', payload);
}

export async function postJournalEntry(id: number): Promise<unknown> {
  return apiPost(`/accounting/journal/${id}/post`, {});
}

export async function voidJournalEntry(id: number): Promise<unknown> {
  return apiPost(`/accounting/journal/${id}/void`, {});
}

export async function getTrialBalance(): Promise<unknown> {
  return apiGet('/accounting/trial-balance');
}

export async function getProfitAndLoss(params?: { start_date?: string; end_date?: string }): Promise<ProfitAndLoss> {
  return apiGet<ProfitAndLoss>('/accounting/statements/profit-loss', params);
}

export async function getBalanceSheet(): Promise<BalanceSheet> {
  return apiGet<BalanceSheet>('/accounting/statements/balance-sheet');
}

export async function getGstinReturn(type: string): Promise<GstinReturn> {
  return apiGet<GstinReturn>(`/accounting/gst/${type}`);
}

export async function reconcileAccount(accountId: number): Promise<unknown> {
  return apiGet(`/accounting/reconciliation/${accountId}`);
}

// ─── Coverage ───────────────────────────────────────────────────────────────

export async function listCoverageAreas(): Promise<CoverageArea[]> {
  const data = await apiGet<CoverageArea[] | { items: CoverageArea[] }>('/coverage/areas');
  return Array.isArray(data) ? data : data.items ?? [];
}

export interface CreateCoverageAreaPayload {
  name: string;
  area_type: string;
}

export async function createCoverageArea(payload: CreateCoverageAreaPayload): Promise<CoverageArea> {
  return apiPost<CoverageArea>('/coverage/areas', payload);
}

export async function updateCoverageArea(id: number, payload: Partial<CoverageArea>): Promise<CoverageArea> {
  return apiSend<CoverageArea>('PUT', `/coverage/areas/${id}`, payload);
}

export async function deleteCoverageArea(id: number): Promise<unknown> {
  return apiSend('DELETE', `/coverage/areas/${id}`);
}

export async function checkCoverage(payload: { pincode?: string; address?: string }): Promise<CoverageCheck> {
  return apiPost<CoverageCheck>('/coverage/check', payload);
}

// ─── Notifications (extended) ───────────────────────────────────────────────

export async function listNotificationTemplates(): Promise<NotificationTemplate[]> {
  const data = await apiGet<NotificationTemplate[] | { items: NotificationTemplate[] }>('/notifications/templates');
  return Array.isArray(data) ? data : (data as { items: NotificationTemplate[] }).items ?? [];
}

export interface CreateNotificationTemplatePayload {
  name: string;
  channel: string;
  body_template: string;
  subject_template?: string;
}

export async function createNotificationTemplate(payload: CreateNotificationTemplatePayload): Promise<NotificationTemplate> {
  return apiPost<NotificationTemplate>('/notifications/templates', payload);
}

export async function updateNotificationTemplate(id: number, payload: Partial<NotificationTemplate>): Promise<NotificationTemplate> {
  return apiSend<NotificationTemplate>('PUT', `/notifications/templates/${id}`, payload);
}

export async function deleteNotificationTemplate(id: number): Promise<unknown> {
  return apiSend('DELETE', `/notifications/templates/${id}`);
}

export async function listNotificationChannels(): Promise<NotificationChannel[]> {
  const data = await apiGet<NotificationChannel[] | { items: NotificationChannel[] }>('/notifications/channels');
  return Array.isArray(data) ? data : (data as { items: NotificationChannel[] }).items ?? [];
}

// ─── Bandwidth ──────────────────────────────────────────────────────────────

export async function listBandwidthProfiles(): Promise<BandwidthProfile[]> {
  const data = await apiGet<BandwidthProfile[] | { items: BandwidthProfile[] }>('/bandwidth/profiles');
  return Array.isArray(data) ? data : (data as { items: BandwidthProfile[] }).items ?? [];
}

export async function getBandwidthProfile(id: number): Promise<BandwidthProfile> {
  return apiGet<BandwidthProfile>(`/bandwidth/profiles/${id}`);
}

export interface CreateBandwidthProfilePayload {
  name: string;
  download_kbps: number;
  upload_kbps: number;
}

export async function createBandwidthProfile(payload: CreateBandwidthProfilePayload): Promise<BandwidthProfile> {
  return apiPost<BandwidthProfile>('/bandwidth/profiles', payload);
}

export async function updateBandwidthProfile(id: number, payload: Partial<BandwidthProfile>): Promise<BandwidthProfile> {
  return apiSend<BandwidthProfile>('PUT', `/bandwidth/profiles/${id}`, payload);
}

export async function deleteBandwidthProfile(id: number): Promise<unknown> {
  return apiSend('DELETE', `/bandwidth/profiles/${id}`);
}

export async function applyProfileToAll(id: number): Promise<unknown> {
  return apiPost(`/bandwidth/profiles/${id}/apply`, {});
}

export async function applyProfileToSubscription(subscriptionId: number, profileId: number): Promise<unknown> {
  return apiPost(`/bandwidth/apply/${subscriptionId}`, { profile_id: profileId });
}

export async function listBandwidthApplications(): Promise<BandwidthApplication[]> {
  const data = await apiGet<BandwidthApplication[] | { items: BandwidthApplication[] }>('/bandwidth/applications');
  return Array.isArray(data) ? data : (data as { items: BandwidthApplication[] }).items ?? [];
}

export async function getBandwidthUsage(subscriptionId: number): Promise<BandwidthUsage> {
  return apiGet<BandwidthUsage>(`/bandwidth/usage/${subscriptionId}`);
}

export async function listBandwidthPolicies(): Promise<BandwidthPolicy[]> {
  const data = await apiGet<BandwidthPolicy[] | { items: BandwidthPolicy[] }>('/bandwidth/policies');
  return Array.isArray(data) ? data : (data as { items: BandwidthPolicy[] }).items ?? [];
}

export interface CreateBandwidthPolicyPayload {
  name: string;
  policy_type: string;
  config?: Record<string, unknown>;
  priority?: number;
}

export async function createBandwidthPolicy(payload: CreateBandwidthPolicyPayload): Promise<BandwidthPolicy> {
  return apiPost<BandwidthPolicy>('/bandwidth/policies', payload);
}

export async function updateBandwidthPolicy(id: number, payload: Partial<BandwidthPolicy>): Promise<BandwidthPolicy> {
  return apiSend<BandwidthPolicy>('PUT', `/bandwidth/policies/${id}`, payload);
}

export async function deleteBandwidthPolicy(id: number): Promise<unknown> {
  return apiSend('DELETE', `/bandwidth/policies/${id}`);
}

// ─── Scheduler ──────────────────────────────────────────────────────────────

export async function listSchedulerJobs(): Promise<SchedulerJob[]> {
  const data = await apiGet<SchedulerJob[] | { items: SchedulerJob[] }>('/scheduler/jobs');
  return Array.isArray(data) ? data : (data as { items: SchedulerJob[] }).items ?? [];
}

export async function getSchedulerJob(id: number): Promise<SchedulerJob> {
  return apiGet<SchedulerJob>(`/scheduler/jobs/${id}`);
}

export interface CreateSchedulerJobPayload {
  name: string;
  description?: string;
  job_type: string;
  schedule: string;
  target_module: string;
  action: string;
  payload?: Record<string, unknown>;
  timeout_seconds?: number;
}

export async function createSchedulerJob(payload: CreateSchedulerJobPayload): Promise<SchedulerJob> {
  return apiPost<SchedulerJob>('/scheduler/jobs', payload);
}

export async function updateSchedulerJob(id: number, payload: Partial<SchedulerJob>): Promise<SchedulerJob> {
  return apiSend<SchedulerJob>('PUT', `/scheduler/jobs/${id}`, payload);
}

export async function deleteSchedulerJob(id: number): Promise<unknown> {
  return apiSend('DELETE', `/scheduler/jobs/${id}`);
}

export async function triggerSchedulerJob(id: number): Promise<unknown> {
  return apiPost(`/scheduler/jobs/${id}/trigger`, {});
}

export async function listSchedulerExecutions(): Promise<SchedulerExecution[]> {
  const data = await apiGet<SchedulerExecution[] | { items: SchedulerExecution[] }>('/scheduler/executions');
  return Array.isArray(data) ? data : (data as { items: SchedulerExecution[] }).items ?? [];
}

export async function getSchedulerStats(): Promise<SchedulerStats> {
  return apiGet<SchedulerStats>('/scheduler/stats');
}

// ─── Compliance ─────────────────────────────────────────────────────────────

export async function listKycVerifications(): Promise<KycVerification[]> {
  const data = await apiGet<KycVerification[] | { items: KycVerification[] }>('/compliance/kyc');
  return Array.isArray(data) ? data : (data as { items: KycVerification[] }).items ?? [];
}

export async function updateKycStatus(id: number, status: string): Promise<unknown> {
  return apiSend('PUT', `/compliance/kyc/${id}`, { status });
}

export async function createKycVerification(payload: Partial<KycVerification>): Promise<KycVerification> {
  return apiPost<KycVerification>('/compliance/kyc', payload);
}

export async function getCustomerKyc(customerId: number): Promise<KycVerification[]> {
  const data = await apiGet<KycVerification[] | { items: KycVerification[] }>(`/compliance/kyc/customer/${customerId}`);
  return Array.isArray(data) ? data : (data as { items: KycVerification[] }).items ?? [];
}

export async function listConsents(customerId: number): Promise<Consent[]> {
  const data = await apiGet<Consent[] | { items: Consent[] }>(`/compliance/consents/${customerId}`);
  return Array.isArray(data) ? data : (data as { items: Consent[] }).items ?? [];
}

export interface GrantConsentPayload {
  customer_id: number;
  consent_type: string;
  collection_channel: string;
  ip_address?: string;
  user_agent?: string;
}

export async function grantConsent(payload: GrantConsentPayload): Promise<unknown> {
  return apiPost('/compliance/consents', payload);
}

export async function revokeConsent(payload: { customer_id: number; consent_type: string }): Promise<unknown> {
  return apiPost('/compliance/consents/revoke', payload);
}

export async function listRetentionPolicies(): Promise<RetentionPolicy[]> {
  const data = await apiGet<RetentionPolicy[] | { items: RetentionPolicy[] }>('/compliance/retention');
  return Array.isArray(data) ? data : (data as { items: RetentionPolicy[] }).items ?? [];
}

export async function createRetentionPolicy(payload: Partial<RetentionPolicy>): Promise<RetentionPolicy> {
  return apiPost<RetentionPolicy>('/compliance/retention', payload);
}

export async function updateRetentionPolicy(id: number, payload: Partial<RetentionPolicy>): Promise<RetentionPolicy> {
  return apiSend<RetentionPolicy>('PUT', `/compliance/retention/${id}`, payload);
}

// ─── Referrals (admin) ──────────────────────────────────────────────────────

export async function listReferralPrograms(): Promise<ReferralProgram[]> {
  const data = await apiGet<ReferralProgram[] | { items: ReferralProgram[] }>('/referrals/admin/referral-programs');
  return Array.isArray(data) ? data : (data as { items: ReferralProgram[] }).items ?? [];
}

export interface CreateReferralProgramPayload {
  name: string;
  reward_type: string;
  reward_value: number;
  max_referrals_per_user?: number;
  valid_from: string;
  valid_until: string;
}

export async function createReferralProgram(payload: CreateReferralProgramPayload): Promise<ReferralProgram> {
  return apiPost<ReferralProgram>('/referrals/admin/referral-programs', payload);
}

export async function updateReferralProgram(id: number, payload: Partial<ReferralProgram>): Promise<ReferralProgram> {
  return apiSend<ReferralProgram>('PUT', `/referrals/admin/referral-programs/${id}`, payload);
}

export async function deleteReferralProgram(id: number): Promise<unknown> {
  return apiSend('DELETE', `/referrals/admin/referral-programs/${id}`);
}

export async function listReferrals(): Promise<ReferralRecord[]> {
  const data = await apiGet<ReferralRecord[] | { items: ReferralRecord[] }>('/referrals');
  return Array.isArray(data) ? data : (data as { items: ReferralRecord[] }).items ?? [];
}

export async function listReferralWallets(): Promise<ReferralWallet[]> {
  const data = await apiGet<ReferralWallet[] | { items: ReferralWallet[] }>('/referrals/admin/wallets');
  return Array.isArray(data) ? data : (data as { items: ReferralWallet[] }).items ?? [];
}

export async function adjustReferralWallet(walletId: number, payload: { amount: number; reason?: string }): Promise<unknown> {
  return apiPost(`/referrals/admin/wallets/${walletId}/adjust`, payload);
}

export async function getReferralAnalytics(): Promise<ReferralAnalytics> {
  return apiGet<ReferralAnalytics>('/referrals/admin/referrals/analytics');
}

// ─── Discovery ──────────────────────────────────────────────────────────────

export async function listDiscoveryScans(): Promise<DiscoveryScan[]> {
  const data = await apiGet<DiscoveryScan[] | { items: DiscoveryScan[] }>('/discovery/scans');
  return Array.isArray(data) ? data : (data as { items: DiscoveryScan[] }).items ?? [];
}

export async function createDiscoveryScan(payload: { name?: string; scan_type: string; target_range?: string }): Promise<DiscoveryScan> {
  return apiPost<DiscoveryScan>('/discovery/scans', payload);
}

export async function listDiscoveryResults(): Promise<DiscoveryResult[]> {
  const data = await apiGet<DiscoveryResult[] | { items: DiscoveryResult[] }>('/discovery/results');
  return Array.isArray(data) ? data : (data as { items: DiscoveryResult[] }).items ?? [];
}

export async function approveDiscoveryResult(id: number): Promise<unknown> {
  return apiPost(`/discovery/results/${id}/approve`, {});
}

// ─── Inventory ──────────────────────────────────────────────────────────────

export async function listInventory(): Promise<InventoryItem[]> {
  const data = await apiGet<InventoryItem[] | { items: InventoryItem[] }>('/inventory');
  return Array.isArray(data) ? data : (data as { items: InventoryItem[] }).items ?? [];
}

export interface CreateInventoryItemPayload {
  item_type: string;
  serial_number?: string;
  barcode?: string;
}

export async function createInventoryItem(payload: CreateInventoryItemPayload): Promise<InventoryItem> {
  return apiPost<InventoryItem>('/inventory', payload);
}

export async function assignInventoryItem(id: number, payload: { assigned_to: number; branch_id?: number }): Promise<unknown> {
  return apiPost(`/inventory/${id}/assign`, payload);
}

// ─── Documents ──────────────────────────────────────────────────────────────

export async function listDocuments(): Promise<DocumentRecord[]> {
  const data = await apiGet<DocumentRecord[] | { items: DocumentRecord[] }>('/documents');
  return Array.isArray(data) ? data : (data as { items: DocumentRecord[] }).items ?? [];
}

export async function listEntityDocuments(entityType: string, entityId: number): Promise<DocumentRecord[]> {
  const data = await apiGet<DocumentRecord[] | { items: DocumentRecord[] }>(`/documents/entity/${entityType}/${entityId}`);
  return Array.isArray(data) ? data : (data as { items: DocumentRecord[] }).items ?? [];
}

export async function getDocument(id: number): Promise<DocumentRecord> {
  return apiGet<DocumentRecord>(`/documents/${id}`);
}

export async function deleteDocument(id: number): Promise<unknown> {
  return apiSend('DELETE', `/documents/${id}`);
}

export interface PresignUploadPayload {
  filename: string;
  mime_type: string;
  file_size: number;
  bucket?: string;
  purpose?: string;
}

export async function presignUpload(payload: PresignUploadPayload): Promise<{ upload_url: string; key: string }> {
  return apiPost('/documents/presign-upload', payload);
}

export async function confirmUpload(payload: { key: string; name: string; entity_type?: string; entity_id?: number }): Promise<DocumentRecord> {
  return apiPost<DocumentRecord>('/documents/confirm', payload);
}

export async function getDocumentDownloadUrl(id: number): Promise<{ download_url: string }> {
  return apiGet(`/documents/${id}/download`);
}

// ─── Gateway (API keys & rate limits) ───────────────────────────────────────

export async function listApiKeys(): Promise<ApiKey[]> {
  const data = await apiGet<ApiKey[] | { items: ApiKey[] }>('/gateway/api-keys');
  return Array.isArray(data) ? data : (data as { items: ApiKey[] }).items ?? [];
}

export interface CreateApiKeyPayload {
  name: string;
  permissions: string;
  branch_id?: number;
  expires_at?: string;
}

export async function createApiKey(payload: CreateApiKeyPayload): Promise<ApiKey> {
  return apiPost<ApiKey>('/gateway/api-keys', payload);
}

export async function revokeApiKey(id: number): Promise<unknown> {
  return apiSend('DELETE', `/gateway/api-keys/${id}`);
}

export async function listRateLimitRules(): Promise<RateLimitRule[]> {
  const data = await apiGet<RateLimitRule[] | { items: RateLimitRule[] }>('/gateway/rate-limits');
  return Array.isArray(data) ? data : (data as { items: RateLimitRule[] }).items ?? [];
}

export interface CreateRateLimitRulePayload {
  route_pattern: string;
  methods: string;
  max_requests: number;
  window_seconds: number;
  role?: string;
  branch_id?: number;
}

export async function createRateLimitRule(payload: CreateRateLimitRulePayload): Promise<RateLimitRule> {
  return apiPost<RateLimitRule>('/gateway/rate-limits', payload);
}

export async function deleteRateLimitRule(id: number): Promise<unknown> {
  return apiSend('DELETE', `/gateway/rate-limits/${id}`);
}

export async function listRequestLogs(): Promise<RequestLog[]> {
  const data = await apiGet<RequestLog[] | { items: RequestLog[] }>('/gateway/logs');
  return Array.isArray(data) ? data : (data as { items: RequestLog[] }).items ?? [];
}

export async function getGatewayStats(): Promise<GatewayStats> {
  return apiGet<GatewayStats>('/gateway/stats');
}

