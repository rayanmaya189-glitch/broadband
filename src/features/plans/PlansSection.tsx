import { usePlans } from '../../hooks/usePlans';
import { useFilterStore } from '../../store/filterStore';
import { usePlanStore } from '../../store/planStore';
import PlanCard from '../../components/sections/PlanCard';
import PlanComparison from '../../components/sections/PlanComparison';
import FilterPanel from '../../components/sections/FilterPanel';
import { PlanCardSkeleton } from '../../components/ui/Skeleton';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { motion } from 'framer-motion';

export default function PlansSection() {
  const { data: plans, isLoading } = usePlans();
  const billingPeriod = useFilterStore((s) => s.billingPeriod);
  const comparisonPlans = usePlanStore((s) => s.comparisonPlans);
  const toggleComparisonPlan = usePlanStore((s) => s.toggleComparisonPlan);
  const showComparison = usePlanStore((s) => s.comparisonPlans.length > 0);
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section id="plans" className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-dark-900/30" />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-12"
        >
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white leading-tight">
            Choose Your{' '}
            <span className="bg-gradient-to-r from-accent-300 via-accent-400 to-primary-400 bg-clip-text text-transparent drop-shadow-[0_0_20px_rgba(6,182,212,0.3)]">
              Perfect Plan
            </span>
          </h2>
          <p className="mt-4 text-lg text-dark-400 max-w-2xl mx-auto">
            Flexible plans designed for every need. All with unlimited data and free installation.
          </p>
        </motion.div>

        <FilterPanel />

        <PlanComparison billingPeriod={billingPeriod} />

        {isLoading ? (
          <div className="grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {[1, 2, 3, 4, 5].map((i) => (
              <PlanCardSkeleton key={i} />
            ))}
          </div>
        ) : plans && plans.length > 0 ? (
          <div className="grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {plans.map((plan, i) => (
              <div key={plan.id} className="relative">
                <PlanCard plan={plan} billingPeriod={billingPeriod} index={i} />
                <button
                  onClick={() => toggleComparisonPlan(plan)}
                  className={`mt-2 w-full text-xs py-2 rounded-lg border transition-all ${
                    comparisonPlans.find((p) => p.id === plan.id)
                      ? 'bg-accent-400/20 text-accent-300 border-accent-400/30'
                      : 'bg-white/[0.04] text-dark-400 border-white/[0.06] hover:bg-white/[0.08]'
                  }`}
                >
                  {comparisonPlans.find((p) => p.id === plan.id) ? 'Remove from Compare' : 'Add to Compare'}
                </button>
              </div>
            ))}
          </div>
        ) : (
          <div className="text-center py-16">
            <p className="text-xl text-dark-400">No plans match your filters.</p>
            <p className="text-sm text-dark-500 mt-2">Try adjusting your speed or price range.</p>
          </div>
        )}
      </div>
    </section>
  );
}
