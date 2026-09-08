import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  ApiError,
  apiGet,
  apiPost,
  apiSend,
  setAdminTokenProvider,
  setAdminUnauthorizedHandler,
  setAdminRefreshHandler,
} from '../client';

// Mock fetch globally
const mockFetch = vi.fn();
vi.stubGlobal('fetch', mockFetch);

describe('ApiError', () => {
  it('stores status and message', () => {
    const err = new ApiError(404, 'Not found');
    expect(err.status).toBe(404);
    expect(err.message).toBe('Not found');
    expect(err.name).toBe('ApiError');
  });

  it('stores details when provided', () => {
    const err = new ApiError(422, 'Validation failed', { field: 'email' });
    expect(err.details).toEqual({ field: 'email' });
  });
});

describe('apiGet', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setAdminTokenProvider(() => 'test-token');
    setAdminUnauthorizedHandler(() => {});
    setAdminRefreshHandler(async () => false);
  });

  it('makes GET request with correct path', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({ data: 'test' })),
    });

    const result = await apiGet('/customers');
    expect(result).toEqual({ data: 'test' });
    expect(mockFetch).toHaveBeenCalledTimes(1);
    const [url, opts] = mockFetch.mock.calls[0];
    expect(url).toContain('/v1/customers');
    expect(opts.method).toBe('GET');
  });

  it('attaches Authorization header', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({})),
    });

    await apiGet('/test');
    const [, opts] = mockFetch.mock.calls[0];
    expect(opts.headers['Authorization']).toBe('Bearer test-token');
  });

  it('appends query parameters', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({})),
    });

    await apiGet('/test', { page: 2, status: 'active' });
    const [url] = mockFetch.mock.calls[0];
    expect(url).toContain('page=2');
    expect(url).toContain('status=active');
  });

  it('skips empty query params', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({})),
    });

    await apiGet('/test', { page: 1, status: '', name: null });
    const [url] = mockFetch.mock.calls[0];
    expect(url).toContain('page=1');
    expect(url).not.toContain('status=');
    expect(url).not.toContain('name=');
  });

  it('throws ApiError on non-ok response', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 404,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({ error: 'Not found' })),
    });

    await expect(apiGet('/missing')).rejects.toThrow(ApiError);
  });

  it('throws ApiError on network failure', async () => {
    mockFetch.mockRejectedValueOnce(new TypeError('fetch failed'));

    await expect(apiGet('/test')).rejects.toThrow('Network error');
  });

  it('returns null json for empty response', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 204,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new Uint8Array(0),
    });

    const result = await apiGet('/delete');
    expect(result).toBeNull();
  });
});

describe('apiPost', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setAdminTokenProvider(() => 'test-token');
    setAdminUnauthorizedHandler(() => {});
    setAdminRefreshHandler(async () => false);
  });

  it('makes POST request with JSON body', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({ id: 1 })),
    });

    const result = await apiPost('/customers', { name: 'John' });
    expect(result).toEqual({ id: 1 });
    const [, opts] = mockFetch.mock.calls[0];
    expect(opts.method).toBe('POST');
    expect(opts.body).toBe(JSON.stringify({ name: 'John' }));
  });

  it('sends no body when body is undefined', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({})),
    });

    await apiPost('/action');
    const [, opts] = mockFetch.mock.calls[0];
    expect(opts.body).toBeUndefined();
  });
});

describe('apiSend', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setAdminTokenProvider(() => 'test-token');
    setAdminUnauthorizedHandler(() => {});
    setAdminRefreshHandler(async () => false);
  });

  it('makes PATCH request', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({})),
    });

    await apiSend('PATCH', '/customers/1', { name: 'Updated' });
    const [, opts] = mockFetch.mock.calls[0];
    expect(opts.method).toBe('PATCH');
  });

  it('makes DELETE request', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({})),
    });

    await apiSend('DELETE', '/customers/1');
    const [, opts] = mockFetch.mock.calls[0];
    expect(opts.method).toBe('DELETE');
  });
});

describe('401 handling', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setAdminTokenProvider(() => 'expired-token');
  });

  it('calls unauthorized handler on 401 when refresh fails', async () => {
    const onUnauthorized = vi.fn();
    setAdminUnauthorizedHandler(onUnauthorized);
    setAdminRefreshHandler(async () => false);

    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 401,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({ error: 'Unauthorized' })),
    });

    await expect(apiGet('/test')).rejects.toThrow(ApiError);
    expect(onUnauthorized).toHaveBeenCalledTimes(1);
  });

  it('retries after successful refresh', async () => {
    const onUnauthorized = vi.fn();
    setAdminUnauthorizedHandler(onUnauthorized);
    let refreshed = false;
    setAdminRefreshHandler(async () => {
      refreshed = true;
      return true;
    });
    // After refresh, token provider returns new token
    setAdminTokenProvider(() => (refreshed ? 'new-token' : 'expired-token'));

    // First call returns 401
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 401,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({ error: 'Unauthorized' })),
    });
    // Retry after refresh succeeds
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ 'content-type': 'application/json' }),
      arrayBuffer: async () => new TextEncoder().encode(JSON.stringify({ data: 'ok' })),
    });

    const result = await apiGet('/test');
    expect(result).toEqual({ data: 'ok' });
    expect(onUnauthorized).not.toHaveBeenCalled();
  });
});
