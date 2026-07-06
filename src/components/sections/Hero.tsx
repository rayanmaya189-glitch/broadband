import { Link } from 'react-router-dom';
import { motion, useScroll, useTransform } from 'framer-motion';
import { ArrowRight, Shield, Users, Zap, Wifi, Signal } from 'lucide-react';

export default function Hero() {
  const { scrollY } = useScroll();

  const bgY = useTransform(scrollY, [0, 500], [0, 120]);
  const opacity = useTransform(scrollY, [0, 400], [1, 0.7]);
  const imgScale = useTransform(scrollY, [0, 400], [1, 1.05]);
  const imgY = useTransform(scrollY, [0, 400], [0, -30]);

  return (
    <section className="relative min-h-screen flex items-center overflow-hidden">
      
      {/* Background */}
      <motion.div style={{ y: bgY }} className="absolute inset-0">
        <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950" />
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_20%_50%,rgba(6,182,212,0.08),transparent_50%),radial-gradient(ellipse_at_80%_30%,rgba(10,102,194,0.06),transparent_50%)]" />
        <div className="absolute inset-0 bg-grid opacity-30" />
      </motion.div>

      {/* Glow blob */}
      <div className="absolute top-1/3 left-1/4 w-80 h-80 bg-accent-500/10 rounded-full blur-3xl" />

      <motion.div style={{ opacity }} className="relative w-full z-10">
        <div className="max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 pt-24 pb-16 lg:pt-32 lg:pb-24">

          <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center min-h-[70vh]">

            {/* LEFT CONTENT */}
            <div className="text-center lg:text-left">
              
              <motion.div
                initial={{ opacity: 0, y: 25 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5 }}
              >
                <span className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-6">
                  <Zap className="w-4 h-4" />
                  Now Serving 1,200+ Happy Customers
                </span>
              </motion.div>

              <motion.h1
                initial={{ opacity: 0, y: 25 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: 0.1 }}
                className="text-4xl sm:text-5xl lg:text-6xl xl:text-7xl font-bold leading-tight"
              >
                Lightning Fast Internet
                <br />
                <span className="bg-gradient-to-r from-accent-400 via-accent-300 to-primary-400 bg-clip-text text-transparent">
                  for Your Home & Business
                </span>
              </motion.h1>

              <motion.p
                initial={{ opacity: 0, y: 25 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: 0.2 }}
                className="mt-6 text-lg sm:text-xl text-dark-300 max-w-xl mx-auto lg:mx-0"
              >
                Experience blazing-fast fiber internet with 99.99% uptime, unlimited data, and 24/7 local support.
              </motion.p>

              {/* CTA */}
              <motion.div
                initial={{ opacity: 0, y: 25 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: 0.3 }}
                className="mt-8 flex flex-col sm:flex-row gap-4 justify-center lg:justify-start"
              >
                <Link
                  to="/plans"
                  className="group inline-flex items-center justify-center gap-2 px-8 py-4 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-xl hover:shadow-accent-500/25 transition-all duration-300"
                >
                  View Plans
                  <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
                </Link>

                <Link
                  to="/check-availability"
                  className="inline-flex items-center justify-center px-8 py-4 border border-white/10 bg-white/5 backdrop-blur-sm text-white font-semibold rounded-xl hover:bg-white/10 transition-all duration-300"
                >
                  Check Availability
                </Link>
              </motion.div>

              {/* Stats */}
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.5 }}
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

            {/* RIGHT IMAGE */}
            <div className="relative flex justify-center lg:justify-end">

              <motion.div
                initial={{ opacity: 0, x: 60 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ duration: 0.7 }}
                className="relative w-full max-w-xl"
              >
                {/* Floating Cards */}
                <div className="absolute -top-6 right-4 z-20">
                  <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-dark-900/70 backdrop-blur-xl border border-white/[0.08] shadow-lg">
                    <Signal className="w-5 h-5 text-accent-400" />
                    <div>
                      <p className="text-xs font-bold text-white">300 Mbps</p>
                      <p className="text-[10px] text-accent-300">Fiber</p>
                    </div>
                  </div>
                </div>

                <div className="absolute -bottom-6 left-4 z-20">
                  <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-dark-900/70 backdrop-blur-xl border border-white/[0.08] shadow-lg">
                    <Wifi className="w-5 h-5 text-accent-400" />
                    <div>
                      <p className="text-xs font-bold text-white">Unlimited</p>
                      <p className="text-[10px] text-dark-400">No Caps</p>
                    </div>
                  </div>
                </div>

                {/* Glow */}
                <div className="absolute inset-0 bg-gradient-to-r from-accent-500/10 via-primary-500/10 to-accent-500/10 rounded-3xl blur-2xl" />

                {/* Image */}
                <motion.img
                  src="/hero-influencer.png"
                  alt="AI Influencer promoting internet service"
                  style={{ scale: imgScale, y: imgY }}
                  className="relative w-full h-auto max-h-[620px] object-contain rounded-2xl z-10"
                />
              </motion.div>
            </div>

          </div>
        </div>
      </motion.div>

      {/* Bottom fade */}
      <div className="absolute bottom-0 left-0 right-0 h-32 bg-gradient-to-t from-dark-950 to-transparent pointer-events-none" />
    </section>
  );
}