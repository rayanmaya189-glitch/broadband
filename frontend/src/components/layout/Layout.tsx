import { Outlet } from 'react-router-dom';
import Navbar from './Navbar';
import Footer from './Footer';
import ErrorBoundary from '../ui/ErrorBoundary';
import ToastContainer from '../ui/Toast';
import ScrollToTop from './ScrollToTop';
import JsonLd from '../seo/JsonLd';

export default function Layout() {
  return (
    <>
      <JsonLd />
      <ScrollToTop />
      <div className="min-h-screen bg-dark-950 text-white flex flex-col">
        <Navbar />
        <ErrorBoundary>
          <main id="main-content" className="flex-1">
            <Outlet />
          </main>
        </ErrorBoundary>
        <Footer />
        <ToastContainer />
      </div>
    </>
  );
}
