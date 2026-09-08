import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  fetchPublicPlans,
  checkCoverage,
  registerCustomer,
  customerLogin,
  customerLogin2fa,
  customerRefresh,
  getMe,
  getMySubscription,
  getMyInvoices,
  getMyTickets,
  createTicket,
  setCustomerTokenProvider,
  setCustomerUnauthorizedHandler,
} from '../customer';

const mockFetch = vi.fn();
vi.stubGlobal('fetch', mockFetch);

describe('customer API client', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setCustomerTokenProvider(() => null);
    setCustomerUnauthorizedHandler(() => {});
  });

  describe('fetchPublicPlans', () => {
    it('makes GET request to /plans', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify([{ id: 1, name: 'Basic' }]),
      });

      const result = await fetchPublicPlans();
      expect(result).toEqual([{ id: 1, name: 'Basic' }]);
      expect(mockFetch).toHaveBeenCalledTimes(1);
      const [url] = mockFetch.mock.calls[0];
      expect(url).toContain('/v1/plans');
    });
  });

  describe('checkCoverage', () => {
    it('makes POST request with pincode', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ available: true, area_name: 'Mumbai' }),
      });

      const result = await checkCoverage('400001');
      expect(result).toEqual({ available: true, area_name: 'Mumbai' });
      const [url, opts] = mockFetch.mock.calls[0];
      expect(url).toContain('/v1/coverage/check');
      expect(opts.body).toBe(JSON.stringify({ pincode: '400001' }));
    });
  });

  describe('customerLogin', () => {
    it('makes POST request with credentials', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ access_token: 'token', refresh_token: 'refresh' }),
      });

      const result = await customerLogin('test@test.com', 'password');
      expect(result.access_token).toBe('token');
      expect(result.refresh_token).toBe('refresh');
    });

    it('returns 2FA state when required', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ requires_2fa: true, pending_token: 'pending123' }),
      });

      const result = await customerLogin('test@test.com', 'password');
      expect(result.requires_2fa).toBe(true);
      expect(result.pending_token).toBe('pending123');
    });
  });

  describe('customerLogin2fa', () => {
    it('makes POST request with pending token and code', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ access_token: 'token', refresh_token: 'refresh' }),
      });

      const result = await customerLogin2fa('pending123', '123456');
      expect(result.access_token).toBe('token');
      const [, opts] = mockFetch.mock.calls[0];
      const body = JSON.parse(opts.body);
      expect(body.pending_token).toBe('pending123');
      expect(body.code).toBe('123456');
    });
  });

  describe('customerRefresh', () => {
    it('makes POST request with refresh token', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ access_token: 'new', refresh_token: 'new-refresh' }),
      });

      const result = await customerRefresh('old-refresh');
      expect(result.access_token).toBe('new');
    });
  });

  describe('getMe', () => {
    it('makes authenticated GET request', async () => {
      setCustomerTokenProvider(() => 'my-token');
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ id: 1, name: 'John' }),
      });

      const result = await getMe();
      expect(result).toEqual({ id: 1, name: 'John' });
      const [, opts] = mockFetch.mock.calls[0];
      expect(opts.headers['Authorization']).toBe('Bearer my-token');
    });
  });

  describe('getMySubscription', () => {
    it('returns first item from array response', async () => {
      setCustomerTokenProvider(() => 'token');
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify([{ id: 1, plan_name: 'Basic' }]),
      });

      const result = await getMySubscription();
      expect(result).toEqual({ id: 1, plan_name: 'Basic' });
    });

    it('returns first item from paginated response', async () => {
      setCustomerTokenProvider(() => 'token');
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ items: [{ id: 2, plan_name: 'Premium' }] }),
      });

      const result = await getMySubscription();
      expect(result).toEqual({ id: 2, plan_name: 'Premium' });
    });

    it('returns null when no subscriptions', async () => {
      setCustomerTokenProvider(() => 'token');
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify([]),
      });

      const result = await getMySubscription();
      expect(result).toBeNull();
    });
  });

  describe('getMyInvoices', () => {
    it('returns invoice array', async () => {
      setCustomerTokenProvider(() => 'token');
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify([{ id: 1, amount: 500 }]),
      });

      const result = await getMyInvoices();
      expect(result).toEqual([{ id: 1, amount: 500 }]);
    });
  });

  describe('getMyTickets', () => {
    it('returns ticket array', async () => {
      setCustomerTokenProvider(() => 'token');
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify([{ id: 1, subject: 'Issue' }]),
      });

      const result = await getMyTickets();
      expect(result).toEqual([{ id: 1, subject: 'Issue' }]);
    });
  });

  describe('createTicket', () => {
    it('makes POST request with ticket data', async () => {
      setCustomerTokenProvider(() => 'token');
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        text: async () => JSON.stringify({ id: 1, subject: 'Test' }),
      });

      const result = await createTicket({ subject: 'Test', category: 'billing' });
      expect(result.subject).toBe('Test');
      const [, opts] = mockFetch.mock.calls[0];
      expect(opts.method).toBe('POST');
    });
  });

  describe('error handling', () => {
    it('throws on network error', async () => {
      mockFetch.mockRejectedValueOnce(new TypeError('fetch failed'));

      await expect(fetchPublicPlans()).rejects.toThrow('Network error');
    });

    it('throws on non-ok response with error message', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 422,
        text: async () => JSON.stringify({ error: { message: 'Validation failed' } }),
      });

      await expect(fetchPublicPlans()).rejects.toThrow('Validation failed');
    });

    it('throws on non-ok response with string error', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 400,
        text: async () => JSON.stringify({ error: 'Bad request' }),
      });

      await expect(fetchPublicPlans()).rejects.toThrow('Bad request');
    });

    it('calls unauthorized handler on 401', async () => {
      const onUnauthorized = vi.fn();
      setCustomerUnauthorizedHandler(onUnauthorized);
      setCustomerTokenProvider(() => 'expired');

      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 401,
        text: async () => JSON.stringify({ error: 'Unauthorized' }),
      });

      await expect(getMe()).rejects.toThrow('Session expired');
      expect(onUnauthorized).toHaveBeenCalledTimes(1);
    });
  });
});
