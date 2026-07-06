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

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6 py-12 lg:py-16">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 lg:gap-12">
          <div className="space-y-4">
            <div>
              <img src="/logo.png" alt={SITE_CONFIG.company.name} className="h-10 w-auto" />
            </div>
            <p className="text-sm text-dark-400 leading-relaxed">{SITE_CONFIG.company.description}</p>
            <div className="flex gap-3">
              {Object.entries(SITE_CONFIG.social).slice(0, 4).map(([key, url]) => (
                <a
                  key={key}
                  href={url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="p-2.5 rounded-lg bg-white/[0.06] text-dark-400 hover:text-accent-400 hover:bg-accent-400/10 transition-all"
                >
                  {socialIcons[key]}
                </a>
              ))}
            </div>
          </div>

          <div>
            <h3 className="text-sm font-semibold text-white uppercase tracking-wider mb-4">Quick Links</h3>
            <ul className="space-y-3">
              {SITE_CONFIG.navLinks.map((link) => (
                <li key={link.label}>
                  <Link
                    to={link.href}
                    className="text-sm text-dark-400 hover:text-accent-400 transition-colors"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="text-sm font-semibold text-white uppercase tracking-wider mb-4">Plans</h3>
            <ul className="space-y-3">
              {SITE_CONFIG.plans.slice(0, 4).map((plan) => (
                <li key={plan.id}>
                  <Link
                    to={`/plan/${plan.id}`}
                    className="text-sm text-dark-400 hover:text-accent-400 transition-colors"
                  >
                    {plan.speed} - {plan.tag}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="text-sm font-semibold text-white uppercase tracking-wider mb-4">Contact</h3>
            <ul className="space-y-3">
              <li>
                <a href={`tel:${SITE_CONFIG.company.phone}`} className="flex items-center gap-2 text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  <Phone className="w-4 h-4" />
                  {SITE_CONFIG.company.phone}
                </a>
              </li>
              <li>
                <a href={`mailto:${SITE_CONFIG.company.email}`} className="flex items-center gap-2 text-sm text-dark-400 hover:text-accent-400 transition-colors">
                  <Mail className="w-4 h-4" />
                  {SITE_CONFIG.company.email}
                </a>
              </li>
              <li className="flex items-start gap-2 text-sm text-dark-400">
                <MapPin className="w-4 h-4 shrink-0 mt-0.5" />
                {SITE_CONFIG.company.address}
              </li>
            </ul>
          </div>
        </div>

        <div className="mt-12 pt-8 border-t border-white/[0.06] flex flex-col sm:flex-row items-center justify-between gap-4">
          <p className="text-sm text-dark-500">
            &copy; {new Date().getFullYear()} {SITE_CONFIG.company.legalName}. All rights reserved.
          </p>
          <div className="flex gap-6">
            <a href="#" className="text-sm text-dark-500 hover:text-dark-400 transition-colors">Privacy Policy</a>
            <a href="#" className="text-sm text-dark-500 hover:text-dark-400 transition-colors">Terms of Service</a>
          </div>
        </div>
      </div>
    </footer>
  );
}
