import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Shield,
  Zap,
  Wifi,
  CheckCircle,
  ArrowRight,
} from 'lucide-react';

const COUNTRY_CODE = '91';

export default function Hero() {
  const [phone, setPhone] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const sendToWhatsApp = (fullNumber: string) => {
    const message = `
Hi, I want a NEW fiber internet connection.

📱 My Number: +${fullNumber}

Please check availability and share best plans.
    `;

    const encoded = encodeURIComponent(message.trim());
    const url = `https://wa.me/${fullNumber}?text=${encoded}`;

    window.open(url, '_blank', 'noopener,noreferrer');
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (loading) return;

    let cleaned = phone.replace(/\D/g, '');

    // Strict validation
    if (cleaned.length !== 10) {
      setError('Enter a valid 10-digit mobile number');
      return;
    }

    const fullNumber = `${COUNTRY_CODE}${cleaned}`;

    setError('');
    setLoading(true);

    setTimeout(() => {
      sendToWhatsApp(fullNumber);
      setLoading(false);
    }, 300);
  };

  return (
    <section className="relative min-h-screen flex items-center overflow-hidden">

      {/* Background */}
      <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950" />
      <div className="absolute inset-0 bg-grid opacity-30" />

      <div className="relative w-full z-10">
        <div className="max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 py-12 sm:py-20 lg:py-28 pt-24 sm:pt-28 lg:pt-32">

          <div className="grid lg:grid-cols-2 gap-8 sm:gap-12 lg:gap-16 items-center">

            {/* LEFT - Mobile First */}
            <div className="text-center lg:text-left space-y-6 sm:space-y-8">

              <div className="inline-flex items-center gap-2 px-3 sm:px-4 py-2 sm:py-2.5 rounded-full bg-green-500/10 border border-green-500/20 text-green-400 text-xs sm:text-sm font-medium mx-auto lg:mx-0">
                🔥 Free Installation + 1 Month Extra
              </div>

              <h1 className="text-3xl sm:text-4xl lg:text-5xl xl:text-6xl font-bold leading-tight">
                Lightning Fast Internet
                <br className="hidden sm:block" />
                <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
                  Up to 300 Mbps
                </span>
              </h1>

              <p className="mt-2 text-sm sm:text-base lg:text-lg text-dark-300 max-w-2xl mx-auto lg:mx-0 leading-relaxed">
                Unlimited data with 24/7 support. Plans starting at ₹400/month.
              </p>

              {/* Benefits */}
              <div className="mt-4 flex flex-col gap-2 sm:gap-3 text-left max-w-2xl mx-auto lg:mx-0">
                {[
                  'Unlimited Data – No FUP',
                  'Fast Activation',
                  'Local 24/7 Support',
                ].map((item) => (
                  <div key={item} className="flex items-center gap-2 text-xs sm:text-sm text-dark-200">
                    <CheckCircle className="w-4 h-4 sm:w-5 sm:h-5 flex-shrink-0 text-green-400" />
                    {item}
                  </div>
                ))}
              </div>

              {/* FORM - Mobile Optimized */}
              <form
                onSubmit={handleSubmit}
                className="mt-6 sm:mt-8 flex flex-col sm:flex-row items-stretch gap-2 sm:gap-3 max-w-2xl mx-auto lg:mx-0"
              >
                <div className="flex-1">
                  <input
                    type="tel"
                    inputMode="numeric"
                    maxLength={10}
                    placeholder="Enter 10-digit mobile"
                    value={phone}
                    onChange={(e) => {
                      const val = e.target.value.replace(/\D/g, '').slice(0, 10);
                      setPhone(val);
                      if (error) setError('');
                    }}
                    className="w-full px-4 sm:px-5 py-3 sm:py-3.5 rounded-lg bg-white/5 border border-white/10 text-white text-sm placeholder:text-dark-400 focus:outline-none focus:ring-2 focus:ring-accent-500 focus:border-transparent transition-all min-h-[48px]"
                    required
                  />

                  {error && (
                    <p className="text-red-400 text-xs mt-1.5">{error}</p>
                  )}
                </div>

                <button
                  type="submit"
                  disabled={loading}
                  className="group px-4 sm:px-5 py-3 sm:py-3 
                             bg-gradient-to-r from-accent-500 to-primary-600 
                             text-white font-semibold text-sm sm:text-base rounded-lg 
                             flex items-center justify-center gap-2 
                             hover:shadow-lg hover:shadow-accent-500/25 
                             transition-all duration-200 
                             disabled:opacity-70 disabled:cursor-not-allowed
                             min-h-[48px] sm:min-h-[44px]
                             whitespace-nowrap active:scale-95"
                >
                  {loading ? (
                    <>
                      <span className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                      <span className="hidden sm:inline">Checking...</span>
                      <span className="sm:hidden">Check...</span>
                    </>
                  ) : (
                    <>
                      <span className="hidden sm:inline">Check Availability</span>
                      <span className="sm:hidden">Check</span>
                      <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
                    </>
                  )}
                </button>
              </form>

              {/* Trust Strip - Mobile Optimized */}
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.01 }}
                className="mt-6 sm:mt-10 flex flex-col sm:flex-row flex-wrap gap-3 sm:gap-4 justify-center lg:justify-start"
              >
                {[
                  { icon: Shield, label: 'Reliable Fiber', value: 'Stability' },
                  { icon: Zap, label: '300 Mbps', value: 'Speed' },
                  { icon: Wifi, label: 'Unlimited', value: 'Data' },
                ].map((item, i) => (
                  <motion.div
                    key={item.label}
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: i * 0.1 }}
                    whileHover={{ y: -2 }}
                    className="flex items-center gap-2 px-3 sm:px-4 py-2.5 sm:py-3 rounded-xl 
                               bg-white/5 backdrop-blur-md border border-white/10
                               min-h-[44px]"
                  >
                    <div className="p-1.5 sm:p-2 rounded-lg bg-gradient-to-br from-accent-400/20 to-primary-500/20 flex-shrink-0">
                      <item.icon className="w-4 h-4 sm:w-5 sm:h-5 text-accent-400" />
                    </div>

                    <div className="min-w-0">
                      <p className="text-xs sm:text-sm font-semibold text-white line-clamp-1">{item.label}</p>
                      <p className="text-[10px] sm:text-xs text-dark-400">{item.value}</p>
                    </div>
                  </motion.div>
                ))}
              </motion.div>
            </div>

            {/* RIGHT - Hidden on mobile, visible on lg+ */}
            <div className="hidden lg:flex relative justify-center">
              <motion.div
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.6 }}
                className="relative w-full max-w-lg"
              >
                <div className="absolute inset-0 bg-gradient-to-r from-accent-500/10 via-primary-500/10 to-accent-500/10 rounded-3xl blur-2xl" />

                <img
                  src="/hero-influencer.png"
                  alt="AI Influencer"
                  className="relative w-full h-auto object-contain rounded-2xl z-10"
                />

                <div className="absolute bottom-4 left-4 bg-dark-900/80 backdrop-blur-xl px-3 py-2 rounded-lg border border-white/10 text-xs font-medium text-white">
                  Trusted across Jalgaon
                </div>
              </motion.div>
            </div>

          </div>
        </div>
      </div>
    </section>
  );
}