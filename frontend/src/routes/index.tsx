import { lazy, Suspense } from 'react';
import { createBrowserRouter } from 'react-router-dom';
import Layout from '../components/layout/Layout';
import CustomerLayout from '../components/layout/CustomerLayout';
import Loader from '../components/ui/Loader';
import { SITE_CONFIG } from '../config/site';
import { adminRoutes } from '../admin/routes';

const LandingPage = lazy(() => import('../pages/LandingPage'));
const PlansPage = lazy(() => import('../features/plans/PlansPage'));
const PlanDetailPage = lazy(() => import('../features/plans/PlanDetailPage'));
const CheckAvailabilityPage = lazy(() => import('../features/availability/CheckAvailabilityPage'));
const ContactPage = lazy(() => import('../features/contact/ContactPage'));
const AboutPage = lazy(() => import('../features/about/AboutPage'));
const SupportPage = lazy(() => import('../features/support/SupportPage'));
const TeamPage = lazy(() => import('../features/team/TeamPage'));
const PrivacyPolicy = lazy(() => import('../features/legal/LegalPage'));
const TermsOfService = lazy(() => import('../features/legal/TermsOfService'));
const RefundPolicy = lazy(() => import('../features/legal/RefundPolicy'));
const NotFoundPage = lazy(() => import('../pages/NotFoundPage'));

// Customer Portal
const CustomerLoginPage = lazy(() => import('../features/customer/CustomerLoginPage'));
const CustomerSignupPage = lazy(() => import('../features/customer/CustomerSignupPage'));
const CustomerDashboard = lazy(() => import('../features/customer/CustomerDashboard'));
const CustomerInvoicesPage = lazy(() => import('../features/customer/CustomerInvoicesPage'));
const CustomerTicketsPage = lazy(() => import('../features/customer/CustomerTicketsPage'));

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
      { path: 'privacy', element: <PrivacyPolicy /> },
      { path: 'terms', element: <TermsOfService /> },
      { path: 'refund', element: <RefundPolicy /> },
      { path: 'team', element: <TeamPage /> },
    ],
  },
  {
    path: '/portal',
    element: (
      <Suspense fallback={<PageLoader />}>
        <CustomerLayout />
      </Suspense>
    ),
    children: [
      { path: 'login', element: <CustomerLoginPage /> },
      { path: 'signup', element: <CustomerSignupPage /> },
      { index: true, element: <CustomerDashboard /> },
      { path: 'dashboard', element: <CustomerDashboard /> },
      { path: 'invoices', element: <CustomerInvoicesPage /> },
      { path: 'tickets', element: <CustomerTicketsPage /> },
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
  adminRoutes,
]);

export const navLinks = SITE_CONFIG.navLinks;
