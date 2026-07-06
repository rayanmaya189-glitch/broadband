import { motion } from 'framer-motion';
import {
  Zap, Infinity as InfinityIcon, Monitor, Headphones, Shield, BadgePercent, Globe,
} from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../ui/TiltCard';

const iconMap: Record<string, React.ReactNode> = {
  Zap: <Zap className="w-6 h-6" />,
  Infinity: <InfinityIcon className="w-6 h-6" />,
  Monitor: <Monitor className="w-6 h-6" />,
  Headphones: <Headphones className="w-6 h-6" />,
  Shield: <Shield className="w-6 h-6" />,
  BadgePercent: <BadgePercent className="w-6 h-6" />,
  Globe: <Globe className="w-6 h-6" />,
};

export default function Features() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section id="features" className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-dark-950" />
      <div
        className="absolute inset-0"
        style={{
          background:
            'radial-gradient(ellipse at 20% 20%, rgba(6,182,212,0.07), transparent 50%), radial-gradient(ellipse at 80% 80%, rgba(10,102,194,0.05), transparent 50%)',
        }}
      />

      <motion.div
        className="absolute top-40 left-0 w-72 h-72 bg-accent-500/5 rounded-full blur-3xl"
        animate={{ scale: [1, 1.2, 1], x: [0, 40, 0] }}
        transition={{ duration: 10, repeat: Infinity, ease: 'easeInOut' }}
      />
      <motion.div
        className="absolute bottom-40 right-0 w-96 h-96 bg-primary-500/5 rounded-full blur-3xl"
        animate={{ scale: [1.1, 1, 1.1], x: [0, -40, 0] }}
        transition={{ duration: 12, repeat: Infinity, ease: 'easeInOut' }}
      />

      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute top-1/4 left-1/4 w-px h-32 bg-gradient-to-b from-accent-400/20 to-transparent" />
        <div className="absolute bottom-1/4 right-1/3 w-px h-40 bg-gradient-to-t from-primary-400/20 to-transparent" />
      </div>

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-14"
        >
          <motion.span
            initial={{ opacity: 0, scale: 0.9 }}
            animate={isVisible ? { opacity: 1, scale: 1 } : {}}
            transition={{ duration: 0.4, delay: 0.1 }}
            className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-5"
          >
            <Shield className="w-4 h-4" />
            Why Choose Us
          </motion.span>

          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white leading-tight">
            Why{' '}
            <span className="bg-gradient-to-r from-accent-300 via-accent-400 to-primary-400 bg-clip-text text-transparent drop-shadow-[0_0_20px_rgba(6,182,212,0.3)]">
              {SITE_CONFIG.company.name}?
            </span>
          </h2>
          <p className="mt-4 text-lg text-dark-400 max-w-2xl mx-auto">
            Eight reasons why thousands trust us for their internet connectivity
          </p>
        </motion.div>

        <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-5">
          {SITE_CONFIG.features.map((feature, i) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 30 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ duration: 0.5, delay: i * 0.07, type: 'spring', stiffness: 80, damping: 15 }}
            >
              <TiltCard className="h-full" tiltDegree={4} glareOpacity={0.1}>
                <div className="group relative h-full rounded-2xl p-6 border border-white/[0.06] bg-white/[0.03] backdrop-blur-sm transition-all duration-400 hover:bg-white/[0.06] hover:border-accent-400/20 hover:shadow-xl hover:shadow-accent-500/5">
                  <div className="absolute top-0 right-0 text-[70px] font-bold text-white/[0.02] leading-none select-none pointer-events-none">
                    {String(i + 1).padStart(2, '0')}
                  </div>

                  <div className="relative z-10">
                    <div className="p-3 rounded-xl bg-gradient-to-br from-accent-400/15 to-accent-400/5 border border-accent-400/10 w-fit mb-4 group-hover:from-accent-400/25 group-hover:to-accent-400/10 group-hover:border-accent-400/20 transition-all duration-300">
                      <div className="text-accent-400">{iconMap[feature.icon] || <Zap className="w-6 h-6" />}</div>
                    </div>
                    <h3 className="text-base font-semibold text-white mb-2 group-hover:text-accent-200 transition-colors">
                      {feature.title}
                    </h3>
                    <p className="text-sm text-dark-400 leading-relaxed group-hover:text-dark-300 transition-colors">
                      {feature.description}
                    </p>
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.5, delay: 0.6 }}
          className="mt-12 p-6 rounded-2xl bg-gradient-to-r from-accent-500/5 via-primary-500/5 to-accent-500/5 border border-accent-400/10 text-center"
        >
          <p className="text-sm text-dark-400">
            <span className="text-accent-400 font-semibold">1,200+ customers</span> in {SITE_CONFIG.location.city} trust{' '}
            {SITE_CONFIG.company.name} for their home and business internet needs.
          </p>
        </motion.div>
      </div>
    </section>
  );
}
