import { useState, useEffect, useRef, useCallback } from 'react';
import { motion, AnimatePresence, useScroll, useSpring } from 'framer-motion';
import { Shield, FileText, RotateCcw, ArrowUpRight, Clock, ChevronRight } from 'lucide-react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { SITE_CONFIG } from '../../config/site';

interface Section {
  title: string;
  content: string;
}

interface TabData {
  id: string;
  icon: typeof Shield;
  title: string;
  subtitle: string;
  intro: string;
  sections: Section[];
  formatContent?: (text: string) => string;
}

const accentColors = [
  'from-accent-400 to-cyan-400',
  'from-primary-400 to-accent-400',
  'from-cyan-400 to-primary-400',
  'from-accent-300 to-primary-400',
  'from-primary-300 to-accent-400',
  'from-accent-400 to-primary-300',
];

const bgGlows = [
  'rgba(6,182,212,0.04)',
  'rgba(10,102,194,0.03)',
  'rgba(34,211,238,0.04)',
  'rgba(6,182,212,0.03)',
  'rgba(10,102,194,0.04)',
  'rgba(34,211,238,0.03)',
];

function extractFirstSentence(text: string): string {
  const match = text.match(/^[^.!?]*[.!?]/);
  return match ? match[0].trim() : text.slice(0, 120) + '...';
}

function splitContent(text: string): { highlight: string; rest: string } {
  const sentences = text.match(/[^.!?]*[.!?]/g) ?? [];
  if (sentences.length <= 1) return { highlight: '', rest: text };
  const mid = Math.ceil(sentences.length / 2);
  return {
    highlight: sentences.slice(0, mid).join(' ').trim(),
    rest: sentences.slice(mid).join(' ').trim(),
  };
}

function getSectionStyle(index: number): 'standard' | 'pullquote' | 'accent' {
  const cycle = index % 3;
  if (cycle === 0) return 'standard';
  if (cycle === 1) return 'pullquote';
  return 'accent';
}

