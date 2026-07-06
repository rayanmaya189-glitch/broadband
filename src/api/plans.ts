import api from './axios';
import type { Plan, AvailabilityResult } from '../types';
import { SITE_CONFIG } from '../config/site';

export async function fetchPlans(): Promise<Plan[]> {
  try {
    const { data } = await api.get<Plan[]>('/plans');
    return data;
  } catch {
    return SITE_CONFIG.plans;
  }
}

export async function fetchPlanById(id: string): Promise<Plan | undefined> {
  try {
    const { data } = await api.get<Plan>(`/plans/${id}`);
    return data;
  } catch {
    return SITE_CONFIG.plans.find((p) => p.id === id);
  }
}

export async function checkAvailability(location: string): Promise<AvailabilityResult> {
  try {
    const { data } = await api.post<AvailabilityResult>('/coverage/check', { location });
    return data;
  } catch {
    const area = SITE_CONFIG.coverageAreas.find(
      (a) => a.name.toLowerCase().includes(location.toLowerCase()) && a.status === 'active'
    );
    if (area) {
      return { available: true, area: area.name, plans: SITE_CONFIG.plans };
    }
    return {
      available: false,
      area: location,
      plans: [],
    };
  }
}
