import { lazy } from 'react';
import SEO from '../components/seo/SEO';
import { generatePageKeywords } from '../config/seoConfig';

const Hero = lazy(() => import('../components/sections/Hero').then(m => ({ default: m.default })));
const Features = lazy(() => import('../components/sections/Features').then(m => ({ default: m.default })));
const AvailabilityChecker = lazy(() => import('../components/sections/AvailabilityChecker').then(m => ({ default: m.default })));
const Testimonials = lazy(() => import('../components/sections/Testimonials').then(m => ({ default: m.default })));
const CTASection = lazy(() => import('../components/sections/CTASection').then(m => ({ default: m.default })));
const PlansSection = lazy(() => import('../features/plans/PlansSection').then(m => ({ default: m.default })));

export default function LandingPage() {
  const keywords = generatePageKeywords(
    ['fiber internet Jalgaon', 'broadband provider Jalgaon', 'high speed internet'],
    ['unlimited internet', 'home broadband', 'fast WiFi']
  );

  return (
    <>
      <SEO
        title="Lightning Fast Fiber Internet in Jalgaon | AeroXe Broadband"
        description="AeroXe Broadband - Premium fiber internet in Jalgaon, Maharashtra. Plans from ₹400/month with unlimited data, free installation, dual-band WiFi router free, and 24/7 local support. Check your coverage area."
        path="/"
        keywords={keywords}
        ogImage="https://aeroxe.in/og-image.png"
        imageAlt="AeroXe Broadband - Fiber Internet Service"
      />
      <Hero />
      <Features />
      <PlansSection />
      <AvailabilityChecker />
      <Testimonials />
      <CTASection />
    </>
  );
}
