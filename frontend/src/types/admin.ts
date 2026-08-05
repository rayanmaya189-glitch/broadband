// ─── Admin Portal shared types ──────────────────────────────────────────────

export type AdminRole =
  | 'super_admin'
  | 'isp_owner'
  | 'network_admin'
  | 'noc_engineer'
  | 'field_technician'
  | 'customer_support'
  | 'sales_agent'
  | 'finance_manager'
  | 'billing_operator'
  | 'customer'
  | string;

export interface JwtClaims {
  sub: string;
  email: string;
  role: string;
  branch_id: number | null;
  is_company_wide: boolean;
  iat?: number;
  exp?: number;
}

export interface AdminUser {
  id: number;
  email: string;
  phone: string;
  name: string;
  avatar_url?: string | null;
  branch_id?: number | null;
  status: string;
  last_login_at?: string | null;
}

export interface AdminSession {
  accessToken: string;
  refreshToken: string;
  user: AdminUser | null;
  role: string;
  branchId: number | null;
  isCompanyWide: boolean;
}

export interface AuthLoginResponse {
  requires_2fa: boolean;
  access_token?: string | null;
  refresh_token?: string | null;
  user?: AdminUser | null;
  pending_token?: string | null;
  message?: string | null;
}

export interface Paginated<T> {
  items: T[];
  total: number;
  page: number;
  limit: number;
}

export interface Branch {
  id: number;
  name: string;
  code?: string;
  city?: string;
  state?: string;
  status?: string;
  is_head_office?: boolean;
  parent_branch_id?: number | null;
}

export interface Plan {
  id: number;
  name: string;
  slug?: string;
  status: string;
  description?: string;
  is_published?: boolean;
  created_at?: string;
  pricing?: PlanPrice[];
}

export interface PlanPrice {
  id?: number;
  plan_id?: number;
  billing_period_months: number;
  monthly_price: number;
  setup_fee?: number;
  is_active?: boolean;
}

export interface Subscription {
  id: number;
  customer_id: number;
  customer_name?: string;
  plan_id: number;
  plan_name?: string;
  status: string;
  start_date?: string;
  expiry_date?: string;
  monthly_fee?: number;
}

export interface Invoice {
  id: number;
  invoice_number: string;
  customer_id?: number;
  customer_name?: string;
  status: string;
  amount: number;
  tax_amount?: number;
  total_amount?: number;
  due_date?: string;
  issued_at?: string;
}

export interface Payment {
  id: number;
  invoice_id?: number;
  customer_id?: number;
  customer_name?: string;
  amount: number;
  method?: string;
  status?: string;
  paid_at?: string;
}

export interface Ticket {
  id: number;
  ticket_number?: string;
  customer_id?: number;
  customer_name?: string;
  subject: string;
  priority: string;
  status: string;
  category?: string;
  assignee_id?: number;
  assignee_name?: string;
  created_at?: string;
}

export interface Lead {
  id: number;
  name: string;
  phone?: string;
  email?: string;
  status: string;
  source?: string;
  assigned_to?: number;
  assigned_to_name?: string;
  created_at?: string;
}

export interface NetworkDevice {
  id: number;
  name: string;
  device_type: string;
  ip_address?: string;
  status: string;
  branch_id?: number;
  model?: string;
  firmware_version?: string;
  mac_address?: string;
}

export interface IpPool {
  id: number;
  name?: string;
  subnet?: string;
  gateway?: string;
  total_addresses?: number;
  used_addresses?: number;
  status?: string;
}

export interface Vlan {
  id: number;
  vlan_id: number;
  name: string;
  subnet?: string;
  description?: string;
}

export interface MonitoringAlert {
  id: number;
  device_id?: number;
  device_name?: string;
  severity: string;
  metric?: string;
  message?: string;
  status: string;
  triggered_at?: string;
}

export interface Installation {
  id: number;
  customer_id?: number;
  customer_name?: string;
  status: string;
  scheduled_at?: string;
  technician_id?: number;
  technician_name?: string;
  address?: string;
}

export interface ApprovalRequest {
  id: number;
  operation: string;
  entity_type?: string;
  entity_id?: string;
  requested_by?: string;
  requested_by_name?: string;
  status: string;
  reason?: string;
  created_at?: string;
}

export interface Notification {
  id: number;
  channel: string;
  recipient?: string;
  recipient_name?: string;
  subject?: string;
  body?: string;
  status: string;
  created_at?: string;
}

export interface UserAccount {
  id: number;
  email: string;
  phone: string;
  name: string;
  status: string;
  branch_id?: number | null;
  last_login_at?: string | null;
}

export interface Role {
  id: number;
  name: string;
  slug: string;
  description?: string;
  is_system?: boolean;
  permissions_count?: number;
}

export interface Permission {
  id: number;
  name: string;
  module: string;
  resource?: string;
  action?: string;
  description?: string;
}

export interface AuditLog {
  id: number;
  user_id?: number;
  user_name?: string;
  action: string;
  entity_type?: string;
  entity_id?: string;
  ip_address?: string;
  created_at?: string;
}

export interface AccountingAccount {
  id: number;
  name: string;
  code?: string;
  account_type: string;
  balance?: number;
  status?: string;
}

export interface JournalEntry {
  id: number;
  entry_number?: string;
  description?: string;
  entry_type?: string;
  amount?: number;
  status?: string;
  entry_date?: string;
}

export interface CoverageArea {
  id: number;
  name: string;
  type?: string;
  status?: string;
  pincodes?: string[];
}

export interface MetricPoint {
  timestamp?: string;
  metric?: string;
  value?: number;
}

export interface DeviceMetricRow {
  id: number;
  device_id?: number;
  device_name?: string;
  cpu_load_percent?: number;
  memory_used_percent?: number;
  uptime_seconds?: number;
  recorded_at?: string;
}
