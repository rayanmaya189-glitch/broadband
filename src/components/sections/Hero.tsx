import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { ArrowRight, Shield, Users, Zap } from 'lucide-react';
import SpeedVisual from '../ui/SpeedVisual';

export default function Hero() {
  return (
    <section className="relative min-h-screen flex items-center overflow-hidden">
      <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950" />
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,rgba(6,182,212,0.12),transparent_50%),radial-gradient(ellipse_at_bottom_left,rgba(10,102,194,0.08),transparent_50%)]" />
      <div className="absolute inset-0 bg-grid" />

      <motion.div
        className="absolute top-1/4 -right-32 w-96 h-96 bg-accent-500/10 rounded-full blur-3xl"
        animate={{ scale: [1, 1.2, 1], opacity: [0.3, 0.5, 0.3] }}
        transition={{ duration: 6, repeat: Infinity, ease: 'easeInOut' }}
      />
      <motion.div
        className="absolute bottom-1/4 -left-32 w-80 h-80 bg-primary-500/10 rounded-full blur-3xl"
        animate={{ scale: [1.2, 1, 1.2], opacity: [0.2, 0.4, 0.2] }}
        transition={{ duration: 8, repeat: Infinity, ease: 'easeInOut' }}
      />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 pt-24 pb-16 lg:pt-32 lg:pb-24">
        <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          <div className="text-center lg:text-left">
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6 }}
            >
              <span className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-6">
                <Zap className="w-4 h-4" />
                Now Serving {1200}+ Happy Customers
              </span>
            </motion.div>

            <motion.h1
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.1 }}
              className="text-4xl sm:text-5xl lg:text-6xl xl:text-7xl font-bold leading-tight text-balance"
            >
              Lightning Fast Internet
              <br />
              <span className="bg-gradient-to-r from-accent-400 via-accent-300 to-primary-400 bg-clip-text text-transparent">
                for Your Home & Business
              </span>
            </motion.h1>

            <motion.p
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.2 }}
              className="mt-6 text-lg sm:text-xl text-dark-300 max-w-xl mx-auto lg:mx-0 leading-relaxed"
            >
              Experience blazing-fast fiber optic internet with 99.99% uptime, unlimited data, and 24/7 local support. Plans starting at just ₹400/month.
            </motion.p>

            <motion.div
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.3 }}
              className="mt-8 flex flex-col sm:flex-row gap-4 justify-center lg:justify-start"
            >
              <Link
                to="/plans"
                className="group inline-flex items-center justify-center gap-2 px-8 py-4 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-xl hover:shadow-accent-500/25 transition-all duration-300 text-lg"
              >
                View Plans
                <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
              </Link>
              <Link
                to="/check-availability"
                className="inline-flex items-center justify-center px-8 py-4 border border-white/10 bg-white/5 backdrop-blur-sm text-white font-semibold rounded-xl hover:bg-white/10 hover:border-white/20 transition-all duration-300 text-lg"
              >
                Check Availability
              </Link>
            </motion.div>

            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.6, delay: 0.5 }}
              className="mt-10 flex flex-wrap gap-8 justify-center lg:justify-start"
            >
              {[
                { icon: Shield, label: '99.99% Uptime', value: 'Reliability' },
                { icon: Zap, label: 'Up to 300 Mbps', value: 'Speed' },
                { icon: Users, label: '1200+ Users', value: 'Trusted' },
              ].map((item) => (
                <div key={item.label} className="flex items-center gap-3">
                  <div className="p-2 rounded-lg bg-accent-400/10">
                    <item.icon className="w-5 h-5 text-accent-400" />
                  </div>
                  <div>
                    <p className="text-sm font-semibold text-white">{item.label}</p>
                    <p className="text-xs text-dark-500">{item.value}</p>
                  </div>
                </div>
              ))}
            </motion.div>
          </div>

          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.8, delay: 0.3 }}
            className="hidden lg:flex flex-col items-center"
          >
            <div className="relative">
              <div className="absolute inset-0 bg-gradient-to-r from-accent-500/20 to-primary-500/20 rounded-3xl blur-3xl" />
              <div className="relative glass-card-strong rounded-3xl p-8">
                <SpeedVisual speed={300} />
                <div className="mt-6 text-center">
                  <p className="text-sm text-dark-400">Real-time speed</p>
                  <p className="text-2xl font-bold text-white">Fiber Optic</p>
                  <p className="text-sm text-accent-400">Low Latency • High Bandwidth</p>
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  );
}
