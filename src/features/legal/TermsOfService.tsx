import { motion } from 'framer-motion';
import { FileText } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';

const sections = [
  {
    title: 'Acceptance of Terms',
    content: `By subscribing to or using {SITE_CONFIG.company.name} broadband internet services, you agree to be bound by these Terms of Service. If you do not agree, please do not use our services.`,
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
    content: `Any equipment provided by us (including WiFi routers) remains our property unless otherwise stated. You are responsible for the safekeeping of the equipment. In case of damage or loss, replacement charges may apply. Equipment provided free with annual plans is subject to our equipment policy.`,
  },
  {
    title: 'Service Level',
    content: `We strive to maintain 99.99% network uptime. However, we do not guarantee uninterrupted service and shall not be liable for service interruptions due to maintenance, network upgrades, force majeure events, or factors beyond our reasonable control.`,
  },
  {
    title: 'Limitation of Liability',
    content: `To the maximum extent permitted by law, {SITE_CONFIG.company.name} shall not be liable for any indirect, incidental, special, or consequential damages arising from your use of our services, including but not limited to loss of data, business interruption, or lost profits.`,
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
    content: `These terms are governed by the laws of India. Any disputes arising from these terms shall be subject to the exclusive jurisdiction of the courts in {SITE_CONFIG.location.city}, {SITE_CONFIG.location.state}.`,
  },
];

function formatContent(text: string) {
  return text.replace(/\{SITE_CONFIG\.company\.name\}/g, SITE_CONFIG.company.name)
    .replace(/\{SITE_CONFIG\.location\.city\}/g, SITE_CONFIG.location.city)
    .replace(/\{SITE_CONFIG\.location\.state\}/g, SITE_CONFIG.location.state);
}

export default function TermsOfService() {
  return (
    <div className="min-h-screen pt-24 pb-16 bg-dark-950">
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.06),transparent_50%)]" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <motion.span
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.4 }}
            className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-5"
          >
            <FileText className="w-4 h-4" />
            Legal
          </motion.span>
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">Terms of Service</h1>
          <p className="mt-4 text-lg text-dark-400">Last updated: July 2026</p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="rounded-2xl border border-white/[0.06] bg-white/[0.02] p-6 sm:p-10 lg:p-12"
        >
          <p className="text-dark-300 leading-relaxed mb-8">
            These Terms of Service govern your use of {SITE_CONFIG.company.name}&apos;s broadband internet services. By subscribing to or using our services, you agree to these terms.
          </p>

          <div className="space-y-6">
            {sections.map((section, i) => (
              <motion.div
                key={section.title}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.2 + i * 0.04 }}
              >
                <h2 className="text-lg font-semibold text-white mb-2 flex items-center gap-3">
                  <span className="w-1.5 h-1.5 rounded-full bg-accent-400 shrink-0" />
                  {section.title}
                </h2>
                <p className="text-dark-400 leading-relaxed pl-5">{formatContent(section.content)}</p>
              </motion.div>
            ))}
          </div>
        </motion.div>
      </div>
    </div>
  );
}
