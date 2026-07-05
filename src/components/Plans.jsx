import { useState } from 'react';
import { motion } from 'framer-motion';
import { FiCheck, FiZap, FiHeadphones, FiShield, FiWifi, FiTool } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';
import { fadeUp, staggerContainer, staggerItem } from '../utils/animations';
import { useScrollTo } from '../hooks/useScrollTo';

/**
 * Plans Section Component
 * Displays all ISP plans as premium pricing cards with
 * 5 speed tiers × 4 durations, highlighting 12-month best value.
 */
export default function Plans() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.05 });
  const scrollTo = useScrollTo();
  const { plans } = SITE_CONFIG;
  const [selectedDuration, setSelectedDuration] = useState(12);

  const durations = [1, 3, 6, 12];

  const durationLabels = {
    1: '1 Month',
    3: '3 Months',
    6: '6 Months',
    12: '12 Months',
  };

  const durationBadges = {
    1: null,
    3: 'Save More',
    6: 'Save More',
    12: 'Best Value',
  };

  const featureIcons = {
    'Unlimited Data': FiZap,
    'Free Installation': FiTool,
    '24/7 Support': FiHeadphones,
    '99.99% Uptime': FiShield,
    'Dual Band WiFi Router Free*': FiWifi,
  };

  return (
    <section id="plans" className="relative py-20 lg:py-32 overflow-hidden" aria-label="Internet plans">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-900 via-dark-950 to-dark-900" />

      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" ref={ref}>
        {/* Section header */}
        <motion.div
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="text-center max-w-3xl mx-auto mb-12 lg:mb-16"
        >
          <span className="text-xs lg:text-sm font-semibold text-accent-400 tracking-[0.2em] uppercase mb-4 block">
            Our Plans
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6">
            Choose Your{' '}
            <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
              Perfect Plan
            </span>
          </h2>
          <p className="text-base lg:text-lg text-dark-300 leading-relaxed">
            Fiber internet plans starting from just ₹400/month. Flexible durations with increasing savings.
          </p>
        </motion.div>

        {/* Duration tabs */}
        <motion.div
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="flex items-center justify-center gap-2 mb-10 lg:mb-12"
        >
          {durations.map((months) => (
            <motion.button
              key={months}
              onClick={() => setSelectedDuration(months)}
              className={`relative px-5 py-2.5 text-sm font-semibold rounded-xl transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400 ${
                selectedDuration === months
                  ? 'text-white bg-gradient-to-r from-primary-600 to-accent-600 shadow-lg shadow-accent-500/25'
                  : 'text-dark-300 bg-white/[0.04] border border-white/[0.06] hover:bg-white/[0.08] hover:text-white'
              }`}
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
            >
              {durationLabels[months]}
              {durationBadges[months] && (
                <span className="absolute -top-2 -right-2 px-1.5 py-0.5 text-[10px] font-bold rounded-full bg-accent-400 text-dark-900">
                  {durationBadges[months]}
                </span>
              )}
            </motion.button>
          ))}
        </motion.div>

        {/* Plans grid */}
        <motion.div
          variants={staggerContainer}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 gap-4 lg:gap-5"
        >
          {plans.map((plan, planIndex) => {
            const durationData = plan.durations[selectedDuration];
            const isPopular = plan.popular;
            const is12Month = selectedDuration === 12;

            return (
              <motion.div
                key={plan.speed}
                variants={staggerItem}
                className={`relative group rounded-2xl transition-all duration-500 ${
                  isPopular
                    ? 'bg-gradient-to-b from-accent-500/10 via-primary-500/5 to-transparent border-2 border-accent-500/40 shadow-xl shadow-accent-500/10'
                    : 'bg-gradient-to-b from-white/[0.04] to-transparent border border-white/[0.06] hover:border-accent-500/30'
                }`}
                whileHover={{ y: -6 }}
              >
                {/* Popular badge */}
                {isPopular && (
                  <div className="absolute -top-3 left-1/2 -translate-x-1/2 px-4 py-1 rounded-full bg-gradient-to-r from-accent-500 to-primary-500 text-xs font-bold text-white shadow-lg shadow-accent-500/30 whitespace-nowrap">
                    Most Popular
                  </div>
                )}

                {/* 12-month best value badge */}
                {is12Month && (
                  <div className="absolute top-3 right-3 px-2 py-0.5 rounded-lg bg-emerald-500/20 border border-emerald-500/30 text-[10px] font-bold text-emerald-400">
                    BEST VALUE
                  </div>
                )}

                <div className="p-5 lg:p-6">
                  {/* Speed & Tag */}
                  <div className="mb-4">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="text-xs font-semibold text-accent-400 tracking-wider uppercase">
                        {plan.tag}
                      </span>
                    </div>
                    <h3 className="text-2xl lg:text-3xl font-bold text-white">
                      {plan.speed}
                    </h3>
                  </div>

                  {/* Price */}
                  <div className="mb-5">
                    <div className="flex items-baseline gap-1">
                      <span className="text-sm text-dark-400">₹</span>
                      <span className="text-4xl lg:text-5xl font-bold text-white">
                        {durationData.price.toLocaleString('en-IN')}
                      </span>
                    </div>
                    <p className="text-sm text-dark-400 mt-1">
                      {durationData.label}
                    </p>
                    {durationData.savings && (
                      <p className="text-xs font-semibold text-emerald-400 mt-0.5">
                        {durationData.savings}
                      </p>
                    )}
                  </div>

                  {/* Divider */}
                  <div className="h-px bg-gradient-to-r from-transparent via-white/10 to-transparent mb-5" />

                  {/* Features */}
                  <ul className="space-y-2.5 mb-6">
                    {plan.features.map((feature) => {
                      const IconComponent = featureIcons[feature] || FiCheck;
                      const isRouter = feature.includes('Router');
                      return (
                        <li key={feature} className="flex items-start gap-2.5">
                          <IconComponent className={`w-4 h-4 mt-0.5 flex-shrink-0 ${
                            isRouter ? 'text-amber-400' : 'text-accent-400'
                          }`} />
                          <span className={`text-xs lg:text-sm ${
                            isRouter ? 'text-amber-300 font-medium' : 'text-dark-300'
                          }`}>
                            {feature}
                          </span>
                        </li>
                      );
                    })}
                  </ul>

                  {/* CTA */}
                  <motion.button
                    onClick={() => scrollTo('contact')}
                    className={`w-full py-3 text-sm font-semibold rounded-xl transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400 ${
                      isPopular
                        ? 'bg-gradient-to-r from-accent-600 to-primary-600 text-white hover:from-accent-500 hover:to-primary-500 shadow-lg'
                        : 'bg-white/[0.06] text-white border border-white/[0.1] hover:bg-white/[0.1] hover:border-accent-500/30'
                    }`}
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                  >
                    Get Connected
                  </motion.button>
                </div>
              </motion.div>
            );
          })}
        </motion.div>

        {/* Footer note */}
        <motion.p
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="text-center text-xs lg:text-sm text-dark-500 mt-8 max-w-2xl mx-auto leading-relaxed"
        >
          * Free Dual Band WiFi Router included with all 12-month plans.
          Free Installation available for limited period. Prices inclusive of all taxes.
          Terms and conditions apply.
        </motion.p>
      </div>
    </section>
  );
}
