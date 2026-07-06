import { motion } from 'framer-motion';
import { RotateCcw } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';

const sections = [
  {
    title: 'Cancellation Within Cooling-Off Period',
    content: `If you cancel your subscription within 7 days of the initial activation and before installation is completed, you are eligible for a full refund of the amount paid, minus any applicable administrative fees.`,
  },
  {
    title: 'Post-Installation Cancellation',
    content: `Once installation has been completed and service has been activated, refunds are provided on a pro-rata basis for the unused portion of the billing cycle. A service fee may apply for the active days of usage.`,
  },
  {
    title: 'Service Quality Issues',
    content: `If you experience persistent service quality issues that we are unable to resolve within 7 business days of being notified, you may cancel your subscription and receive a pro-rata refund for the remaining period.`,
  },
  {
    title: 'Non-Refundable Charges',
    content: `The following are non-refundable: installation fees (if any), equipment delivery charges, and any promotional discounts already availed. Fees for specialized services or custom configurations are also non-refundable.`,
  },
  {
    title: 'Equipment Return',
    content: `In case of cancellation, any equipment provided by {SITE_CONFIG.company.name} (including WiFi routers) must be returned within 7 days. Failure to return equipment may result in additional charges as per our equipment policy.`,
  },
  {
    title: 'Refund Processing',
    content: `Refunds are processed within 7-14 business days after the cancellation request is approved and equipment (if applicable) is returned. Refunds are credited to the original payment method or as store credit, at our discretion.`,
  },
  {
    title: 'How to Request a Refund',
    content: `To request a refund, please contact our support team at ${SITE_CONFIG.company.email} or call ${SITE_CONFIG.company.phone}. Include your account details and the reason for your cancellation request.`,
  },
];

function formatContent(text: string) {
  return text.replace(/\{SITE_CONFIG\.company\.name\}/g, SITE_CONFIG.company.name);
}

export default function RefundPolicy() {
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
            <RotateCcw className="w-4 h-4" />
            Legal
          </motion.span>
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">Refund & Cancellation Policy</h1>
          <p className="mt-4 text-lg text-dark-400">Last updated: July 2026</p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="rounded-2xl border border-white/[0.06] bg-white/[0.02] p-6 sm:p-10 lg:p-12"
        >
          <p className="text-dark-300 leading-relaxed mb-8">
            At {SITE_CONFIG.company.name}, we strive to ensure your satisfaction with our broadband services. This Refund & Cancellation Policy outlines the terms under which cancellations and refunds are processed.
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
