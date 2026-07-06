import { Helmet } from 'react-helmet-async';

const BASE_URL = 'https://aeroxebroadband.com';

/**
 * Product/Plan Schema for Google Shopping and Rich Results
 */
export const PricingSchema = ({ plan }: { plan: any }) => {
  const offer = {
    '@context': 'https://schema.org',
    '@type': 'Product',
    '@id': `${BASE_URL}/plans/${plan.id}#product`,
    name: `AeroXe ${plan.speed} Broadband Plan`,
    description: `High-speed ${plan.speed} fiber internet plan with unlimited data`,
    image: [
      `${BASE_URL}/og-image.png`,
      `${BASE_URL}/plans-${plan.speedMbps}.jpg`,
    ],
    brand: {
      '@type': 'Brand',
      name: 'AeroXe Broadband',
    },
    offers: {
      '@type': 'AggregateOffer',
      priceCurrency: 'INR',
      lowPrice: Math.min(...Object.values(plan.durations).map((d: any) => d.price)),
      highPrice: Math.max(...Object.values(plan.durations).map((d: any) => d.price)),
      offerCount: Object.keys(plan.durations).length,
      offers: Object.entries(plan.durations).map(([duration, details]: [string, any]) => ({
        '@type': 'Offer',
        price: details.price.toString(),
        priceCurrency: 'INR',
        priceValidUntil: new Date(Date.now() + 90 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
        availability: 'https://schema.org/InStock',
        url: `${BASE_URL}/plans`,
        name: `${plan.speed} - ${details.label}`,
      })),
    },
    aggregateRating: {
      '@type': 'AggregateRating',
      ratingValue: '4.8',
      reviewCount: '350',
      bestRating: '5',
      worstRating: '1',
    },
  };

  return (
    <Helmet>
      <script type="application/ld+json">{JSON.stringify(offer)}</script>
    </Helmet>
  );
};

/**
 * Breadcrumb Schema for Navigation
 */
export const BreadcrumbSchema = ({
  items,
}: {
  items: Array<{ name: string; url: string }>;
}) => {
  const breadcrumb = {
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: items.map((item, index) => ({
      '@type': 'ListItem',
      position: index + 1,
      name: item.name,
      item: item.url,
    })),
  };

  return (
    <Helmet>
      <script type="application/ld+json">{JSON.stringify(breadcrumb)}</script>
    </Helmet>
  );
};

/**
 * Article/NewsArticle Schema
 */
export const ArticleSchema = ({
  title,
  description,
  image,
  datePublished,
  dateModified,
  author = 'AeroXe Broadband',
}: {
  title: string;
  description: string;
  image: string;
  datePublished: string;
  dateModified: string;
  author?: string;
}) => {
  const article = {
    '@context': 'https://schema.org',
    '@type': 'NewsArticle',
    headline: title,
    description,
    image,
    datePublished,
    dateModified,
    author: {
      '@type': 'Organization',
      name: author,
    },
    publisher: {
      '@type': 'Organization',
      name: 'AeroXe Broadband',
      logo: {
        '@type': 'ImageObject',
        url: `${BASE_URL}/logo.png`,
      },
    },
  };

  return (
    <Helmet>
      <script type="application/ld+json">{JSON.stringify(article)}</script>
    </Helmet>
  );
};

/**
 * Video Schema for embedded videos
 */
export const VideoSchema = ({
  title,
  description,
  thumbnailUrl,
  uploadDate,
  duration = 'PT5M',
}: {
  title: string;
  description: string;
  thumbnailUrl: string;
  uploadDate: string;
  duration?: string;
}) => {
  const video = {
    '@context': 'https://schema.org',
    '@type': 'VideoObject',
    name: title,
    description,
    thumbnailUrl,
    uploadDate,
    duration,
  };

  return (
    <Helmet>
      <script type="application/ld+json">{JSON.stringify(video)}</script>
    </Helmet>
  );
};

/**
 * Service Schema for ISP
 */
export const ServiceSchema = () => {
  const service = {
    '@context': 'https://schema.org',
    '@type': 'Service',
    '@id': `${BASE_URL}/#service`,
    name: 'Fiber Internet Service',
    description: 'High-speed fiber optic internet service for residential and commercial use',
    provider: {
      '@type': 'LocalBusiness',
      name: 'AeroXe Broadband',
      url: BASE_URL,
    },
    areaServed: {
      '@type': 'City',
      name: 'Jalgaon',
    },
    serviceType: 'Internet Service Provider',
  };

  return (
    <Helmet>
      <script type="application/ld+json">{JSON.stringify(service)}</script>
    </Helmet>
  );
};

/**
 * Review/Rating Schema
 */
export const ReviewSchema = ({
  rating = 4.8,
  reviewCount = 350,
}: {
  rating?: number;
  reviewCount?: number;
}) => {
  const review = {
    '@context': 'https://schema.org',
    '@type': 'AggregateRating',
    ratingValue: rating.toString(),
    reviewCount: reviewCount.toString(),
    bestRating: '5',
    worstRating: '1',
  };

  return (
    <Helmet>
      <script type="application/ld+json">{JSON.stringify(review)}</script>
    </Helmet>
  );
};
