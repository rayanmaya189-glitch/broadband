import { create } from 'zustand';
import type { FilterState } from '../types';

interface FilterStore extends FilterState {
  setBillingPeriod: (period: number) => void;
  resetFilters: () => void;
}

const initialState: FilterState = {
  billingPeriod: 1,
};

export const useFilterStore = create<FilterStore>((set) => ({
  ...initialState,
  setBillingPeriod: (period) => set({ billingPeriod: period }),
  resetFilters: () => set(initialState),
}));
