import { create } from 'zustand';
import type { FilterState, UsageType } from '../types';

interface FilterStore extends FilterState {
  setSpeedRange: (range: [number, number]) => void;
  setPriceRange: (range: [number, number]) => void;
  setUsageType: (type: UsageType) => void;
  setBillingPeriod: (period: number) => void;
  resetFilters: () => void;
}

const initialState: FilterState = {
  speedRange: [0, 1000],
  priceRange: [0, 50000],
  usageType: 'all',
  billingPeriod: 1,
};

export const useFilterStore = create<FilterStore>((set) => ({
  ...initialState,
  setSpeedRange: (range) => set({ speedRange: range }),
  setPriceRange: (range) => set({ priceRange: range }),
  setUsageType: (type) => set({ usageType: type }),
  setBillingPeriod: (period) => set({ billingPeriod: period }),
  resetFilters: () => set(initialState),
}));
