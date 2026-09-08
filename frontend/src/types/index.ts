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
  domain: string;
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
  socialMedia: {
    facebook: string;
    twitter: string;
    instagram: string;
    linkedin: string;
    youtube: string;
    whatsapp: string;
  };
  keywords: {
    primary: string[];
    secondary: string[];
    locationKeywords: string[];
  };
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

// ─── Customer Portal types ─────────────────────────────────────────────────

export interface CustomerUser {
  id: number;
  email: string;
  name: string;
  phone: string;
  branch_id?: number;
  status?: string;
}

export interface CustomerPlan {
  id: number;
  name: string;
  slug?: string;
  description?: string;
  speed_download?: string;
  speed_upload?: string;
  price_monthly?: number;
  features?: string[];
  is_popular?: boolean;
}

export interface CustomerSubscription {
  id: number;
  plan_id: number;
  plan_name?: string;
  status: string;
  start_date?: string;
  expiry_date?: string;
  monthly_fee?: number;
  download_kbps?: number;
  upload_kbps?: number;
}

export interface CustomerInvoice {
  id: number;
  invoice_number: string;
  amount: number;
  tax_amount?: number;
  total_amount?: number;
  status: string;
  due_date?: string;
  issued_at?: string;
  paid_at?: string;
}

export interface CustomerTicket {
  id: number;
  ticket_number?: string;
  subject: string;
  category?: string;
  priority: string;
  status: string;
  created_at?: string;
}

export interface CustomerTicketComment {
  id: number;
  user_name?: string;
  content: string;
  is_internal?: boolean;
  created_at?: string;
}
