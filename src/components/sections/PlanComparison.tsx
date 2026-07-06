import { motion, AnimatePresence } from 'framer-motion';
import { X, Check } from 'lucide-react';
import { usePlanStore } from '../../store/planStore';
import { formatPrice } from '../../utils/helpers';

interface PlanComparisonProps {
  billingPeriod: number;
}

export default function PlanComparison({ billingPeriod }: PlanComparisonProps) {
  const comparisonPlans = usePlanStore((s) => s.comparisonPlans);
  const clearComparison = usePlanStore((s) => s.clearComparison);
  const toggleComparisonPlan = usePlanStore((s) => s.toggleComparisonPlan);

  if (comparisonPlans.length === 0) return null;

  const allFeatures = [...new Set(comparisonPlans.flatMap((p) => p.features))];

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, height: 0 }}
        animate={{ opacity: 1, height: 'auto' }}
        exit={{ opacity: 0, height: 0 }}
        className="mb-8 overflow-hidden"
      >
        <div className="glass-card-strong rounded-2xl p-6 overflow-x-auto">
          <div className="flex items-center justify-between mb-6">
            <h3 className="text-lg font-bold text-white">Compare Plans</h3>
            <button
              onClick={clearComparison}
              className="text-sm text-dark-400 hover:text-white transition-colors"
            >
              Clear all
            </button>
          </div>

          <table className="w-full min-w-[600px]">
            <thead>
              <tr>
                <th className="text-left text-sm text-dark-400 font-medium pb-4 pr-4">Features</th>
                {comparisonPlans.map((plan) => (
                  <th key={plan.id} className="pb-4 px-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="text-lg font-bold text-white">{plan.speed}</p>
                        <p className="text-sm text-dark-400">{plan.tag}</p>
                        <p className="text-xl font-bold text-accent-400 mt-1">
                          {formatPrice(plan.durations[billingPeriod]?.price ?? 0)}
                          <span className="text-sm text-dark-400">/mo</span>
                        </p>
                      </div>
                      <button
                        onClick={() => toggleComparisonPlan(plan)}
                        className="p-1 rounded-lg hover:bg-white/[0.06] text-dark-400 hover:text-white transition-all"
                      >
                        <X className="w-4 h-4" />
                      </button>
                    </div>
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {allFeatures.map((feature) => (
                <tr key={feature} className="border-t border-white/[0.06]">
                  <td className="py-3 pr-4 text-sm text-dark-300">{feature}</td>
                  {comparisonPlans.map((plan) => (
                    <td key={plan.id} className="py-3 px-4">
                      {plan.features.includes(feature) ? (
                        <Check className="w-5 h-5 text-accent-400" />
                      ) : (
                        <span className="text-dark-600">—</span>
                      )}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </motion.div>
    </AnimatePresence>
  );
}
