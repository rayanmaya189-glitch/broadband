import { motion } from 'framer-motion';
import {
  Zap, InfinityIcon, Monitor, Headphones, Shield, BadgePercent, Globe, ArrowRight,
} from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../ui/TiltCard';

const iconMap: Record<string, React.ReactNode> = {
  Zap: <Zap className="w-5 h-5" />,
  Infinity: <InfinityIcon className="w-5 h-5" />,
  Monitor: <Monitor className="w-5 h-5" />,
  Headphones: <Headphones className="w-5 h-5" />,
  Shield: <Shield className="w-5 h-5" />,
  BadgePercent: <BadgePercent className="w-5 h-5" />,
  Globe: <Globe className="w-5 h-5" />,
};

const coreFeatures = SITE_CONFIG.features.slice(0, 4);
const extendedFeatures = SITE_CONFIG.features.slice(4);

export default function Features() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section id="features" className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-dark-950" />
      <div
        className="absolute inset-0"
        style={{
          background:
            'radial-gradient(ellipse at 25% 30%, rgba(6,182,212,0.06), transparent 50%), radial-gradient(ellipse at 75% 70%, rgba(10,102,194,0.04), transparent 50%)',
        }}
      />

      <motion.div
        className="absolute top-60 left-0 w-64 h-64 bg-accent-500/5 rounded-full blur-3xl"
        animate={{ scale: [1, 1.15, 1], x: [0, 30, 0] }}
        transition={{ duration: 12, repeat: Infinity, ease: 'easeInOut' }}
      />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
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

        <div className="grid md:grid-cols-2 gap-6 mb-6">
          {coreFeatures.map((feature, i) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 30 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ duration: 0.5, delay: i * 0.1, type: 'spring', stiffness: 70, damping: 16 }}
            >
              <TiltCard tiltDegree={3} glareOpacity={0.08}>
                <div className="group relative h-full rounded-2xl p-6 sm:p-8 bg-white/[0.03] border border-white/[0.06] hover:border-accent-400/20 transition-all duration-500 hover:shadow-xl hover:shadow-accent-500/5">
                  <div className="flex gap-5">
                    <div className="shrink-0">
                      <div className="p-3 rounded-xl bg-gradient-to-br from-accent-400/15 to-accent-400/5 border border-accent-400/10 group-hover:from-accent-400/25 group-hover:to-accent-400/10 group-hover:border-accent-400/20 transition-all duration-300">
                        <div className="text-accent-400">{iconMap[feature.icon] || <Zap className="w-5 h-5" />}</div>
                      </div>
                    </div>
                    <div className="min-w-0">
                      <div className="flex items-center gap-3 mb-1.5">
                        <span className="text-xs font-mono text-accent-500/60">{(i + 1).toString().padStart(2, '0')}</span>
                        <h3 className="text-lg font-semibold text-white group-hover:text-accent-200 transition-colors">
                          {feature.title}
                        </h3>
                      </div>
                      <p className="text-sm text-dark-400 leading-relaxed group-hover:text-dark-300 transition-colors">
                        {feature.description}
                      </p>
                    </div>
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.5, delay: 0.4 }}
          className="relative mb-6 p-6 sm:p-8 rounded-2xl overflow-hidden"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-accent-500/10 via-primary-500/5 to-accent-500/10" />
          <div className="absolute inset-0 border border-accent-400/10 rounded-2xl" />
          <div className="relative grid sm:grid-cols-3 gap-6 sm:gap-8">
            {[
              { value: '1,200+', label: 'Happy Customers', desc: 'Across Jalgaon' },
              { value: '99.99%', label: 'Network Uptime', desc: 'Enterprise reliability' },
              { value: '24/7', label: 'Local Support', desc: 'Based in Jalgaon' },
            ].map((stat) => (
              <div key={stat.label} className="text-center">
                <p className="text-3xl sm:text-4xl font-bold bg-gradient-to-r from-accent-300 to-primary-400 bg-clip-text text-transparent">
                  {stat.value}
                </p>
                <p className="text-sm font-semibold text-white mt-1">{stat.label}</p>
                <p className="text-xs text-dark-500">{stat.desc}</p>
              </div>
            ))}
          </div>
        </motion.div>

        <div className="rounded-2xl border border-white/[0.06] bg-white/[0.02] overflow-hidden">
          {extendedFeatures.map((feature, i) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, x: -20 }}
              animate={isVisible ? { opacity: 1, x: 0 } : {}}
              transition={{ duration: 0.4, delay: 0.5 + i * 0.06 }}
            >
              <TiltCard tiltDegree={2} glareOpacity={0.05}>
                <div className={`flex items-center gap-4 px-5 sm:px-8 py-4 hover:bg-white/[0.03] transition-colors duration-300 cursor-default ${i < extendedFeatures.length - 1 ? 'border-b border-white/[0.04]' : ''}`}>
                  <div className="shrink-0 p-2 rounded-lg bg-accent-400/10">
                    <div className="text-accent-400">{iconMap[feature.icon] || <Zap className="w-4 h-4" />}</div>
                  </div>
                  <div className="min-w-0 flex-1">
                    <h3 className="text-sm font-semibold text-white">{feature.title}</h3>
                    <p className="text-xs text-dark-400 truncate">{feature.description}</p>
                  </div>
                  <ArrowRight className="w-4 h-4 text-dark-600 shrink-0 group-hover:text-accent-400 transition-colors" />
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
