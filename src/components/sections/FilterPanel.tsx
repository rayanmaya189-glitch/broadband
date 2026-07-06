import { motion } from 'framer-motion';
import { RotateCcw } from 'lucide-react';
import { useFilterStore } from '../../store/filterStore';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import type { UsageType } from '../../types';

const usageOptions: { value: UsageType; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'gaming', label: 'Gaming' },
  { value: 'streaming', label: 'Streaming' },
  { value: 'business', label: 'Business' },
];

const billingOptions = [
  { value: 1, label: 'Monthly' },
  { value: 3, label: 'Quarterly' },
  { value: 6, label: 'Half Yearly' },
  { value: 12, label: 'Yearly' },
];

export default function FilterPanel() {
  const { ref, isVisible } = useIntersectionObserver();
  const {
    speedRange, priceRange, usageType, billingPeriod,
    setSpeedRange, setPriceRange, setUsageType, setBillingPeriod, resetFilters,
  } = useFilterStore();

  return (
    <motion.div
      ref={ref}
      initial={{ opacity: 0, y: 20 }}
      animate={isVisible ? { opacity: 1, y: 0 } : {}}
      className="glass-card rounded-2xl p-4 sm:p-6 mb-8"
    >
      <div className="flex flex-wrap items-center justify-between gap-4 mb-6">
        <h3 className="text-lg font-bold text-white">Filter Plans</h3>
        <button
          onClick={resetFilters}
          className="flex items-center gap-1.5 text-sm text-dark-400 hover:text-accent-400 transition-colors"
        >
          <RotateCcw className="w-4 h-4" />
          Reset
        </button>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
        <div>
          <label className="block text-sm text-dark-400 mb-2">Speed: {speedRange[0]}-{speedRange[1]} Mbps</label>
          <input
            type="range"
            min={0}
            max={1000}
            value={speedRange[1]}
            onChange={(e) => setSpeedRange([speedRange[0], Number(e.target.value)])}
            className="w-full h-2 rounded-full appearance-none bg-dark-700 accent-accent-400 cursor-pointer"
          />
          <div className="flex justify-between text-xs text-dark-500 mt-1">
            <span>0 Mbps</span>
            <span>1000 Mbps</span>
          </div>
        </div>

        <div>
          <label className="block text-sm text-dark-400 mb-2">Max Price: ₹{priceRange[1]}</label>
          <input
            type="range"
            min={0}
            max={50000}
            step={100}
            value={priceRange[1]}
            onChange={(e) => setPriceRange([priceRange[0], Number(e.target.value)])}
            className="w-full h-2 rounded-full appearance-none bg-dark-700 accent-accent-400 cursor-pointer"
          />
          <div className="flex justify-between text-xs text-dark-500 mt-1">
            <span>₹0</span>
            <span>₹50,000</span>
          </div>
        </div>

        <div>
          <label className="block text-sm text-dark-400 mb-2">Usage Type</label>
          <div className="flex flex-wrap gap-2">
            {usageOptions.map((opt) => (
              <button
                key={opt.value}
                onClick={() => setUsageType(opt.value)}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-all ${
                  usageType === opt.value
                    ? 'bg-accent-400/20 text-accent-300 border border-accent-400/30'
                    : 'bg-white/[0.04] text-dark-400 border border-white/[0.06] hover:bg-white/[0.08]'
                }`}
              >
                {opt.label}
              </button>
            ))}
          </div>
        </div>

        <div>
          <label className="block text-sm text-dark-400 mb-2">Billing Period</label>
          <div className="flex flex-wrap gap-2">
            {billingOptions.map((opt) => (
              <button
                key={opt.value}
                onClick={() => setBillingPeriod(opt.value)}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-all ${
                  billingPeriod === opt.value
                    ? 'bg-accent-400/20 text-accent-300 border border-accent-400/30'
                    : 'bg-white/[0.04] text-dark-400 border border-white/[0.06] hover:bg-white/[0.08]'
                }`}
              >
                {opt.label}
              </button>
            ))}
          </div>
        </div>
      </div>
    </motion.div>
  );
}
