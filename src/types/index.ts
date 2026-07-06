export interface PlanDuration {
  price: number;
  label: string;
  savings?: string;
}

export interface Plan {
  id: string;
  speed: string;
  speedMbps: number;
  tag: string;
  popular: boolean;
  durations: Record<number, PlanDuration>;
  features: string[];
  priceDisplay?: string;
}

export interface Feature {
  icon: string;
  title: string;
  description: string;
}

export interface WhyChooseUsItem {
  title: string;
  description: string;
}

export interface InstallationStep {
  step: number;
  title: string;
  description: string;
}

export interface Testimonial {
  name: string;
  role: string;
  feedback: string;
  rating: number;
}

export interface FAQ {
  question: string;
  answer: string;
}

export interface CoverageArea {
  name: string;
  status: 'active' | 'coming-soon';
  type: 'city' | 'area';
  pincodes?: string[];
}

export interface TeamMember {
  name: string;
  photo: string;
  designation: string;
  about: string;
}

export interface NavLink {
  label: string;
  href: string;
}

export interface SiteConfig {
  whatsapp: string;
  location: {
    mapUrl: string;
    city: string;
    state: string;
    country: string;
  };
  company: {
    name: string;
    legalName: string;
    tagline: string;
    description: string;
    phone: string;
    email: string;
    address: string;
  };
  social: Record<string, string>;
  navLinks: NavLink[];
  plans: Plan[];
  features: Feature[];
  whyChooseUs: WhyChooseUsItem[];
  installationSteps: InstallationStep[];
  testimonials: Testimonial[];
  faqs: FAQ[];
  coverageAreas: CoverageArea[];
  team: TeamMember[];
}

export interface AvailabilityResult {
  available: boolean;
  area: string;
  plans: Plan[];
}

export interface AuthState {
  token: string | null;
  isAuthenticated: boolean;
}

export interface FilterState {
  speedRange: [number, number];
  priceRange: [number, number];
  usageType: 'all' | 'gaming' | 'streaming' | 'business';
  billingPeriod: number;
}

export interface UIState {
  theme: 'dark' | 'light';
  isMobileMenuOpen: boolean;
  showComparison: boolean;
  toasts: Toast[];
}

export interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
}

export type UsageType = 'all' | 'gaming' | 'streaming' | 'business';