const tabs: TabData[] = [
  {
    id: 'privacy',
    icon: Shield,
    title: 'Privacy Policy',
    subtitle: 'How we handle your data',
    intro: `At ${SITE_CONFIG.company.name}, we take your privacy seriously. This Privacy Policy explains how we collect, use, disclose, and safeguard your information when you use our broadband internet services and visit our website.`,
    sections: [
      { title: 'Information We Collect', content: `We collect information you provide directly, such as your name, address, email, phone number, and payment details when you subscribe to our services. We also collect usage data including bandwidth consumption, connection logs, and device information to maintain and improve our network.` },
      { title: 'How We Use Your Information', content: `Your information is used to provide, maintain, and improve our broadband services; process transactions; send service-related communications; respond to your inquiries; and ensure network security and compliance with applicable laws.` },
      { title: 'Data Sharing & Disclosure', content: `We do not sell your personal information to third parties. We may share data with government authorities as required by Indian law, with service providers who assist in our operations (subject to confidentiality agreements), or in connection with a business transfer.` },
      { title: 'Data Retention', content: `We retain your personal information for as long as your account is active and for a reasonable period thereafter to comply with legal obligations, resolve disputes, and enforce our agreements. Usage logs are retained in accordance with Indian telecom regulations.` },
      { title: 'Your Rights', content: `You have the right to access, update, or delete your personal information. You may also object to or restrict certain processing activities. To exercise these rights, please contact us using the information provided below.` },
      { title: 'Security', content: `We implement industry-standard security measures including encryption, firewalls, and access controls to protect your data from unauthorized access, alteration, or destruction. However, no method of transmission over the internet is 100% secure.` },
      { title: 'Cookies', content: `Our website uses cookies and similar technologies to enhance your browsing experience, analyze site traffic, and support our marketing efforts. You can control cookie preferences through your browser settings.` },
      { title: 'Updates to This Policy', content: `We may update this Privacy Policy from time to time. We will notify you of material changes by posting the updated policy on this page and, where appropriate, through other communication channels.` },
      { title: 'Contact Us', content: `If you have questions about this Privacy Policy or wish to exercise your data rights, please contact us at ${SITE_CONFIG.company.email} or call ${SITE_CONFIG.company.phone}.` },
    ],
  },
  {
    id: 'terms',
    icon: FileText,
    title: 'Terms of Service',
    subtitle: 'Rules & guidelines for using our service',
    intro: `These Terms of Service govern your use of ${SITE_CONFIG.company.name}'s broadband internet services. By subscribing to or using our services, you agree to these terms.`,
    sections: [
      { title: 'Acceptance of Terms', content: `By subscribing to or using {name} broadband internet services, you agree to be bound by these Terms of Service. If you do not agree, please do not use our services.` },
      { title: 'Service Description', content: `We provide fiber optic broadband internet services with speeds and features as described in your selected plan. Actual speeds may vary based on network conditions, equipment, and other factors. Speed tiers represent maximum achievable speeds and are not guaranteed at all times.` },
      { title: 'Subscription & Billing', content: `Services are billed on a prepaid basis for the selected duration (monthly, quarterly, half-yearly, or yearly). Payments must be made before the service period begins. Late payments may result in service suspension. All fees are non-refundable except as expressly stated in our Refund Policy.` },
      { title: 'Acceptable Use', content: `You agree to use our services in compliance with all applicable Indian laws and regulations. Prohibited activities include but are not limited to: unauthorized access to systems, distribution of malware, copyright infringement, sending spam, and any activity that disrupts our network or other users' experience.` },
      { title: 'Equipment', content: `Any equipment provided by {name} (including WiFi routers) remains our property unless otherwise stated. You are responsible for the safekeeping of the equipment. In case of damage or loss, replacement charges may apply. Equipment provided free with annual plans is subject to our equipment policy.` },
      { title: 'Service Level', content: `We strive to maintain 99.99% network uptime. However, we do not guarantee uninterrupted service and shall not be liable for service interruptions due to maintenance, network upgrades, force majeure events, or factors beyond our reasonable control.` },
      { title: 'Limitation of Liability', content: `To the maximum extent permitted by law, {name} shall not be liable for any indirect, incidental, special, or consequential damages arising from your use of our services, including but not limited to loss of data, business interruption, or lost profits.` },
      { title: 'Termination', content: `We reserve the right to suspend or terminate services for violation of these terms, non-payment, or any activity that threatens network integrity. You may terminate your subscription by providing written notice as per our cancellation policy.` },
      { title: 'Modifications', content: `We reserve the right to modify these terms at any time. We will notify you of material changes via email or through our website. Continued use of services after changes constitutes acceptance of the updated terms.` },
      { title: 'Governing Law', content: `These terms are governed by the laws of India. Any disputes arising from these terms shall be subject to the exclusive jurisdiction of the courts in {city}, {state}.` },
    ],
    formatContent: (text: string) =>
      text.replace(/\{name\}/g, SITE_CONFIG.company.name)
        .replace(/\{city\}/g, SITE_CONFIG.location.city)
        .replace(/\{state\}/g, SITE_CONFIG.location.state),
  },
  {
    id: 'refund',
    icon: RotateCcw,
    title: 'Refund & Cancellation',
    subtitle: 'Our refund and cancellation process',
    intro: `At ${SITE_CONFIG.company.name}, we strive to ensure your satisfaction with our broadband services. This Refund & Cancellation Policy outlines the terms under which cancellations and refunds are processed.`,
    sections: [
      { title: 'Cancellation Within Cooling-Off Period', content: `If you cancel your subscription within 7 days of the initial activation and before installation is completed, you are eligible for a full refund of the amount paid, minus any applicable administrative fees.` },
      { title: 'Post-Installation Cancellation', content: `Once installation has been completed and service has been activated, refunds are provided on a pro-rata basis for the unused portion of the billing cycle. A service fee may apply for the active days of usage.` },
      { title: 'Service Quality Issues', content: `If you experience persistent service quality issues that we are unable to resolve within 7 business days of being notified, you may cancel your subscription and receive a pro-rata refund for the remaining period.` },
      { title: 'Non-Refundable Charges', content: `The following are non-refundable: installation fees (if any), equipment delivery charges, and any promotional discounts already availed. Fees for specialized services or custom configurations are also non-refundable.` },
      { title: 'Equipment Return', content: `In case of cancellation, any equipment provided by {name} (including WiFi routers) must be returned within 7 days. Failure to return equipment may result in additional charges as per our equipment policy.` },
      { title: 'Refund Processing', content: `Refunds are processed within 7-14 business days after the cancellation request is approved and equipment (if applicable) is returned. Refunds are credited to the original payment method or as store credit, at our discretion.` },
      { title: 'How to Request a Refund', content: `To request a refund, please contact our support team at ${SITE_CONFIG.company.email} or call ${SITE_CONFIG.company.phone}. Include your account details and the reason for your cancellation request.` },
    ],
    formatContent: (text: string) => text.replace(/\{name\}/g, SITE_CONFIG.company.name),
  },
];

