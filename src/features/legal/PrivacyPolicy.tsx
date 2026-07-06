import { motion } from 'framer-motion';
import { Shield } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';

const sections = [
  {
    title: 'Information We Collect',
    content: `We collect information you provide directly, such as your name, address, email, phone number, and payment details when you subscribe to our services. We also collect usage data including bandwidth consumption, connection logs, and device information to maintain and improve our network.`,
  },
  {
    title: 'How We Use Your Information',
    content: `Your information is used to provide, maintain, and improve our broadband services; process transactions; send service-related communications; respond to your inquiries; and ensure network security and compliance with applicable laws.`,
  },
  {
    title: 'Data Sharing & Disclosure',
    content: `We do not sell your personal information to third parties. We may share data with government authorities as required by Indian law, with service providers who assist in our operations (subject to confidentiality agreements), or in connection with a business transfer.`,
  },
  {
    title: 'Data Retention',
    content: `We retain your personal information for as long as your account is active and for a reasonable period thereafter to comply with legal obligations, resolve disputes, and enforce our agreements. Usage logs are retained in accordance with Indian telecom regulations.`,
  },
  {
    title: 'Your Rights',
    content: `You have the right to access, update, or delete your personal information. You may also object to or restrict certain processing activities. To exercise these rights, please contact us using the information provided below.`,
  },
  {
    title: 'Security',
    content: `We implement industry-standard security measures including encryption, firewalls, and access controls to protect your data from unauthorized access, alteration, or destruction. However, no method of transmission over the internet is 100% secure.`,
  },
  {
    title: 'Cookies',
    content: `Our website uses cookies and similar technologies to enhance your browsing experience, analyze site traffic, and support our marketing efforts. You can control cookie preferences through your browser settings.`,
  },
  {
    title: 'Updates to This Policy',
    content: `We may update this Privacy Policy from time to time. We will notify you of material changes by posting the updated policy on this page and, where appropriate, through other communication channels.`,
  },
  {
    title: 'Contact Us',
    content: `If you have questions about this Privacy Policy or wish to exercise your data rights, please contact us at ${SITE_CONFIG.company.email} or call ${SITE_CONFIG.company.phone}.`,
  },
];

export default function PrivacyPolicy() {
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
            <Shield className="w-4 h-4" />
            Legal
          </motion.span>
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">Privacy Policy</h1>
          <p className="mt-4 text-lg text-dark-400">Last updated: July 2026</p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="rounded-2xl border border-white/[0.06] bg-white/[0.02] p-6 sm:p-10 lg:p-12"
        >
          <p className="text-dark-300 leading-relaxed mb-8">
            At {SITE_CONFIG.company.name}, we take your privacy seriously. This Privacy Policy explains how we collect, use, disclose, and safeguard your information when you use our broadband internet services and visit our website.
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
                <p className="text-dark-400 leading-relaxed pl-5">{section.content}</p>
              </motion.div>
            ))}
          </div>
        </motion.div>
      </div>
    </div>
  );
}
