import { useState } from 'react';
import { motion } from 'framer-motion';
import { Phone, Mail, MapPin, Send, MessageSquare, ArrowRight } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../../components/ui/TiltCard';

export default function ContactPage() {
  const [form, setForm] = useState({ name: '', email: '', phone: '', message: '' });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const text = `Hi! I'm ${form.name}. ${form.message}`;
    window.open(`https://wa.me/${SITE_CONFIG.whatsapp}?text=${encodeURIComponent(text)}`, '_blank');
  };

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
            <MessageSquare className="w-4 h-4" />
            Get in Touch
          </motion.span>
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">Contact Us</h1>
          <p className="mt-4 text-lg text-dark-400">
            We&apos;re here to help. Reach out to us anytime.
          </p>
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-8">
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.2 }}
          >
            <TiltCard tiltDegree={3} glareOpacity={0.1}>
              <div className="glass-card-strong rounded-2xl p-8 border border-white/[0.08]">
                <h2 className="text-xl font-bold text-white mb-6">Send us a message</h2>
                <form onSubmit={handleSubmit} className="space-y-4">
                  <div>
                    <input
                      type="text"
                      placeholder="Your Name"
                      value={form.name}
                      onChange={(e) => setForm({ ...form, name: e.target.value })}
                      required
                      className="w-full px-4 py-3.5 bg-white/[0.06] border border-white/[0.1] rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:border-accent-400/50 transition-all"
                    />
                  </div>
                  <div className="grid sm:grid-cols-2 gap-4">
                    <input
                      type="email"
                      placeholder="Email"
                      value={form.email}
                      onChange={(e) => setForm({ ...form, email: e.target.value })}
                      required
                      className="w-full px-4 py-3.5 bg-white/[0.06] border border-white/[0.1] rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:border-accent-400/50 transition-all"
                    />
                    <input
                      type="tel"
                      placeholder="Phone"
                      value={form.phone}
                      onChange={(e) => setForm({ ...form, phone: e.target.value })}
                      required
                      className="w-full px-4 py-3.5 bg-white/[0.06] border border-white/[0.1] rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:border-accent-400/50 transition-all"
                    />
                  </div>
                  <textarea
                    placeholder="Your message..."
                    value={form.message}
                    onChange={(e) => setForm({ ...form, message: e.target.value })}
                    required
                    rows={4}
                    className="w-full px-4 py-3.5 bg-white/[0.06] border border-white/[0.1] rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:border-accent-400/50 transition-all resize-none"
                  />
                  <button
                    type="submit"
                    className="w-full inline-flex items-center justify-center gap-2 px-6 py-3.5 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-lg hover:shadow-accent-500/25 transition-all"
                  >
                    <Send className="w-4 h-4" />
                    Send Message
                  </button>
                </form>
              </div>
            </TiltCard>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.3 }}
          >
            <div className="rounded-2xl border border-white/[0.06] bg-white/[0.02] p-6 sm:p-8">
              <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-2">
                <span className="w-1.5 h-1.5 rounded-full bg-accent-400 animate-pulse" />
                Contact Information
              </h2>
              <div className="space-y-1">
                {[
                  { icon: Phone, label: 'Phone', value: SITE_CONFIG.company.phone, href: `tel:${SITE_CONFIG.company.phone}` },
                  { icon: Mail, label: 'Email', value: SITE_CONFIG.company.email, href: `mailto:${SITE_CONFIG.company.email}` },
                  { icon: MapPin, label: 'Address', value: SITE_CONFIG.company.address },
                  { icon: MessageSquare, label: 'WhatsApp', value: 'Chat with us', href: `https://wa.me/${SITE_CONFIG.whatsapp}` },
                ].map((item, i) => (
                  <motion.div
                    key={item.label}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: 0.4 + i * 0.06 }}
                    className="flex items-start gap-4 p-4 rounded-xl hover:bg-white/[0.04] transition-colors duration-300 cursor-default group"
                  >
                    <div className="shrink-0 w-1 self-stretch rounded-full bg-gradient-to-b from-accent-400 to-primary-500 opacity-40 group-hover:opacity-100 transition-opacity" />
                    <div className="shrink-0 p-2.5 rounded-xl bg-accent-400/10 group-hover:bg-accent-400/20 transition-colors">
                      <item.icon className="w-5 h-5 text-accent-400" />
                    </div>
                    <div className="min-w-0">
                      <p className="text-sm text-dark-500">{item.label}</p>
                      {item.href ? (
                        <a href={item.href} className="text-white font-medium hover:text-accent-400 transition-colors">
                          {item.value}
                        </a>
                      ) : (
                        <p className="text-white font-medium">{item.value}</p>
                      )}
                    </div>
                    {item.href && (
                      <ArrowRight className="w-4 h-4 text-dark-600 ml-auto shrink-0 mt-1.5 group-hover:text-accent-400 transition-colors" />
                    )}
                  </motion.div>
                ))}
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  );
}
