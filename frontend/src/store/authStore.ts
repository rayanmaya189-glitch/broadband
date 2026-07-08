import { create } from 'zustand';
import type { AuthState } from '../types';

interface AuthStore extends AuthState {
  setToken: (token: string) => void;
  logout: () => void;
  hydrate: () => void;
}

export const useAuthStore = create<AuthStore>((set) => ({
  token: null,
  isAuthenticated: false,
  setToken: (token: string) => {
    sessionStorage.setItem('auth_token', token);
    set({ token, isAuthenticated: true });
  },
  logout: () => {
    sessionStorage.removeItem('auth_token');
    set({ token: null, isAuthenticated: false });
  },
  hydrate: () => {
    const token = sessionStorage.getItem('auth_token');
    if (token) {
      set({ token, isAuthenticated: true });
    }
  },
}));
