import { lazy, Suspense } from 'react';
import { createBrowserRouter } from 'react-router-dom';
import Layout from '../components/layout/Layout';
import Loader from '../components/ui/Loader';
import { SITE_CONFIG } from '../config/site';

const LandingPage = lazy(() => import('../pages/LandingPage'));
const PlansPage = lazy(() => import('../features/plans/PlansPage'));
const PlanDetailPage = lazy(() => import('../features/plans/PlanDetailPage'));
const CheckAvailabilityPage = lazy(() => import('../features/availability/CheckAvailabilityPage'));
const ContactPage = lazy(() => import('../features/contact/ContactPage'));
const AboutPage = lazy(() => import('../features/about/AboutPage'));
const SupportPage = lazy(() => import('../features/support/SupportPage'));
const TeamPage = lazy(() => import('../features/team/TeamPage'));
const LegalPage = lazy(() => import('../features/legal/LegalPage'));
const NotFoundPage = lazy(() => import('../pages/NotFoundPage'));

function PageLoader() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-dark-950">
      <Loader />
    </div>
  );
}

export const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <Suspense fallback={<PageLoader />}>
        <Layout />
      </Suspense>
    ),
    children: [
      { index: true, element: <LandingPage /> },
      { path: 'plans', element: <PlansPage /> },
      { path: 'plan/:id', element: <PlanDetailPage /> },
      { path: 'check-availability', element: <CheckAvailabilityPage /> },
      { path: 'contact', element: <ContactPage /> },
      { path: 'about', element: <AboutPage /> },
      { path: 'support', element: <SupportPage /> },
      { path: 'privacy', element: <LegalPage /> },
      { path: 'terms', element: <LegalPage /> },
      { path: 'refund', element: <LegalPage /> },
      { path: 'team', element: <TeamPage /> },
    ],
  },
  {
    path: '*',
    element: (
      <Suspense fallback={<PageLoader />}>
        <NotFoundPage />
      </Suspense>
    ),
  },
]);

export const navLinks = SITE_CONFIG.navLinks;
