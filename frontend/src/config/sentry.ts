import * as Sentry from '@sentry/react';

/**
 * Initialize Sentry error tracking.
 *
 * Only activates when SENTRY_DSN is set (production/staging).
 * In development, errors stay in the console for faster iteration.
 *
 * Environment variables:
 *   SENTRY_DSN        — Your Sentry project DSN (required)
 *   SENTRY_ENVIRONMENT — "production" | "staging" (defaults to VITE_APP_ENV)
 *   SENTRY_TRACES_SAMPLE_RATE — 0.0 to 1.0 (default: 0.1 = 10% of transactions)
 */
export function initSentry() {
  const dsn = import.meta.env.VITE_SENTRY_DSN as string | undefined;
  if (!dsn) {
    console.info('[Sentry] No DSN configured — error tracking disabled (dev mode)');
    return;
  }

  const environment = (import.meta.env.VITE_APP_ENV as string) || 'development';
  const tracesSampleRate = parseFloat(
    (import.meta.env.VITE_SENTRY_TRACES_SAMPLE_RATE as string) || '0.1'
  );

  Sentry.init({
    dsn,
    environment,
    tracesSampleRate,

    // Don't track health checks or metrics endpoints
    beforeSend(event) {
      const url = event.request?.url || '';
      if (url.includes('/health') || url.includes('/ready') || url.includes('/metrics')) {
        return null;
      }
      return event;
    },

    // Integrate with React Router for better breadcrumb navigation
    integrations: [
      Sentry.browserTracingIntegration(),
      Sentry.replayIntegration({
        maskAllText: true,
        blockAllMedia: true,
      }),
    ],

    // Session Replay: capture 100% of sessions with errors, 10% otherwise
    replaysSessionSampleRate: 0.1,
    replaysOnErrorSampleRate: 1.0,

    // Don't report these common browser errors
    ignoreErrors: [
      'ResizeObserver loop limit exceeded',
      'ResizeObserver loop completed with undelivered notifications',
      'NetworkError',
      'AbortError',
      'Non-Error promise rejection captured',
    ],
  });

  console.info(`[Sentry] Initialized (env=${environment}, traces=${tracesSampleRate})`);
}
