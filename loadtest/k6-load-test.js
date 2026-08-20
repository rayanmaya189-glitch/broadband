/**
 * AeroXe Backend Load Test
 *
 * Usage:
 *   k6 run loadtest/k6-load-test.js
 *   k6 run --vus 50 --duration 2m loadtest/k6-load-test.js
 *   k6 run --out influxdb=http://localhost:8086/k6 loadtest/k6-load-test.js
 *
 * Prerequisites:
 *   - k6 installed: https://grafana.com/docs/k6/latest/set-up/install-k6/
 *   - Backend running at BASE_URL (default: http://localhost:8000)
 */

import http from 'k6/http';
import { check, sleep, group } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';
import { textSummary } from 'https://jslib.k6.io/k6-summary/0.1.0/index.js';

// Custom metrics
const errorRate = new Rate('errors');
const requestDuration = new Trend('request_duration', true);
const authFailures = new Counter('auth_failures');

// Configuration
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8000';
const AUTH_USERNAME = __ENV.AUTH_USERNAME || 'admin@aeroxe.com';
const AUTH_PASSWORD = __ENV.AUTH_PASSWORD || 'Aeroxe@123';

// Ramp-up load test stages
export const options = {
  stages: [
    { duration: '30s', target: 10 },  // Ramp up to 10 VUs
    { duration: '1m', target: 10 },   // Stay at 10 VUs
    { duration: '30s', target: 25 },  // Ramp up to 25 VUs
    { duration: '2m', target: 25 },   // Stay at 25 VUs (sustained load)
    { duration: '30s', target: 50 },  // Spike to 50 VUs
    { duration: '1m', target: 50 },   // Stay at 50 VUs
    { duration: '30s', target: 0 },   // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<2000', 'p(99)<5000'], // 95% < 2s, 99% < 5s
    errors: ['rate<0.1'],                              // Error rate < 10%
    http_req_failed: ['rate<0.1'],                     // Failed requests < 10%
  },
};

// Shared state
let authToken = null;

function getHeaders() {
  const headers = { 'Content-Type': 'application/json' };
  if (authToken) {
    headers['Authorization'] = `Bearer ${authToken}`;
  }
  return headers;
}

function request(method, path, body = null) {
  const url = `${BASE_URL}${path}`;
  const params = { headers: getHeaders() };
  const start = Date.now();

  let res;
  if (body) {
    res = http.request(method, url, JSON.stringify(body), params);
  } else {
    res = http.request(method, url, null, params);
  }

  const duration = Date.now() - start;
  requestDuration.add(duration);
  errorRate.add(res.status >= 400);

  return res;
}

export default function () {
  group('Health Check', () => {
    const res = request('GET', '/health');
    check(res, {
      'health status is 200': (r) => r.status === 200,
      'health response has status field': (r) => {
        const body = JSON.parse(r.body);
        return body.status === 'healthy';
      },
    });
  });

  group('Readiness Check', () => {
    const res = request('GET', '/ready');
    check(res, {
      'ready status is 200 or 503': (r) => [200, 503].includes(r.status),
    });
  });

  group('Auth - Login', () => {
    const res = request('POST', '/api/v1/auth/login', {
      email: AUTH_USERNAME,
      password: AUTH_PASSWORD,
    });

    const success = check(res, {
      'login returns 200': (r) => r.status === 200,
      'login returns token': (r) => {
        try {
          const body = JSON.parse(r.body);
          return body.access_token !== undefined || body.token !== undefined;
        } catch {
          return false;
        }
      },
    });

    if (success && res.status === 200) {
      try {
        const body = JSON.parse(res.body);
        authToken = body.access_token || body.token;
      } catch {
        // ignore parse errors
      }
    } else {
      authFailures.add(1);
    }
  });

  // Only run authenticated endpoints if we have a token
  if (authToken) {
    group('API - List Plans', () => {
      const res = request('GET', '/api/v1/plans');
      check(res, {
        'plans status is 200': (r) => r.status === 200,
        'plans returns array': (r) => {
          try {
            const body = JSON.parse(r.body);
            return Array.isArray(body) || Array.isArray(body.data);
          } catch {
            return false;
          }
        },
      });
    });

    group('API - List Customers', () => {
      const res = request('POST', '/api/v1/customers/list', {
        page: 1,
        limit: 20,
      });
      check(res, {
        'customers status is 200 or 403': (r) => [200, 403].includes(r.status),
      });
    });

    group('API - List Subscriptions', () => {
      const res = request('GET', '/api/v1/subscriptions');
      check(res, {
        'subscriptions status is 200': (r) => r.status === 200,
      });
    });

    group('API - List Tickets', () => {
      const res = request('GET', '/api/v1/tickets');
      check(res, {
        'tickets status is 200': (r) => r.status === 200,
      });
    });

    group('API - List Notifications', () => {
      const res = request('GET', '/api/v1/notifications/list');
      check(res, {
        'notifications status is 200 or 403': (r) => [200, 403].includes(r.status),
      });
    });
  }

  sleep(1);
}

// Summary handler
export function handleSummary(data) {
  const summary = {
    timestamp: new Date().toISOString(),
    total_requests: data.metrics.http_reqs?.values?.count || 0,
    avg_response_time: data.metrics.http_req_duration?.values?.avg || 0,
    p95_response_time: data.metrics.http_req_duration?.values['p(95)'] || 0,
    p99_response_time: data.metrics.http_req_duration?.values['p(99)'] || 0,
    error_rate: data.metrics.errors?.values?.rate || 0,
    http_failures: data.metrics.http_req_failed?.values?.rate || 0,
  };

  console.log('\n=== AeroXe Load Test Summary ===');
  console.log(JSON.stringify(summary, null, 2));

  return {
    'loadtest/results/summary.json': JSON.stringify(summary, null, 2),
    stdout: textSummary(data, { indent: ' ', enableColors: true }),
  };
}
