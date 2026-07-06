import { useQuery } from '@tanstack/react-query';
import { fetchPlans, fetchPlanById } from '../api/plans';
import { useFilterStore } from '../store/filterStore';
import type { Plan, UsageType } from '../types';

const usageFilters: Record<UsageType, (plan: Plan) => boolean> = {
  all: () => true,
  gaming: (p) => p.speedMbps >= 100,
  streaming: (p) => p.speedMbps >= 50,
  business: (p) => p.features.includes('Business Grade') || p.speedMbps >= 200,
};

export function usePlans() {
  const { speedRange, priceRange, usageType, billingPeriod } = useFilterStore();

  return useQuery<Plan[]>({
    queryKey: ['plans', speedRange, priceRange, usageType, billingPeriod],
    queryFn: fetchPlans,
    staleTime: 5 * 60 * 1000,
    select: (plans) =>
      plans.filter((plan) => {
        const speed = plan.speedMbps;
        const price = plan.durations[billingPeriod]?.price ?? 0;
        const matchesSpeed = speed >= speedRange[0] && speed <= speedRange[1];
        const matchesPrice = price >= priceRange[0] && price <= priceRange[1];
        const matchesUsage = usageFilters[usageType](plan);
        return matchesSpeed && matchesPrice && matchesUsage;
      }),
  });
}

export function usePlan(id: string) {
  return useQuery<Plan | undefined>({
    queryKey: ['plan', id],
    queryFn: () => fetchPlanById(id),
    staleTime: 5 * 60 * 1000,
    enabled: !!id,
  });
}
