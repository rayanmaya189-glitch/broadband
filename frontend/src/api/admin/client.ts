// ─── Admin API HTTP client ──────────────────────────────────────────────────
// Thin fetch wrapper: injects Bearer token, centralises error + 401 handling.

export class ApiError extends Error {
  status: number;
  details?: unknown;

  constructor(status: number, message: string, details?: unknown) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.details = details;
  }
}

export const PROTOBUF_CONTENT_TYPE = 'application/protobuf';

let tokenProvider: (() => string | null) | null = null;
let unauthorizedHandler: (() => void) | null = null;

export function setAdminTokenProvider(fn: () => string | null) {
  tokenProvider = fn;
}

export function setAdminUnauthorizedHandler(fn: () => void) {
  unauthorizedHandler = fn;
}

function apiBase(): string {
  return import.meta.env.VITE_API_URL || 'http://localhost:8000/api';
}

interface RawResult {
  status: number;
  json: unknown;
  bytes: Uint8Array | null;
}

async function raw(
  method: string,
  path: string,
  opts: { body?: BodyInit; proto?: boolean; query?: Record<string, string | number | undefined | null> } = {}
): Promise<RawResult> {
  const token = tokenProvider?.() ?? null;
  const headers: Record<string, string> = {};
  if (token) headers['Authorization'] = `Bearer ${token}`;
  if (opts.proto) headers['Content-Type'] = PROTOBUF_CONTENT_TYPE;
  else if (opts.body) headers['Content-Type'] = 'application/json';

  let url = `${apiBase()}${path}`;
  if (opts.query) {
    const search = new URLSearchParams();
    for (const [key, value] of Object.entries(opts.query)) {
      if (value !== undefined && value !== null && value !== '') search.set(key, String(value));
    }
    const qs = search.toString();
    if (qs) url += `${url.includes('?') ? '&' : '?'}${qs}`;
  }

  let resp: Response;
  try {
    resp = await fetch(url, {
      method,
      headers,
      body: opts.body,
    });
  } catch {
    throw new ApiError(0, 'Network error — cannot reach the API server');
  }

  if (resp.status === 401) {
    unauthorizedHandler?.();
    throw new ApiError(401, 'Session expired. Please sign in again.');
  }

  const contentType = resp.headers.get('content-type') ?? '';
  const bytes = new Uint8Array(await resp.arrayBuffer());
  const isProto = contentType.includes('protobuf');
  const isJson = contentType.includes('json');

  if (resp.ok) {
    if (isProto) return { status: resp.status, json: null, bytes };
    if (bytes.length === 0) return { status: resp.status, json: null, bytes };
    if (isJson) {
      try {
        return { status: resp.status, json: JSON.parse(new TextDecoder().decode(bytes)), bytes };
      } catch {
        return { status: resp.status, json: null, bytes };
      }
    }
    return { status: resp.status, json: null, bytes };
  }

  // Error payloads are JSON in this backend.
  let message = `Request failed (${resp.status})`;
  let details: unknown;
  if (bytes.length > 0) {
    try {
      const parsed = JSON.parse(new TextDecoder().decode(bytes));
      details = parsed;
      if (parsed?.error?.message) message = parsed.error.message;
      else if (parsed?.message) message = parsed.message;
      else if (parsed?.detail) message = String(parsed.detail);
      else if (Array.isArray(parsed?.errors) && parsed.errors.length) {
        message = parsed.errors
          .map((e: { message?: string; field?: string }) => (e.field ? `${e.field}: ` : '') + (e.message ?? ''))
          .join('; ');
      }
    } catch {
      /* keep generic message */
    }
  }
  throw new ApiError(resp.status, message, details);
}

/** JSON GET */
export async function apiGet<T>(
  path: string,
  query?: Record<string, string | number | undefined | null>
): Promise<T> {
  const result = await raw('GET', path, { query });
  return result.json as T;
}

/** JSON POST */
export async function apiPost<T>(path: string, body?: unknown): Promise<T> {
  const result = await raw('POST', path, {
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  return result.json as T;
}

/** JSON PATCH / PUT / DELETE */
export async function apiSend<T>(method: 'PATCH' | 'PUT' | 'DELETE', path: string, body?: unknown): Promise<T> {
  const result = await raw(method, path, {
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  return result.json as T;
}

/** Protobuf POST — returns the raw inner `data` bytes of the Response envelope. */
export async function apiProtoPost(path: string, payload: number[], method: 'POST' | 'PATCH' | 'DELETE' = 'POST'): Promise<Uint8Array> {
  const result = await raw(method, path, {
    proto: true,
    body: new Uint8Array(payload),
  });
  return result.bytes ?? new Uint8Array(0);
}
