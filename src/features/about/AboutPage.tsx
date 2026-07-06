import { motion } from 'framer-motion';
import { Shield, Zap, Users, Target } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';

const stats = [
  { icon: Users, label: 'Happy Customers', value: '1200+' },
  { icon: Shield, label: 'Network Uptime', value: '99.99%' },
  { icon: Zap, label: 'Max Speed', value: '300 Mbps' },
  { icon: Target, label: 'Coverage Area', value: `${SITE_CONFIG.location.city}+` },
];

export default function AboutPage() {
  return (
    <div className="min-h-screen pt-24 pb-16 bg-dark-950">
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.06),transparent_50%)]" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-16"
        >
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">About Us</h1>
          <p className="mt-4 text-lg text-dark-400 max-w-3xl mx-auto">
            {SITE_CONFIG.company.description}
          </p>
        </motion.div>

        <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-16">
          {stats.map((stat, i) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.1 }}
              className="glass-card rounded-2xl p-6 text-center"
            >
              <div className="p-3 rounded-xl bg-accent-400/10 w-fit mx-auto mb-4">
                <stat.icon className="w-6 h-6 text-accent-400" />
              </div>
              <p className="text-3xl font-bold text-white">{stat.value}</p>
              <p className="text-sm text-dark-400 mt-1">{stat.label}</p>
            </motion.div>
          ))}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
          className="glass-card-strong rounded-2xl p-8 sm:p-12"
        >
          <h2 className="text-2xl font-bold text-white mb-6">Our Story</h2>
          <div className="space-y-4 text-dark-300 leading-relaxed">
            <p>
              Founded in {SITE_CONFIG.location.city}, {SITE_CONFIG.company.name} was built with a simple mission: 
              deliver world-class fiber internet to every home and business in our community.
            </p>
            <p>
              We believe that fast, reliable internet is no longer a luxury — it's a necessity. 
              That's why we've invested in state-of-the-art fiber optic infrastructure to provide 
              our customers with the speeds they need to work, learn, stream, and play.
            </p>
            <p>
              Our team of local experts is passionate about connectivity. We're not just an ISP — 
              we're your neighbors, and we take pride in delivering the best possible internet 
              experience with 24/7 local support.
            </p>
          </div>

          <div className="mt-8 grid sm:grid-cols-3 gap-6">
            {['Mission', 'Vision', 'Values'].map((title) => (
              <div key={title} className="p-6 rounded-xl bg-white/[0.04] border border-white/[0.06]">
                <h3 className="text-lg font-semibold text-white mb-3">{title}</h3>
                <p className="text-sm text-dark-400">
                  {title === 'Mission' && 'To provide reliable, high-speed fiber internet that empowers our community.'}
                  {title === 'Vision' && 'To be the most trusted internet service provider in Maharashtra.'}
                  {title === 'Values' && 'Reliability, transparency, local support, and customer-first approach.'}
                </p>
              </div>
            ))}
          </div>
        </motion.div>
      </div>
    </div>
  );
}
