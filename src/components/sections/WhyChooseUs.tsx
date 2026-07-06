import { motion } from 'framer-motion';
import { CheckCircle, ArrowRight } from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../ui/TiltCard';

const items = SITE_CONFIG.whyChooseUs.slice(0, 6);

export default function WhyChooseUs() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section id="about" className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-dark-950" />
      <div
        className="absolute inset-0"
        style={{
          background:
            'radial-gradient(ellipse at 40% 50%, rgba(6,182,212,0.06), transparent 50%), radial-gradient(ellipse at 70% 30%, rgba(10,102,194,0.04), transparent 50%)',
        }}
      />

      <motion.div
        className="absolute top-1/3 right-0 w-80 h-80 bg-primary-500/5 rounded-full blur-3xl"
        animate={{ scale: [1, 1.1, 1], y: [0, -20, 0] }}
        transition={{ duration: 10, repeat: Infinity, ease: 'easeInOut' }}
      />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <div className="grid lg:grid-cols-2 gap-10 lg:gap-16 items-center">
          <motion.div
            initial={{ opacity: 0, x: -30 }}
            animate={isVisible ? { opacity: 1, x: 0 } : {}}
            transition={{ duration: 0.6 }}
          >
            <motion.span
              initial={{ opacity: 0, scale: 0.9 }}
              animate={isVisible ? { opacity: 1, scale: 1 } : {}}
              transition={{ duration: 0.4, delay: 0.1 }}
              className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-5"
            >
              <CheckCircle className="w-4 h-4" />
              Built Different
            </motion.span>

            <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white leading-tight">
              Engineered for{' '}
              <span className="bg-gradient-to-r from-accent-300 to-primary-400 bg-clip-text text-transparent">
                Reliability
              </span>
            </h2>

            <p className="mt-4 text-base sm:text-lg text-dark-400 leading-relaxed max-w-xl">
              We don&apos;t just provide internet — we deliver an experience. Every layer of our network is built with redundancy, speed, and your satisfaction in mind.
            </p>

            <div className="mt-8 space-y-4">
              {items.slice(0, 3).map((item, i) => (
                <motion.div
                  key={item.title}
                  initial={{ opacity: 0, x: -20 }}
                  animate={isVisible ? { opacity: 1, x: 0 } : {}}
                  transition={{ duration: 0.4, delay: 0.3 + i * 0.08 }}
                >
                  <TiltCard tiltDegree={2} glareOpacity={0.06}>
                    <div className="flex items-start gap-4 p-4 rounded-xl hover:bg-white/[0.03] transition-colors duration-300 cursor-default group">
                      <div className="shrink-0 w-1 self-stretch rounded-full bg-gradient-to-b from-accent-400 to-primary-500 opacity-60 group-hover:opacity-100 transition-opacity" />
                      <div className="shrink-0 p-1.5 rounded-lg bg-accent-400/10 group-hover:bg-accent-400/20 transition-colors mt-0.5">
                        <CheckCircle className="w-4 h-4 text-accent-400" />
                      </div>
                      <div>
                        <h3 className="text-base font-semibold text-white">{item.title}</h3>
                        <p className="text-sm text-dark-400 leading-relaxed">{item.description}</p>
                      </div>
                    </div>
                  </TiltCard>
                </motion.div>
              ))}
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, x: 30 }}
            animate={isVisible ? { opacity: 1, x: 0 } : {}}
            transition={{ duration: 0.6, delay: 0.2 }}
          >
            <div className="rounded-2xl bg-white/[0.03] border border-white/[0.06] p-6 sm:p-8">
              <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2">
                <span className="w-1.5 h-1.5 rounded-full bg-accent-400 animate-pulse" />
                More reasons to switch
              </h3>
              <div className="space-y-1">
                {items.slice(3).map((item, i) => (
                  <motion.div
                    key={item.title}
                    initial={{ opacity: 0, y: 10 }}
                    animate={isVisible ? { opacity: 1, y: 0 } : {}}
                    transition={{ duration: 0.4, delay: 0.5 + i * 0.08 }}
                  >
                    <TiltCard tiltDegree={1} glareOpacity={0.04}>
                      <div className="flex items-center gap-3 px-4 py-3.5 rounded-xl hover:bg-white/[0.04] transition-colors duration-300 cursor-default group">
                        <CheckCircle className="w-4 h-4 text-accent-400 shrink-0" />
                        <span className="text-sm text-dark-200 group-hover:text-white transition-colors">
                          {item.title}
                        </span>
                        <ArrowRight className="w-3.5 h-3.5 text-dark-600 ml-auto shrink-0 group-hover:text-accent-400 transition-colors" />
                      </div>
                    </TiltCard>
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
