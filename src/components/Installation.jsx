import { motion } from 'framer-motion';
import { FiPhone, FiClipboard, FiTool, FiCheckCircle } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';
import { fadeUp, staggerContainer, staggerItem } from '../utils/animations';

const iconMap = {
  1: FiPhone,
  2: FiClipboard,
  3: FiTool,
  4: FiCheckCircle,
};

/**
 * Installation Process Component
 * Animated timeline showing the 4-step installation process.
 */
export default function Installation() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.1 });
  const { installationSteps } = SITE_CONFIG;

  return (
    <section id="installation" className="relative py-20 lg:py-32 overflow-hidden" aria-label="Installation process">
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
            Installation
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6">
            Get Connected in{' '}
            <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
              4 Simple Steps
            </span>
          </h2>
          <p className="text-base lg:text-lg text-dark-300 leading-relaxed">
            From sign-up to surfing — we make it fast and hassle-free.
          </p>
        </motion.div>

        {/* Timeline */}
        <div className="relative max-w-4xl mx-auto">
          {/* Vertical line */}
          <div className="absolute left-8 lg:left-1/2 top-0 bottom-0 w-0.5 bg-gradient-to-b from-accent-500/30 via-primary-500/20 to-accent-500/30 -translate-x-1/2 hidden lg:block" />

          {/* Steps */}
          <motion.div
            variants={staggerContainer}
            initial="hidden"
            animate={isVisible ? 'visible' : 'hidden'}
            className="space-y-8 lg:space-y-12"
          >
            {installationSteps.map((step, index) => {
              const IconComponent = iconMap[step.step] || FiCheckCircle;
              const isEven = index % 2 === 0;

              return (
                <motion.div
                  key={step.step}
                  variants={staggerItem}
                  className={`relative flex items-start gap-6 lg:gap-8 ${
                    isEven ? 'lg:flex-row' : 'lg:flex-row-reverse'
                  }`}
                >
                  {/* Content card */}
                  <div className={`flex-1 ${isEven ? 'lg:text-right' : 'lg:text-left'}`}>
                    <div
                      className={`p-5 lg:p-6 rounded-2xl bg-gradient-to-b from-white/[0.04] to-transparent border border-white/[0.06] hover:border-accent-500/20 transition-all duration-500 ${
                        isEven ? 'lg:mr-12' : 'lg:ml-12'
                      }`}
                    >
                      <span className="text-xs font-semibold text-accent-400 tracking-widest mb-2 block">
                        STEP {step.step}
                      </span>
                      <h3 className="text-lg lg:text-xl font-semibold text-white mb-2">
                        {step.title}
                      </h3>
                      <p className="text-sm lg:text-base text-dark-400 leading-relaxed">
                        {step.description}
                      </p>
                    </div>
                  </div>

                  {/* Timeline node */}
                  <div className="relative flex-shrink-0 z-10">
                    <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-accent-500 to-primary-600 flex items-center justify-center shadow-lg shadow-accent-500/25">
                      <IconComponent className="w-7 h-7 text-white" />
                    </div>
                  </div>

                  {/* Spacer for alignment */}
                  <div className="flex-1 hidden lg:block" />
                </motion.div>
              );
            })}
          </motion.div>
        </div>
      </div>
    </section>
  );
}
