import SEO from '../../components/seo/SEO';
import PlansSection from './PlansSection';

export default function PlansPage() {
  return (
    <>
      <SEO
        title="Fiber Internet Plans from ₹400/mo in Jalgaon"
        description="Compare AeroXe Broadband's fiber internet plans. Speeds from 50 to 300 Mbps with unlimited data, free installation, free WiFi router, and 24/7 support. Save with quarterly, half-yearly, or yearly billing."
        path="/plans"
      />
      <PlansSection />
    </>
  );
}
