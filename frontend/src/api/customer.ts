// ─── Customer Portal API Client ──────────────────────────────────────────────
// Thin fetch wrapper for customer-facing endpoints.

const API_BASE = (import.meta.env.VITE_API_URL || '/api') + '/v1';

let _tokenProvider: (() => string | null) | null = null;
let _onUnauthorized: (() => void) | null = null;
let _refreshHandler: (() => Promise<boolean>) | null = null;

export function setCustomerTokenProvider(fn: () => string | null) {
  _tokenProvider = fn;
}
export function setCustomerUnauthorizedHandler(fn: () => void) {
  _onUnauthorized = fn;
}
export function setCustomerRefreshHandler(fn: () => Promise<boolean>) {
  _refreshHandler = fn;
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
): Promise<T> {
  // The refresh endpoint must never trigger another refresh attempt —
  // an expired refresh token would otherwise recurse forever.
  const isRefreshCall = path === '/auth/refresh';

  const doFetch = async (): Promise<Response> => {
    const token = _tokenProvider?.();
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (token) headers['Authorization'] = `Bearer ${token}`;
    try {
      return await fetch(`${API_BASE}${path}`, {
        method,
        headers,
        body: body !== undefined ? JSON.stringify(body) : undefined,
      });
    } catch {
      throw new Error('Network error — cannot reach the server');
    }
  };

  let resp = await doFetch();

  if (resp.status === 401) {
    // Attempt a silent session refresh before forcing a sign-out.
    const refreshed = !isRefreshCall && _refreshHandler ? await _refreshHandler() : false;
    if (refreshed) {
      resp = await doFetch();
    }
    if (resp.status === 401) {
      _onUnauthorized?.();
      throw new Error('Session expired. Please sign in again.');
    }
  }

  const text = await resp.text();
  if (!resp.ok) {
    let message = `Request failed (${resp.status})`;
    try {
      const parsed = JSON.parse(text);
      if (parsed?.error) message = typeof parsed.error === 'string' ? parsed.error : parsed.error.message ?? message;
      else if (parsed?.message) message = parsed.message;
    } catch { /* keep generic */ }
    throw new Error(message);
  }

  if (!text) return undefined as T;
  return JSON.parse(text) as T;
}

// ─── Public (no auth) ────────────────────────────────────────────────────────

export async function fetchPublicPlans() {
  return request<Array<Record<string, unknown>>>('GET', '/plans');
}

export async function checkCoverage(pincode: string) {
  return request<{ available: boolean; area_name?: string; message?: string }>('POST', '/coverage/check', { pincode });
}

export async function registerCustomer(payload: {
  name: string;
  phone: string;
  email?: string;
  password: string;
  pincode?: string;
  address?: string;
  plan_id?: number;
}) {
  return request<{ access_token: string; refresh_token: string; user: Record<string, unknown> }>('POST', '/auth/register', payload);
}

export async function customerLogin(email: string, password: string) {
  return request<{ access_token?: string; refresh_token?: string; requires_2fa?: boolean; pending_token?: string; user?: Record<string, unknown> }>('POST', '/auth/login', { email, password });
}

export async function customerLogin2fa(pendingToken: string, code: string) {
  return request<{ access_token: string; refresh_token: string; user: Record<string, unknown> }>('POST', '/auth/login/2fa', { pending_token: pendingToken, code });
}

export async function customerRefresh(refreshToken: string) {
  return request<{ access_token: string; refresh_token: string }>('POST', '/auth/refresh', { refresh_token: refreshToken });
}

// ─── Authenticated ───────────────────────────────────────────────────────────

export async function getMe() {
  return request<Record<string, unknown>>('GET', '/users/me');
}

export async function getMySubscription() {
  const data = await request<Record<string, unknown> | { items: Record<string, unknown>[] }>('GET', '/subscriptions');
  const items = Array.isArray(data) ? data : (data as { items?: Record<string, unknown>[] }).items ?? [];
  return items[0] ?? null;
}

export async function getMyInvoices() {
  const data = await request<Record<string, unknown> | { items: Record<string, unknown>[] }>('GET', '/billing/invoices');
  return Array.isArray(data) ? data : (data as { items?: Record<string, unknown>[] }).items ?? [];
}

export async function getMyInvoice(id: number) {
  return request<Record<string, unknown>>(`GET`, `/billing/invoices/${id}`);
}

export async function createPaymentLink(invoiceId: number) {
  return request<{ payment_url: string; order_id: string }>('POST', '/payments/create-link', { invoice_id: invoiceId });
}

export async function getMyTickets() {
  const data = await request<Record<string, unknown> | { items: Record<string, unknown>[] }>('GET', '/tickets');
  return Array.isArray(data) ? data : (data as { items?: Record<string, unknown>[] }).items ?? [];
}

export async function createTicket(payload: { subject: string; category?: string; priority?: string; description?: string }) {
  return request<Record<string, unknown>>('POST', '/tickets', payload);
}

export async function getTicketComments(ticketId: number) {
  const data = await request<Record<string, unknown> | { items: Record<string, unknown>[] }>(`GET`, `/tickets/${ticketId}/comments`);
  return Array.isArray(data) ? data : (data as { items?: Record<string, unknown>[] }).items ?? [];
}

export async function addTicketComment(ticketId: number, content: string) {
  return request<Record<string, unknown>>('POST', `/tickets/${ticketId}/comments`, { content });
}

export async function getBandwidthUsage(subscriptionId: number) {
  return request<Record<string, unknown>>(`GET`, `/bandwidth/usage/${subscriptionId}`);
}
