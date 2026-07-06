/**
 * Sitemap API Route
 * Generates XML sitemaps dynamically for better SEO
 * Deploy this as a serverless function or API route
 */

import { generateSitemapXML, generateSitemapIndex, getStaticSitemapUrls, getProductSitemapUrls } from '../utils/sitemapGenerator';
import { SITE_CONFIG } from '../config/site';

/**
 * Handler for /api/sitemaps/static.xml
 * Returns static pages sitemap
 */
export const handleStaticSitemap = () => {
  const urls = getStaticSitemapUrls();
  const xml = generateSitemapXML(urls);
  
  return new Response(xml, {
    headers: {
      'Content-Type': 'application/xml',
      'Cache-Control': 'public, max-age=86400', // Cache for 24 hours
    },
  });
};

/**
 * Handler for /api/sitemaps/products.xml
 * Returns products/plans sitemap (would need to fetch plans from your API)
 */
export const handleProductSitemap = async (plans: any[]) => {
  const urls = getProductSitemapUrls(plans);
  const xml = generateSitemapXML(urls);
  
  return new Response(xml, {
    headers: {
      'Content-Type': 'application/xml',
      'Cache-Control': 'public, max-age=43200', // Cache for 12 hours
    },
  });
};

/**
 * Handler for /api/sitemaps/index.xml
 * Returns sitemap index pointing to all sitemaps
 */
export const handleSitemapIndex = () => {
  const sitemapUrls = [
    `${SITE_CONFIG.domain}/sitemap-static.xml`,
    `${SITE_CONFIG.domain}/sitemap-products.xml`,
  ];
  
  const xml = generateSitemapIndex(sitemapUrls);
  
  return new Response(xml, {
    headers: {
      'Content-Type': 'application/xml',
      'Cache-Control': 'public, max-age=86400', // Cache for 24 hours
    },
  });
};

/**
 * Robots.txt API
 * Serves robots.txt dynamically (if needed)
 */
export const handleRobotsTxt = () => {
  const content = `User-agent: *
Allow: /
Disallow: /api/
Disallow: /admin/
Disallow: /private/
Crawl-delay: 1

User-agent: Googlebot
Allow: /
Crawl-delay: 0

Sitemap: ${SITE_CONFIG.domain}/sitemap.xml
`;

  return new Response(content, {
    headers: {
      'Content-Type': 'text/plain',
      'Cache-Control': 'public, max-age=604800', // Cache for 1 week
    },
  });
};

/**
 * SEO Status Endpoint
 * Returns current SEO metrics and status
 */
export const getSEOStatus = () => {
  const status = {
    domain: SITE_CONFIG.domain,
    seo_metrics: {
      has_robots_txt: true,
      has_sitemap: true,
      has_structured_data: true,
      mobile_friendly: true,
      https_enabled: SITE_CONFIG.domain.startsWith('https://'),
    },
    implemented_features: [
      'Open Graph',
      'Twitter Cards',
      'JSON-LD Schemas',
      'Mobile Optimization',
      'Performance Optimization',
      'Core Web Vitals Monitoring',
      'Geo-targeting',
      'Local Business Schema',
    ],
    keywords: SITE_CONFIG.keywords,
    social_links: SITE_CONFIG.socialMedia,
    last_updated: new Date().toISOString(),
  };

  return new Response(JSON.stringify(status), {
    headers: {
      'Content-Type': 'application/json',
      'Cache-Control': 'public, max-age=3600',
    },
  });
};
