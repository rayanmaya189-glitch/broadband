import { motion } from 'framer-motion';
import { Monitor, Globe, Zap, Headphones } from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import TiltCard from '../ui/TiltCard';

const highlights = [
  { icon: Monitor, title: 'Free WiFi Router', desc: 'Complimentary Dual Band router with 12-month plans.' },
  { icon: Globe, title: 'Fiber Backbone', desc: 'Modern fiber optic infrastructure across Jalgaon.' },
  { icon: Zap, title: 'Free Installation', desc: 'Professional setup at no cost. Limited period.' },
  { icon: Headphones, title: 'Local Support', desc: 'Jalgaon-based team, available 24/7.' },
];

export default function Features() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section className="relative py-12 sm:py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-dark-950" />
      <div
        className="absolute inset-0"
        style={{
          background:
            'radial-gradient(ellipse at 25% 30%, rgba(6,182,212,0.06), transparent 50%), radial-gradient(ellipse at 75% 70%, rgba(10,102,194,0.04), transparent 50%)',
        }}
      />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-8 sm:mb-12 lg:mb-14"
        >
          <h2 className="text-2xl sm:text-3xl lg:text-4xl xl:text-5xl font-bold text-white leading-tight">
            What&apos;s{' '}
            <span className="bg-gradient-to-r from-accent-300 via-accent-400 to-primary-400 bg-clip-text text-transparent drop-shadow-[0_0_20px_rgba(6,182,212,0.3)]">
              Included
            </span>
          </h2>
          <p className="mt-2 sm:mt-3 text-xs sm:text-sm text-dark-400 max-w-2xl mx-auto">
            Everything you need for blazing-fast internet
          </p>
        </motion.div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4 lg:gap-5">
          {highlights.map((item, i) => (
            <motion.div
              key={item.title}
              initial={{ opacity: 0, y: 30 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ duration: 0.5, delay: i * 0.1 }}
            >
              <TiltCard tiltDegree={2} glareOpacity={0.06}>
                <div className="group h-full rounded-2xl p-4 sm:p-5 lg:p-6 bg-white/[0.03] border border-white/[0.06] hover:border-accent-400/20 transition-all duration-500 text-center min-h-[200px] sm:min-h-[220px] flex flex-col items-center justify-center">
                  <div className="inline-flex p-2.5 sm:p-3 rounded-xl bg-gradient-to-br from-accent-400/15 to-accent-400/5 border border-accent-400/10 group-hover:from-accent-400/25 group-hover:to-accent-400/10 transition-all duration-300 mb-3 sm:mb-4">
                    <item.icon className="w-5 h-5 sm:w-6 sm:h-6 text-accent-400" />
                  </div>
                  <h3 className="text-sm sm:text-base font-semibold text-white group-hover:text-accent-200 transition-colors mb-1.5 sm:mb-2">
                    {item.title}
                  </h3>
                  <p className="text-xs sm:text-sm text-dark-400 leading-relaxed line-clamp-3">
                    {item.desc}
                  </p>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
