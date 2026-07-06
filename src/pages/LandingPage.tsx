import { lazy } from 'react';

const Hero = lazy(() => import('../components/sections/Hero').then(m => ({ default: m.default })));
const Features = lazy(() => import('../components/sections/Features').then(m => ({ default: m.default })));
const AvailabilityChecker = lazy(() => import('../components/sections/AvailabilityChecker').then(m => ({ default: m.default })));
const Testimonials = lazy(() => import('../components/sections/Testimonials').then(m => ({ default: m.default })));
const CTASection = lazy(() => import('../components/sections/CTASection').then(m => ({ default: m.default })));
const PlansSection = lazy(() => import('../features/plans/PlansSection').then(m => ({ default: m.default })));

export default function LandingPage() {
  return (
    <>
      <Hero />
      <Features />
      <PlansSection />
      <AvailabilityChecker />
      <Testimonials />
      <CTASection />
    </>
  );
}
