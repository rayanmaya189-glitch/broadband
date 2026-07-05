import { motion } from 'framer-motion';
import { FiCheckCircle, FiShield, FiZap, FiGlobe, FiTrendingUp, FiUsers } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';
import { fadeLeft, fadeRight, staggerContainer, staggerItem } from '../utils/animations';

/**
 * Why Choose Us Section Component
 * Split layout with animated checklist of ISP advantages.
 */
export default function WhyChooseUs() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.1 });
  const { whyChooseUs } = SITE_CONFIG;

  return (
    <section id="about" className="relative py-20 lg:py-32 overflow-hidden" aria-label="Why choose us">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" ref={ref}>
        <motion.div
          variants={fadeRight}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="text-center max-w-3xl mx-auto mb-16 lg:mb-20"
        >
          <span className="text-xs lg:text-sm font-semibold text-accent-400 tracking-[0.2em] uppercase mb-4 block">
            Why Choose Us
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6">
            The{' '}
            <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
              AeroXe
            </span>{' '}
            Advantage
          </h2>
          <p className="text-base lg:text-lg text-dark-300 leading-relaxed">
            We combine cutting-edge fiber technology with exceptional service 
            to deliver the best internet experience possible.
          </p>
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          {/* Left side - Illustration */}
          <motion.div
            variants={fadeLeft}
            initial="hidden"
            animate={isVisible ? 'visible' : 'hidden'}
            className="relative"
          >
            <div className="relative aspect-square max-w-lg mx-auto">
              {/* Decorative elements */}
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="w-full h-full relative">
                  {/* Central circle */}
                  <motion.div
                    className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-48 h-48 rounded-full bg-gradient-to-br from-accent-500/20 to-primary-500/20 blur-2xl"
                    animate={{ scale: [1, 1.2, 1] }}
                    transition={{ duration: 4, repeat: Infinity, ease: 'easeInOut' }}
                  />

                  {/* Fiber network visualization */}
                  <svg viewBox="0 0 400 400" className="w-full h-full">
                    <defs>
                      <linearGradient id="lineGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                        <stop offset="0%" stopColor="#06b6d4" stopOpacity="0.3" />
                        <stop offset="50%" stopColor="#0a66c2" stopOpacity="0.6" />
                        <stop offset="100%" stopColor="#06b6d4" stopOpacity="0.3" />
                      </linearGradient>
                    </defs>

                    {/* Network nodes */}
                    <circle cx="200" cy="200" r="60" fill="none" stroke="url(#lineGrad)" strokeWidth="1.5" />
                    <circle cx="200" cy="200" r="90" fill="none" stroke="url(#lineGrad)" strokeWidth="1" opacity="0.5" />
                    <circle cx="200" cy="200" r="120" fill="none" stroke="url(#lineGrad)" strokeWidth="0.5" opacity="0.3" />

                    {/* Connection lines */}
                    {[
                      { x1: 200, y1: 200, x2: 100, y2: 100 },
                      { x1: 200, y1: 200, x2: 300, y2: 100 },
                      { x1: 200, y1: 200, x2: 100, y2: 300 },
                      { x1: 200, y1: 200, x2: 300, y2: 300 },
                      { x1: 200, y1: 200, x2: 200, y2: 50 },
                      { x1: 200, y1: 200, x2: 200, y2: 350 },
                      { x1: 200, y1: 200, x2: 50, y2: 200 },
                      { x1: 200, y1: 200, x2: 350, y2: 200 },
                    ].map((line, i) => (
                      <motion.line
                        key={i}
                        x1={line.x1}
                        y1={line.y1}
                        x2={line.x2}
                        y2={line.y2}
                        stroke="#06b6d4"
                        strokeWidth="1"
                        opacity="0.15"
                        initial={{ pathLength: 0 }}
                        animate={{ pathLength: [0, 1, 0] }}
                        transition={{ duration: 3, delay: i * 0.2, repeat: Infinity }}
                      />
                    ))}

                    {/* Pulse dots on nodes */}
                    {[
                      { cx: 100, cy: 100 },
                      { cx: 300, cy: 100 },
                      { cx: 100, cy: 300 },
                      { cx: 300, cy: 300 },
                      { cx: 200, cy: 50 },
                      { cx: 200, cy: 350 },
                      { cx: 50, cy: 200 },
                      { cx: 350, cy: 200 },
                    ].map((dot, i) => (
                      <motion.circle
                        key={`dot-${i}`}
                        cx={dot.cx}
                        cy={dot.cy}
                        r="4"
                        fill="#06b6d4"
                        opacity="0.4"
                        animate={{ scale: [1, 1.5, 1], opacity: [0.4, 0.8, 0.4] }}
                        transition={{ duration: 2, delay: i * 0.3, repeat: Infinity }}
                      />
                    ))}

                    {/* Center node */}
                    <motion.circle
                      cx="200"
                      cy="200"
                      r="8"
                      fill="#06b6d4"
                      opacity="0.8"
                      animate={{ scale: [1, 1.3, 1] }}
                      transition={{ duration: 2, repeat: Infinity }}
                    />
                    <motion.circle
                      cx="200"
                      cy="200"
                      r="15"
                      fill="none"
                      stroke="#06b6d4"
                      strokeWidth="2"
                      opacity="0.3"
                      animate={{ scale: [1, 1.5, 1], opacity: [0.3, 0, 0.3] }}
                      transition={{ duration: 2, repeat: Infinity }}
                    />
                  </svg>

                  {/* Center icons */}
                  <motion.div
                    className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
                    animate={{ scale: [1, 1.1, 1] }}
                    transition={{ duration: 3, repeat: Infinity }}
                  >
                    <FiGlobe className="w-12 h-12 text-accent-400" />
                  </motion.div>
                </div>
              </div>
            </div>
          </motion.div>

          {/* Right side - Checklist */}
          <motion.div
            variants={staggerContainer}
            initial="hidden"
            animate={isVisible ? 'visible' : 'hidden'}
            className="space-y-4 lg:space-y-5"
          >
            {whyChooseUs.map((item, index) => (
              <motion.div
                key={item.title}
                variants={staggerItem}
                className="group flex items-start gap-4 p-4 lg:p-5 rounded-2xl bg-white/[0.02] border border-white/[0.04] hover:bg-white/[0.04] hover:border-accent-500/20 transition-all duration-300"
                whileHover={{ x: 4 }}
              >
                <div className="flex-shrink-0 w-10 h-10 rounded-xl bg-gradient-to-br from-accent-500/20 to-primary-500/20 flex items-center justify-center group-hover:scale-110 transition-transform duration-300">
                  <FiCheckCircle className="w-5 h-5 text-accent-400" />
                </div>
                <div>
                  <h3 className="text-base lg:text-lg font-semibold text-white mb-1.5">
                    {item.title}
                  </h3>
                  <p className="text-sm lg:text-base text-dark-400 leading-relaxed">
                    {item.description}
                  </p>
                </div>
              </motion.div>
            ))}
          </motion.div>
        </div>
      </div>
    </section>
  );
}
