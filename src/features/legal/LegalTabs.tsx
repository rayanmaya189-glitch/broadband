import { useState, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ArrowUpRight, Clock, ChevronRight } from 'lucide-react';
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

const gradientPairs = [
  ['from-accent-400', 'to-primary-500'],
  ['from-primary-400', 'to-accent-500'],
  ['from-cyan-400', 'to-accent-400'],
  ['from-accent-300', 'to-primary-400'],
  ['from-primary-300', 'to-accent-400'],
  ['from-accent-400', 'to-cyan-400'],
];

const bgGlows = [
  'rgba(6,182,212,0.05)',
  'rgba(10,102,194,0.04)',
  'rgba(34,211,238,0.05)',
  'rgba(6,182,212,0.04)',
  'rgba(10,102,194,0.05)',
  'rgba(34,211,238,0.04)',
];

export default function LegalTabs({ icon, title, intro, sections, formatContent, lastUpdated }: Props) {
  const [activeIndex, setActiveIndex] = useState(0);
  const tabBarRef = useRef<HTMLDivElement>(null);

  const activeSection = sections[activeIndex];
  const content = activeSection && formatContent
    ? formatContent(activeSection.content)
    : activeSection?.content ?? '';

  const readingTime = Math.max(1, Math.ceil(
    sections.reduce((acc, s) => acc + s.content.length, 0) / 1500
  ));

  return (
    <div className="min-h-screen bg-dark-950">
      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.05),transparent_50%)] pointer-events-none" />
      <div className="fixed inset-0 bg-[radial-gradient(ellipse_at_bottom_right,rgba(10,102,194,0.03),transparent_50%)] pointer-events-none" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 pt-28 pb-20">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="mb-10"
        >
          <div className="flex items-center gap-3 text-sm text-dark-500 mb-5">
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

          <div className="relative">
            <div className="absolute -top-12 -left-12 w-56 h-56 rounded-full bg-accent-400/5 blur-[80px]" />
            <div className="relative">
              <div className="flex items-center gap-3 mb-4">
                <span className="p-2 rounded-xl bg-gradient-to-br from-accent-400/20 to-primary-400/20 border border-accent-400/20">
                  {icon}
                </span>
              </div>
              <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold text-white leading-[1.1] tracking-tight">
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

          <p className="mt-5 text-base sm:text-lg text-dark-300 leading-relaxed max-w-3xl font-light">
            {intro}
          </p>
        </motion.div>

        <div ref={tabBarRef} className="relative mb-10">
          <div className="flex gap-2 overflow-x-auto pb-2 scrollbar-none -mx-4 px-4 sm:mx-0 sm:px-0">
            {sections.map((section, i) => {
              const isActive = i === activeIndex;
              const [from, to] = gradientPairs[i % gradientPairs.length];
              return (
                <button
                  key={section.title}
                  onClick={() => setActiveIndex(i)}
                  className={`group relative flex items-center gap-2.5 px-4 py-2.5 rounded-xl whitespace-nowrap transition-all duration-300 shrink-0 ${
                    isActive
                      ? 'bg-gradient-to-br from-accent-500/20 via-primary-500/15 to-accent-500/20 shadow-[inset_0_1px_0_rgba(6,182,212,0.15)]'
                      : 'bg-white/[0.03] hover:bg-white/[0.06]'
                  }`}
                  style={isActive ? { boxShadow: '0 0 24px rgba(6,182,212,0.06), inset 0 1px 0 rgba(6,182,212,0.12)' } : {}}
                >
                  {isActive && (
                    <motion.span
                      layoutId="tabGlow"
                      className="absolute inset-0 rounded-xl border border-accent-400/30"
                      transition={{ type: 'spring', stiffness: 400, damping: 30 }}
                    />
                  )}

                  <span className={`relative flex items-center justify-center w-7 h-7 rounded-lg text-[11px] font-bold transition-all ${
                    isActive
                      ? `bg-gradient-to-br ${from} ${to} text-white shadow-md`
                      : 'bg-white/[0.06] text-dark-500 group-hover:text-dark-300'
                  }`}>
                    {String(i + 1).padStart(2, '0')}
                  </span>

                  <span className={`relative text-sm font-medium transition-colors ${
                    isActive ? 'text-white' : 'text-dark-400 group-hover:text-dark-200'
                  }`}>
                    {section.title}
                  </span>
                </button>
              );
            })}
          </div>

          <div className="absolute bottom-0 left-0 right-0 h-px bg-gradient-to-r from-accent-400/0 via-accent-400/10 to-accent-400/0" />
        </div>

        <AnimatePresence mode="wait">
          <motion.div
            key={activeIndex}
            initial={{ opacity: 0, x: 30 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            transition={{ duration: 0.3, ease: 'easeInOut' }}
          >
            <div className="relative">
              <div
                className="absolute -inset-x-6 -inset-y-4 rounded-3xl pointer-events-none"
                style={{
                  background: `radial-gradient(ellipse at 40% 50%, ${bgGlows[activeIndex % bgGlows.length]}, transparent 70%)`,
                }}
              />

              <div className="relative flex items-start gap-8 sm:gap-12">
                <div className="hidden sm:flex flex-col items-center">
                  <span className={`w-14 h-14 rounded-2xl bg-gradient-to-br ${gradientPairs[activeIndex % gradientPairs.length][0]} ${gradientPairs[activeIndex % gradientPairs.length][1]} flex items-center justify-center text-white font-bold text-lg shadow-xl`}>
                    {String(activeIndex + 1).padStart(2, '0')}
                  </span>
                  <div className="flex-1 w-px bg-gradient-to-b from-accent-400/20 to-transparent mt-4" />
                </div>

                <div className="flex-1 min-w-0 pt-1">
                  <div className={`inline-block h-1 w-14 rounded-full bg-gradient-to-r ${gradientPairs[activeIndex % gradientPairs.length][0]} ${gradientPairs[activeIndex % gradientPairs.length][1]} mb-5`} />

                  <h2 className="text-2xl sm:text-3xl font-bold text-white mb-5 leading-tight">
                    {activeSection.title}
                  </h2>

                  <div className="relative">
                    <div className="absolute -left-3 top-0 bottom-0 w-0.5 bg-gradient-to-b from-accent-400/30 via-accent-400/10 to-transparent rounded-full" />
                    <div className="space-y-4 pl-5">
                      {content.split(/(?<=[.!?])\s+/).map((sentence, si) => {
                        const isKeyPoint =
                          sentence.toLowerCase().includes('must') ||
                          sentence.toLowerCase().includes('shall') ||
                          sentence.toLowerCase().includes('will not') ||
                          sentence.toLowerCase().includes('you have the right') ||
                          sentence.toLowerCase().includes('you may');

                        if (isKeyPoint && sentence.length > 30) {
                          return (
                            <div key={si} className="relative p-4 sm:p-5 rounded-xl bg-gradient-to-r from-accent-400/8 to-primary-400/5 border border-accent-400/15 overflow-hidden group">
                              <div className="absolute top-0 left-0 w-1 h-full bg-gradient-to-b from-accent-400 to-primary-400" />
                              <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,rgba(6,182,212,0.06),transparent_50%)] opacity-0 group-hover:opacity-100 transition-opacity" />
                              <p className="text-dark-200 text-[15px] sm:text-base leading-relaxed relative">
                                {sentence}
                              </p>
                            </div>
                          );
                        }

                        return (
                          <p key={si} className="text-dark-400 text-[15px] sm:text-base leading-relaxed">
                            {sentence}
                          </p>
                        );
                      })}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </motion.div>
        </AnimatePresence>

        <div className="flex items-center justify-between mt-10 pt-6 border-t border-white/[0.06]">
          <button
            onClick={() => setActiveIndex(Math.max(0, activeIndex - 1))}
            disabled={activeIndex === 0}
            className="flex items-center gap-1.5 px-4 py-2 rounded-xl text-sm font-medium text-dark-400 hover:text-white hover:bg-white/[0.06] transition-all disabled:opacity-30 disabled:cursor-not-allowed"
          >
            <ChevronRight className="w-4 h-4 rotate-180" />
            Previous
          </button>

          <span className="text-xs text-dark-500 font-mono">
            {String(activeIndex + 1).padStart(2, '0')} / {String(sections.length).padStart(2, '0')}
          </span>

          {activeIndex < sections.length - 1 ? (
            <button
              onClick={() => setActiveIndex(Math.min(sections.length - 1, activeIndex + 1))}
              className="flex items-center gap-1.5 px-4 py-2 rounded-xl text-sm font-medium text-dark-400 hover:text-white hover:bg-white/[0.06] transition-all"
            >
              Next
              <ChevronRight className="w-4 h-4" />
            </button>
          ) : (
            <Link
              to="/contact"
              className="inline-flex items-center gap-1.5 px-4 py-2 rounded-xl text-sm font-medium bg-gradient-to-r from-accent-500 to-primary-600 text-white hover:shadow-lg hover:shadow-accent-500/25 transition-all"
            >
              Contact Us
              <ArrowUpRight className="w-4 h-4" />
            </Link>
          )}
        </div>
      </div>
    </div>
  );
}
