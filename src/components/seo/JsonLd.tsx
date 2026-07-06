import { Helmet } from 'react-helmet-async';
import { SITE_CONFIG } from '../../config/site';

const c = SITE_CONFIG.company;
const l = SITE_CONFIG.location;
const BASE_URL = 'https://aeroxebroadband.com';

/**
 * MAIN ORGANIZATION SCHEMA
 * Core entity definition for Google Knowledge Graph
 */
const organization = {
  '@context': 'https://schema.org',
  '@type': 'InternetServiceProvider',
  '@id': `${BASE_URL}/#organization`,
  name: c.name,
  legalName: c.legalName,
  alternateName: ['AeroXe', 'Aeroxe Enterprises'],
  url: BASE_URL,
  telephone: c.phone,
  email: c.email,
  description: c.description,
  foundingDate: '2024',
  foundingLocation: {
    '@type': 'City',
    name: l.city,
  },
  areaServed: [
    {
      '@type': 'City',
      name: l.city,
      addressCountry: 'IN',
    },
    {
      '@type': 'State',
      name: l.state,
      addressCountry: 'IN',
    },
  ],
  address: {
    '@type': 'PostalAddress',
    streetAddress: c.address,
    addressLocality: l.city,
    addressRegion: l.state,
    postalCode: '425001',
    addressCountry: 'IN',
  },
  contactPoint: [
    {
      '@type': 'ContactPoint',
      telephone: c.phone,
      contactType: 'customer service',
      availableLanguage: ['English', 'Hindi', 'Marathi'],
      hoursAvailable: 'Mo-Su 00:00-23:59',
    },
    {
      '@type': 'ContactPoint',
      email: c.email,
      contactType: 'customer support',
      availableLanguage: ['English', 'Hindi', 'Marathi'],
    },
  ],
  sameAs: [
    SITE_CONFIG.social.facebook,
    SITE_CONFIG.social.twitter,
    SITE_CONFIG.social.instagram,
    SITE_CONFIG.social.linkedin,
    SITE_CONFIG.social.youtube,
  ],
  aggregateRating: {
    '@type': 'AggregateRating',
    ratingValue: '4.8',
    reviewCount: '350',
    bestRating: '5',
    worstRating: '1',
  },
  knowsAbout: [
    'Fiber Internet',
    'Broadband Service',
    'High Speed Internet',
    'Internet Service Provider',
    'WiFi Router',
    'Network Service',
  ],
  slogan: c.tagline,
};

/**
 * LOCAL BUSINESS SCHEMA
 * Essential for local search rankings
 */
const localBusiness = {
  '@context': 'https://schema.org',
  '@type': 'LocalBusiness',
  '@id': `${BASE_URL}/#business`,
  parentOrganization: { '@id': `${BASE_URL}/#organization` },
  name: c.name,
  image: `${BASE_URL}/logo.png`,
  telephone: c.phone,
  email: c.email,
  address: {
    '@type': 'PostalAddress',
    streetAddress: c.address,
    addressLocality: l.city,
    addressRegion: l.state,
    postalCode: '425001',
    addressCountry: 'IN',
  },
  geo: {
    '@type': 'GeoCoordinates',
    latitude: 21.0077,
    longitude: 75.5626,
    elevation: '250',
  },
  openingHoursSpecification: {
    '@type': 'OpeningHoursSpecification',
    dayOfWeek: ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'],
    opens: '00:00',
    closes: '23:59',
  },
  priceRange: '₹400 - ₹14,000',
  url: BASE_URL,
  sameAs: [
    SITE_CONFIG.social.facebook,
    SITE_CONFIG.social.twitter,
    SITE_CONFIG.social.instagram,
  ],
  aggregateRating: {
    '@type': 'AggregateRating',
    ratingValue: '4.8',
    reviewCount: '350',
  },
};

/**
 * SERVICE SCHEMA
 * Describes the internet service offering
 */
const serviceSchema = {
  '@context': 'https://schema.org',
  '@type': 'Service',
  '@id': `${BASE_URL}/#service`,
  name: 'High-Speed Fiber Internet Service',
  description: 'Premium fiber optic broadband with unlimited data, free installation, and 24/7 support',
  provider: {
    '@type': 'LocalBusiness',
    name: c.name,
    url: BASE_URL,
  },
  areaServed: {
    '@type': 'City',
    name: l.city,
    addressCountry: 'IN',
  },
  serviceType: 'Internet Service Provider',
  offers: {
    '@type': 'AggregateOffer',
    priceCurrency: 'INR',
    lowPrice: '400',
    highPrice: '1300',
  },
  aggregateRating: {
    '@type': 'AggregateRating',
    ratingValue: '4.8',
    reviewCount: '350',
  },
};

/**
 * FAQ SCHEMA
 * Improves visibility in Google's featured snippets
 */
const faqSchema = {
  '@context': 'https://schema.org',
  '@type': 'FAQPage',
  '@id': `${BASE_URL}/#faq`,
  mainEntity: SITE_CONFIG.faqs.map((faq) => ({
    '@type': 'Question',
    name: faq.question,
    acceptedAnswer: {
      '@type': 'Answer',
      text: faq.answer,
    },
  })),
};

/**
 * WEBSITE SCHEMA
 * Enables sitelinks and search box in Google Search
 */
const websiteSchema = {
  '@context': 'https://schema.org',
  '@type': 'WebSite',
  '@id': `${BASE_URL}/#website`,
  url: BASE_URL,
  name: c.name,
  description: c.description,
  potentialAction: {
    '@type': 'SearchAction',
    target: {
      '@type': 'EntryPoint',
      urlTemplate: `${BASE_URL}/search?q={search_term_string}`,
    },
    'query-input': 'required name=search_term_string',
  },
  sameAs: [
    SITE_CONFIG.social.facebook,
    SITE_CONFIG.social.twitter,
    SITE_CONFIG.social.instagram,
  ],
};

/**
 * BREADCRUMB SCHEMA
 * Improves site navigation and SERP appearance
 */
const breadcrumbSchema = {
  '@context': 'https://schema.org',
  '@type': 'BreadcrumbList',
  '@id': `${BASE_URL}/#breadcrumb`,
  itemListElement: [
    {
      '@type': 'ListItem',
      position: 1,
      name: 'Home',
      item: BASE_URL,
    },
  ],
};

export default function JsonLd() {
  return (
    <Helmet>
      <script type="application/ld+json">{JSON.stringify(organization)}</script>
      <script type="application/ld+json">{JSON.stringify(localBusiness)}</script>
      <script type="application/ld+json">{JSON.stringify(serviceSchema)}</script>
      <script type="application/ld+json">{JSON.stringify(faqSchema)}</script>
      <script type="application/ld+json">{JSON.stringify(websiteSchema)}</script>
      <script type="application/ld+json">{JSON.stringify(breadcrumbSchema)}</script>
    </Helmet>
  );
}
