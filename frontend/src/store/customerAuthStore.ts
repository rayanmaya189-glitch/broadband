import { create } from 'zustand';
import type { CustomerUser } from '../types';
import {
  customerLogin,
  customerLogin2fa,
  customerRefresh,
  getMe,
  setCustomerTokenProvider,
  setCustomerRefreshHandler,
  setCustomerUnauthorizedHandler,
} from '../api/customer';

// Session-scoped storage (matches the admin store): tokens live for the tab
// lifetime only, which shrinks the XSS exposure window. Customers sign in
// again when they reopen the portal.
const KEYS = {
  access: 'customer_access_token',
  refresh: 'customer_refresh_token',
  user: 'customer_user',
} as const;

interface CustomerAuthState {
  token: string | null;
  refreshToken: string | null;
  user: CustomerUser | null;
  isAuthenticated: boolean;
  requiresTwoFactor: boolean;
  pendingToken: string | null;
  loginError: string | null;

  hydrate: () => void;
  login: (email: string, password: string) => Promise<'2fa' | 'ok'>;
  login2fa: (code: string) => Promise<void>;
  logout: () => void;
  refreshSession: () => Promise<boolean>;
}

function decodeJwtPayload(token: string): Record<string, unknown> | null {
  try {
    const base64 = token.split('.')[1];
    const json = atob(base64.replace(/-/g, '+').replace(/_/g, '/'));
    return JSON.parse(json);
  } catch {
    return null;
  }
}

export const useCustomerAuthStore = create<CustomerAuthState>((set, get) => ({
  token: null,
  refreshToken: null,
  user: null,
  isAuthenticated: false,
  requiresTwoFactor: false,
  pendingToken: null,
  loginError: null,

  hydrate: () => {
    const token = sessionStorage.getItem(KEYS.access);
    const refreshToken = sessionStorage.getItem(KEYS.refresh);
    let user: CustomerUser | null = null;
    try {
      const raw = sessionStorage.getItem(KEYS.user);
      if (raw) user = JSON.parse(raw) as CustomerUser;
    } catch { user = null; }

    if (token) {
      set({ token, refreshToken, user, isAuthenticated: true });
    }
  },

  login: async (email, password) => {
    set({ loginError: null });
    try {
      const res = await customerLogin(email, password);
      if (res.requires_2fa) {
        set({ requiresTwoFactor: true, pendingToken: res.pending_token ?? null });
        return '2fa';
      }
      if (!res.access_token || !res.refresh_token) {
        set({ loginError: 'Login succeeded but returned no session.' });
        return 'ok';
      }
      sessionStorage.setItem(KEYS.access, res.access_token);
      sessionStorage.setItem(KEYS.refresh, res.refresh_token);
      const user = res.user as unknown as CustomerUser;
      if (user) sessionStorage.setItem(KEYS.user, JSON.stringify(user));
      set({
        token: res.access_token,
        refreshToken: res.refresh_token,
        user: user ?? null,
        isAuthenticated: true,
        requiresTwoFactor: false,
        pendingToken: null,
        loginError: null,
      });
      // Fetch fresh profile
      try { const fresh = await getMe() as unknown as CustomerUser; sessionStorage.setItem(KEYS.user, JSON.stringify(fresh)); set({ user: fresh }); } catch { /* ok */ }
      return 'ok';
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Login failed';
      set({ loginError: msg });
      throw e;
    }
  },

  login2fa: async (code) => {
    const { pendingToken } = get();
    if (!pendingToken) { set({ loginError: 'Session missing. Sign in again.' }); return; }
    set({ loginError: null });
    try {
      const res = await customerLogin2fa(pendingToken, code);
      if (!res.access_token || !res.refresh_token) { set({ loginError: 'Verification returned no session.' }); return; }
      sessionStorage.setItem(KEYS.access, res.access_token);
      sessionStorage.setItem(KEYS.refresh, res.refresh_token);
      const user = res.user as unknown as CustomerUser;
      if (user) sessionStorage.setItem(KEYS.user, JSON.stringify(user));
      set({ token: res.access_token, refreshToken: res.refresh_token, user: user ?? null, isAuthenticated: true, requiresTwoFactor: false, pendingToken: null, loginError: null });
    } catch (e) {
      set({ loginError: e instanceof Error ? e.message : 'Verification failed' });
      throw e;
    }
  },

  logout: () => {
    sessionStorage.removeItem(KEYS.access);
    sessionStorage.removeItem(KEYS.refresh);
    sessionStorage.removeItem(KEYS.user);
    set({ token: null, refreshToken: null, user: null, isAuthenticated: false, requiresTwoFactor: false, pendingToken: null, loginError: null });
  },

  refreshSession: async () => {
    const { refreshToken } = get();
    if (!refreshToken) return false;
    try {
      const res = await customerRefresh(refreshToken);
      sessionStorage.setItem(KEYS.access, res.access_token);
      sessionStorage.setItem(KEYS.refresh, res.refresh_token);
      set({ token: res.access_token, refreshToken: res.refresh_token, isAuthenticated: true });
      return true;
    } catch {
      get().logout();
      return false;
    }
  },
}));

// Wire up API client
setCustomerTokenProvider(() => useCustomerAuthStore.getState().token);
setCustomerRefreshHandler(() => useCustomerAuthStore.getState().refreshSession());
setCustomerUnauthorizedHandler(() => useCustomerAuthStore.getState().logout());

// Hydrate on import
useCustomerAuthStore.getState().hydrate();
