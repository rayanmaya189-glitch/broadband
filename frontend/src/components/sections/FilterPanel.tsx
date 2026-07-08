import { motion } from 'framer-motion';
import { useFilterStore } from '../../store/filterStore';

const billingOptions = [
  { value: 1, label: 'Monthly' },
  { value: 3, label: 'Quarterly' },
  { value: 6, label: 'Half Yearly' },
  { value: 12, label: 'Yearly' },
];

export default function FilterPanel() {
  const billingPeriod = useFilterStore((s) => s.billingPeriod);
  const setBillingPeriod = useFilterStore((s) => s.setBillingPeriod);

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="mb-6 sm:mb-8"
    >
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-center gap-2 sm:gap-3">
        <span className="text-xs sm:text-sm font-medium text-dark-400 sm:mr-2">Billing:</span>
        <div className="flex flex-wrap gap-2 sm:gap-3">
          {billingOptions.map((opt) => {
            const isActive = billingPeriod === opt.value;
            return (
              <button
                key={opt.value}
                onClick={() => setBillingPeriod(opt.value)}
                className={`px-3 sm:px-4 py-2 sm:py-2.5 rounded-lg sm:rounded-xl text-xs sm:text-sm font-semibold transition-all duration-200 min-h-[40px] sm:min-h-[44px] flex items-center justify-center ${
                  isActive
                    ? 'bg-gradient-to-r from-accent-500/20 to-primary-500/20 text-accent-300 border border-accent-400/30 shadow-[inset_0_1px_0_rgba(6,182,212,0.1)]'
                    : 'bg-white/[0.04] text-dark-400 border border-white/[0.06] hover:bg-white/[0.08] hover:text-dark-200'
                }`}
              >
                <span className="hidden sm:inline">{opt.label}</span>
                <span className="sm:hidden">{opt.label.split(' ')[0]}</span>
              </button>
            );
          })}
        </div>
      </div>
    </motion.div>
  );
}
