import { motion } from 'framer-motion';
import { FiMapPin, FiCheckCircle, FiClock } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';
import { fadeUp, staggerContainer, staggerItem } from '../utils/animations';

/**
 * Coverage Section Component
 * Displays coverage areas with animated cards and a vector-style map illustration.
 */
export default function Coverage() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.1 });
  const { coverageAreas } = SITE_CONFIG;

  const activeCities = coverageAreas.filter((a) => a.status === 'active');
  const comingSoon = coverageAreas.filter((a) => a.status === 'coming-soon');

  return (
    <section id="coverage" className="relative py-20 lg:py-32 overflow-hidden" aria-label="Coverage areas">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" ref={ref}>
        {/* Section header */}
        <motion.div
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="text-center max-w-3xl mx-auto mb-16 lg:mb-20"
        >
          <span className="text-xs lg:text-sm font-semibold text-accent-400 tracking-[0.2em] uppercase mb-4 block">
            Coverage
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6">
            We're{' '}
            <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
              Expanding
            </span>{' '}
            Everywhere
          </h2>
          <p className="text-base lg:text-lg text-dark-300 leading-relaxed">
            Currently serving Jalgaon with rapid expansion to nearby cities underway.
          </p>
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-start">
          {/* Map visualization */}
          <motion.div
            variants={fadeUp}
            initial="hidden"
            animate={isVisible ? 'visible' : 'hidden'}
            className="relative"
          >
            <div className="relative aspect-[4/3] rounded-2xl overflow-hidden bg-gradient-to-br from-dark-800 to-dark-900 border border-white/[0.06] p-6 lg:p-8">
              {/* Vector map */}
              <svg viewBox="0 0 400 300" className="w-full h-full">
                <defs>
                  <radialGradient id="cityGlow">
                    <stop offset="0%" stopColor="#06b6d4" stopOpacity="0.6" />
                    <stop offset="100%" stopColor="#06b6d4" stopOpacity="0" />
                  </radialGradient>
                  <radialGradient id="cityGlowComing">
                    <stop offset="0%" stopColor="#0a66c2" stopOpacity="0.3" />
                    <stop offset="100%" stopColor="#0a66c2" stopOpacity="0" />
                  </radialGradient>
                </defs>

                {/* Grid lines */}
                {[50, 100, 150, 200, 250].map((y) => (
                  <line key={`h-${y}`} x1="20" y1={y} x2="380" y2={y} stroke="white" strokeOpacity="0.04" strokeWidth="0.5" />
                ))}
                {[60, 130, 200, 270, 340].map((x) => (
                  <line key={`v-${x}`} x1={x} y1="30" x2={x} y2="270" stroke="white" strokeOpacity="0.04" strokeWidth="0.5" />
                ))}

                {/* Connection lines between cities */}
                {[
                  { x1: 200, y1: 150, x2: 100, y2: 80 },
                  { x1: 200, y1: 150, x2: 300, y2: 100 },
                  { x1: 200, y1: 150, x2: 250, y2: 220 },
                  { x1: 200, y1: 150, x2: 150, y2: 230 },
                  { x1: 200, y1: 150, x2: 320, y2: 180 },
                  { x1: 200, y1: 150, x2: 80, y2: 170 },
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
                    animate={isVisible ? { pathLength: [0, 1] } : {}}
                    transition={{ duration: 2, delay: i * 0.1 }}
                  />
                ))}

                {/* Active city markers */}
                {[
                  { cx: 200, cy: 150, name: 'Jalgaon' },
                  { cx: 100, cy: 80, name: 'Jalgaon C.' },
                  { cx: 150, cy: 110, name: 'Shirpur Rd' },
                  { cx: 250, cy: 130, name: 'Mahabal' },
                  { cx: 180, cy: 200, name: 'Ravivar P' },
                  { cx: 220, cy: 100, name: 'Navipeth' },
                  { cx: 300, cy: 180, name: 'Bhusawal R' },
                  { cx: 120, cy: 190, name: 'MIDC' },
                ].map((city, i) => (
                  <g key={`active-${i}`}>
                    <circle cx={city.cx} cy={city.cy} r="20" fill="url(#cityGlow)" />
                    <motion.circle
                      cx={city.cx}
                      cy={city.cy}
                      r="4"
                      fill="#06b6d4"
                      animate={{ scale: [1, 1.5, 1], opacity: [0.8, 1, 0.8] }}
                      transition={{ duration: 2, delay: i * 0.2, repeat: Infinity }}
                    />
                    <text x={city.cx} y={city.cy + 15} textAnchor="middle" fill="#06b6d4" fontSize="8" fontWeight="600">
                      {city.name}
                    </text>
                  </g>
                ))}

                {/* Coming soon markers */}
                {[
                  { cx: 100, cy: 50, name: 'Bhusawal' },
                  { cx: 320, cy: 100, name: 'Mumbai' },
                  { cx: 280, cy: 140, name: 'Navi Mumb' },
                  { cx: 80, cy: 230, name: 'Barhanpur' },
                ].map((city, i) => (
                  <g key={`coming-${i}`}>
                    <circle cx={city.cx} cy={city.cy} r="15" fill="url(#cityGlowComing)" />
                    <circle cx={city.cx} cy={city.cy} r="3" fill="#0a66c2" opacity="0.6" />
                    <text x={city.cx} y={city.cy + 12} textAnchor="middle" fill="#0a66c2" fontSize="7" fontWeight="500">
                      {city.name}
                    </text>
                  </g>
                ))}
              </svg>

              {/* Map legend */}
              <div className="absolute bottom-4 left-4 right-4 flex items-center gap-6 text-xs">
                <div className="flex items-center gap-2">
                  <span className="w-2 h-2 rounded-full bg-accent-400" />
                  <span className="text-dark-400">Active</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="w-2 h-2 rounded-full bg-primary-600" />
                  <span className="text-dark-400">Coming Soon</span>
                </div>
              </div>
            </div>
          </motion.div>

          {/* Coverage lists */}
          <motion.div
            variants={staggerContainer}
            initial="hidden"
            animate={isVisible ? 'visible' : 'hidden'}
            className="space-y-8"
          >
            {/* Active cities */}
            <div>
              <h3 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
                <FiCheckCircle className="w-4 h-4 text-accent-400" />
                Available Now
              </h3>
              <div className="grid grid-cols-2 gap-3">
                {activeCities.map((area, i) => (
                  <motion.div
                    key={area.name}
                    variants={staggerItem}
                    className="flex items-center gap-3 p-3 rounded-xl bg-white/[0.03] border border-white/[0.04] hover:bg-accent-500/10 hover:border-accent-500/20 transition-all duration-300"
                    whileHover={{ x: 4 }}
                  >
                    <FiMapPin className="w-4 h-4 text-accent-400 flex-shrink-0" />
                    <span className="text-sm text-dark-200">{area.name}</span>
                  </motion.div>
                ))}
              </div>
            </div>

            {/* Coming soon */}
            <div>
              <h3 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
                <FiClock className="w-4 h-4 text-primary-400" />
                Coming Soon
              </h3>
              <div className="grid grid-cols-2 gap-3">
                {comingSoon.map((area, i) => (
                  <motion.div
                    key={area.name}
                    variants={staggerItem}
                    className="flex items-center gap-3 p-3 rounded-xl bg-white/[0.02] border border-white/[0.03] border-dashed opacity-60 hover:opacity-100 transition-all duration-300"
                    whileHover={{ x: 4 }}
                  >
                    <FiMapPin className="w-4 h-4 text-primary-400 flex-shrink-0" />
                    <span className="text-sm text-dark-300">{area.name}</span>
                  </motion.div>
                ))}
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  );
}
