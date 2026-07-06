import { useState, useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { RotateCcw, Gauge, IndianRupee, Gamepad2, Monitor, Briefcase, Globe, ChevronDown } from 'lucide-react';
import { useFilterStore } from '../../store/filterStore';
import type { UsageType } from '../../types';

const usageOptions: { value: UsageType; label: string; icon: typeof Globe; desc: string }[] = [
  { value: 'all', label: 'All', icon: Globe, desc: 'Show all plans' },
  { value: 'gaming', label: 'Gaming', icon: Gamepad2, desc: '100 Mbps+' },
  { value: 'streaming', label: 'Streaming', icon: Monitor, desc: '50 Mbps+' },
  { value: 'business', label: 'Business', icon: Briefcase, desc: 'Business grade' },
];

const billingOptions = [
  { value: 1, label: 'Monthly', savings: null },
  { value: 3, label: 'Quarterly', savings: null },
  { value: 6, label: 'Half Yearly', savings: 'Save ₹150+' },
  { value: 12, label: 'Yearly', savings: 'Save ₹500+' },
];

function RangeSlider({
  value, min, max, step, onChange, icon: Icon, label, formatValue,
}: {
  value: number; min: number; max: number; step?: number;
  onChange: (v: number) => void;
  icon: typeof Gauge; label: string; formatValue: (v: number) => string;
}) {
  const pct = ((value - min) / (max - min)) * 100;

  return (
    <div>
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <span className="p-1.5 rounded-lg bg-accent-400/10">
            <Icon className="w-3.5 h-3.5 text-accent-400" />
          </span>
          <span className="text-sm font-medium text-white">{label}</span>
        </div>
        <span className="text-xs font-semibold text-accent-300 bg-accent-400/10 px-2.5 py-1 rounded-full">
          {formatValue(value)}
        </span>
      </div>

      <div className="relative h-7 flex items-center">
        <div className="absolute inset-x-0 h-2 rounded-full bg-dark-700" />
        <div
          className="absolute h-2 rounded-full bg-gradient-to-r from-accent-500 to-primary-500"
          style={{ width: `${pct}%` }}
        />
        <div
          className="absolute w-5 h-5 rounded-full bg-white border-[3px] border-accent-400 shadow-lg shadow-accent-400/20"
          style={{ left: `calc(${pct}% - 10px)` }}
        />
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          onChange={(e) => onChange(Number(e.target.value))}
          className="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
        />
      </div>

      <div className="flex justify-between text-xs text-dark-500 mt-1.5">
        <span>{formatValue(min)}</span>
        <span>{formatValue(max)}</span>
      </div>
    </div>
  );
}

function ActiveBadge({ count }: { count: number }) {
  if (count === 0) return null;
  return (
    <span className="inline-flex items-center justify-center w-5 h-5 rounded-full bg-accent-400 text-[10px] font-bold text-white">
      {count}
    </span>
  );
}

export default function FilterPanel() {
  const {
    speedRange, priceRange, usageType, billingPeriod,
    setSpeedRange, setPriceRange, setUsageType, setBillingPeriod, resetFilters,
  } = useFilterStore();

  const [showFilters, setShowFilters] = useState(true);

  const activeCount = useMemo(() => {
    let count = 0;
    if (speedRange[1] < 1000) count++;
    if (priceRange[1] < 50000) count++;
    if (usageType !== 'all') count++;
    return count;
  }, [speedRange, priceRange, usageType]);

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="mb-8"
    >
      <div className="rounded-2xl border border-white/[0.06] bg-dark-900/50 backdrop-blur-sm overflow-hidden">
        <button
          onClick={() => setShowFilters(!showFilters)}
          className="w-full flex items-center justify-between p-4 sm:p-5 hover:bg-white/[0.02] transition-colors"
        >
          <div className="flex items-center gap-3">
            <h3 className="text-base font-bold text-white">
              Filter Plans
            </h3>
            <ActiveBadge count={activeCount} />
          </div>
          <div className="flex items-center gap-3">
            {activeCount > 0 && (
              <span
                onClick={(e) => { e.stopPropagation(); resetFilters(); }}
                className="flex items-center gap-1 text-xs text-dark-400 hover:text-accent-400 transition-colors cursor-pointer"
              >
                <RotateCcw className="w-3 h-3" />
                Reset
              </span>
            )}
            <ChevronDown className={`w-4 h-4 text-dark-400 transition-transform duration-300 ${showFilters ? 'rotate-180' : ''}`} />
          </div>
        </button>

        <AnimatePresence initial={false}>
          {showFilters && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: 'auto', opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              transition={{ duration: 0.25, ease: 'easeInOut' }}
              className="overflow-hidden"
            >
          <div className="p-4 sm:p-5 pt-0 border-t border-white/[0.06]">
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 mb-5">
              <RangeSlider
                value={speedRange[1]}
                min={0}
                max={1000}
                step={50}
                onChange={(v) => setSpeedRange([speedRange[0], v])}
                icon={Gauge}
                label="Max Speed"
                formatValue={(v) => `${v} Mbps`}
              />

              <RangeSlider
                value={priceRange[1]}
                min={0}
                max={50000}
                step={100}
                onChange={(v) => setPriceRange([priceRange[0], v])}
                icon={IndianRupee}
                label="Max Price"
                formatValue={(v) => `₹${v.toLocaleString('en-IN')}`}
              />
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
              <div>
                <span className="text-sm font-medium text-white mb-3 block">Usage Type</span>
                <div className="grid grid-cols-2 gap-2">
                  {usageOptions.map((opt) => {
                    const isActive = usageType === opt.value;
                    const Icon = opt.icon;
                    return (
                      <button
                        key={opt.value}
                        onClick={() => setUsageType(opt.value)}
                        className={`group relative flex items-center gap-2.5 px-3 py-2.5 rounded-xl text-sm font-medium transition-all duration-200 ${
                          isActive
                            ? 'bg-gradient-to-r from-accent-500/20 to-primary-500/20 text-accent-300 border border-accent-400/30 shadow-[inset_0_1px_0_rgba(6,182,212,0.1)]'
                            : 'bg-white/[0.04] text-dark-400 border border-white/[0.06] hover:bg-white/[0.08] hover:text-dark-200'
                        }`}
                      >
                        <span className={`p-1 rounded-md ${isActive ? 'bg-accent-400/20' : 'bg-white/[0.06]'} transition-colors`}>
                          <Icon className={`w-3.5 h-3.5 ${isActive ? 'text-accent-300' : 'text-dark-400'}`} />
                        </span>
                        <div className="text-left">
                          <span className="block text-xs font-semibold">{opt.label}</span>
                          <span className={`block text-[10px] ${isActive ? 'text-accent-400/70' : 'text-dark-500'}`}>
                            {opt.desc}
                          </span>
                        </div>
                        {isActive && (
                          <span className="absolute -top-0.5 -right-0.5 w-2 h-2 rounded-full bg-accent-400 shadow-[0_0_6px_rgba(6,182,212,0.5)]" />
                        )}
                      </button>
                    );
                  })}
                </div>
              </div>

              <div>
                <span className="text-sm font-medium text-white mb-3 block">Billing Period</span>
                <div className="flex flex-col gap-2">
                  {billingOptions.map((opt) => {
                    const isActive = billingPeriod === opt.value;
                    return (
                      <button
                        key={opt.value}
                        onClick={() => setBillingPeriod(opt.value)}
                        className={`group relative flex items-center justify-between px-3.5 py-2.5 rounded-xl text-sm font-medium transition-all duration-200 ${
                          isActive
                            ? 'bg-gradient-to-r from-accent-500/20 to-primary-500/20 text-white border border-accent-400/30 shadow-[inset_0_1px_0_rgba(6,182,212,0.1)]'
                            : 'bg-white/[0.04] text-dark-400 border border-white/[0.06] hover:bg-white/[0.08] hover:text-dark-200'
                        }`}
                      >
                        <span className="flex items-center gap-2">
                          <span className={`w-2 h-2 rounded-full transition-all duration-200 ${
                            isActive ? 'bg-accent-400 shadow-[0_0_6px_rgba(6,182,212,0.5)]' : 'bg-dark-600'
                          }`} />
                          <span className="text-xs font-semibold">{opt.label}</span>
                        </span>
                        {opt.savings && (
                          <span className={`text-[10px] font-medium px-1.5 py-0.5 rounded-md ${
                            isActive
                              ? 'bg-accent-400/20 text-accent-300'
                              : 'bg-accent-400/5 text-accent-400/60'
                          }`}>
                            {opt.savings}
                          </span>
                        )}
                      </button>
                    );
                  })}
                </div>
              </div>
            </div>
          </div>
        </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
