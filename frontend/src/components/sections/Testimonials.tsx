import { motion } from 'framer-motion';
import { Star, Quote } from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../ui/TiltCard';

const [featured, ...others] = SITE_CONFIG.testimonials;

export default function Testimonials() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section className="relative py-12 sm:py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-dark-950" />
      <div
        className="absolute inset-0"
        style={{
          background:
            'radial-gradient(ellipse at 50% 30%, rgba(6,182,212,0.06), transparent 50%), radial-gradient(ellipse at 30% 70%, rgba(10,102,194,0.04), transparent 50%)',
        }}
      />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-8 sm:mb-12"
        >
          <h2 className="text-2xl sm:text-3xl lg:text-4xl xl:text-5xl font-bold text-white">
            What Our{' '}
            <span className="bg-gradient-to-r from-accent-300 to-primary-400 bg-clip-text text-transparent">
              Customers
            </span>{' '}
            Say
          </h2>
          <p className="mt-3 sm:mt-4 text-xs sm:text-sm lg:text-base text-dark-400 max-w-2xl mx-auto">
            Real feedback from real people in {SITE_CONFIG.location.city}
          </p>
        </motion.div>

        {/* Featured Testimonial */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.5, delay: 0.1 }}
        >
          <div className="relative rounded-2xl bg-gradient-to-br from-accent-500/5 via-primary-500/5 to-accent-500/5 border border-accent-400/10 p-6 sm:p-8 lg:p-10 xl:p-12 mb-6 sm:mb-8 text-center">
            <Quote className="absolute top-4 sm:top-6 left-4 sm:left-6 w-8 h-8 sm:w-10 sm:h-10 lg:w-12 lg:h-12 text-accent-400/10" />
            <Quote className="absolute bottom-4 sm:bottom-6 right-4 sm:right-6 w-8 h-8 sm:w-10 sm:h-10 lg:w-12 lg:h-12 text-accent-400/10 rotate-180" />

            <div className="flex justify-center gap-1 mb-4 sm:mb-5">
              {Array.from({ length: featured.rating }).map((_, j) => (
                <Star key={j} className="w-4 h-4 sm:w-5 sm:h-5 fill-accent-400 text-accent-400" />
              ))}
            </div>

            <blockquote className="text-base sm:text-lg lg:text-xl xl:text-2xl text-dark-200 leading-relaxed max-w-3xl mx-auto font-medium italic">
              &ldquo;{featured.feedback}&rdquo;
            </blockquote>

            <div className="mt-6 flex items-center justify-center gap-3">
              <div className="w-12 h-12 rounded-full bg-gradient-to-br from-accent-400 to-primary-600 flex items-center justify-center text-white text-lg font-bold">
                {featured.name.charAt(0)}
              </div>
              <div className="text-left">
                <p className="text-base font-semibold text-white">{featured.name}</p>
                <p className="text-sm text-dark-500">{featured.role}</p>
              </div>
            </div>
          </div>
        </motion.div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
          {others.map((t, i) => (
            <motion.div
              key={t.name}
              initial={{ opacity: 0, y: 20 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ duration: 0.4, delay: 0.2 + i * 0.08 }}
            >
              <TiltCard tiltDegree={3} glareOpacity={0.06}>
                <div className="h-full p-4 sm:p-5 rounded-xl bg-white/[0.03] border border-white/[0.06] hover:border-accent-400/15 transition-all duration-300 hover:shadow-lg hover:shadow-accent-500/5 group min-h-[220px] sm:min-h-[240px] flex flex-col">
                  <div className="flex gap-1 mb-2.5 sm:mb-3">
                    {Array.from({ length: t.rating }).map((_, j) => (
                      <Star key={j} className="w-3 h-3 sm:w-3.5 sm:h-3.5 fill-accent-400 text-accent-400" />
                    ))}
                  </div>
                  <p className="text-xs sm:text-sm text-dark-300 leading-relaxed mb-4 sm:mb-5 line-clamp-4 flex-grow">
                    &ldquo;{t.feedback}&rdquo;
                  </p>
                  <div className="flex items-center gap-2">
                    <div className="w-7 h-7 sm:w-8 sm:h-8 rounded-full bg-gradient-to-br from-accent-400/80 to-primary-600/80 flex items-center justify-center text-white text-xs font-bold flex-shrink-0">
                      {t.name.charAt(0)}
                    </div>
                    <div className="min-w-0 flex-grow">
                      <p className="text-xs sm:text-sm font-semibold text-white truncate">{t.name}</p>
                      <p className="text-[10px] sm:text-xs text-dark-500 truncate">{t.role}</p>
                    </div>
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
