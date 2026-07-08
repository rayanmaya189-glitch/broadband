import { create } from 'zustand';
import type { Plan } from '../types';

interface PlanStore {
  selectedPlan: Plan | null;
  comparisonPlans: Plan[];
  setSelectedPlan: (plan: Plan | null) => void;
  toggleComparisonPlan: (plan: Plan) => void;
  clearComparison: () => void;
}

export const usePlanStore = create<PlanStore>((set) => ({
  selectedPlan: null,
  comparisonPlans: [],
  setSelectedPlan: (plan) => set({ selectedPlan: plan }),
  toggleComparisonPlan: (plan) =>
    set((s) => {
      const exists = s.comparisonPlans.find((p) => p.id === plan.id);
      if (exists) {
        return { comparisonPlans: s.comparisonPlans.filter((p) => p.id !== plan.id) };
      }
      if (s.comparisonPlans.length >= 3) {
        return { comparisonPlans: [...s.comparisonPlans.slice(1), plan] };
      }
      return { comparisonPlans: [...s.comparisonPlans, plan] };
    }),
  clearComparison: () => set({ comparisonPlans: [] }),
}));
