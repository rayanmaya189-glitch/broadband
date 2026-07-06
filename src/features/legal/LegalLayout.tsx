import { useState, useEffect, useRef } from 'react';
import { motion } from 'framer-motion';
import { ArrowUpRight } from 'lucide-react';
import { Link } from 'react-router-dom';
import { SITE_CONFIG } from '../../config/site';

interface Section {
  title: string;
  content: string;
}

interface Props {
  icon: React.ReactNode;
  title: string;
  intro: string;
  sections: Section[];
  formatContent?: (text: string) => string;
  lastUpdated?: string;
}

export default function LegalLayout({ icon, title, intro, sections, formatContent, lastUpdated }: Props) {
  const [activeId, setActiveId] = useState<string>(sections[0]?.title ?? '');
  const sectionRefs = useRef<(HTMLDivElement | null)[]>([]);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveId(entry.target.getAttribute('data-section') ?? '');
          }
        }
      },
      { rootMargin: '-80px 0px -60% 0px', threshold: 0 }
    );

    sectionRefs.current.forEach((ref) => {
      if (ref) observer.observe(ref);
    });

    return () => observer.disconnect();
  }, []);

  return (
    <div className="min-h-screen pt-24 pb-16 bg-dark-950">
      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.06),transparent_50%)] pointer-events-none" />
      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_bottom_right,rgba(10,102,194,0.04),transparent_50%)] pointer-events-none" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <div className="lg:grid lg:grid-cols-[1fr_280px] lg:gap-10 items-start">
          <div>
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="mb-12"
            >
              <motion.span
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.4 }}
                className="relative inline-flex items-center gap-2.5 px-5 py-2 rounded-full bg-dark-900/80 border border-accent-400/20 text-accent-300 text-sm font-medium mb-6 overflow-hidden group cursor-default"
              >
                <span className="absolute inset-0 bg-gradient-to-r from-accent-400/5 to-primary-400/5 opacity-0 group-hover:opacity-100 transition-opacity" />
                <span className="relative flex items-center gap-2.5">
                  <span className="p-1 rounded-md bg-accent-400/10">
                    {icon}
                  </span>
                  <span>Legal</span>
                </span>
              </motion.span>

              <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white leading-tight">
                {title.split(' ').map((word, i, arr) =>
                  i === arr.length - 1 ? (
                    <span key={i} className="bg-gradient-to-r from-accent-300 to-primary-400 bg-clip-text text-transparent drop-shadow-[0_0_12px_rgba(6,182,212,0.2)]">
                      {word}
                    </span>
                  ) : (
                    <span key={i}>{word} </span>
                  )
                )}
              </h1>

              <div className="mt-4 flex items-center gap-4 text-sm text-dark-500">
                {lastUpdated && (
                  <span className="flex items-center gap-1.5">
                    <span className="w-1.5 h-1.5 rounded-full bg-accent-400/60" />
                    Last updated: {lastUpdated}
                  </span>
                )}
                <span className="flex items-center gap-1.5">
                  <span className="w-1.5 h-1.5 rounded-full bg-accent-400/60" />
                  {sections.length} sections
                </span>
              </div>

              <div className="relative mt-6 p-5 sm:p-6 rounded-xl bg-gradient-to-r from-accent-400/5 via-primary-400/5 to-accent-400/5 border border-accent-400/10">
                <div className="absolute top-0 left-0 w-1 h-full rounded-l-xl bg-gradient-to-b from-accent-400 to-primary-400" />
                <p className="text-dark-300 leading-relaxed pl-4">
                  {intro}
                </p>
              </div>
            </motion.div>

            <div className="space-y-8">
              {sections.map((section, i) => {
                const content = formatContent ? formatContent(section.content) : section.content;
                return (
                  <motion.div
                    key={section.title}
                    ref={(el) => { sectionRefs.current[i] = el; }}
                    data-section={section.title}
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: 0.1 + i * 0.05 }}
                    className="group relative rounded-xl border border-white/[0.06] bg-white/[0.02] p-6 sm:p-8 hover:border-accent-400/15 hover:bg-accent-400/[0.01] transition-all duration-300"
                  >
                    <div className="flex items-start gap-4">
                      <span className="hidden sm:flex items-center justify-center w-10 h-10 rounded-lg bg-gradient-to-br from-accent-400/20 to-primary-400/20 border border-accent-400/20 text-accent-300 text-sm font-bold shrink-0 group-hover:scale-105 transition-transform">
                        {String(i + 1).padStart(2, '0')}
                      </span>
                      <div className="min-w-0 flex-1">
                        <h2 className="text-lg sm:text-xl font-bold text-white mb-3 flex items-center gap-3">
                          <span className="w-1 h-6 rounded-full bg-gradient-to-b from-accent-400 to-primary-400 shrink-0 opacity-60 group-hover:opacity-100 transition-opacity" />
                          {section.title}
                        </h2>
                        <p className="text-dark-400 leading-relaxed pl-4">{content}</p>
                      </div>
                    </div>

                    <div className="absolute bottom-0 left-6 right-6 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/10 to-accent-400/0 opacity-0 group-hover:opacity-100 transition-opacity" />
                  </motion.div>
                );
              })}
            </div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.4 }}
              className="mt-10 p-6 sm:p-8 rounded-xl bg-gradient-to-r from-accent-400/5 to-primary-400/5 border border-accent-400/15 text-center sm:text-left sm:flex sm:items-center sm:justify-between gap-6"
            >
              <div>
                <p className="text-white font-semibold">Have questions about this policy?</p>
                <p className="text-dark-400 text-sm mt-1">Reach out to us and we&apos;ll help clarify.</p>
              </div>
              <div className="flex items-center gap-3 mt-4 sm:mt-0 justify-center sm:justify-end">
                <Link
                  to="/contact"
                  className="inline-flex items-center gap-2 px-5 py-2.5 bg-gradient-to-r from-accent-500 to-primary-600 text-white text-sm font-semibold rounded-xl hover:shadow-lg hover:shadow-accent-500/25 transition-all"
                >
                  Contact Us <ArrowUpRight className="w-4 h-4" />
                </Link>
                <a
                  href={`tel:${SITE_CONFIG.company.phone}`}
                  className="inline-flex items-center gap-2 px-5 py-2.5 border border-white/10 text-dark-300 text-sm font-medium rounded-xl hover:bg-white/[0.04] transition-all"
                >
                  {SITE_CONFIG.company.phone}
                </a>
              </div>
            </motion.div>
          </div>

          <nav className="hidden lg:block sticky top-28">
            <div className="p-4 rounded-xl border border-white/[0.06] bg-dark-900/50 backdrop-blur-sm">
              <p className="text-xs font-semibold text-dark-500 uppercase tracking-wider mb-3">On this page</p>
              <ul className="space-y-1">
                {sections.map((section) => (
                  <li key={section.title}>
                    <button
                      onClick={() => {
                        const idx = sections.findIndex((s) => s.title === section.title);
                        sectionRefs.current[idx]?.scrollIntoView({ behavior: 'smooth', block: 'start' });
                      }}
                      className={`w-full text-left px-3 py-1.5 rounded-lg text-sm transition-all duration-200 ${
                        activeId === section.title
                          ? 'text-accent-300 bg-accent-400/10 font-medium'
                          : 'text-dark-400 hover:text-dark-200 hover:bg-white/[0.04]'
                      }`}
                    >
                      {section.title}
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
