import { useParams, Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Check, ArrowLeft, Phone, ArrowRight, Zap } from 'lucide-react';
import { usePlan } from '../../hooks/usePlans';
import { useFilterStore } from '../../store/filterStore';
import { formatPrice } from '../../utils/helpers';
import { SITE_CONFIG } from '../../config/site';
import Skeleton from '../../components/ui/Skeleton';

export default function PlanDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { data: plan, isLoading } = usePlan(id || '');
  const billingPeriod = useFilterStore((s) => s.billingPeriod);
  const setBillingPeriod = useFilterStore((s) => s.setBillingPeriod);

  if (isLoading) {
    return (
      <div className="min-h-screen pt-24 pb-16 px-4 bg-dark-950">
        <div className="max-w-4xl mx-auto space-y-6">
          <Skeleton className="h-8 w-32" />
          <Skeleton className="h-16 w-64" />
          <Skeleton className="h-8 w-48" />
          <Skeleton className="h-64 w-full rounded-2xl" />
        </div>
      </div>
    );
  }

  if (!plan) {
    return (
      <div className="min-h-screen pt-24 pb-16 px-4 bg-dark-950 flex items-center justify-center">
        <div className="text-center">
          <p className="text-2xl text-dark-400">Plan not found</p>
          <Link to="/plans" className="mt-4 inline-flex items-center gap-2 text-accent-400 hover:text-accent-300">
            <ArrowLeft className="w-4 h-4" /> Back to Plans
          </Link>
        </div>
      </div>
    );
  }

  const monthlyPrice = plan.durations[billingPeriod]?.price ?? plan.durations[1].price;

  return (
    <div className="min-h-screen pt-24 pb-16 bg-dark-950">
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.08),transparent_50%)]" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <Link to="/plans" className="inline-flex items-center gap-2 text-dark-400 hover:text-accent-400 transition-colors mb-8">
          <ArrowLeft className="w-4 h-4" />
          Back to Plans
        </Link>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="glass-card-strong rounded-2xl p-8 sm:p-12"
        >
          {plan.popular && (
            <span className="inline-flex items-center gap-1.5 px-4 py-1.5 bg-gradient-to-r from-accent-500 to-primary-600 text-white text-xs font-bold rounded-full shadow-lg shadow-accent-500/25 mb-6">
              <Zap className="w-3.5 h-3.5" />
              Most Popular
            </span>
          )}

          <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-8">
            <div>
              <span className="text-sm text-dark-500 font-medium">up to</span>
              <h1 className="text-4xl sm:text-5xl font-bold text-white">{plan.speed}</h1>
              <p className="text-lg text-dark-400 mt-1">{plan.tag} Plan</p>
              <div className="mt-4 flex items-baseline gap-2">
                <span className="text-5xl sm:text-6xl font-bold text-white">
                  {formatPrice(monthlyPrice)}
                </span>
                <span className="text-xl text-dark-400">/mo</span>
              </div>
            </div>

            <div className="flex flex-wrap gap-2">
              {[1, 3, 6, 12].map((period) => {
                const d = plan.durations[period];
                return (
                  <button
                    key={period}
                    onClick={() => setBillingPeriod(period)}
                    className={`px-4 py-2 rounded-xl text-sm font-medium transition-all ${
                      billingPeriod === period
                        ? 'bg-accent-400/20 text-accent-300 border border-accent-400/30'
                        : 'bg-white/[0.04] text-dark-400 border border-white/[0.06] hover:bg-white/[0.08]'
                    }`}
                  >
                    {d?.label}
                    {d?.savings && (
                      <span className="block text-xs text-accent-500 mt-0.5">{d.savings}</span>
                    )}
                  </button>
                );
              })}
            </div>
          </div>

          <div className="mt-10 grid sm:grid-cols-2 gap-4">
            {plan.features.map((feature) => (
              <div key={feature} className="flex items-center gap-3 p-3 rounded-lg bg-white/[0.04]">
                <Check className="w-5 h-5 text-accent-400 shrink-0" />
                <span className="text-sm text-dark-200">{feature}</span>
              </div>
            ))}
          </div>

          <div className="mt-10 flex flex-col sm:flex-row gap-4">
            <a
              href={`https://wa.me/${SITE_CONFIG.whatsapp}?text=${encodeURIComponent(`Hi! I'm interested in the ${plan.speed} ${plan.tag} plan (₹${monthlyPrice}/mo). Please help me get connected.`)}`}
              target="_blank"
              rel="noopener noreferrer"
              className="flex-1 inline-flex items-center justify-center gap-2 px-8 py-4 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-xl hover:shadow-accent-500/25 transition-all duration-300"
            >
              Get Connected Now
              <ArrowRight className="w-5 h-5" />
            </a>
            <a
              href={`tel:${SITE_CONFIG.company.phone}`}
              className="flex-1 inline-flex items-center justify-center gap-2 px-8 py-4 border border-white/10 bg-white/5 backdrop-blur-sm text-white font-semibold rounded-xl hover:bg-white/10 hover:border-white/20 transition-all duration-300"
            >
              <Phone className="w-5 h-5" />
              Call {SITE_CONFIG.company.phone}
            </a>
          </div>
        </motion.div>
      </div>
    </div>
  );
}
