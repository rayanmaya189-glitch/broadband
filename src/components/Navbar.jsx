import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { FiMenu, FiX } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useScrollTo } from '../hooks/useScrollTo';

/**
 * Navbar Component
 * Sticky navigation with mobile menu, scroll effects, and CTA button.
 */
export default function Navbar() {
  const [isOpen, setIsOpen] = useState(false);
  const [isScrolled, setIsScrolled] = useState(false);
  const scrollTo = useScrollTo();
  const { navLinks } = SITE_CONFIG;

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 20);
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const handleNavClick = (href) => {
    setIsOpen(false);
    const sectionId = href.replace('#', '');
    scrollTo(sectionId);
  };

  const handleCTA = () => {
    setIsOpen(false);
    scrollTo('contact');
  };

  const navVariants = {
    hidden: { y: -100 },
    visible: {
      y: 0,
      transition: { duration: 0.6, ease: 'easeOut' },
    },
  };

  const mobileMenuVariants = {
    closed: {
      opacity: 0,
      y: -20,
      transition: { duration: 0.3, ease: 'easeIn' },
    },
    open: {
      opacity: 1,
      y: 0,
      transition: { duration: 0.3, ease: 'easeOut' },
    },
  };

  return (
    <motion.nav
      initial="hidden"
      animate="visible"
      variants={navVariants}
      className={`fixed top-0 left-0 right-0 z-[100] transition-all duration-500 ${
        isScrolled
          ? 'bg-dark-950/90 backdrop-blur-xl shadow-lg shadow-dark-900/50 border-b border-white/5'
          : 'bg-transparent'
      }`}
      role="navigation"
      aria-label="Main navigation"
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16 lg:h-20">
          {/* Logo */}
          <motion.button
            onClick={() => handleNavClick('home')}
            className="flex items-center gap-3 group focus:outline-none focus:ring-2 focus:ring-accent-400 rounded-lg"
            whileHover={{ scale: 1.02 }}
            aria-label="Go to home"
          >
            <div className="relative w-9 h-9 lg:w-10 lg:h-10 rounded-lg overflow-hidden bg-gradient-to-br from-primary-500 to-accent-500 p-0.5">
              <div className="w-full h-full rounded-lg bg-dark-900 flex items-center justify-center">
                <img
                  src="/logo.png"
                  alt="AeroXe Broadband"
                  className="w-full h-full object-contain rounded-lg"
                  loading="eager"
                />
              </div>
            </div>
            <span className="text-lg lg:text-xl font-bold bg-gradient-to-r from-white to-dark-200 bg-clip-text text-transparent">
              AeroXe
            </span>
          </motion.button>

          {/* Desktop Navigation */}
          <div className="hidden lg:flex items-center gap-1">
            {navLinks.map((link) => (
              <motion.button
                key={link.label}
                onClick={() => handleNavClick(link.href)}
                className="px-4 py-2 text-sm font-medium text-dark-300 hover:text-white rounded-lg transition-colors duration-300 relative group focus:outline-none focus:ring-2 focus:ring-accent-400"
                whileHover={{ y: -1 }}
              >
                {link.label}
                <span className="absolute bottom-0 left-1/2 -translate-x-1/2 w-0 h-0.5 bg-gradient-to-r from-accent-400 to-primary-500 rounded-full transition-all duration-300 group-hover:w-3/4" />
              </motion.button>
            ))}
          </div>

          {/* CTA Button */}
          <div className="hidden lg:flex items-center">
            <motion.button
              onClick={handleCTA}
              className="relative px-6 py-2.5 text-sm font-semibold text-white rounded-xl overflow-hidden group focus:outline-none focus:ring-2 focus:ring-accent-400 focus:ring-offset-2 focus:ring-offset-dark-900"
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
            >
              <span className="absolute inset-0 bg-gradient-to-r from-primary-600 to-accent-600 group-hover:from-primary-500 group-hover:to-accent-500 transition-all duration-300" />
              <span className="absolute inset-0 bg-gradient-to-r from-accent-500 to-primary-500 opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
              <span className="relative z-10">Get Connected</span>
            </motion.button>
          </div>

          {/* Mobile Menu Button */}
          <motion.button
            onClick={() => setIsOpen(!isOpen)}
            className="lg:hidden p-2.5 text-dark-300 hover:text-white rounded-lg transition-colors focus:outline-none focus:ring-2 focus:ring-accent-400"
            whileTap={{ scale: 0.95 }}
            aria-label={isOpen ? 'Close menu' : 'Open menu'}
            aria-expanded={isOpen}
          >
            {isOpen ? <FiX className="w-5 h-5" /> : <FiMenu className="w-5 h-5" />}
          </motion.button>
        </div>
      </div>

      {/* Mobile Menu */}
      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial="closed"
            animate="open"
            exit="closed"
            variants={mobileMenuVariants}
            className="lg:hidden border-t border-white/5 bg-dark-950/95 backdrop-blur-xl"
          >
            <div className="px-4 py-6 space-y-2">
              {navLinks.map((link, index) => (
                <motion.button
                  key={link.label}
                  onClick={() => handleNavClick(link.href)}
                  className="block w-full text-left px-4 py-3 text-dark-300 hover:text-white hover:bg-white/5 rounded-xl transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400"
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: index * 0.05 }}
                >
                  {link.label}
                </motion.button>
              ))}
              <motion.button
                onClick={handleCTA}
                className="w-full mt-4 px-6 py-3.5 text-sm font-semibold text-white rounded-xl bg-gradient-to-r from-primary-600 to-accent-600 hover:from-primary-500 hover:to-accent-500 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.3 }}
              >
                Get Connected
              </motion.button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.nav>
  );
}
