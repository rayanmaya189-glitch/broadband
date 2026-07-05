import { useState, useEffect } from 'react';
import { BrowserRouter } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import Navbar from './components/Navbar';
import Hero from './components/Hero';
import Features from './components/Features';
import WhyChooseUs from './components/WhyChooseUs';
import Plans from './components/Plans';
import Coverage from './components/Coverage';
import Installation from './components/Installation';
import Testimonials from './components/Testimonials';
import FAQ from './components/FAQ';
import ContactForm from './components/ContactForm';
import CTA from './components/CTA';
import Footer from './components/Footer';
import ScrollProgress from './components/ScrollProgress';
import BackToTop from './components/BackToTop';
import FloatingShapes from './components/FloatingShapes';
import MouseGlow from './components/MouseGlow';
import Loader from './components/Loader';

/**
 * App Component
 * Main application shell that composes all section components.
 * Includes loading screen, navigation, and global UI elements.
 */
export default function App() {
  const [isLoaded, setIsLoaded] = useState(false);

  return (
    <BrowserRouter>
      <AnimatePresence mode="wait">
        {!isLoaded && <Loader key="loader" onLoaded={() => setIsLoaded(true)} />}
      </AnimatePresence>

      {isLoaded && (
        <div className="relative min-h-screen">
          {/* Global UI Elements */}
          <ScrollProgress />
          <FloatingShapes />
          <MouseGlow />
          <BackToTop />

          {/* Navigation */}
          <Navbar />

          {/* Main Content */}
          <main>
            {/* Hero Section */}
            <Hero />

            {/* Features Section */}
            <Features />

            {/* Why Choose Us / About Section */}
            <WhyChooseUs />

            {/* Internet Plans Section */}
            <Plans />

            {/* Coverage Section */}
            <Coverage />

            {/* Installation Process Section */}
            <Installation />

            {/* Testimonials Section */}
            <Testimonials />

            {/* FAQ Section */}
            <FAQ />

            {/* Contact Form Section */}
            <ContactForm />

            {/* CTA Banner Section */}
            <CTA />
          </main>

          {/* Footer */}
          <Footer />
        </div>
      )}
    </BrowserRouter>
  );
}
