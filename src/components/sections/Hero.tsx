import { Link } from 'react-router-dom';
import { motion, useScroll, useTransform } from 'framer-motion';
import { ArrowRight, Shield, Users, Zap, Wifi, Signal, Sparkles } from 'lucide-react';
import TiltCard from '../ui/TiltCard';

export default function Hero() {
  const { scrollY } = useScroll();
  const bgY = useTransform(scrollY, [0, 500], [0, 150]);
  const opacity = useTransform(scrollY, [0, 400], [1, 0.6]);

  return (
    <section className="relative min-h-screen flex items-center overflow-hidden">
      <motion.div style={{ y: bgY }} className="absolute inset-0">
        <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950" />
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,rgba(6,182,212,0.15),transparent_50%),radial-gradient(ellipse_at_bottom_left,rgba(10,102,194,0.08),transparent_50%)]" />
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
      <motion.div
        className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-accent-400/5 rounded-full blur-3xl"
        animate={{ scale: [1, 1.15, 1], rotate: [0, 180, 360] }}
        transition={{ duration: 20, repeat: Infinity, ease: 'linear' }}
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
              className="hidden lg:flex flex-col items-center relative"
              style={{ perspective: '1000px' }}
            >
              <motion.div
                className="absolute -top-8 -right-4 z-20"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.8, type: 'spring', stiffness: 80 }}
              >
                <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-accent-500/20 backdrop-blur-xl border border-accent-400/30 shadow-lg shadow-accent-500/10">
                  <Signal className="w-5 h-5 text-accent-400" />
                  <div>
                    <p className="text-xs font-bold text-white">300 Mbps</p>
                    <p className="text-[10px] text-accent-300">Fiber Optic</p>
                  </div>
                </div>
              </motion.div>

              <motion.div
                className="absolute -bottom-6 -left-8 z-20"
                initial={{ opacity: 0, y: -10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 1, type: 'spring', stiffness: 80 }}
              >
                <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-dark-900/80 backdrop-blur-xl border border-white/[0.08] shadow-lg">
                  <Wifi className="w-5 h-5 text-accent-400" />
                  <div>
                    <p className="text-xs font-bold text-white">Unlimited</p>
                    <p className="text-[10px] text-dark-400">No Data Caps</p>
                  </div>
                </div>
              </motion.div>

              <motion.div
                className="absolute -top-4 -left-4 w-16 h-16 bg-accent-400/10 rounded-full blur-2xl"
                animate={{ scale: [1, 1.3, 1], opacity: [0.3, 0.6, 0.3] }}
                transition={{ duration: 4, repeat: Infinity, ease: 'easeInOut' }}
              />

              <TiltCard tiltDegree={3} glareOpacity={0.12} perspective={1000}>
                <div className="relative">
                  <div className="absolute -inset-1 bg-gradient-to-r from-accent-500/30 via-primary-500/30 to-accent-500/30 rounded-[32px] blur-xl" />
                  <div className="absolute -inset-[2px] bg-gradient-to-r from-accent-400/40 via-primary-500/40 to-accent-400/40 rounded-[32px] opacity-60" />
                  <div className="relative rounded-3xl overflow-hidden border border-white/[0.08]">
                    <img
                      src="/hero-influncer.png"
                      alt="AeroXe Broadband"
                      className="w-full h-auto max-h-[500px] object-cover"
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-dark-950/40 via-transparent to-transparent pointer-events-none" />
                    <div className="absolute inset-x-0 bottom-0 p-5 bg-gradient-to-t from-dark-950/80 to-transparent pointer-events-none">
                      <div className="flex items-center gap-2">
                        <Sparkles className="w-4 h-4 text-accent-400" />
                        <span className="text-sm font-semibold text-white">AeroXe Broadband — Speed You Can Trust</span>
                      </div>
                    </div>
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          </div>
        </div>
      </motion.div>
    </section>
  );
}
