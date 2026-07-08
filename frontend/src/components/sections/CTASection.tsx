import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { ArrowRight, Phone } from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../ui/TiltCard';

export default function CTASection() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section className="relative py-12 sm:py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950" />
      <div
        className="absolute inset-0"
        style={{
          background: 'radial-gradient(ellipse at 30% 30%, rgba(6,182,212,0.08), transparent 60%), radial-gradient(ellipse at 70% 70%, rgba(10,102,194,0.06), transparent 60%)',
        }}
      />

      <motion.div
        className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[400px] sm:w-[500px] lg:w-[600px] h-[400px] sm:h-[500px] lg:h-[600px] bg-accent-500/5 rounded-full blur-3xl"
        animate={{ scale: [1, 1.1, 1] }}
        transition={{ duration: 6, repeat: Infinity, ease: 'easeInOut' }}
      />

      <div ref={ref} className="relative max-w-4xl mx-auto px-4 sm:px-6 text-center">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
        >
          <h2 className="text-2xl sm:text-3xl lg:text-4xl xl:text-5xl font-bold text-white text-balance leading-tight">
            Ready for <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent drop-shadow-[0_0_15px_rgba(6,182,212,0.25)]">Lightning-Fast</span> Internet?
          </h2>
          <p className="mt-3 sm:mt-4 text-xs sm:text-base text-dark-400 max-w-2xl mx-auto">
            Join customers across {SITE_CONFIG.location.city}. Get connected with free installation.
          </p>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={isVisible ? { opacity: 1, y: 0 } : {}}
            transition={{ duration: 0.6, delay: 0.2 }}
            className="mt-6 sm:mt-8 lg:mt-10 flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center"
          >
            <TiltCard tiltDegree={3} glareOpacity={0.12}>
              <Link
                to="/plans"
                className="group inline-flex items-center justify-center gap-2 px-5 sm:px-6 lg:px-8 py-3 sm:py-4 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold text-sm sm:text-base rounded-xl hover:shadow-xl hover:shadow-accent-500/25 transition-all duration-300 min-h-[44px] sm:min-h-[48px] active:scale-95"
              >
                Get Connected Today
                <ArrowRight className="w-4 h-4 sm:w-5 sm:h-5 group-hover:translate-x-1 transition-transform" />
              </Link>
            </TiltCard>
            <TiltCard tiltDegree={3} glareOpacity={0.08}>
              <a
                href={`tel:${SITE_CONFIG.company.phone}`}
                className="inline-flex items-center justify-center gap-2 px-5 sm:px-6 lg:px-8 py-3 sm:py-4 border border-white/10 bg-white/5 backdrop-blur-sm text-white font-semibold text-sm sm:text-base rounded-xl hover:bg-white/10 hover:border-white/20 transition-all duration-300 min-h-[44px] sm:min-h-[48px] active:scale-95"
              >
                <Phone className="w-4 h-4 sm:w-5 sm:h-5" />
                <span className="hidden sm:inline">Call {SITE_CONFIG.company.phone}</span>
                <span className="sm:hidden">Call Now</span>
              </a>
            </TiltCard>
          </motion.div>
        </motion.div>
      </div>
    </section>
  );
}
