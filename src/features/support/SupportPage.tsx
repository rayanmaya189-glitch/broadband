import { useState } from 'react';
import { motion } from 'framer-motion';
import { ChevronDown, Phone, MessageSquare, Mail, Search } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';
import { cn } from '../../utils/helpers';

export default function SupportPage() {
  const [openFaq, setOpenFaq] = useState<number | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  const filteredFaqs = SITE_CONFIG.faqs.filter(
    (faq) =>
      faq.question.toLowerCase().includes(searchQuery.toLowerCase()) ||
      faq.answer.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen pt-24 pb-16 bg-dark-950">
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.06),transparent_50%)]" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">Support</h1>
          <p className="mt-4 text-lg text-dark-400">
            We're here to help 24/7. Find answers or reach out to us.
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="flex flex-wrap gap-4 justify-center mb-12"
        >
          {[
            { icon: Phone, label: 'Call Us', href: `tel:${SITE_CONFIG.company.phone}` },
            { icon: MessageSquare, label: 'WhatsApp', href: `https://wa.me/${SITE_CONFIG.whatsapp}` },
            { icon: Mail, label: 'Email', href: `mailto:${SITE_CONFIG.company.email}` },
          ].map((item) => (
            <a
              key={item.label}
              href={item.href}
              className="inline-flex items-center gap-2 px-6 py-3 glass-card rounded-xl hover:bg-white/[0.08] transition-all"
            >
              <item.icon className="w-5 h-5 text-accent-400" />
              <span className="text-white font-medium">{item.label}</span>
            </a>
          ))}
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="relative mb-8"
        >
          <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-dark-400" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search FAQs..."
            className="w-full pl-12 pr-4 py-4 bg-white/[0.06] border border-white/[0.1] rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:border-accent-400/50 transition-all"
          />
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className="space-y-3"
        >
          {filteredFaqs.map((faq, i) => (
            <div key={i} className="glass-card rounded-xl overflow-hidden">
              <button
                onClick={() => setOpenFaq(openFaq === i ? null : i)}
                className="w-full flex items-center justify-between p-5 text-left hover:bg-white/[0.04] transition-all"
              >
                <span className="text-white font-medium pr-4">{faq.question}</span>
                <ChevronDown
                  className={cn(
                    'w-5 h-5 text-dark-400 shrink-0 transition-transform duration-300',
                    openFaq === i && 'rotate-180'
                  )}
                />
              </button>
              <motion.div
                initial={false}
                animate={{
                  height: openFaq === i ? 'auto' : 0,
                  opacity: openFaq === i ? 1 : 0,
                }}
                transition={{ duration: 0.3 }}
                className="overflow-hidden"
              >
                <p className="px-5 pb-5 text-sm text-dark-400 leading-relaxed">{faq.answer}</p>
              </motion.div>
            </div>
          ))}
          {filteredFaqs.length === 0 && (
            <p className="text-center text-dark-400 py-8">No FAQs match your search.</p>
          )}
        </motion.div>
      </div>
    </div>
  );
}
