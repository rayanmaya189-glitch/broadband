import { motion } from 'framer-motion';
import { FiZap, FiGlobe, FiMonitor, FiHeadphones, FiAward, FiShield } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';
import { staggerContainer, staggerItem, fadeUp } from '../utils/animations';

const iconMap = {
  Zap: FiZap,
  Infinity: FiGlobe,
  Gamepad2: FiMonitor,
  Headphones: FiHeadphones,
  BadgePercent: FiAward,
  Shield: FiShield,
  Monitor: FiMonitor,
  Globe: FiGlobe,
};

/**
 * Features Section Component
 * Premium cards showcasing ISP features with hover animations.
 */
export default function Features() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.1 });
  const { features } = SITE_CONFIG;

  return (
    <section id="features" className="relative py-20 lg:py-32 overflow-hidden" aria-label="Features">
      {/* Section background */}
      <div className="absolute inset-0 bg-gradient-to-b from-dark-900 via-dark-950 to-dark-900" />

      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" ref={ref}>
        {/* Section header */}
        <motion.div
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="text-center max-w-3xl mx-auto mb-16 lg:mb-20"
        >
          <span className="text-xs lg:text-sm font-semibold text-accent-400 tracking-[0.2em] uppercase mb-4 block">
            Why AeroXe
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6">
            Built for{' '}
            <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
              Speed & Reliability
            </span>
          </h2>
          <p className="text-base lg:text-lg text-dark-300 leading-relaxed">
            Every feature designed to deliver the best internet experience 
            for your home and business needs.
          </p>
        </motion.div>

        {/* Features grid */}
        <motion.div
          variants={staggerContainer}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4 lg:gap-6"
        >
          {features.map((feature, index) => {
            const IconComponent = iconMap[feature.icon] || FiZap;
            return (
              <motion.div
                key={feature.title}
                variants={staggerItem}
                className="group relative p-6 lg:p-8 rounded-2xl bg-gradient-to-b from-white/[0.04] to-transparent border border-white/[0.06] hover:border-accent-500/30 transition-all duration-500"
                whileHover={{
                  y: -8,
                  transition: { duration: 0.3 },
                }}
              >
                {/* Icon */}
                <div className="relative w-12 h-12 lg:w-14 lg:h-14 rounded-xl bg-gradient-to-br from-accent-500/20 to-primary-500/20 flex items-center justify-center mb-5 group-hover:scale-110 transition-transform duration-300">
                  <IconComponent className="w-6 h-6 lg:w-7 lg:h-7 text-accent-400" />
                  <div className="absolute inset-0 rounded-xl bg-accent-500/10 group-hover:bg-accent-500/20 transition-colors duration-300" />
                </div>

                {/* Title */}
                <h3 className="text-lg lg:text-xl font-semibold text-white mb-3 group-hover:text-accent-300 transition-colors duration-300">
                  {feature.title}
                </h3>

                {/* Description */}
                <p className="text-sm lg:text-base text-dark-400 leading-relaxed group-hover:text-dark-300 transition-colors duration-300">
                  {feature.description}
                </p>

                {/* Bottom gradient line */}
                <div className="absolute bottom-0 left-6 right-6 h-0.5 bg-gradient-to-r from-transparent via-accent-500/30 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500 rounded-full scale-x-0 group-hover:scale-x-100 origin-center transition-transform duration-500" />
              </motion.div>
            );
          })}
        </motion.div>
      </div>
    </section>
  );
}
