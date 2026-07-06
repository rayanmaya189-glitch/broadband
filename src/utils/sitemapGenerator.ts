/**
 * Sitemap Generator for SEO
 * Generates XML sitemaps for search engine crawlers
 */

const BASE_URL = 'https://aeroxebroadband.com';

interface UrlEntry {
  loc: string;
  lastmod?: string;
  changefreq?: 'always' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'yearly' | 'never';
  priority?: number;
  images?: Array<{ loc: string; title?: string; caption?: string }>;
}

export const generateSitemapXML = (urls: UrlEntry[]): string => {
  const xmlHeader = '<?xml version="1.0" encoding="UTF-8"?>';
  const urlset = urls
    .map(
      (url) =>
        `  <url>
    <loc>${escapeXml(url.loc)}</loc>
    ${url.lastmod ? `<lastmod>${url.lastmod}</lastmod>` : ''}
    ${url.changefreq ? `<changefreq>${url.changefreq}</changefreq>` : ''}
    ${url.priority !== undefined ? `<priority>${url.priority}</priority>` : ''}
    ${
      url.images && url.images.length > 0
        ? url.images
            .map(
              (img) => `<image:image>
      <image:loc>${escapeXml(img.loc)}</image:loc>
      ${img.title ? `<image:title>${escapeXml(img.title)}</image:title>` : ''}
      ${img.caption ? `<image:caption>${escapeXml(img.caption)}</image:caption>` : ''}
    </image:image>`
            )
            .join('\n    ')
        : ''
    }
  </url>`
    )
    .join('\n');

  return `${xmlHeader}
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
${urlset}
</urlset>`;
};

export const generateSitemapIndex = (sitemapUrls: string[]): string => {
  const xmlHeader = '<?xml version="1.0" encoding="UTF-8"?>';
  const sitemaps = sitemapUrls
    .map(
      (url) => `  <sitemap>
    <loc>${escapeXml(url)}</loc>
    <lastmod>${new Date().toISOString().split('T')[0]}</lastmod>
  </sitemap>`
    )
    .join('\n');

  return `${xmlHeader}
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${sitemaps}
</sitemapindex>`;
};

// Static pages for sitemap
export const getStaticSitemapUrls = (): UrlEntry[] => {
  return [
    {
      loc: BASE_URL,
      lastmod: new Date().toISOString().split('T')[0],
      changefreq: 'daily',
      priority: 1.0,
    },
    {
      loc: `${BASE_URL}/plans`,
      lastmod: new Date().toISOString().split('T')[0],
      changefreq: 'weekly',
      priority: 0.9,
    },
    {
      loc: `${BASE_URL}/check-availability`,
      lastmod: new Date().toISOString().split('T')[0],
      changefreq: 'weekly',
      priority: 0.9,
    },
    {
      loc: `${BASE_URL}/about`,
      changefreq: 'monthly',
      priority: 0.7,
    },
    {
      loc: `${BASE_URL}/support`,
      changefreq: 'weekly',
      priority: 0.8,
    },
    {
      loc: `${BASE_URL}/contact`,
      changefreq: 'monthly',
      priority: 0.7,
    },
    {
      loc: `${BASE_URL}/team`,
      changefreq: 'monthly',
      priority: 0.5,
    },
    {
      loc: `${BASE_URL}/terms-of-service`,
      changefreq: 'yearly',
      priority: 0.3,
    },
    {
      loc: `${BASE_URL}/refund-policy`,
      changefreq: 'yearly',
      priority: 0.3,
    },
  ];
};

// Product sitemaps
export const getProductSitemapUrls = (plans: any[]): UrlEntry[] => {
  return plans.map((plan) => ({
    loc: `${BASE_URL}/plans/${plan.id}`,
    lastmod: new Date().toISOString().split('T')[0],
    changefreq: 'weekly' as const,
    priority: 0.8,
  }));
};

// Escape XML special characters
const escapeXml = (str: string): string => {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&apos;');
};
