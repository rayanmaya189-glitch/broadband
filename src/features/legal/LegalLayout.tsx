import { useState, useEffect, useRef, useCallback } from 'react';
import { motion, useScroll, useSpring } from 'framer-motion';
import { ArrowUpRight, ChevronRight, Clock } from 'lucide-react';
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

function getSectionStyle(index: number): 'standard' | 'pullquote' | 'accent' {
  const cycle = index % 3;
  if (cycle === 0) return 'standard';
  if (cycle === 1) return 'pullquote';
  return 'accent';
}

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

export default function LegalLayout({ title, intro, sections, formatContent, lastUpdated }: Props) {
  const [activeId, setActiveId] = useState(sections[0]?.title ?? '');
  const sectionRefs = useRef<(HTMLDivElement | null)[]>([]);
  const contentRef = useRef<HTMLDivElement>(null);

  const { scrollYProgress } = useScroll({
    container: typeof window !== 'undefined' ? undefined : undefined,
    target: contentRef,
    offset: ['start start', 'end end'],
  });

  const scaleX = useSpring(scrollYProgress, { stiffness: 200, damping: 30 });

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveId(entry.target.getAttribute('data-section') ?? '');
          }
        }
      },
      { rootMargin: '-80px 0px -55% 0px', threshold: 0 }
    );

    sectionRefs.current.forEach((ref) => {
      if (ref) observer.observe(ref);
    });

    return () => observer.disconnect();
  }, []);

  const scrollTo = useCallback((title: string) => {
    const idx = sections.findIndex((s) => s.title === title);
    sectionRefs.current[idx]?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }, [sections]);

  const readingTime = Math.max(1, Math.ceil(sections.reduce((acc, s) => acc + s.content.length, 0) / 1500));

  return (
    <div className="min-h-screen bg-dark-950">
      <motion.div
        className="fixed top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-accent-400 via-primary-400 to-accent-400 z-50 origin-left"
        style={{ scaleX }}
      />

      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.05),transparent_50%)] pointer-events-none" />
      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_bottom_right,rgba(10,102,194,0.03),transparent_50%)] pointer-events-none" />
      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_bottom_left,rgba(34,211,238,0.02),transparent_50%)] pointer-events-none" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 pt-28 pb-20">
        <div className="lg:grid lg:grid-cols-[1fr_220px] lg:gap-12 items-start">
          <div ref={contentRef}>
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              className="mb-16 lg:mb-20"
            >
              <div className="relative">
                <div className="absolute -top-16 -left-16 w-64 h-64 rounded-full bg-accent-400/5 blur-[80px]" />
                <div className="absolute -top-8 -right-8 w-48 h-48 rounded-full bg-primary-400/5 blur-[60px]" />

                <div className="relative">
                  <div className="flex items-center gap-3 text-sm text-dark-500 mb-6">
                    <span className="flex items-center gap-1.5">
                      <Clock className="w-3.5 h-3.5" />
                      {readingTime} min read
                    </span>
                    <span className="w-1 h-1 rounded-full bg-dark-600" />
                    <span>{sections.length} sections</span>
                    {lastUpdated && (
                      <>
                        <span className="w-1 h-1 rounded-full bg-dark-600" />
                        <span>{lastUpdated}</span>
                      </>
                    )}
                  </div>

                  <h1 className="text-4xl sm:text-5xl lg:text-6xl xl:text-7xl font-bold text-white leading-[1.1] tracking-tight">
                    {title.split(' ').map((word, i, arr) =>
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

              <div className="relative mt-8 pl-6 sm:pl-8 border-l-2 border-accent-400/30">
                <div className="absolute top-0 left-0 w-0.5 h-full bg-gradient-to-b from-accent-400 via-primary-400 to-accent-400/20" />
                <p className="text-lg sm:text-xl text-dark-300 leading-relaxed max-w-3xl font-light">
                  {intro}
                </p>
              </div>
            </motion.div>

            <div className="space-y-16 lg:space-y-20">
              {sections.map((section, i) => {
                const content = formatContent ? formatContent(section.content) : section.content;
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
                      <div className="absolute -left-4 top-0 bottom-0 w-px bg-gradient-to-b from-accent-400/0 via-accent-400/20 to-accent-400/0" />

                      <div className="flex items-center gap-4 mb-6">
                        <span className="text-5xl sm:text-6xl font-black text-transparent bg-clip-text bg-gradient-to-br from-accent-400/20 to-primary-400/10 leading-none">
                          {num}
                        </span>
                        <h2 className="text-xl sm:text-2xl font-bold text-white">{section.title}</h2>
                      </div>

                      <blockquote className="relative pl-6 sm:pl-8 mb-6">
                        <div className="absolute left-0 top-0 bottom-0 w-1 rounded-full bg-gradient-to-b from-accent-400 to-primary-400" />
                        <p className="text-lg sm:text-xl text-accent-200 font-medium leading-relaxed italic">
                          &ldquo;{highlight}&rdquo;
                        </p>
                      </blockquote>

                      {rest && (
                        <p className="text-dark-400 leading-relaxed pl-6 sm:pl-8">{rest}</p>
                      )}

                      <div className="absolute -bottom-8 left-0 right-0 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/8 to-accent-400/0" />
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
                      <div
                        className="absolute -inset-x-4 -inset-y-2 rounded-3xl pointer-events-none"
                        style={{ background: `radial-gradient(ellipse at 30% 50%, ${glow}, transparent 70%)` }}
                      />

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

                      <div className="absolute -bottom-8 left-0 right-0 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/8 to-accent-400/0" />
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
                    <div
                      className="absolute -top-8 -right-8 w-40 h-40 rounded-full pointer-events-none"
                      style={{ background: `radial-gradient(circle, ${glow}, transparent 70%)` }}
                    />

                    <div className="relative flex items-start gap-6 sm:gap-10">
                      <span className="hidden sm:block text-7xl lg:text-8xl font-black text-transparent bg-clip-text bg-gradient-to-b from-white/[0.04] to-white/[0.01] leading-none shrink-0 select-none">
                        {num}
                      </span>

                      <div className="flex-1 min-w-0 pt-2 sm:pt-4">
                        <h2 className="text-xl sm:text-2xl font-bold text-white mb-4">{section.title}</h2>
                        <p className="text-dark-400 leading-relaxed text-[15px] sm:text-base">{content}</p>
                      </div>
                    </div>

                    <div className="absolute -bottom-8 left-0 right-0 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/8 to-accent-400/0" />
                  </motion.div>
                );
              })}
            </div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ delay: 0.2 }}
              className="mt-16 sm:mt-20 relative overflow-hidden rounded-2xl"
            >
              <div className="absolute inset-0 bg-gradient-to-br from-accent-400/10 via-primary-400/5 to-accent-400/10" />
              <div className="absolute inset-0 border border-accent-400/20 rounded-2xl" />
              <div className="relative p-8 sm:p-10 text-center">
                <p className="text-xl font-bold text-white mb-2">Have questions about this policy?</p>
                <p className="text-dark-400 mb-6">Our team is here to help clarify anything you need.</p>
                <div className="flex flex-wrap items-center justify-center gap-3">
                  <Link
                    to="/contact"
                    className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-accent-500 to-primary-600 text-white font-semibold rounded-xl hover:shadow-xl hover:shadow-accent-500/25 transition-all"
                  >
                    Contact Us <ArrowUpRight className="w-4 h-4" />
                  </Link>
                  <a
                    href={`tel:${SITE_CONFIG.company.phone}`}
                    className="inline-flex items-center gap-2 px-6 py-3 border border-white/10 text-dark-300 font-medium rounded-xl hover:bg-white/[0.04] transition-all"
                  >
                    {SITE_CONFIG.company.phone}
                  </a>
                </div>
              </div>
            </motion.div>
          </div>

          <nav className="hidden lg:block sticky top-28">
            <div className="relative pl-5 border-l border-white/[0.06]">
              <p className="text-xs font-semibold text-dark-500 uppercase tracking-widest mb-5">Chapters</p>
              <ul className="space-y-3">
                {sections.map((section, i) => (
                  <li key={section.title}>
                    <button
                      onClick={() => scrollTo(section.title)}
                      className="group flex items-start gap-3 w-full text-left"
                    >
                      <span className={`shrink-0 text-[10px] font-mono font-bold leading-6 transition-colors duration-300 ${
                        activeId === section.title ? 'text-accent-400' : 'text-dark-600'
                      }`}>
                        {String(i + 1).padStart(2, '0')}
                      </span>
                      <span className={`text-xs leading-6 transition-all duration-300 ${
                        activeId === section.title
                          ? 'text-accent-300 font-medium'
                          : 'text-dark-400 group-hover:text-dark-200'
                      }`}>
                        {section.title}
                      </span>
                      {activeId === section.title && (
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
