/**
 * SEO Performance & Analytics Utility
 * Tracks Core Web Vitals and SEO metrics
 */

interface WebVitals {
  lcp?: number; // Largest Contentful Paint
  fid?: number; // First Input Delay
  cls?: number; // Cumulative Layout Shift
  fcp?: number; // First Contentful Paint
  ttfb?: number; // Time to First Byte
}

/**
 * Report Web Vitals to analytics
 * Helps identify performance issues affecting SEO
 */
export const reportWebVitals = (metrics: WebVitals): void => {
  // Google Analytics 4
  if (typeof window !== 'undefined' && (window as any).gtag) {
    (window as any).gtag('event', 'page_view', {
      event_category: 'web_vitals',
      event_label: metrics,
    });
  }

  // Console logging for development
  if (process.env.NODE_ENV === 'development') {
    console.log('Web Vitals:', metrics);
  }
};

/**
 * Load Web Vitals using Web Vitals library
 */
export const loadWebVitals = (): void => {
  if (typeof window === 'undefined') return;

  // Using native Web Vitals API
  if ('PerformanceObserver' in window) {
    try {
      // Largest Contentful Paint
      if ('PerformanceObserver' in window) {
        const lcpObserver = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          const lastEntry = entries[entries.length - 1];
          reportWebVitals({ lcp: lastEntry.renderTime || lastEntry.loadTime });
        });
        lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });
      }

      // Cumulative Layout Shift
      let clsValue = 0;
      const clsObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if ((entry as any).hadRecentInput) continue;
          clsValue += (entry as any).value;
          reportWebVitals({ cls: clsValue });
        }
      });
      clsObserver.observe({ entryTypes: ['layout-shift'] });

      // First Contentful Paint
      const fcpObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        reportWebVitals({ fcp: entries[entries.length - 1].startTime });
      });
      fcpObserver.observe({ entryTypes: ['paint'] });
    } catch (e) {
      console.error('Web Vitals measurement error:', e);
    }
  }
};

/**
 * Track SEO-relevant events
 */
export const trackSEOEvent = (
  eventName: string,
  data: Record<string, any>
): void => {
  if (typeof window !== 'undefined' && (window as any).gtag) {
    (window as any).gtag('event', eventName, {
      event_category: 'seo',
      ...data,
    });
  }
};

/**
 * Check if page is properly indexed
 */
export const checkIndexability = (): {
  isIndexable: boolean;
  reasons: string[];
} => {
  const reasons: string[] = [];

  const metaRobots = document.querySelector('meta[name="robots"]');
  if (metaRobots?.getAttribute('content')?.includes('noindex')) {
    reasons.push('Page is marked as noindex');
  }

  const canonicalLink = document.querySelector('link[rel="canonical"]');
  if (!canonicalLink) {
    reasons.push('No canonical URL found');
  }

  const pageTitle = document.title;
  if (!pageTitle || pageTitle.length < 10) {
    reasons.push('Title tag is missing or too short');
  }

  const metaDescription = document.querySelector('meta[name="description"]');
  if (!metaDescription) {
    reasons.push('Meta description is missing');
  }

  return {
    isIndexable: reasons.length === 0,
    reasons,
  };
};

/**
 * Get structured data validation
 */
export const validateStructuredData = (): boolean => {
  const scripts = document.querySelectorAll('script[type="application/ld+json"]');
  return scripts.length > 0;
};

/**
 * Monitor page visibility for SEO signals
 */
export const monitorPageInteraction = (): void => {
  let totalInteractionTime = 0;
  let interactionCount = 0;

  const trackInteraction = () => {
    interactionCount++;
    if (interactionCount === 1) {
      // First interaction
      trackSEOEvent('first_interaction', {
        timestamp: Date.now(),
      });
    }
  };

  // Track various interactions
  document.addEventListener('click', trackInteraction, { once: true });
  document.addEventListener('scroll', trackInteraction, { once: true });
  document.addEventListener('keydown', trackInteraction, { once: true });
  document.addEventListener('touchstart', trackInteraction, { once: true });

  // Track time on page
  window.addEventListener('beforeunload', () => {
    const timeOnPage = Math.round((Date.now() - performance.timing.navigationStart) / 1000);
    trackSEOEvent('page_session', {
      time_on_page: timeOnPage,
      interaction_count: interactionCount,
    });
  });
};
