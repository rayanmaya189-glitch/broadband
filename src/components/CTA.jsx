import { motion } from 'framer-motion';
import { FiPhone, FiMessageCircle, FiArrowRight } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useScrollTo } from '../hooks/useScrollTo';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';

/**
 * CTA Banner Component
 * Large premium call-to-action banner encouraging users to get connected.
 */
export default function CTA() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.2 });
  const scrollTo = useScrollTo();

  const handleWhatsApp = () => {
    const url = `https://wa.me/${SITE_CONFIG.whatsapp.replace(/\D/g, '')}?text=${encodeURIComponent(
      'Hi, I am interested in AeroXe Broadband internet plans. Please share details.'
    )}`;
    window.open(url, '_blank', 'noopener,noreferrer');
  };

  const handleCall = () => {
    window.location.href = `tel:${SITE_CONFIG.company.phone.replace(/\D/g, '')}`;
  };

  return (
    <section className="relative py-20 lg:py-32 overflow-hidden" aria-label="Call to action">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-900 via-dark-950 to-dark-900" />

      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" ref={ref}>
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.7, ease: 'easeOut' }}
          className="relative overflow-hidden rounded-3xl"
        >
          {/* Gradient background */}
          <div className="absolute inset-0 bg-gradient-to-br from-primary-600 via-accent-700 to-dark-900" />
          <div className="absolute inset-0 bg-gradient-to-r from-accent-500/20 to-primary-500/20" />
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-white/10 via-transparent to-transparent" />

          {/* Pattern overlay */}
          <div
            className="absolute inset-0 opacity-[0.03]"
            style={{
              backgroundImage: `radial-gradient(circle at 2px 2px, rgba(255,255,255,0.5) 1px, transparent 0)`,
              backgroundSize: '30px 30px',
            }}
          />

          {/* Content */}
          <div className="relative px-6 lg:px-16 py-16 lg:py-24 text-center">
            <motion.h2
              initial={{ opacity: 0, y: 20 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ delay: 0.2 }}
              className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-4"
            >
              Need High Speed Internet?
            </motion.h2>
            <motion.p
              initial={{ opacity: 0, y: 20 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ delay: 0.3 }}
              className="text-lg lg:text-xl text-white/70 max-w-2xl mx-auto mb-10"
            >
              Get connected today with our premium fiber optic plans. 
              Fast installation, reliable service, and 24/7 support.
            </motion.p>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ delay: 0.4 }}
              className="flex flex-col sm:flex-row items-center justify-center gap-4"
            >
              <motion.button
                onClick={handleCall}
                className="group px-8 py-4 bg-white text-dark-900 font-semibold text-base rounded-2xl hover:bg-dark-100 transition-all duration-300 shadow-xl shadow-black/20 flex items-center gap-3 focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-accent-700"
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
              >
                <FiPhone className="w-5 h-5" />
                Call Now
              </motion.button>

              <motion.button
                onClick={handleWhatsApp}
                className="group px-8 py-4 bg-white/10 backdrop-blur-md border border-white/20 text-white font-semibold text-base rounded-2xl hover:bg-white/20 transition-all duration-300 flex items-center gap-3 focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-accent-700"
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
              >
                <FiMessageCircle className="w-5 h-5" />
                WhatsApp
              </motion.button>
            </motion.div>
          </div>
        </motion.div>
      </div>
    </section>
  );
}
