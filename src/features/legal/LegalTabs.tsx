import { useState } from 'react';
import { ChevronRight } from 'lucide-react';
import { Link } from 'react-router-dom';
import { SITE_CONFIG } from '../../config/site';

interface Section {
  title: string;
  content: string;
}

interface Props {
  title: string;
  intro: string;
  sections: Section[];
  formatContent?: (text: string) => string;
}

export default function LegalTabs({ title, intro, sections, formatContent }: Props) {
  const [activeIndex, setActiveIndex] = useState(0);
  const activeSection = sections[activeIndex];
  const content = activeSection && formatContent
    ? formatContent(activeSection.content)
    : activeSection?.content ?? '';

  return (
    <div className="min-h-screen bg-dark-950">
      <div className="max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 pt-28 pb-20">
        <div className="max-w-3xl">
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white leading-tight">
            {title}
          </h1>
          <p className="mt-4 text-base sm:text-lg text-dark-400 leading-relaxed">
            {intro}
          </p>
        </div>

        <div className="mt-10 flex flex-wrap gap-2">
          {sections.map((section, i) => {
            const isActive = i === activeIndex;
            return (
              <button
                key={section.title}
                onClick={() => setActiveIndex(i)}
                className={`px-3.5 py-2 rounded-xl text-sm font-medium transition-all whitespace-nowrap ${
                  isActive
                    ? 'bg-accent-400/15 text-accent-300'
                    : 'bg-white/[0.04] text-dark-400 hover:text-dark-200 hover:bg-white/[0.08]'
                }`}
              >
                {section.title}
              </button>
            );
          })}
        </div>

        <div className="mt-10">
          <h2 className="text-xl sm:text-2xl font-bold text-white mb-4">
            {activeSection.title}
          </h2>
          <p className="text-dark-400 leading-relaxed text-[15px] sm:text-base max-w-3xl">
            {content}
          </p>
        </div>

        <div className="flex items-center justify-between mt-10 pt-6 border-t border-white/[0.06] max-w-3xl">
          <button
            onClick={() => setActiveIndex(Math.max(0, activeIndex - 1))}
            disabled={activeIndex === 0}
            className="flex items-center gap-1.5 px-4 py-2 rounded-xl text-sm font-medium text-dark-400 hover:text-white hover:bg-white/[0.06] transition-all disabled:opacity-30 disabled:cursor-not-allowed"
          >
            <ChevronRight className="w-4 h-4 rotate-180" />
            Previous
          </button>

          <span className="text-sm text-dark-500">
            {activeIndex + 1} / {sections.length}
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
              className="flex items-center gap-1.5 px-4 py-2 rounded-xl text-sm font-medium bg-accent-500/20 text-accent-300 hover:bg-accent-500/30 transition-all"
            >
              Contact Us
            </Link>
          )}
        </div>
      </div>
    </div>
  );
}
