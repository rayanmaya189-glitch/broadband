import { motion } from 'framer-motion';
import {
  Zap, Infinity, Monitor, Headphones, Shield, BadgePercent, Globe,
} from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';

const iconMap: Record<string, React.ReactNode> = {
  Zap: <Zap className="w-6 h-6" />,
  Infinity: <Infinity className="w-6 h-6" />,
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
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">
            Why <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">AeroXe?</span>
          </h2>
          <p className="mt-4 text-lg text-dark-400 max-w-2xl mx-auto">
            Everything you need for a seamless internet experience
          </p>
        </motion.div>

        <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
          {SITE_CONFIG.features.map((feature, i) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 30 }}
              animate={isVisible ? { opacity: 1, y: 0 } : {}}
              transition={{ duration: 0.5, delay: i * 0.08 }}
              className="glass-card rounded-2xl p-6 hover:bg-white/[0.06] transition-all duration-300 group"
            >
              <div className="p-3 rounded-xl bg-accent-400/10 group-hover:bg-accent-400/20 w-fit mb-4 transition-colors">
                <div className="text-accent-400">{iconMap[feature.icon] || <Zap className="w-6 h-6" />}</div>
              </div>
              <h3 className="text-lg font-semibold text-white mb-2">{feature.title}</h3>
              <p className="text-sm text-dark-400 leading-relaxed">{feature.description}</p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
