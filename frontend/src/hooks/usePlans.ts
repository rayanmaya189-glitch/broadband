import { useQuery } from '@tanstack/react-query';
import { fetchPlans, fetchPlanById } from '../api/plans';
import { useFilterStore } from '../store/filterStore';

export function usePlans() {
  const billingPeriod = useFilterStore((s) => s.billingPeriod);

  return useQuery({
    queryKey: ['plans', billingPeriod],
    queryFn: fetchPlans,
    staleTime: 5 * 60 * 1000,
  });
}

export function usePlan(id: string) {
  return useQuery({
    queryKey: ['plan', id],
    queryFn: () => fetchPlanById(id),
    staleTime: 5 * 60 * 1000,
    enabled: !!id,
  });
}
