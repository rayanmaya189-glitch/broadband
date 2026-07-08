import { Helmet } from 'react-helmet-async';

const BASE_URL = 'https://aeroxebroadband.com';

interface SEOProps {
  title: string;
  description: string;
  path?: string;
  ogImage?: string;
  ogType?: string;
  noIndex?: boolean;
  keywords?: string;
  author?: string;
  twitterHandle?: string;
  imageAlt?: string;
  ogImageWidth?: string;
  ogImageHeight?: string;
  articlePublishedTime?: string;
  articleModifiedTime?: string;
  articleAuthor?: string;
  articleSection?: string;
  articleTag?: string[];
  isArticle?: boolean;
}

export default function SEO({
  title,
  description,
  path = '',
  ogImage,
  ogType = 'website',
  noIndex = false,
  keywords,
  author = 'AeroXe Broadband',
  twitterHandle = '@aeroXe',
  imageAlt,
  ogImageWidth = '1200',
  ogImageHeight = '630',
  articlePublishedTime,
  articleModifiedTime,
  articleAuthor,
  articleSection,
  articleTag,
  isArticle = false,
}: SEOProps) {
  const fullTitle = `${title} | AeroXe Broadband`;
  const url = `${BASE_URL}${path}`;
  const image = ogImage || `${BASE_URL}/og-image.png`;
  const desc = description.substring(0, 160); // Meta description limit

  return (
    <Helmet>
      {/* Primary Meta Tags - Essential for Google */}
      <title>{fullTitle}</title>
      <meta charSet="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5.0" />
      <meta name="description" content={desc} />
      {keywords && <meta name="keywords" content={keywords} />}
      <meta name="author" content={author} />
      <meta name="publisher" content="AeroXe Broadband" />
      <link rel="canonical" href={url} />

      {/* Robots Meta Tags */}
      {noIndex ? (
        <meta name="robots" content="noindex, nofollow, noodp, noydir" />
      ) : (
        <meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1" />
      )}

      {/* Google Search Console & Verification */}
      <meta name="google-site-verification" content="your-verification-code-here" />

      {/* Language & Locale */}
      <meta name="language" content="English" />
      <meta httpEquiv="content-language" content="en-in" />
      <meta property="og:locale" content="en_IN" />

      {/* Open Graph Meta Tags - Critical for Social Sharing */}
      <meta property="og:title" content={fullTitle} />
      <meta property="og:description" content={desc} />
      <meta property="og:url" content={url} />
      <meta property="og:type" content={isArticle ? 'article' : ogType} />
      <meta property="og:image" content={image} />
      <meta property="og:image:width" content={ogImageWidth} />
      <meta property="og:image:height" content={ogImageHeight} />
      {imageAlt && <meta property="og:image:alt" content={imageAlt} />}
      <meta property="og:site_name" content="AeroXe Broadband" />

      {/* Article Meta Tags */}
      {isArticle && (
        <>
          {articlePublishedTime && <meta property="article:published_time" content={articlePublishedTime} />}
          {articleModifiedTime && <meta property="article:modified_time" content={articleModifiedTime} />}
          {articleAuthor && <meta property="article:author" content={articleAuthor} />}
          {articleSection && <meta property="article:section" content={articleSection} />}
          {articleTag && articleTag.map((tag) => <meta key={tag} property="article:tag" content={tag} />)}
        </>
      )}

      {/* Twitter Card Meta Tags */}
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:site" content={twitterHandle} />
      <meta name="twitter:title" content={fullTitle} />
      <meta name="twitter:description" content={desc} />
      <meta name="twitter:image" content={image} />
      <meta name="twitter:image:alt" content={imageAlt || fullTitle} />
      <meta name="twitter:creator" content={twitterHandle} />

      {/* Performance & Crawling Hints */}
      <meta name="format-detection" content="telephone=no, email=no, address=no" />
      <meta name="theme-color" content="#0a66c2" />

      {/* Prefetch & Preconnect for Performance */}
      <link rel="prefetch" href={`${BASE_URL}/api/plans`} as="fetch" crossOrigin="anonymous" />
      <link rel="dns-prefetch" href="https://fonts.googleapis.com" />
      <link rel="dns-prefetch" href="https://cdn.example.com" />

      {/* Preload Critical Resources */}
      <link rel="preload" href="/fonts/Inter-Regular.woff2" as="font" type="font/woff2" crossOrigin="anonymous" />

      {/* Microsoft & Apple */}
      <meta name="msapplication-TileColor" content="#0a66c2" />
      <meta name="apple-mobile-web-app-capable" content="yes" />
      <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
      <meta name="apple-mobile-web-app-title" content="AeroXe" />
    </Helmet>
  );
}
