import { motion } from 'framer-motion';
import { CheckCircle } from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../ui/TiltCard';

export default function WhyChooseUs() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section id="about" className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-dark-900/50" />
      <div
        className="absolute inset-0"
        style={{
          background: 'radial-gradient(ellipse at 30% 50%, rgba(6,182,212,0.06), transparent 60%), radial-gradient(ellipse at 70% 50%, rgba(10,102,194,0.04), transparent 60%)',
        }}
      />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">
            Built for <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">Reliability</span>
          </h2>
          <p className="mt-4 text-lg text-dark-400 max-w-2xl mx-auto">
            We don&apos;t just provide internet — we deliver an experience
          </p>
        </motion.div>

        <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {SITE_CONFIG.whyChooseUs.slice(0, 6).map((item, i) => (
            <motion.div
              key={item.title}
              initial={{ opacity: 0, x: -30 }}
              animate={isVisible ? { opacity: 1, x: 0 } : {}}
              transition={{ duration: 0.5, delay: i * 0.08, type: 'spring', stiffness: 70 }}
            >
              <TiltCard className="h-full" tiltDegree={4} glareOpacity={0.1}>
                <div className="flex items-start gap-4 p-6 rounded-2xl glass-card border border-white/[0.06] h-full transition-shadow duration-300 hover:shadow-2xl hover:shadow-accent-500/5 group">
                  <div className="shrink-0 p-2 rounded-xl bg-accent-400/10 group-hover:bg-accent-400/20 transition-colors">
                    <CheckCircle className="w-5 h-5 text-accent-400" />
                  </div>
                  <div>
                    <h3 className="text-lg font-semibold text-white mb-1">{item.title}</h3>
                    <p className="text-sm text-dark-400 leading-relaxed">{item.description}</p>
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
