import { useState, useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import { Menu, X, Sun, Moon } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';
import { useUIStore } from '../../store/uiStore';
import { useTheme } from '../../hooks/useTheme';
import { cn } from '../../utils/helpers';

export default function Navbar() {
  const [scrolled, setScrolled] = useState(false);
  const location = useLocation();
  const { isMobileMenuOpen, setMobileMenuOpen } = useUIStore();
  const { theme, toggleTheme } = useTheme();

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 20);
    window.addEventListener('scroll', onScroll, { passive: true });
    return () => window.removeEventListener('scroll', onScroll);
  }, []);

  useEffect(() => {
    setMobileMenuOpen(false);
  }, [location, setMobileMenuOpen]);

  return (
    <>
      <motion.nav
        initial={{ y: -100 }}
        animate={{ y: 0 }}
        transition={{ duration: 0.6, ease: 'easeOut' }}
        className={cn(
          'fixed top-0 left-0 right-0 z-50 transition-all duration-300',
          scrolled
            ? 'bg-dark-950/80 backdrop-blur-xl border-b border-white/[0.06]'
            : 'bg-transparent'
        )}
      >
        <div className="max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
          <div className="flex items-center justify-between h-16 lg:h-20">
            <Link to="/">
              <img src="/logo.png" alt={SITE_CONFIG.company.name} className="h-8 w-auto" />
            </Link>

            <div className="hidden lg:flex items-center gap-1">
              {SITE_CONFIG.navLinks.map((link) => {
                const isActive = link.href === '/'
                  ? location.pathname === '/'
                  : location.pathname.startsWith(link.href.split('/')[1] ? '/' + link.href.split('/')[1] : '');
                return (
                  <Link
                    key={link.label}
                    to={link.href}
                    className={cn(
                      'px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200',
                      isActive
                        ? 'text-accent-400 bg-accent-400/10'
                        : 'text-dark-300 hover:text-white hover:bg-white/[0.04]'
                    )}
                  >
                    {link.label}
                  </Link>
                );
              })}
            </div>

            <div className="flex items-center gap-3">
              <button
                onClick={toggleTheme}
                className="p-2 rounded-lg text-dark-400 hover:text-white hover:bg-white/[0.06] transition-all"
                aria-label="Toggle theme"
              >
                {theme === 'dark' ? <Sun className="w-5 h-5" /> : <Moon className="w-5 h-5" />}
              </button>

              <Link
                to="/plans"
                className="hidden sm:inline-flex items-center px-5 py-2.5 bg-gradient-to-r from-accent-500 to-primary-600 text-white text-sm font-semibold rounded-xl hover:shadow-lg hover:shadow-accent-500/25 transition-all duration-300"
              >
                View Plans
              </Link>

              <button
                onClick={() => setMobileMenuOpen(!isMobileMenuOpen)}
                className="lg:hidden p-2 rounded-lg text-dark-400 hover:text-white hover:bg-white/[0.06] transition-all"
                aria-label="Toggle menu"
              >
                {isMobileMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
              </button>
            </div>
          </div>
        </div>
      </motion.nav>

      <AnimatePresence>
        {isMobileMenuOpen && (
          <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ duration: 0.2 }}
            className="fixed inset-0 top-16 z-40 lg:hidden"
          >
            <div className="absolute inset-0 bg-dark-950/95 backdrop-blur-xl" onClick={() => setMobileMenuOpen(false)} />
            <div className="relative p-6 pt-4 space-y-2">
              {SITE_CONFIG.navLinks.map((link) => (
                <Link
                  key={link.label}
                  to={link.href}
                  className="block px-4 py-3 rounded-xl text-lg font-medium text-dark-300 hover:text-white hover:bg-white/[0.06] transition-all"
                >
                  {link.label}
                </Link>
              ))}
              <Link
                to="/plans"
                className="block w-full mt-4 px-6 py-3.5 bg-gradient-to-r from-accent-500 to-primary-600 text-white text-center font-semibold rounded-xl"
              >
                View Plans
              </Link>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </>
  );
}
