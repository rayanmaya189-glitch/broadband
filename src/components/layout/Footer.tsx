import { Link } from 'react-router-dom';
import { Phone, Mail, MapPin, Facebook, Twitter, Instagram, Youtube } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';

const socialIcons: Record<string, React.ReactNode> = {
  facebook: <Facebook className="w-5 h-5" />,
  twitter: <Twitter className="w-5 h-5" />,
  instagram: <Instagram className="w-5 h-5" />,
  linkedin: <Twitter className="w-5 h-5" />,
  youtube: <Youtube className="w-5 h-5" />,
};

export default function Footer() {
  return (
    <footer className="relative border-t border-white/[0.06] bg-dark-950">
      <div className="absolute inset-0 bg-gradient-to-b from-transparent via-primary-500/3 to-accent-500/5 pointer-events-none" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 py-8 sm:py-12 lg:py-16">
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-4 sm:gap-6 lg:gap-8">
          <div className="col-span-2 sm:col-span-1 space-y-3 sm:space-y-4">
            <div>
              <img src="/logo.png" alt={SITE_CONFIG.company.name} className="h-12 sm:h-16 lg:h-20 w-auto" />
            </div>
            <p className="text-xs sm:text-sm text-dark-400 leading-relaxed line-clamp-3">{SITE_CONFIG.company.description}</p>
            <div className="flex gap-2 sm:gap-3">
              {Object.entries(SITE_CONFIG.social).slice(0, 4).map(([key, url]) => (
                <a
                  key={key}
                  href={url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="p-2 sm:p-2.5 rounded-lg bg-white/[0.06] text-dark-400 hover:text-accent-400 hover:bg-accent-400/10 transition-all min-h-[40px] min-w-[40px] flex items-center justify-center"
                  aria-label={key}
                >
                  {socialIcons[key]}
                </a>
              ))}
            </div>
          </div>

          <div>
            <h3 className="text-xs sm:text-sm font-semibold text-white uppercase tracking-wider mb-3 sm:mb-4">Quick Links</h3>
            <ul className="space-y-2 sm:space-y-3">
              {SITE_CONFIG.navLinks.slice(0, 3).map((link) => (
                <li key={link.label}>
                  <Link
                    to={link.href}
                    className="text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
              <li>
                <Link to="/team" className="text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  Our Team
                </Link>
              </li>
            </ul>
          </div>

          <div>
            <h3 className="text-xs sm:text-sm font-semibold text-white uppercase tracking-wider mb-3 sm:mb-4">Plans</h3>
            <ul className="space-y-2 sm:space-y-3">
              {SITE_CONFIG.plans.slice(0, 3).map((plan) => (
                <li key={plan.id}>
                  <Link
                    to={`/plan/${plan.id}`}
                    className="text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors line-clamp-1"
                  >
                    {plan.speed}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="text-xs sm:text-sm font-semibold text-white uppercase tracking-wider mb-3 sm:mb-4">Legal</h3>
            <ul className="space-y-2 sm:space-y-3">
              <li>
                <Link to="/privacy" className="text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  Privacy
                </Link>
              </li>
              <li>
                <Link to="/terms" className="text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  Terms
                </Link>
              </li>
              <li>
                <Link to="/refund" className="text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  Refund
                </Link>
              </li>
            </ul>
          </div>

          <div>
            <h3 className="text-xs sm:text-sm font-semibold text-white uppercase tracking-wider mb-3 sm:mb-4">Contact</h3>
            <ul className="space-y-2 sm:space-y-3">
              <li>
                <a href={`tel:${SITE_CONFIG.company.phone}`} className="flex items-center gap-1.5 sm:gap-2 text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  <Phone className="w-3 h-3 sm:w-4 sm:h-4 flex-shrink-0" />
                  <span className="truncate">{SITE_CONFIG.company.phone}</span>
                </a>
              </li>
              <li>
                <a href={`mailto:${SITE_CONFIG.company.email}`} className="flex items-center gap-1.5 sm:gap-2 text-xs sm:text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  <Mail className="w-3 h-3 sm:w-4 sm:h-4 flex-shrink-0" />
                  <span className="truncate text-[10px] sm:text-xs">{SITE_CONFIG.company.email}</span>
                </a>
              </li>
            </ul>
          </div>
        </div>

        <div className="mt-8 sm:mt-12 pt-6 sm:pt-8 border-t border-white/[0.06] flex flex-col sm:flex-row items-center justify-center sm:justify-between gap-3 sm:gap-4 text-center sm:text-left">
          <p className="text-xs sm:text-sm text-dark-500">
            &copy; {new Date().getFullYear()} {SITE_CONFIG.company.legalName}
          </p>
          <div className="flex gap-3 sm:gap-6 flex-wrap justify-center">
            <Link to="/privacy" className="text-xs sm:text-sm text-dark-500 hover:text-dark-400 transition-colors">Privacy</Link>
            <Link to="/terms" className="text-xs sm:text-sm text-dark-500 hover:text-dark-400 transition-colors">Terms</Link>
            <Link to="/refund" className="text-xs sm:text-sm text-dark-500 hover:text-dark-400 transition-colors">Refund</Link>
          </div>
        </div>
      </div>
    </footer>
  );
}
