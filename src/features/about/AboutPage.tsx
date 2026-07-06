import { motion } from 'framer-motion';
import { Shield, Zap, Users, Target, ArrowRight } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';

const stats = [
  { icon: Users, label: 'Happy Customers', value: '1,200+' },
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
          <motion.span
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.4 }}
            className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-5"
          >
            <Users className="w-4 h-4" />
            About Us
          </motion.span>
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">
            Built for{' '}
            <span className="bg-gradient-to-r from-accent-300 to-primary-400 bg-clip-text text-transparent">
              {SITE_CONFIG.location.city}
            </span>
          </h1>
          <p className="mt-4 text-lg text-dark-400 max-w-3xl mx-auto">
            {SITE_CONFIG.company.description}
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="relative mb-16 p-6 sm:p-8 rounded-2xl overflow-hidden"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-accent-500/10 via-primary-500/5 to-accent-500/10" />
          <div className="absolute inset-0 border border-accent-400/10 rounded-2xl" />
          <div className="relative grid sm:grid-cols-2 lg:grid-cols-4 gap-6 sm:gap-8">
            {stats.map((stat, i) => (
              <motion.div
                key={stat.label}
                initial={{ opacity: 0, y: 15 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.2 + i * 0.08 }}
                className="text-center"
              >
                <div className="p-2 rounded-xl bg-accent-400/10 w-fit mx-auto mb-3">
                  <stat.icon className="w-5 h-5 text-accent-400" />
                </div>
                <p className="text-2xl sm:text-3xl font-bold bg-gradient-to-r from-accent-300 to-primary-400 bg-clip-text text-transparent">
                  {stat.value}
                </p>
                <p className="text-sm text-dark-400 mt-1">{stat.label}</p>
              </motion.div>
            ))}
          </div>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
          className="rounded-2xl border border-white/[0.06] bg-white/[0.02] p-8 sm:p-12 mb-16"
        >
          <h2 className="text-2xl font-bold text-white mb-6 flex items-center gap-3">
            <span className="w-1.5 h-1.5 rounded-full bg-accent-400 animate-pulse" />
            Our Story
          </h2>
          <div className="space-y-4 text-dark-300 leading-relaxed max-w-3xl">
            <p>
              Founded in {SITE_CONFIG.location.city}, {SITE_CONFIG.company.name} was built with a simple mission: 
              deliver world-class fiber internet to every home and business in our community.
            </p>
            <p>
              We believe that fast, reliable internet is no longer a luxury — it&apos;s a necessity. 
              That&apos;s why we&apos;ve invested in state-of-the-art fiber optic infrastructure to provide 
              our customers with the speeds they need to work, learn, stream, and play.
            </p>
            <p>
              Our team of local experts is passionate about connectivity. We&apos;re not just an ISP — 
              we&apos;re your neighbors, and we take pride in delivering the best possible internet 
              experience with 24/7 local support.
            </p>
          </div>

          <div className="mt-10 grid sm:grid-cols-3 gap-4">
            {[
              { title: 'Mission', desc: 'To provide reliable, high-speed fiber internet that empowers our community.', color: 'from-accent-400' },
              { title: 'Vision', desc: 'To be the most trusted internet service provider in Maharashtra.', color: 'from-primary-400' },
              { title: 'Values', desc: 'Reliability, transparency, local support, and customer-first approach.', color: 'from-accent-500' },
            ].map((item, i) => (
              <motion.div
                key={item.title}
                initial={{ opacity: 0, y: 15 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.5 + i * 0.08 }}
                className="group"
              >
                <div className="p-5 rounded-xl h-full hover:bg-white/[0.04] transition-colors duration-300 border border-transparent hover:border-white/[0.06]">
                  <div className={`w-8 h-1 rounded-full bg-gradient-to-r ${item.color} to-accent-300 mb-4 opacity-60 group-hover:opacity-100 transition-opacity`} />
                  <h3 className="text-lg font-semibold text-white mb-2">{item.title}</h3>
                  <p className="text-sm text-dark-400 leading-relaxed">{item.desc}</p>
                </div>
              </motion.div>
            ))}
          </div>
        </motion.div>
      </div>
    </div>
  );
}
