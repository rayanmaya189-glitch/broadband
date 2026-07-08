/**
 * SEO Configuration & Utilities
 * Central configuration for all SEO-related settings
 */

export const SEO_CONFIG = {
  // Domain configuration
  domain: 'https://aeroxebroadband.com',
  siteTitle: 'AeroXe Broadband',
  siteDescription:
    'Lightning-fast fiber internet in Jalgaon, Maharashtra. Plans from ₹400/month with unlimited data, free installation, free WiFi router, and 24/7 support.',

  // Default meta tags
  defaultMeta: {
    ogImageWidth: '1200',
    ogImageHeight: '630',
    twitterHandle: '@aeroXe',
  },

  // Priority pages for sitemap
  priorityPages: {
    home: 1.0,
    plans: 0.9,
    availability: 0.9,
    about: 0.7,
    support: 0.8,
    contact: 0.7,
    team: 0.5,
    legal: 0.3,
  },

  // Keywords research & mapping
  keywords: {
    primary: [
      'fiber internet Jalgaon',
      'broadband provider Jalgaon',
      'high speed internet Jalgaon',
      'WiFi internet Jalgaon',
      'home internet Jalgaon',
    ],
    secondary: [
      'ISP Jalgaon',
      'business broadband Jalgaon',
      'unlimited internet Jalgaon',
      'fiber optic internet',
      'broadband service',
    ],
    locationKeywords: [
      'Jalgaon internet',
      'Jalgaon broadband',
      'internet service provider Maharashtra',
      'fiber internet provider India',
    ],
  },

  // Open Graph defaults
  openGraph: {
    type: 'website',
    locale: 'en_IN',
    siteName: 'AeroXe Broadband',
    imageType: 'image/png',
  },

  // Structured data defaults
  structuredData: {
    context: 'https://schema.org',
    baseUrl: 'https://aeroxe.in',
  },

  // Performance metrics (Core Web Vitals targets)
  performanceTargets: {
    lcp: 2500, // Largest Contentful Paint in milliseconds
    fid: 100, // First Input Delay in milliseconds
    cls: 0.1, // Cumulative Layout Shift
  },

  // Social media URLs
  socialMedia: {
    facebook: 'https://facebook.com/aeroXe',
    twitter: 'https://twitter.com/aeroXe',
    instagram: 'https://instagram.com/aeroXe',
    linkedin: 'https://linkedin.com/company/aeroXe',
    youtube: 'https://youtube.com/@aeroXe',
    whatsapp: 'https://wa.me/917770033326',
  },

  // Robots.txt settings
  robots: {
    allowedCrawlers: ['googlebot', 'bingbot', 'yandexbot'],
    crawlDelay: 1,
  },

  // Local SEO
  localBusiness: {
    type: 'LocalBusiness',
    latitude: 21.0077,
    longitude: 75.5626,
    city: 'Jalgaon',
    state: 'Maharashtra',
    country: 'India',
    zipCode: '425001',
  },

  // Google Search Console
  verificationCodes: {
    google: 'your-verification-code-here',
    bing: 'your-bing-verification-code',
    yandex: 'your-yandex-verification-code',
  },

  // Canonical URL helper
  getCanonicalUrl: (path: string): string => {
    return `${SEO_CONFIG.domain}${path}`;
  },

  // Generate schema markup ID
  getSchemaId: (type: string, id?: string): string => {
    return `${SEO_CONFIG.domain}/#${id || type.toLowerCase()}`;
  },
};

/**
 * Generate meta description (160 chars limit for desktop, 120 for mobile)
 */
export const generateMetaDescription = (text: string, limit: number = 160): string => {
  return text.length > limit ? `${text.substring(0, limit - 3)}...` : text;
};

/**
 * Generate page-specific keywords
 */
export const generatePageKeywords = (primary: string[], secondary: string[] = []): string => {
  return [...primary, ...secondary, ...SEO_CONFIG.keywords.locationKeywords].join(', ');
};

/**
 * Generate breadcrumb data
 */
export const generateBreadcrumbs = (
  segments: Array<{ name: string; path: string }>
): Array<{ name: string; url: string }> => {
  return [
    { name: 'Home', url: SEO_CONFIG.domain },
    ...segments.map((seg) => ({
      name: seg.name,
      url: `${SEO_CONFIG.domain}${seg.path}`,
    })),
  ];
};

/**
 * Generate Open Graph image URL with fallback
 */
export const getOGImageUrl = (imagePath?: string): string => {
  if (!imagePath) {
    return `${SEO_CONFIG.domain}/og-image.png`;
  }
  return imagePath.startsWith('http') ? imagePath : `${SEO_CONFIG.domain}${imagePath}`;
};

/**
 * Get page title with site name
 */
export const getPageTitle = (pageTitle: string): string => {
  return `${pageTitle} | ${SEO_CONFIG.siteTitle}`;
};

/**
 * Validate meta description length
 */
export const isValidMetaDescription = (text: string): boolean => {
  return text.length >= 50 && text.length <= 160;
};

/**
 * Validate meta title length
 */
export const isValidMetaTitle = (text: string): boolean => {
  return text.length >= 30 && text.length <= 60;
};
