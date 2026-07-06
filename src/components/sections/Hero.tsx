import { Link } from 'react-router-dom';
import { motion, useScroll, useTransform } from 'framer-motion';
import { ArrowRight, Shield, Users, Zap } from 'lucide-react';

export default function Hero() {
  const { scrollY } = useScroll();
  const bgY = useTransform(scrollY, [0, 500], [0, 150]);
  const opacity = useTransform(scrollY, [0, 400], [1, 0.6]);

  return (
    <section className="relative min-h-screen flex items-center overflow-hidden">
      <motion.div style={{ y: bgY }} className="absolute inset-0">
        <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950" />
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,rgba(6,182,212,0.12),transparent_50%),radial-gradient(ellipse_at_bottom_left,rgba(10,102,194,0.08),transparent_50%)]" />
        <div className="absolute inset-0 bg-grid" />
      </motion.div>

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

      <motion.div style={{ opacity }} className="relative w-full">
        <div className="max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 pt-24 pb-16 lg:pt-32 lg:pb-24">
          <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
            <div className="text-center lg:text-left">
              <motion.div
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6 }}
              >
                <span className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-6">
                  <Zap className="w-4 h-4" />
                  Now Serving 1,200+ Happy Customers
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
                <span className="bg-gradient-to-r from-accent-400 via-accent-300 to-primary-400 bg-clip-text text-transparent drop-shadow-[0_0_20px_rgba(6,182,212,0.3)]">
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
                  { icon: Users, label: '1,200+ Users', value: 'Trusted' },
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
              initial={{ opacity: 0, scale: 0.9, rotateY: -10 }}
              animate={{ opacity: 1, scale: 1, rotateY: 0 }}
              transition={{ duration: 0.8, delay: 0.3, type: 'spring', stiffness: 60 }}
              className="hidden lg:flex items-center justify-center"
              style={{ perspective: '1000px' }}
            >
              <div className="relative w-full max-w-[500px]">
                {/* Glow effects behind the image */}
                <motion.div
                  className="absolute -inset-10 bg-gradient-to-r from-accent-500/20 via-primary-500/15 to-accent-500/20 rounded-full blur-3xl"
                  animate={{ scale: [1, 1.1, 1], opacity: [0.3, 0.5, 0.3] }}
                  transition={{ duration: 5, repeat: Infinity, ease: 'easeInOut' }}
                />

                {/* Decorative rings */}
                <div className="absolute inset-0 flex items-center justify-center">
                  <motion.div
                    className="w-[120%] h-[120%] rounded-full border border-accent-400/10"
                    animate={{ rotate: 360 }}
                    transition={{ duration: 30, repeat: Infinity, ease: 'linear' }}
                  />
                  <motion.div
                    className="absolute w-[140%] h-[140%] rounded-full border border-primary-400/5"
                    animate={{ rotate: -360 }}
                    transition={{ duration: 45, repeat: Infinity, ease: 'linear' }}
                  />
                </div>

                {/* Network lines */}
                <svg className="absolute inset-0 w-full h-full opacity-20" viewBox="0 0 400 400">
                  <motion.circle
                    cx="200" cy="200" r="150"
                    stroke="url(#glow-gradient)"
                    strokeWidth="0.5"
                    fill="none"
                    initial={{ pathLength: 0 }}
                    animate={{ pathLength: 1 }}
                    transition={{ duration: 3, ease: 'easeInOut' }}
                  />
                  <motion.circle
                    cx="200" cy="200" r="180"
                    stroke="url(#glow-gradient)"
                    strokeWidth="0.3"
                    fill="none"
                    initial={{ pathLength: 0 }}
                    animate={{ pathLength: 1 }}
                    transition={{ duration: 4, delay: 0.5, ease: 'easeInOut' }}
                  />
                  <defs>
                    <linearGradient id="glow-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                      <stop offset="0%" stopColor="#06b6d4" />
                      <stop offset="50%" stopColor="#0a66c2" />
                      <stop offset="100%" stopColor="#06b6d4" />
                    </linearGradient>
                  </defs>
                </svg>

                {/* Placeholder image container */}
                <div className="relative rounded-2xl overflow-hidden">
                  <div className="aspect-[16/10] bg-gradient-to-br from-dark-800 via-dark-900 to-dark-800 flex items-center justify-center">
                    {/* Replace this div with actual image: */}
                    {/* <img src="/hero-influencer.png" alt="Brand Ambassador" className="w-full h-full object-cover" /> */}
                    <div className="text-center p-8">
                      <div className="w-20 h-20 mx-auto mb-4 rounded-full bg-accent-400/10 flex items-center justify-center">
                        <svg className="w-10 h-10 text-accent-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" />
                        </svg>
                      </div>
                      <p className="text-sm text-dark-400">Place your</p>
                      <p className="text-sm text-dark-400">influencer image here</p>
                    </div>
                  </div>
                </div>

                {/* WiFi signal decorative element */}
                <motion.div
                  className="absolute -top-4 -right-4"
                  animate={{ scale: [1, 1.1, 1], opacity: [0.5, 0.8, 0.5] }}
                  transition={{ duration: 3, repeat: Infinity, ease: 'easeInOut' }}
                >
                  <div className="p-3 rounded-xl bg-accent-400/10 border border-accent-400/20">
                    <Zap className="w-6 h-6 text-accent-400" />
                  </div>
                </motion.div>

                {/* Speed badge */}
                <motion.div
                  className="absolute -bottom-4 -left-4"
                  animate={{ y: [0, -5, 0] }}
                  transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}
                >
                  <div className="px-4 py-2 rounded-xl bg-gradient-to-r from-accent-500 to-primary-600 text-white text-sm font-bold shadow-lg shadow-accent-500/25">
                    300 Mbps
                  </div>
                </motion.div>
              </div>
            </motion.div>
          </div>
        </div>
      </motion.div>
    </section>
  );
}
