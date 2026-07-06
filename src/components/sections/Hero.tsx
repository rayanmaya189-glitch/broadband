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
        <div className="max-w-[90rem] mx-auto px-4 sm:px-6 py-24">

          <div className="grid lg:grid-cols-2 gap-12 items-center">

            {/* LEFT */}
            <div className="text-center lg:text-left">

              <div className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-green-500/10 border border-green-500/20 text-green-400 text-sm font-medium mb-6">
                🔥 Free Installation + 1 Month Extra
              </div>

              <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold leading-tight">
                Lightning Fast Internet
                <br />
                <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
                  Up to 300 Mbps for Home & Business
                </span>
              </h1>

              <p className="mt-6 text-lg text-dark-300 max-w-xl mx-auto lg:mx-0">
                Unlimited data. Zero buffering. 99.99% uptime with 24/7 support.
                Plans starting at ₹400/month.
              </p>

              {/* Benefits */}
              <div className="mt-6 flex flex-col gap-3 text-left max-w-md mx-auto lg:mx-0">
                {[
                  'Unlimited Data – No FUP',
                  'Same-Day Activation',
                  'Local 24/7 Support',
                ].map((item) => (
                  <div key={item} className="flex items-center gap-2 text-sm text-dark-200">
                    <CheckCircle className="w-4 h-4 text-green-400" />
                    {item}
                  </div>
                ))}
              </div>

              {/* FORM */}
              <form
                onSubmit={handleSubmit}
                className="mt-8 flex flex-col sm:flex-row items-stretch gap-3 max-w-md mx-auto lg:mx-0"
              >
                <div className="flex-1">
                  <input
                    type="tel"
                    inputMode="numeric"
                    maxLength={10}
                    placeholder="Enter 10-digit mobile number"
                    value={phone}
                    onChange={(e) => {
                      const val = e.target.value.replace(/\D/g, '').slice(0, 10);
                      setPhone(val);
                      if (error) setError('');
                    }}
                    className="w-full px-5 py-3.5 rounded-lg bg-white/5 border border-white/10 text-white placeholder:text-dark-400 focus:outline-none focus:ring-2 focus:ring-accent-500"
                    required
                  />

                  {error && (
                    <p className="text-red-400 text-xs mt-2">{error}</p>
                  )}
                </div>

                <button
                  type="submit"
                  disabled={loading}
                  className="group px-5 py-3.5 sm:px-6 sm:py-3 
                             bg-gradient-to-r from-accent-500 to-primary-600 
                             text-white font-semibold rounded-lg 
                             flex items-center justify-center gap-2 
                             hover:shadow-lg hover:shadow-accent-500/25 
                             transition-all duration-200 
                             disabled:opacity-70 disabled:cursor-not-allowed"
                >
                  {loading ? (
                    <>
                      <span className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                      Checking...
                    </>
                  ) : (
                    <>
                      Check Availability
                      <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
                    </>
                  )}
                </button>
              </form>

              {/* Trust Strip */}
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.01 }}
                className="mt-10 flex flex-wrap gap-4 sm:gap-6 justify-center lg:justify-start"
              >
                {[
                  { icon: Shield, label: '99.99% Uptime', value: 'Reliability' },
                  { icon: Zap, label: 'Up to 300 Mbps', value: 'Speed' },
                  { icon: Wifi, label: 'Unlimited Data', value: 'Trusted' },
                ].map((item, i) => (
                  <motion.div
                    key={item.label}
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: i * 0.1 }}
                    whileHover={{ y: -3 }}
                    className="flex items-center gap-3 px-4 py-3 rounded-xl 
                               bg-white/5 backdrop-blur-md border border-white/10"
                  >
                    <div className="p-2 rounded-lg bg-gradient-to-br from-accent-400/20 to-primary-500/20">
                      <item.icon className="w-5 h-5 text-accent-400" />
                    </div>

                    <div>
                      <p className="text-sm font-semibold text-white">{item.label}</p>
                      <p className="text-xs text-dark-400">{item.value}</p>
                    </div>
                  </motion.div>
                ))}
              </motion.div>
            </div>

            {/* RIGHT */}
            <div className="relative flex justify-center">
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

                <div className="absolute bottom-4 left-4 bg-dark-900/80 backdrop-blur-xl px-4 py-2 rounded-xl border border-white/10 text-xs">
                  ⭐ Rated 4.9 by 1,200+ users
                </div>
              </motion.div>
            </div>

          </div>
        </div>
      </div>
    </section>
  );
}