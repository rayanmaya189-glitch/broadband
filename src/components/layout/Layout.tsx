import { Outlet } from 'react-router-dom';
import Navbar from './Navbar';
import Footer from './Footer';
import ErrorBoundary from '../ui/ErrorBoundary';
import ToastContainer from '../ui/Toast';
import ScrollToTop from './ScrollToTop';

export default function Layout() {
  return (
    <div className="min-h-screen bg-dark-950 text-white">
      <ScrollToTop />
      <Navbar />
      <ErrorBoundary>
        <main>
          <Outlet />
        </main>
      </ErrorBoundary>
      <Footer />
      <ToastContainer />
    </div>
  );
}
