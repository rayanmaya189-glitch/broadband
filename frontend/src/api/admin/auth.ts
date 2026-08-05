import type { AuthLoginResponse, AdminUser } from '../../types/admin';
import { apiGet, apiPost, apiSend } from './client';

export interface LoginPayload {
  email: string;
  password: string;
  remember_me?: boolean;
}

export interface OtpRequestPayload {
  channel: string;
  phone?: string;
  email?: string;
}

export interface OtpVerifyPayload {
  otp: string;
  pending_token: string;
}

export async function adminLogin(payload: LoginPayload): Promise<AuthLoginResponse> {
  return apiPost<AuthLoginResponse>('/auth/login', payload);
}

export async function adminLogin2fa(payload: { pending_token: string; code: string; method?: string }): Promise<AuthLoginResponse> {
  return apiPost<AuthLoginResponse>('/auth/login/2fa', payload);
}

export async function adminRefresh(refreshToken: string): Promise<AuthLoginResponse> {
  return apiPost<AuthLoginResponse>('/auth/refresh', { refresh_token: refreshToken });
}

export async function adminLogout(): Promise<void> {
  try {
    await apiPost('/auth/logout', {});
  } catch {
    /* token may already be invalid */
  }
}

export async function adminMe(): Promise<AdminUser> {
  return apiGet<AdminUser>('/users/me');
}

export async function adminChangePassword(payload: { current_password: string; new_password: string }): Promise<void> {
  return apiPost('/auth/change-password', payload);
}

export async function adminListSessions(): Promise<unknown[]> {
  return apiGet<unknown[]>('/auth/sessions');
}

export async function adminRevokeSession(sessionId: string | number): Promise<void> {
  return apiSend('DELETE', `/auth/sessions/${sessionId}`);
}

export interface TotpSetupResult {
  secret_base32: string;
  otpauth_uri: string;
  backup_codes: string[];
}

export async function adminSetup2fa(): Promise<TotpSetupResult> {
  return apiPost<TotpSetupResult>('/auth/2fa/setup', {});
}

export async function adminConfirm2fa(code: string): Promise<unknown> {
  return apiPost('/auth/2fa/confirm', { code });
}

export async function adminDisable2fa(code: string): Promise<unknown> {
  return apiSend('DELETE', '/auth/2fa/disable', { code });
}

export async function adminRequestOtp(payload: OtpRequestPayload): Promise<unknown> {
  return apiPost('/auth/otp/request', payload);
}

export async function adminVerifyOtp(payload: OtpVerifyPayload): Promise<unknown> {
  return apiPost('/auth/otp/verify', payload);
}
