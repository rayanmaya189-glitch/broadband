import { create } from 'zustand';
import type { AdminUser } from '../types/admin';
import {
  adminLogin,
  adminLogin2fa,
  adminLogout,
  adminMe,
  adminRefresh,
} from '../api/admin/auth';
import { setAdminTokenProvider, setAdminUnauthorizedHandler, setAdminRefreshHandler } from '../api/admin/client';
import { decodeJwt } from '../admin/lib/jwt';

const KEYS = {
  access: 'admin_access_token',
  refresh: 'admin_refresh_token',
  user: 'admin_user',
  role: 'admin_role',
  branchId: 'admin_branch_id',
  companyWide: 'admin_company_wide',
} as const;

interface AdminState {
  accessToken: string | null;
  refreshToken: string | null;
  user: AdminUser | null;
  role: string | null;
  branchId: number | null;
  isCompanyWide: boolean;
  authenticated: boolean;
  requiresTwoFactor: boolean;
  pendingToken: string | null;
  loginError: string | null;

  hydrate: () => void;
  login: (email: string, password: string) => Promise<'2fa' | 'ok'>;
  login2fa: (code: string) => Promise<void>;
  logout: () => Promise<void>;
  refreshSession: () => Promise<boolean>;
  setUser: (user: AdminUser) => void;
}

function readNumber(key: string): number | null {
  const raw = sessionStorage.getItem(key);
  if (!raw) return null;
  const n = Number(raw);
  return Number.isNaN(n) ? null : n;
}

