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
  speed_label?: string;
  download_mbps?: number;
  upload_mbps?: number;
  burst_mbps?: number;
  is_business?: boolean;
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
  branch_id?: number;
  billing_period_months?: number;
  status: string;
  start_date?: string;
  expiry_date?: string;
  next_billing_date?: string;
  auto_renew?: boolean;
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
  payment_method?: string;
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
  cidr?: string;
  subnet?: string;
  gateway?: string;
  total_count?: number;
  allocated_count?: number;
  total_addresses?: number;
  used_addresses?: number;
  status?: string;
}

export interface Vlan {
  id: number;
  vlan_id: number;
  name: string;
  vlan_type?: string;
  branch_id?: number;
  subnet?: string;
  description?: string;
}

export interface MonitoringAlert {
  id: number;
  device_id?: number;
  device_name?: string;
  severity: string;
  title?: string;
  metric?: string;
  message?: string;
  status: string;
  triggered_at?: string;
  created_at?: string;
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
  area_type?: string;
  type?: string;
  status?: string;
  is_active?: boolean;
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

// ─── Billing extensions ─────────────────────────────────────────────────────

export interface Discount {
  id: number;
  name: string;
  type: string;
  value: number;
  is_active?: boolean;
  valid_from?: string;
  valid_until?: string;
}

export interface Refund {
  id: number;
  payment_id?: number;
  invoice_id?: number;
  customer_name?: string;
  amount: number;
  status: string;
  reason?: string;
  requested_at?: string;
}

export interface InvoiceItem {
  id: number;
  invoice_id: number;
  description: string;
  quantity: number;
  unit_price: number;
  amount: number;
}

// ─── Ticket extensions ──────────────────────────────────────────────────────

export interface TicketComment {
  id: number;
  ticket_id: number;
  user_name?: string;
  content: string;
  is_internal?: boolean;
  created_at?: string;
}

export interface TicketMetrics {
  total: number;
  open: number;
  in_progress: number;
  resolved: number;
  closed: number;
  avg_resolution_hours?: number;
}

// ─── Subscription extensions ────────────────────────────────────────────────

export interface SubscriptionHistory {
  id: number;
  subscription_id: number;
  action: string;
  old_value?: string;
  new_value?: string;
  performed_by_name?: string;
  created_at?: string;
}

// ─── Network extensions ─────────────────────────────────────────────────────

export interface PppoeSession {
  id: number;
  session_id?: string;
  customer_name?: string;
  username?: string;
  ip_address?: string;
  nas_ip?: string;
  status: string;
  uptime_seconds?: number;
  bytes_in?: number;
  bytes_out?: number;
  started_at?: string;
}

export interface MacBinding {
  id: number;
  mac_address: string;
  ip_address?: string;
  customer_name?: string;
  port?: string;
  is_active?: boolean;
  created_at?: string;
}

// ─── Monitoring extensions ──────────────────────────────────────────────────

export interface AlertStats {
  total_firing: number;
  total_acknowledged: number;
  total_resolved: number;
  avg_acknowledgment_minutes?: number;
}

export interface AlertRule {
  id: number;
  name: string;
  metric: string;
  condition: string;
  threshold: number;
  severity: string;
  is_active?: boolean;
  notify_channels?: string[];
}

// ─── Accounting extensions ──────────────────────────────────────────────────

export interface JournalEntryLine {
  account_id: number;
  account_name?: string;
  debit?: number;
  credit?: number;
}

export interface ProfitAndLoss {
  revenue: { account: string; amount: number }[];
  expenses: { account: string; amount: number }[];
  total_revenue: number;
  total_expenses: number;
  net_profit: number;
  period_start?: string;
  period_end?: string;
}

export interface BalanceSheet {
  assets: { account: string; amount: number }[];
  liabilities: { account: string; amount: number }[];
  equity: { account: string; amount: number }[];
  total_assets: number;
  total_liabilities: number;
  total_equity: number;
}

export interface GstinReturn {
  period?: string;
  total_taxable: number;
  cgst: number;
  sgst: number;
  igst: number;
  total: number;
  filing_status?: string;
}

// ─── Branch extensions ──────────────────────────────────────────────────────

export interface WorkingHours {
  branch_id: number;
  day_of_week: number;
  open_time: string;
  close_time: string;
  is_closed?: boolean;
}

export interface BranchStats {
  total_customers: number;
  active_subscriptions: number;
  total_users: number;
  total_devices: number;
}

// ─── Notification extensions ────────────────────────────────────────────────

export interface NotificationTemplate {
  id: number;
  name: string;
  channel: string;
  subject?: string;
  subject_template?: string;
  body: string;
  body_template?: string;
  is_active?: boolean;
  created_at?: string;
}

export interface NotificationChannel {
  id: number;
  name: string;
  type: string;
  is_enabled?: boolean;
  config?: Record<string, unknown>;
}

// ─── Coverage extensions ────────────────────────────────────────────────────

export interface CoverageCheck {
  available: boolean;
  area_name?: string;
  message?: string;
}

// ─── Compliance extensions ──────────────────────────────────────────────────

export interface KycVerification {
  id: number;
  customer_id: number;
  customer_name?: string;
  document_type: string;
  document_number?: string;
  status: string;
  verified_at?: string;
  created_at?: string;
}

// ─── Audit extensions ───────────────────────────────────────────────────────

export interface AuditEvent {
  id: number;
  event_type: string;
  entity_type?: string;
  entity_id?: string;
  payload?: Record<string, unknown>;
  created_at?: string;
}

// ─── Bandwidth extensions ───────────────────────────────────────────────────

export interface BandwidthProfile {
  id: number;
  name: string;
  description?: string;
  download_kbps: number;
  upload_kbps: number;
  burst_download_kbps?: number;
  burst_upload_kbps?: number;
  burst_threshold_download_kbps?: number;
  burst_threshold_upload_kbps?: number;
  burst_time_seconds?: number;
  priority?: number;
  is_active?: boolean;
  created_at?: string;
}

export interface BandwidthPolicy {
  id: number;
  name: string;
  description?: string;
  type: string;
  config?: Record<string, unknown>;
  is_active?: boolean;
  created_at?: string;
}

export interface BandwidthUsage {
  subscription_id: number;
  customer_name?: string;
  profile_name?: string;
  download_kbps?: number;
  upload_kbps?: number;
  bytes_downloaded?: number;
  bytes_uploaded?: number;
  period_start?: string;
  period_end?: string;
}

export interface BandwidthApplication {
  id: number;
  subscription_id: number;
  profile_id: number;
  profile_name?: string;
  customer_name?: string;
  applied_at?: string;
  status?: string;
}

// ─── Scheduler extensions ───────────────────────────────────────────────────

export interface SchedulerJob {
  id: number;
  name: string;
  description?: string;
  job_type: string;
  schedule?: string;
  cron_expression?: string;
  interval_seconds?: number;
  target_module?: string;
  action?: string;
  payload?: Record<string, unknown>;
  is_active?: boolean;
  last_run_at?: string;
  next_run_at?: string;
  last_status?: string;
  created_at?: string;
}

export interface SchedulerExecution {
  id: number;
  job_id: number;
  job_name?: string;
  status: string;
  started_at?: string;
  finished_at?: string;
  duration_ms?: number;
  error_message?: string;
  result?: Record<string, unknown>;
}

export interface SchedulerStats {
  total_jobs: number;
  active_jobs: number;
  total_executions: number;
  successful_executions: number;
  failed_executions: number;
  avg_duration_ms?: number;
}

// ─── Compliance extensions ──────────────────────────────────────────────────

export interface Consent {
  id: number;
  customer_id: number;
  customer_name?: string;
  consent_type: string;
  granted: boolean;
  granted_at?: string;
  revoked_at?: string;
  ip_address?: string;
}

export interface RetentionPolicy {
  id: number;
  name: string;
  entity_type: string;
  retention_days: number;
  action: string;
  is_active?: boolean;
  created_at?: string;
}

// ─── Referral extensions ────────────────────────────────────────────────────

export interface ReferralProgram {
  id: number;
  name: string;
  description?: string;
  reward_type: string;
  reward_value?: number;
  reward_amount: number;
  referrer_reward_amount?: number;
  is_active?: boolean;
  valid_from?: string;
  valid_until?: string;
  created_at?: string;
}

export interface ReferralRecord {
  id: number;
  referrer_name?: string;
  referrer_id?: number;
  referred_name?: string;
  referred_id?: number;
  program_name?: string;
  status: string;
  reward_amount?: number;
  created_at?: string;
}

export interface ReferralWallet {
  id: number;
  customer_id: number;
  customer_name?: string;
  balance: number;
  total_earned: number;
  total_redeemed: number;
}

export interface ReferralAnalytics {
  total_referrals: number;
  successful_conversions: number;
  pending_rewards: number;
  total_rewards_paid: number;
  conversion_rate: number;
}

// ─── Discovery extensions ───────────────────────────────────────────────────

export interface DiscoveryScan {
  id: number;
  name?: string;
  scan_type: string;
  target_range?: string;
  status: string;
  devices_found: number;
  started_at?: string;
  completed_at?: string;
  created_at?: string;
}

export interface DiscoveryResult {
  id: number;
  scan_id: number;
  ip_address: string;
  mac_address?: string;
  hostname?: string;
  device_type?: string;
  manufacturer?: string;
  status: string;
  discovered_at?: string;
}

// ─── Inventory extensions ───────────────────────────────────────────────────

export interface InventoryItem {
  id: number;
  name?: string;
  item_type?: string;
  description?: string;
  category?: string;
  serial_number?: string;
  model?: string;
  status: string;
  assigned_to_name?: string;
  assigned_to_id?: number;
  branch_name?: string;
  created_at?: string;
}

// ─── Document extensions ────────────────────────────────────────────────────

export interface DocumentRecord {
  id: number;
  name: string;
  file_name?: string;
  content_type?: string;
  size_bytes?: number;
  entity_type?: string;
  entity_id?: number;
  uploaded_by_name?: string;
  status?: string;
  created_at?: string;
}

// ─── Gateway extensions ─────────────────────────────────────────────────────

export interface ApiKey {
  id: number;
  name: string;
  key_prefix?: string;
  permissions?: string;
  scopes?: string[];
  is_active?: boolean;
  expires_at?: string;
  last_used_at?: string;
  created_at?: string;
}

export interface RateLimitRule {
  id: number;
  name?: string;
  route_pattern: string;
  methods?: string;
  max_requests: number;
  window_seconds: number;
  role?: string | null;
  is_active?: boolean;
  created_at?: string;
}

export interface RequestLog {
  id: number;
  method: string;
  path: string;
  status_code: number;
  ip_address?: string;
  user_agent?: string;
  duration_ms?: number;
  created_at?: string;
}

export interface GatewayStats {
  total_requests: number;
  successful_requests: number;
  failed_requests: number;
  avg_response_time_ms: number;
  active_api_keys: number;
  rate_limited_requests: number;
}
