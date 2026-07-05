import { motion } from 'framer-motion';
import { FiFacebook, FiTwitter, FiInstagram, FiLinkedin, FiYoutube, FiMail, FiPhone, FiMapPin } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useScrollTo } from '../hooks/useScrollTo';

const socialIcons = [
  { icon: FiFacebook, href: 'https://facebook.com', label: 'Facebook' },
  { icon: FiTwitter, href: 'https://twitter.com', label: 'Twitter' },
  { icon: FiInstagram, href: 'https://instagram.com', label: 'Instagram' },
  { icon: FiLinkedin, href: 'https://linkedin.com', label: 'LinkedIn' },
  { icon: FiYoutube, href: 'https://youtube.com', label: 'YouTube' },
];

/**
 * Footer Component
 * Professional footer with links, social icons, and company info.
 */
export default function Footer() {
  const scrollTo = useScrollTo();
  const { company, navLinks } = SITE_CONFIG;
  const currentYear = new Date().getFullYear();

  const footerLinks = {
    QuickLinks: navLinks,
    Services: [
      { label: 'Home Internet', href: '#features' },
      { label: 'Business Internet', href: '#features' },
      { label: 'Gaming Plans', href: '#plans' },
      { label: 'Enterprise', href: '#contact' },
    ],
    Support: [
      { label: 'Contact Us', href: '#contact' },
      { label: 'FAQ', href: '#faq' },
      { label: 'Coverage', href: '#coverage' },
      { label: 'Installation', href: '#installation' },
    ],
  };

  return (
    <footer className="relative bg-dark-950 border-t border-white/[0.04]" role="contentinfo">
      {/* Top wave gradient */}
      <div className="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-accent-500/30 to-transparent" />

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16 lg:py-20">
        <div className="grid grid-cols-2 lg:grid-cols-5 gap-8 lg:gap-12">
          {/* Brand column */}
          <div className="col-span-2 lg:col-span-2">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5 }}
            >
              <button
                onClick={() => scrollTo('home')}
                className="flex items-center gap-3 mb-4 group focus:outline-none focus:ring-2 focus:ring-accent-400 rounded-lg"
              >
                <div className="relative w-9 h-9 rounded-lg overflow-hidden bg-gradient-to-br from-primary-500 to-accent-500 p-0.5">
                  <div className="w-full h-full rounded-lg bg-dark-900 flex items-center justify-center">
                    <img
                      src="/logo.png"
                      alt="AeroXe Broadband"
                      className="w-full h-full object-contain rounded-lg"
                      loading="lazy"
                    />
                  </div>
                </div>
                <span className="text-lg font-bold text-white">AeroXe</span>
              </button>

              <p className="text-sm text-dark-400 leading-relaxed mb-6 max-w-xs">
                {company.description}
              </p>

              {/* Contact info */}
              <div className="space-y-3 mb-6">
                <a
                  href={`tel:${company.phone}`}
                  className="flex items-center gap-3 text-sm text-dark-400 hover:text-accent-300 transition-colors group"
                >
                  <FiPhone className="w-4 h-4 text-accent-500 group-hover:scale-110 transition-transform" />
                  {company.phone}
                </a>
                <a
                  href={`mailto:${company.email}`}
                  className="flex items-center gap-3 text-sm text-dark-400 hover:text-accent-300 transition-colors group"
                >
                  <FiMail className="w-4 h-4 text-accent-500 group-hover:scale-110 transition-transform" />
                  {company.email}
                </a>
                <div className="flex items-start gap-3 text-sm text-dark-400">
                  <FiMapPin className="w-4 h-4 text-accent-500 flex-shrink-0 mt-0.5" />
                  <div>
                    <span>{company.address}</span>
                    <a 
                      href={SITE_CONFIG.location.mapUrl}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="block mt-1 text-accent-400 hover:text-accent-300 transition-colors underline underline-offset-2"
                    >
                      View on Google Maps
                    </a>
                  </div>
                </div>
              </div>

              {/* Social icons */}
              <div className="flex items-center gap-3">
                {socialIcons.map((social) => (
                  <motion.a
                    key={social.label}
                    href={social.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="w-9 h-9 rounded-xl bg-white/[0.03] border border-white/[0.06] flex items-center justify-center text-dark-400 hover:text-white hover:bg-accent-500/20 hover:border-accent-500/30 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400"
                    whileHover={{ y: -2, scale: 1.05 }}
                    aria-label={social.label}
                  >
                    <social.icon className="w-4 h-4" />
                  </motion.a>
                ))}
              </div>
            </motion.div>
          </div>

          {/* Links columns */}
          {Object.entries(footerLinks).map(([title, links], colIndex) => (
            <div key={title}>
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.5, delay: colIndex * 0.1 }}
              >
                <h3 className="text-sm font-semibold text-white mb-4 tracking-wide uppercase">
                  {title}
                </h3>
                <ul className="space-y-3">
                  {links.map((link) => (
                    <li key={link.label}>
                      <button
                        onClick={() => scrollTo(link.href.replace('#', ''))}
                        className="text-sm text-dark-400 hover:text-accent-300 transition-colors duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400 rounded"
                      >
                        {link.label}
                      </button>
                    </li>
                  ))}
                </ul>
              </motion.div>
            </div>
          ))}
        </div>

        {/* Bottom bar */}
        <div className="mt-12 lg:mt-16 pt-8 border-t border-white/[0.04]">
          <div className="flex flex-col lg:flex-row items-center justify-between gap-4">
            <p className="text-sm text-dark-500 text-center lg:text-left">
              &copy; {currentYear} {company.name}. All rights reserved.
            </p>
            <div className="flex items-center gap-6">
              <button className="text-sm text-dark-500 hover:text-dark-300 transition-colors focus:outline-none focus:ring-2 focus:ring-accent-400 rounded">
                Privacy Policy
              </button>
              <button className="text-sm text-dark-500 hover:text-dark-300 transition-colors focus:outline-none focus:ring-2 focus:ring-accent-400 rounded">
                Terms of Service
              </button>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}