export const useAdminStore = create<AdminState>((set, get) => {
  function applyTokens(accessToken: string | null, refreshToken: string | null) {
    let role: string | null = null;
    let branchId: number | null = null;
    let isCompanyWide = false;
    if (accessToken) {
      const claims = decodeJwt(accessToken);
      role = claims?.role ?? null;
      branchId = claims?.branch_id ?? null;
      isCompanyWide = claims?.is_company_wide ?? false;
    }
    if (accessToken) sessionStorage.setItem(KEYS.access, accessToken);
    else sessionStorage.removeItem(KEYS.access);
    if (refreshToken) sessionStorage.setItem(KEYS.refresh, refreshToken);
    else sessionStorage.removeItem(KEYS.refresh);
    sessionStorage.setItem(KEYS.role, role ?? '');
    sessionStorage.setItem(KEYS.branchId, String(branchId ?? ''));
    sessionStorage.setItem(KEYS.companyWide, String(isCompanyWide));
    return { role, branchId, isCompanyWide };
  }

  function persistUser(user: AdminUser | null) {
    if (user) sessionStorage.setItem(KEYS.user, JSON.stringify(user));
    else sessionStorage.removeItem(KEYS.user);
  }

  async function loadProfile(): Promise<AdminUser | null> {
    try {
      const user = await adminMe();
      persistUser(user);
      set({ user });
      return user;
    } catch {
      return null;
    }
  }

  return {
    accessToken: null,
    refreshToken: null,
    user: null,
    role: null,
    branchId: null,
    isCompanyWide: false,
    authenticated: false,
    requiresTwoFactor: false,
    pendingToken: null,
    loginError: null,

    hydrate: () => {
      const accessToken = sessionStorage.getItem(KEYS.access);
      const refreshToken = sessionStorage.getItem(KEYS.refresh);
      let user: AdminUser | null = null;
      try {
        const raw = sessionStorage.getItem(KEYS.user);
        if (raw) user = JSON.parse(raw) as AdminUser;
      } catch {
        user = null;
      }
      const { role, branchId, isCompanyWide } = applyTokens(accessToken, refreshToken);
      set({
        accessToken,
        refreshToken,
        user,
        role,
        branchId,
        isCompanyWide,
        authenticated: Boolean(accessToken),
      });
      if (accessToken) void loadProfile();
    },

    login: async (email, password) => {
      set({ loginError: null });
      try {
        const res = await adminLogin({ email, password });
        if (res.requires_2fa) {
          set({ requiresTwoFactor: true, pendingToken: res.pending_token ?? null });
          return '2fa';
        }
        if (!res.access_token || !res.refresh_token) {
          set({ loginError: 'Login succeeded but returned no session tokens.' });
          return 'ok';
        }
        const claims = applyTokens(res.access_token, res.refresh_token);
        persistUser(res.user ?? null);
        set({
          accessToken: res.access_token,
          refreshToken: res.refresh_token,
          user: res.user ?? null,
          role: claims.role,
          branchId: claims.branchId,
          isCompanyWide: claims.isCompanyWide,
          authenticated: true,
          requiresTwoFactor: false,
          pendingToken: null,
          loginError: null,
        });
        void loadProfile();
        return 'ok';
      } catch (e) {
        const message = e instanceof Error ? e.message : 'Sign in failed';
        set({ loginError: message });
        throw e;
      }
    },

    login2fa: async (code) => {
      const { pendingToken } = get();
      if (!pendingToken) {
        set({ loginError: 'Two-factor session missing. Please sign in again.' });
        return;
      }
      set({ loginError: null });
      try {
        const res = await adminLogin2fa({ pending_token: pendingToken, code });
        if (!res.access_token || !res.refresh_token) {
          set({ loginError: 'Two-factor verification returned no session.' });
          return;
        }
        const claims = applyTokens(res.access_token, res.refresh_token);
        persistUser(res.user ?? null);
        set({
          accessToken: res.access_token,
          refreshToken: res.refresh_token,
          user: res.user ?? null,
          role: claims.role,
          branchId: claims.branchId,
          isCompanyWide: claims.isCompanyWide,
          authenticated: true,
          requiresTwoFactor: false,
          pendingToken: null,
          loginError: null,
        });
        void loadProfile();
      } catch (e) {
        const message = e instanceof Error ? e.message : 'Verification failed';
        set({ loginError: message });
        throw e;
      }
    },

    logout: async () => {
      const hadToken = Boolean(get().accessToken);
      if (hadToken) await adminLogout().catch(() => undefined);
      sessionStorage.removeItem(KEYS.access);
      sessionStorage.removeItem(KEYS.refresh);
      sessionStorage.removeItem(KEYS.user);
      sessionStorage.removeItem(KEYS.role);
      sessionStorage.removeItem(KEYS.branchId);
      sessionStorage.removeItem(KEYS.companyWide);
      set({
        accessToken: null,
        refreshToken: null,
        user: null,
        role: null,
        branchId: null,
        isCompanyWide: false,
        authenticated: false,
        requiresTwoFactor: false,
        pendingToken: null,
        loginError: null,
      });
    },

    refreshSession: async () => {
      const { refreshToken } = get();
      if (!refreshToken) return false;
      try {
        const res = await adminRefresh(refreshToken);
        if (!res.access_token || !res.refresh_token) return false;
        const claims = applyTokens(res.access_token, res.refresh_token);
        set({
          accessToken: res.access_token,
          refreshToken: res.refresh_token,
          role: claims.role,
          branchId: claims.branchId,
          isCompanyWide: claims.isCompanyWide,
          authenticated: true,
        });
        void loadProfile();
        return true;
      } catch {
        return false;
      }
    },

    setUser: (user) => {
      persistUser(user);
      set({ user });
    },
  };
});

setAdminTokenProvider(() => useAdminStore.getState().accessToken);
setAdminRefreshHandler(() => useAdminStore.getState().refreshSession());
setAdminUnauthorizedHandler(() => {
  sessionStorage.removeItem(KEYS.access);
  sessionStorage.removeItem(KEYS.refresh);
  sessionStorage.removeItem(KEYS.user);
  useAdminStore.setState({
    accessToken: null,
    refreshToken: null,
    user: null,
    authenticated: false,
    requiresTwoFactor: false,
    pendingToken: null,
    loginError: null,
  });
});

useAdminStore.getState().hydrate();
