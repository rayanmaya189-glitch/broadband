import { useState } from 'react';
import { motion } from 'framer-motion';
import { Search, MapPin, CheckCircle, XCircle, Loader2 } from 'lucide-react';
import { Link } from 'react-router-dom';
import { useAvailability } from '../../hooks/useAvailability';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';

export default function AvailabilityChecker() {
  const [location, setLocation] = useState('');
  const { ref, isVisible } = useIntersectionObserver();
  const { mutate, data, isPending } = useAvailability();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (location.trim().length < 2) return;
    mutate(location.trim());
  };

  return (
    <section id="coverage" className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-12"
        >
          <span className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-primary-500/10 border border-primary-500/20 text-primary-300 text-sm font-medium mb-4">
            <MapPin className="w-4 h-4" />
            Check Availability
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mt-3">
            Is {SITE_CONFIG.location.city} Covered?
          </h2>
          <p className="mt-4 text-lg text-dark-400 max-w-2xl mx-auto">
            Enter your location to check if our fiber internet is available at your address.
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6, delay: 0.2 }}
          className="max-w-xl mx-auto"
        >
          <form onSubmit={handleSubmit} className="relative">
            <div className="flex flex-col sm:flex-row gap-3">
              <div className="flex-1 relative">
                <MapPin className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-dark-400" />
                <input
                  type="text"
                  value={location}
                  onChange={(e) => setLocation(e.target.value)}
                  placeholder="Enter your area or ZIP code..."
                  className="w-full pl-12 pr-4 py-4 bg-white/[0.06] backdrop-blur-sm border border-white/[0.1] rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:border-accent-400/50 focus:ring-1 focus:ring-accent-400/30 transition-all"
                  minLength={2}
                />
              </div>
              <button
                type="submit"
                disabled={isPending || location.trim().length < 2}
                className="inline-flex items-center justify-center gap-2 px-8 py-4 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-lg hover:shadow-accent-500/25 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed shrink-0"
              >
                {isPending ? (
                  <Loader2 className="w-5 h-5 animate-spin" />
                ) : (
                  <Search className="w-5 h-5" />
                )}
                Check Coverage
              </button>
            </div>
          </form>

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
                        Check out our available plans below and get connected today.
                      </p>
                      <Link
                        to="/plans"
                        className="inline-flex items-center gap-2 mt-3 text-accent-400 hover:text-accent-300 font-medium transition-colors"
                      >
                        View Plans &rarr;
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
                    </div>
                  </div>
                </div>
              )}
            </motion.div>
          )}
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6, delay: 0.4 }}
          className="mt-16"
        >
          <h3 className="text-center text-lg font-semibold text-white mb-8">Coverage Areas</h3>
          <div className="flex flex-wrap justify-center gap-3">
            {SITE_CONFIG.coverageAreas.filter((a) => a.type === 'city').map((area) => (
              <span
                key={area.name}
                className={`inline-flex items-center gap-2 px-4 py-2 rounded-full text-sm font-medium ${
                  area.status === 'active'
                    ? 'bg-accent-400/10 text-accent-300 border border-accent-400/20'
                    : 'bg-dark-800 text-dark-400 border border-white/[0.06]'
                }`}
              >
                {area.status === 'active' ? (
                  <CheckCircle className="w-3.5 h-3.5" />
                ) : (
                  <XCircle className="w-3.5 h-3.5" />
                )}
                {area.name}
              </span>
            ))}
          </div>
        </motion.div>
      </div>
    </section>
  );
}
