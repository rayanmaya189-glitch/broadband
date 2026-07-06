import { useState } from 'react';
import { motion } from 'framer-motion';
import { Search, MapPin, CheckCircle, XCircle, Loader2 } from 'lucide-react';
import { Link } from 'react-router-dom';
import { useAvailability } from '../../hooks/useAvailability';
import { SITE_CONFIG } from '../../config/site';
import { cn } from '../../utils/helpers';
import SEO from '../../components/seo/SEO';

export default function CheckAvailabilityPage() {
  const [location, setLocation] = useState('');
  const { mutate, data, isPending } = useAvailability();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (location.trim().length < 2) return;
    mutate(location.trim());
  };

  return (
    <>
      <SEO
        title="Check Fiber Internet Availability in Jalgaon"
        description="Check if AeroXe Broadband fiber internet is available in your area of Jalgaon. Enter your location or pincode to see coverage. Same-day activation in most areas."
        path="/check-availability"
      />
    <div className="min-h-screen pt-24 pb-16 bg-dark-950">
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.06),transparent_50%)]" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <span className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-primary-500/10 border border-primary-500/20 text-primary-300 text-sm font-medium mb-4">
            <MapPin className="w-4 h-4" />
            Check Availability
          </span>
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mt-3">
            Is {SITE_CONFIG.location.city} Covered?
          </h1>
          <p className="mt-4 text-lg text-dark-400">
            Enter your location below to check if our fiber internet is available at your address.
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="glass-card-strong rounded-2xl p-8"
        >
          <form onSubmit={handleSubmit} className="flex flex-col sm:flex-row gap-3">
            <div className="flex-1 relative">
              <MapPin className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-dark-400" />
              <input
                type="text"
                value={location}
                onChange={(e) => setLocation(e.target.value)}
                placeholder="Enter your area, city, or ZIP code..."
                className="w-full pl-12 pr-4 py-4 bg-white/[0.06] border border-white/[0.1] rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:border-accent-400/50 focus:ring-1 focus:ring-accent-400/30 transition-all"
                minLength={2}
              />
            </div>
            <button
              type="submit"
              disabled={isPending || location.trim().length < 2}
              className="inline-flex items-center justify-center gap-2 px-8 py-4 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-lg hover:shadow-accent-500/25 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending ? (
                <Loader2 className="w-5 h-5 animate-spin" />
              ) : (
                <Search className="w-5 h-5" />
              )}
              Check Coverage
            </button>
          </form>
        </motion.div>

        {data && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="mt-6"
          >
            {data.available ? (
              <div className="p-6 rounded-xl bg-accent-400/10 border border-accent-400/20">
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-6 h-6 text-accent-400 shrink-0 mt-0.5" />
                  <div>
                    <p className="text-lg font-semibold text-accent-300">
                      Great news! We serve <span className="text-white">{data.area}</span>
                    </p>
                    <p className="text-dark-400 mt-1">
                      Fiber internet is available at your location. Check out our plans below.
                    </p>
                    <Link
                      to="/plans"
                      className="inline-flex items-center gap-2 mt-3 px-5 py-2.5 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-medium rounded-xl hover:shadow-lg transition-all"
                    >
                      View Available Plans
                    </Link>
                  </div>
                </div>
              </div>
            ) : (
              <div className="p-6 rounded-xl bg-red-400/10 border border-red-400/20">
                <div className="flex items-start gap-3">
                  <XCircle className="w-6 h-6 text-red-400 shrink-0 mt-0.5" />
                  <div>
                    <p className="text-lg font-semibold text-red-300">
                      Not available in your area yet
                    </p>
                    <p className="text-dark-400 mt-1">
                      We're expanding rapidly. Contact us to be notified when we launch in your area.
                    </p>
                    <a
                      href={`tel:${SITE_CONFIG.company.phone}`}
                      className="inline-flex items-center gap-2 mt-3 px-5 py-2.5 bg-white/[0.06] border border-white/10 text-white font-medium rounded-xl hover:bg-white/10 transition-all"
                    >
                      Contact Us
                    </a>
                  </div>
                </div>
              </div>
            )}
          </motion.div>
        )}

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
          className="mt-16"
        >
          <h3 className="text-lg font-semibold text-white mb-6 text-center">Coverage Areas</h3>
          <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {SITE_CONFIG.coverageAreas.map((area) => (
              <div
                key={area.name}
                className={cn(
                  'p-4 rounded-xl border transition-all',
                  area.status === 'active'
                    ? 'bg-accent-400/5 border-accent-400/20'
                    : 'bg-white/[0.03] border-white/[0.06]'
                )}
              >
                <div className="flex items-center gap-3">
                  {area.status === 'active' ? (
                    <CheckCircle className="w-5 h-5 text-accent-400 shrink-0" />
                  ) : (
                    <XCircle className="w-5 h-5 text-dark-500 shrink-0" />
                  )}
                  <div>
                    <p className="text-sm font-medium text-white">{area.name}</p>
                    <p className="text-xs text-dark-500">
                      {area.status === 'active' ? 'Available Now' : 'Coming Soon'}
                    </p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </motion.div>
      </div>
    </div>
    </>
  );
}
