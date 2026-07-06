import { RotateCcw } from 'lucide-react';
import LegalTabs from './LegalTabs';
import { SITE_CONFIG } from '../../config/site';

const sections = [
  {
    title: 'Cooling-Off Period',
    content: `If you cancel your subscription within 7 days of the initial activation and before installation is completed, you are eligible for a full refund of the amount paid, minus any applicable administrative fees.`,
  },
  {
    title: 'Post-Installation',
    content: `Once installation has been completed and service has been activated, refunds are provided on a pro-rata basis for the unused portion of the billing cycle. A service fee may apply for the active days of usage.`,
  },
  {
    title: 'Service Quality',
    content: `If you experience persistent service quality issues that we are unable to resolve within 7 business days of being notified, you may cancel your subscription and receive a pro-rata refund for the remaining period.`,
  },
  {
    title: 'Non-Refundable Charges',
    content: `The following are non-refundable: installation fees (if any), equipment delivery charges, and any promotional discounts already availed. Fees for specialized services or custom configurations are also non-refundable.`,
  },
  {
    title: 'Equipment Return',
    content: `In case of cancellation, any equipment provided by {name} (including WiFi routers) must be returned within 7 days. Failure to return equipment may result in additional charges as per our equipment policy.`,
  },
  {
    title: 'Refund Processing',
    content: `Refunds are processed within 7-14 business days after the cancellation request is approved and equipment (if applicable) is returned. Refunds are credited to the original payment method or as store credit, at our discretion.`,
  },
  {
    title: 'How to Request',
    content: `To request a refund, please contact our support team at ${SITE_CONFIG.company.email} or call ${SITE_CONFIG.company.phone}. Include your account details and the reason for your cancellation request.`,
  },
];

function formatContent(text: string) {
  return text.replace(/\{name\}/g, SITE_CONFIG.company.name);
}

export default function RefundPolicy() {
  return (
    <LegalTabs
      icon={<RotateCcw className="w-4 h-4 text-accent-400" />}
      title="Refund & Cancellation Policy"
      intro={`At ${SITE_CONFIG.company.name}, we strive to ensure your satisfaction with our broadband services. This Refund & Cancellation Policy outlines the terms under which cancellations and refunds are processed.`}
      sections={sections}
      formatContent={formatContent}
      lastUpdated="July 2026"
    />
  );
}
