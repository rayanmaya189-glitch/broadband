import { FileText } from 'lucide-react';
import LegalLayout from './LegalLayout';
import { SITE_CONFIG } from '../../config/site';

const sections = [
  {
    title: 'Acceptance of Terms',
    content: `By subscribing to or using {name} broadband internet services, you agree to be bound by these Terms of Service. If you do not agree, please do not use our services.`,
  },
  {
    title: 'Service Description',
    content: `We provide fiber optic broadband internet services with speeds and features as described in your selected plan. Actual speeds may vary based on network conditions, equipment, and other factors. Speed tiers represent maximum achievable speeds and are not guaranteed at all times.`,
  },
  {
    title: 'Subscription & Billing',
    content: `Services are billed on a prepaid basis for the selected duration (monthly, quarterly, half-yearly, or yearly). Payments must be made before the service period begins. Late payments may result in service suspension. All fees are non-refundable except as expressly stated in our Refund Policy.`,
  },
  {
    title: 'Acceptable Use',
    content: `You agree to use our services in compliance with all applicable Indian laws and regulations. Prohibited activities include but are not limited to: unauthorized access to systems, distribution of malware, copyright infringement, sending spam, and any activity that disrupts our network or other users' experience.`,
  },
  {
    title: 'Equipment',
    content: `Any equipment provided by {name} (including WiFi routers) remains our property unless otherwise stated. You are responsible for the safekeeping of the equipment. In case of damage or loss, replacement charges may apply. Equipment provided free with annual plans is subject to our equipment policy.`,
  },
  {
    title: 'Service Level',
    content: `We strive to maintain 99.99% network uptime. However, we do not guarantee uninterrupted service and shall not be liable for service interruptions due to maintenance, network upgrades, force majeure events, or factors beyond our reasonable control.`,
  },
  {
    title: 'Limitation of Liability',
    content: `To the maximum extent permitted by law, {name} shall not be liable for any indirect, incidental, special, or consequential damages arising from your use of our services, including but not limited to loss of data, business interruption, or lost profits.`,
  },
  {
    title: 'Termination',
    content: `We reserve the right to suspend or terminate services for violation of these terms, non-payment, or any activity that threatens network integrity. You may terminate your subscription by providing written notice as per our cancellation policy.`,
  },
  {
    title: 'Modifications',
    content: `We reserve the right to modify these terms at any time. We will notify you of material changes via email or through our website. Continued use of services after changes constitutes acceptance of the updated terms.`,
  },
  {
    title: 'Governing Law',
    content: `These terms are governed by the laws of India. Any disputes arising from these terms shall be subject to the exclusive jurisdiction of the courts in {city}, {state}.`,
  },
];

function formatContent(text: string) {
  return text
    .replace(/\{name\}/g, SITE_CONFIG.company.name)
    .replace(/\{city\}/g, SITE_CONFIG.location.city)
    .replace(/\{state\}/g, SITE_CONFIG.location.state);
}

export default function TermsOfService() {
  return (
    <LegalLayout
      icon={<FileText className="w-3.5 h-3.5" />}
      title="Terms of Service"
      intro={`These Terms of Service govern your use of ${SITE_CONFIG.company.name}'s broadband internet services. By subscribing to or using our services, you agree to these terms.`}
      sections={sections}
      formatContent={formatContent}
      lastUpdated="July 2026"
    />
  );
}