const tabFromPath: Record<string, string> = {
  privacy: 'privacy',
  terms: 'terms',
  refund: 'refund',
};

export default function LegalPage() {
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const pathTab = pathname.split('/').pop() ?? '';
  const [activeTab, setActiveTab] = useState(tabFromPath[pathTab] ?? 'privacy');
  const activeData = tabs.find((t) => t.id === activeTab)!;

  const sectionRefs = useRef<(HTMLDivElement | null)[]>([]);
  const contentRef = useRef<HTMLDivElement>(null);
  const [activeSection, setActiveSection] = useState('');

  const { scrollYProgress } = useScroll({ target: contentRef, offset: ['start start', 'end end'] });
  const scaleX = useSpring(scrollYProgress, { stiffness: 200, damping: 30 });

  useEffect(() => {
    setActiveSection(activeData.sections[0]?.title ?? '');
    sectionRefs.current = sectionRefs.current.slice(0, activeData.sections.length);
  }, [activeTab]);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveSection(entry.target.getAttribute('data-section') ?? '');
          }
        }
      },
      { rootMargin: '-80px 0px -55% 0px', threshold: 0 }
    );
    sectionRefs.current.forEach((ref) => { if (ref) observer.observe(ref); });
    return () => observer.disconnect();
  }, [activeData.sections]);

  const scrollTo = useCallback((title: string) => {
    const idx = activeData.sections.findIndex((s) => s.title === title);
    sectionRefs.current[idx]?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }, [activeData.sections]);

  const readingTime = Math.max(1, Math.ceil(
    activeData.sections.reduce((acc, s) => acc + s.content.length, 0) / 1500
  ));

  return (
    <div className="min-h-screen bg-dark-950">
      <motion.div
        className="fixed top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-accent-400 via-primary-400 to-accent-400 z-50 origin-left"
        style={{ scaleX }}
      />

      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.05),transparent_50%)] pointer-events-none" />
      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_bottom_right,rgba(10,102,194,0.03),transparent_50%)] pointer-events-none" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 pt-28 pb-20">
        <div className="flex flex-wrap items-center justify-center gap-3 mb-12 lg:mb-16">
          {tabs.map((tab, i) => {
            const isActive = activeTab === tab.id;
            const Icon = tab.icon;
            return (
              <motion.button
                key={tab.id}
                onClick={() => { setActiveTab(tab.id); navigate(`/${tab.id}`); }}
                layout
                className={`group relative flex items-center gap-3 px-4 sm:px-5 py-3 rounded-2xl text-left transition-all duration-300 ${
                  isActive
                    ? 'bg-gradient-to-br from-accent-500/20 via-primary-500/15 to-accent-500/20 shadow-[inset_0_1px_0_rgba(6,182,212,0.15)]'
                    : 'bg-white/[0.03] hover:bg-white/[0.06]'
                }`}
                style={isActive ? { boxShadow: '0 0 30px rgba(6,182,212,0.08), inset 0 1px 0 rgba(6,182,212,0.15)' } : {}}
              >
                {isActive && (
                  <motion.div
                    layoutId="tabGlow"
                    className="absolute inset-0 rounded-2xl border border-accent-400/30"
                    transition={{ type: 'spring', stiffness: 300, damping: 25 }}
                  />
                )}

                <span className={`relative flex items-center justify-center w-10 h-10 rounded-xl transition-all duration-300 ${
                  isActive
                    ? 'bg-gradient-to-br from-accent-400 to-primary-500 shadow-lg shadow-accent-400/20'
                    : 'bg-white/[0.06] group-hover:bg-white/[0.10]'
                }`}>
                  <Icon className={`w-5 h-5 ${isActive ? 'text-white' : 'text-dark-400 group-hover:text-dark-200'}`} />
                </span>

                <div className="relative min-w-0">
                  <div className="flex items-center gap-2">
                    <span className={`text-xs font-mono font-bold ${isActive ? 'text-accent-400' : 'text-dark-600'}`}>
                      {String(i + 1).padStart(2, '0')}
                    </span>
                    <span className={`text-sm font-bold whitespace-nowrap ${isActive ? 'text-white' : 'text-dark-300 group-hover:text-white'}`}>
                      {tab.title}
                    </span>
                  </div>
                  <p className={`text-[10px] leading-tight mt-0.5 ${isActive ? 'text-accent-300/70' : 'text-dark-500'}`}>
                    {tab.subtitle}
                  </p>
                </div>

                <div className={`absolute -bottom-px left-4 right-4 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/30 to-accent-400/0 transition-opacity ${isActive ? 'opacity-100' : 'opacity-0'}`} />
              </motion.button>
            );
          })}
        </div>

        <div className="lg:grid lg:grid-cols-[1fr_220px] lg:gap-12 items-start">
          <div ref={contentRef}>
            <AnimatePresence mode="wait">
              <motion.div
                key={activeTab}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                transition={{ duration: 0.3 }}
              >
                <div className="mb-14 lg:mb-18">
                  <div className="relative">
                    <div className="absolute -top-12 -left-12 w-56 h-56 rounded-full bg-accent-400/5 blur-[80px]" />
                    <div className="relative">
                      <div className="flex items-center gap-3 text-sm text-dark-500 mb-5">
                        <span className="flex items-center gap-1.5">
                          <Clock className="w-3.5 h-3.5" />
                          {readingTime} min read
                        </span>
                        <span className="w-1 h-1 rounded-full bg-dark-600" />
                        <span>{activeData.sections.length} sections</span>
                      </div>
                      <h1 className="text-4xl sm:text-5xl lg:text-6xl xl:text-7xl font-bold text-white leading-[1.1] tracking-tight">
                        {activeData.title.split(' ').map((word, i, arr) =>
                          i === arr.length - 1 ? (
                            <span key={i} className="bg-gradient-to-r from-accent-300 via-accent-400 to-primary-400 bg-clip-text text-transparent drop-shadow-[0_0_20px_rgba(6,182,212,0.15)]">
                              {word}
                            </span>
                          ) : (
                            <span key={i}>{word} </span>
                          )
                        )}
                      </h1>
                    </div>
                  </div>
                  <div className="relative mt-6 pl-6 sm:pl-8 border-l-2 border-accent-400/30">
                    <div className="absolute top-0 left-0 w-0.5 h-full bg-gradient-to-b from-accent-400 via-primary-400 to-accent-400/20" />
                    <p className="text-base sm:text-lg text-dark-300 leading-relaxed max-w-3xl font-light">
                      {activeData.intro}
                    </p>
                  </div>
                </div>

                <div className="space-y-14 lg:space-y-18">
                  {activeData.sections.map((section, i) => {
                    const content = activeData.formatContent
                      ? activeData.formatContent(section.content)
                      : section.content;
                    const style = getSectionStyle(i);
                    const color = accentColors[i % accentColors.length];
                    const glow = bgGlows[i % bgGlows.length];
                    const num = String(i + 1).padStart(2, '0');

                    if (style === 'pullquote') {
                      const { highlight, rest } = splitContent(content);
                      return (
                        <motion.div
                          key={section.title}
                          ref={(el) => { sectionRefs.current[i] = el; }}
                          data-section={section.title}
                          initial={{ opacity: 0, y: 30 }}
                          whileInView={{ opacity: 1, y: 0 }}
                          viewport={{ once: true, margin: '-80px' }}
                          transition={{ duration: 0.5 }}
                          className="relative"
                        >
                          <div className="absolute -left-4 top-0 bottom-0 w-px bg-gradient-to-b from-accent-400/0 via-accent-400/15 to-accent-400/0" />
                          <div className="flex items-center gap-4 mb-5">
                            <span className="text-5xl sm:text-6xl font-black text-transparent bg-clip-text bg-gradient-to-br from-accent-400/15 to-primary-400/10 leading-none">
                              {num}
                            </span>
                            <h2 className="text-xl sm:text-2xl font-bold text-white">{section.title}</h2>
                          </div>
                          <blockquote className="relative pl-6 sm:pl-8 mb-5">
                            <div className="absolute left-0 top-0 bottom-0 w-1 rounded-full bg-gradient-to-b from-accent-400 to-primary-400" />
                            <p className="text-base sm:text-lg text-accent-200 font-medium leading-relaxed italic">
                              &ldquo;{highlight}&rdquo;
                            </p>
                          </blockquote>
                          {rest && <p className="text-dark-400 leading-relaxed pl-6 sm:pl-8">{rest}</p>}
                          <div className="absolute -bottom-7 left-0 right-0 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/8 to-accent-400/0" />
                        </motion.div>
                      );
                    }

                    if (style === 'accent') {
                      return (
                        <motion.div
                          key={section.title}
                          ref={(el) => { sectionRefs.current[i] = el; }}
                          data-section={section.title}
                          initial={{ opacity: 0, y: 30 }}
                          whileInView={{ opacity: 1, y: 0 }}
                          viewport={{ once: true, margin: '-80px' }}
                          transition={{ duration: 0.5 }}
                          className="relative"
                        >
                          <div className="absolute -inset-x-4 -inset-y-2 rounded-3xl pointer-events-none" style={{ background: `radial-gradient(ellipse at 30% 50%, ${glow}, transparent 70%)` }} />
                          <div className="relative flex gap-6 sm:gap-8">
                            <div className="hidden sm:flex flex-col items-center">
                              <div className={`w-12 h-12 rounded-xl bg-gradient-to-br ${color} flex items-center justify-center text-white font-bold text-sm shadow-lg`}>
                                {num}
                              </div>
                              <div className="flex-1 w-px bg-gradient-to-b from-accent-400/20 to-transparent mt-3" />
                            </div>
                            <div className="flex-1 min-w-0">
                              <div className={`inline-block h-1 w-12 rounded-full bg-gradient-to-r ${color} mb-4`} />
                              <h2 className="text-xl sm:text-2xl font-bold text-white mb-3">{section.title}</h2>
                              <p className="text-dark-400 leading-relaxed">{content}</p>
                            </div>
                          </div>
                          <div className="absolute -bottom-7 left-0 right-0 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/8 to-accent-400/0" />
                        </motion.div>
                      );
                    }

                    return (
                      <motion.div
                        key={section.title}
                        ref={(el) => { sectionRefs.current[i] = el; }}
                        data-section={section.title}
                        initial={{ opacity: 0, y: 30 }}
                        whileInView={{ opacity: 1, y: 0 }}
                        viewport={{ once: true, margin: '-80px' }}
                        transition={{ duration: 0.5 }}
                        className="relative"
                      >
                        <div className="absolute -top-8 -right-8 w-40 h-40 rounded-full pointer-events-none" style={{ background: `radial-gradient(circle, ${glow}, transparent 70%)` }} />
                        <div className="relative flex items-start gap-6 sm:gap-10">
                          <span className="hidden sm:block text-7xl lg:text-8xl font-black text-transparent bg-clip-text bg-gradient-to-b from-white/[0.04] to-white/[0.01] leading-none shrink-0 select-none">
                            {num}
                          </span>
                          <div className="flex-1 min-w-0 pt-2 sm:pt-4">
                            <h2 className="text-xl sm:text-2xl font-bold text-white mb-4">{section.title}</h2>
                            <p className="text-dark-400 leading-relaxed text-[15px] sm:text-base">{content}</p>
                          </div>
                        </div>
                        <div className="absolute -bottom-7 left-0 right-0 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/8 to-accent-400/0" />
                      </motion.div>
                    );
                  })}
                </div>

                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: 0.2 }}
                  className="mt-14 sm:mt-18 relative overflow-hidden rounded-2xl"
                >
                  <div className="absolute inset-0 bg-gradient-to-br from-accent-400/10 via-primary-400/5 to-accent-400/10" />
                  <div className="absolute inset-0 border border-accent-400/20 rounded-2xl" />
                  <div className="relative p-8 sm:p-10 text-center">
                    <p className="text-xl font-bold text-white mb-2">Have questions?</p>
                    <p className="text-dark-400 mb-6">Our team is here to help clarify anything you need.</p>
                    <div className="flex flex-wrap items-center justify-center gap-3">
                      <Link to="/contact" className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-xl hover:shadow-accent-500/25 transition-all">
                        Contact Us <ArrowUpRight className="w-4 h-4" />
                      </Link>
                      <a href={`tel:${SITE_CONFIG.company.phone}`} className="inline-flex items-center gap-2 px-6 py-3 border border-white/10 text-dark-300 font-medium rounded-xl hover:bg-white/[0.04] transition-all">
                        {SITE_CONFIG.company.phone}
                      </a>
                    </div>
                  </div>
                </motion.div>
              </motion.div>
            </AnimatePresence>
          </div>

          <nav className="hidden lg:block sticky top-28">
            <div className="relative pl-5 border-l border-white/[0.06]">
              <p className="text-xs font-semibold text-dark-500 uppercase tracking-widest mb-5">Chapters</p>
              <ul className="space-y-3">
                {activeData.sections.map((section, i) => (
                  <li key={section.title}>
                    <button
                      onClick={() => scrollTo(section.title)}
                      className="group flex items-start gap-3 w-full text-left"
                    >
                      <span className={`shrink-0 text-[10px] font-mono font-bold leading-6 transition-colors duration-300 ${
                        activeSection === section.title ? 'text-accent-400' : 'text-dark-600'
                      }`}>
                        {String(i + 1).padStart(2, '0')}
                      </span>
                      <span className={`text-xs leading-6 transition-all duration-300 ${
                        activeSection === section.title
                          ? 'text-accent-300 font-medium'
                          : 'text-dark-400 group-hover:text-dark-200'
                      }`}>
                        {section.title}
                      </span>
                      {activeSection === section.title && (
                        <motion.span
                          layoutId="activeChapter"
                          className="absolute -left-px top-0 w-0.5 h-6 rounded-full bg-accent-400 shadow-[0_0_8px_rgba(6,182,212,0.4)]"
                        />
                      )}
                    </button>
                  </li>
                ))}
              </ul>
            </div>
          </nav>
        </div>
      </div>
    </div>
  );
}
