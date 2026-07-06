import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { ArrowRight, Phone } from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';

export default function CTASection() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950" />
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,rgba(6,182,212,0.1),transparent_60%)]" />

      <motion.div
        className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-accent-500/5 rounded-full blur-3xl"
        animate={{ scale: [1, 1.1, 1] }}
        transition={{ duration: 6, repeat: Infinity, ease: 'easeInOut' }}
      />

      <div ref={ref} className="relative max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
        >
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white text-balance">
            Ready for <span className="text-accent-400">Lightning-Fast</span> Internet?
          </h2>
          <p className="mt-4 text-lg text-dark-400 max-w-2xl mx-auto">
            Join 1,200+ happy customers in {SITE_CONFIG.location.city}. Get connected in 24-48 hours with free installation.
          </p>

          <div className="mt-10 flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              to="/plans"
              className="group inline-flex items-center justify-center gap-2 px-8 py-4 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-xl hover:shadow-accent-500/25 transition-all duration-300 text-lg"
            >
              Get Connected Today
              <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
            </Link>
            <a
              href={`tel:${SITE_CONFIG.company.phone}`}
              className="inline-flex items-center justify-center gap-2 px-8 py-4 border border-white/10 bg-white/5 backdrop-blur-sm text-white font-semibold rounded-xl hover:bg-white/10 hover:border-white/20 transition-all duration-300 text-lg"
            >
              <Phone className="w-5 h-5" />
              Call {SITE_CONFIG.company.phone}
            </a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}
