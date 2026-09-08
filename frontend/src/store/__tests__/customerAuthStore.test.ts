import { describe, it, expect, beforeEach, vi } from 'vitest';

// Mock API module (prevent network calls during module init)
vi.mock('../../api/customer', () => ({
  customerLogin: vi.fn(),
  customerLogin2fa: vi.fn(),
  customerRefresh: vi.fn(),
  getMe: vi.fn(),
  setCustomerTokenProvider: vi.fn(),
  setCustomerRefreshHandler: vi.fn(),
  setCustomerUnauthorizedHandler: vi.fn(),
}));

import { useCustomerAuthStore } from '../customerAuthStore';
import { customerLogin, customerRefresh, getMe } from '../../api/customer';

const KEYS = {
  access: 'customer_access_token',
  refresh: 'customer_refresh_token',
  user: 'customer_user',
};

describe('customerAuthStore', () => {
  beforeEach(() => {
    sessionStorage.clear();
    vi.clearAllMocks();
    useCustomerAuthStore.setState({
      token: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
      requiresTwoFactor: false,
      pendingToken: null,
      loginError: null,
    });
  });

  describe('hydrate', () => {
    it('sets isAuthenticated to true when token exists', () => {
      sessionStorage.setItem(KEYS.access, 'test-token');
      sessionStorage.setItem(KEYS.refresh, 'test-refresh');
      useCustomerAuthStore.getState().hydrate();
      const state = useCustomerAuthStore.getState();
      expect(state.isAuthenticated).toBe(true);
      expect(state.token).toBe('test-token');
    });

    it('sets isAuthenticated to false when no token', () => {
      useCustomerAuthStore.getState().hydrate();
      const state = useCustomerAuthStore.getState();
      expect(state.isAuthenticated).toBe(false);
    });

    it('loads user from sessionStorage', () => {
      const testUser = { id: 1, name: 'Test User', email: 'test@test.com', phone: '123' };
      sessionStorage.setItem(KEYS.access, 'token');
      sessionStorage.setItem(KEYS.user, JSON.stringify(testUser));
      useCustomerAuthStore.getState().hydrate();
      expect(useCustomerAuthStore.getState().user).toEqual(testUser);
    });
  });

  describe('logout', () => {
    it('clears all tokens and user', () => {
      sessionStorage.setItem(KEYS.access, 'token');
      sessionStorage.setItem(KEYS.refresh, 'refresh');
      sessionStorage.setItem(KEYS.user, JSON.stringify({ id: 1 }));
      useCustomerAuthStore.setState({ isAuthenticated: true, token: 'token' });

      useCustomerAuthStore.getState().logout();

      const state = useCustomerAuthStore.getState();
      expect(state.isAuthenticated).toBe(false);
      expect(state.token).toBeNull();
      expect(state.user).toBeNull();
      expect(sessionStorage.getItem(KEYS.access)).toBeNull();
      expect(sessionStorage.getItem(KEYS.refresh)).toBeNull();
      expect(sessionStorage.getItem(KEYS.user)).toBeNull();
    });
  });

  describe('login', () => {
    it('sets error state on failure', async () => {
      vi.mocked(customerLogin).mockRejectedValue(new Error('Bad credentials'));

      try {
        await useCustomerAuthStore.getState().login('a@b.com', 'pass');
      } catch { /* expected */ }
      expect(useCustomerAuthStore.getState().loginError).toBe('Bad credentials');
    });

    it('returns "ok" and stores tokens on success', async () => {
      vi.mocked(customerLogin).mockResolvedValue({
        access_token: 'new-access',
        refresh_token: 'new-refresh',
        user: { id: 1, name: 'John' },
      });
      vi.mocked(getMe).mockResolvedValue({ id: 1, name: 'John' } as any);

      const result = await useCustomerAuthStore.getState().login('a@b.com', 'pass');
      expect(result).toBe('ok');
      expect(useCustomerAuthStore.getState().isAuthenticated).toBe(true);
      expect(sessionStorage.getItem(KEYS.access)).toBe('new-access');
      expect(sessionStorage.getItem(KEYS.refresh)).toBe('new-refresh');
    });

    it('returns "2fa" when 2FA is required', async () => {
      vi.mocked(customerLogin).mockResolvedValue({
        requires_2fa: true,
        pending_token: 'pending-xyz',
      });

      const result = await useCustomerAuthStore.getState().login('a@b.com', 'pass');
      expect(result).toBe('2fa');
      expect(useCustomerAuthStore.getState().requiresTwoFactor).toBe(true);
      expect(useCustomerAuthStore.getState().pendingToken).toBe('pending-xyz');
    });
  });

  describe('refreshSession', () => {
    it('returns false when no refresh token', async () => {
      const result = await useCustomerAuthStore.getState().refreshSession();
      expect(result).toBe(false);
    });

    it('refreshes and updates tokens on success', async () => {
      sessionStorage.setItem(KEYS.refresh, 'old-refresh');
      useCustomerAuthStore.setState({ refreshToken: 'old-refresh' });

      vi.mocked(customerRefresh).mockResolvedValue({
        access_token: 'refreshed-access',
        refresh_token: 'refreshed-refresh',
      });

      const result = await useCustomerAuthStore.getState().refreshSession();
      expect(result).toBe(true);
      expect(useCustomerAuthStore.getState().token).toBe('refreshed-access');
      expect(useCustomerAuthStore.getState().isAuthenticated).toBe(true);
    });

    it('logs out on refresh failure', async () => {
      sessionStorage.setItem(KEYS.refresh, 'bad-refresh');
      useCustomerAuthStore.setState({ refreshToken: 'bad-refresh', isAuthenticated: true });

      vi.mocked(customerRefresh).mockRejectedValue(new Error('expired'));

      const result = await useCustomerAuthStore.getState().refreshSession();
      expect(result).toBe(false);
      expect(useCustomerAuthStore.getState().isAuthenticated).toBe(false);
    });
  });
});
