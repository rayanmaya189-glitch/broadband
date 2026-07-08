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

        <div className="mt-12 space-y-1">
          {sections.map((section, i) => {
            const content = formatContent ? formatContent(section.content) : section.content;
            return (
              <div
                key={section.title}
                className="group relative pl-5 py-5 rounded-xl transition-colors hover:bg-white/[0.02]"
              >
                <div className="absolute left-0 top-5 bottom-5 w-0.5 rounded-full bg-dark-700 group-hover:bg-accent-400/50 transition-colors" />
                <div className="flex items-start gap-4">
                  <span className="mt-0.5 text-xs font-mono font-bold text-dark-600 group-hover:text-accent-400/70 transition-colors w-6 shrink-0">
                    {String(i + 1).padStart(2, '0')}
                  </span>
                  <div className="min-w-0">
                    <h2 className="text-base sm:text-lg font-semibold text-white mb-1.5 group-hover:text-accent-200 transition-colors">
                      {section.title}
                    </h2>
                    <p className="text-sm sm:text-base text-dark-400 leading-relaxed group-hover:text-dark-300 transition-colors">
                      {content}
                    </p>
                  </div>
                </div>
              </div>
            );
          })}
        </div>

        <div className="mt-12 pt-8 border-t border-white/[0.06] max-w-3xl">
          <Link
            to="/contact"
            className="inline-flex items-center gap-2 text-sm text-accent-400 hover:text-accent-300 transition-colors font-medium"
          >
            Have questions? Contact us &rarr;
          </Link>
        </div>
      </div>
    </div>
  );
}
