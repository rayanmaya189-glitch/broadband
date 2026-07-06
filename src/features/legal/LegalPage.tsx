import LegalTabs from './LegalTabs';
import { SITE_CONFIG } from '../../config/site';
import SEO from '../../components/seo/SEO';

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
    <>
      <SEO
        title="Privacy Policy — AeroXe Broadband"
        description="Read the AeroXe Broadband privacy policy. Learn how we collect, use, and protect your personal information when you use our fiber internet services in Jalgaon."
        path="/privacy"
      />
      <LegalTabs
        title="Privacy Policy"
        intro={`At ${SITE_CONFIG.company.name}, we take your privacy seriously. This Privacy Policy explains how we collect, use, disclose, and safeguard your information when you use our broadband internet services and visit our website.`}
        sections={sections}
      />
    </>
  );
}
