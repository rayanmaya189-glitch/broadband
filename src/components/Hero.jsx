import { motion } from 'framer-motion';
import { FiArrowRight, FiCheck, FiTrendingUp, FiHeadphones, FiActivity } from 'react-icons/fi';
import { useScrollTo } from '../hooks/useScrollTo';
import { fadeUp, staggerContainer } from '../utils/animations';

/**
 * Hero Section Component
 * Premium hero with AI influencer lady illustration,
 * CTA buttons, and key statistics.
 */
export default function Hero() {
  const scrollTo = useScrollTo();

  const stats = [
    { icon: FiTrendingUp, value: '500+', label: 'Happy Customers' },
    { icon: FiCheck, value: '5', label: 'Speed Plans' },
    { icon: FiHeadphones, value: '24/7', label: 'Support' },
    { icon: FiActivity, value: '99.99%', label: 'Uptime' },
  ];

  const floatingCards = [
    { text: '✓ Fast Internet', delay: 0 },
    { text: '✓ Unlimited Data', delay: 0.5 },
    { text: '✓ 24/7 Support', delay: 1 },
    { text: '✓ Fiber Connection', delay: 1.5 },
  ];

  return (
    <section
      id="home"
      className="relative min-h-screen pt-24 lg:pt-0 flex items-center overflow-hidden"
      aria-label="Hero section"
    >
      {/* Background gradient */}
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      {/* Grid overlay */}
      <div
        className="absolute inset-0 opacity-[0.03]"
        style={{
          backgroundImage: `radial-gradient(circle at 1px 1px, rgba(255,255,255,0.3) 1px, transparent 0)`,
          backgroundSize: '40px 40px',
        }}
      />

      {/* Hero content */}
      <div className="relative w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 lg:py-24">
        <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          {/* Left side - Text content */}
          <motion.div
            variants={staggerContainer}
            initial="hidden"
            animate="visible"
            className="text-center lg:text-left"
          >
            {/* Badge */}
            <motion.div
              variants={fadeUp}
              className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-accent-500/10 border border-accent-500/20 mb-6 lg:mb-8"
            >
              <span className="w-2 h-2 rounded-full bg-accent-400 animate-pulse" />
              <span className="text-xs lg:text-sm font-medium text-accent-300 tracking-wider uppercase">
                Now Available in Jalgaon
              </span>
            </motion.div>

            {/* Main heading */}
            <motion.h1
              variants={fadeUp}
              className="text-4xl sm:text-5xl lg:text-6xl xl:text-7xl font-bold leading-tight tracking-tight mb-6"
            >
              <span className="bg-gradient-to-r from-white via-white to-dark-200 bg-clip-text text-transparent">
                Lightning Fast
              </span>
              <br />
              <span className="bg-gradient-to-r from-accent-400 via-primary-400 to-accent-500 bg-clip-text text-transparent">
                Fiber Internet
              </span>
              <br />
              <span className="text-white">
                In Jalgaon
              </span>
            </motion.h1>

            {/* Description */}
            <motion.p
              variants={fadeUp}
              className="text-base lg:text-lg text-dark-300 max-w-xl mx-auto lg:mx-0 leading-relaxed mb-8"
            >
              Experience the power of true fiber optic technology in Jalgaon. 
              Ultra-fast speeds up to 300 Mbps, free installation, and 24/7 
              local support — starting at just ₹400/month.
            </motion.p>

            {/* CTA Buttons */}
            <motion.div
              variants={fadeUp}
              className="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start mb-12"
            >
              <motion.button
                onClick={() => scrollTo('plans')}
                className="group relative px-8 py-4 text-base font-semibold text-white rounded-2xl overflow-hidden focus:outline-none focus:ring-2 focus:ring-accent-400 focus:ring-offset-2 focus:ring-offset-dark-900"
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
              >
                <span className="absolute inset-0 bg-gradient-to-r from-primary-600 to-accent-600" />
                <span className="absolute inset-0 bg-gradient-to-r from-accent-500 to-primary-500 opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                <span className="relative z-10 flex items-center gap-2">
                  View Plans
                  <FiArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform duration-300" />
                </span>
              </motion.button>

              <motion.button
                onClick={() => scrollTo('contact')}
                className="px-8 py-4 text-base font-semibold text-white rounded-2xl border border-white/10 bg-white/5 backdrop-blur-sm hover:bg-white/10 hover:border-white/20 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400 focus:ring-offset-2 focus:ring-offset-dark-900"
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
              >
                Contact Us
              </motion.button>
            </motion.div>

            {/* Statistics */}
            <motion.div
              variants={fadeUp}
              className="grid grid-cols-2 lg:grid-cols-4 gap-4 lg:gap-6"
            >
              {stats.map((stat, index) => (
                <motion.div
                  key={stat.label}
                  className="glass-card p-4 rounded-xl text-center"
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.6 + index * 0.1, duration: 0.5 }}
                  whileHover={{ y: -4, transition: { duration: 0.2 } }}
                >
                  <stat.icon className="w-5 h-5 text-accent-400 mx-auto mb-2" />
                  <div className="text-lg lg:text-xl font-bold text-white">{stat.value}</div>
                  <div className="text-xs text-dark-400 mt-0.5">{stat.label}</div>
                </motion.div>
              ))}
            </motion.div>
          </motion.div>

          {/* Right side - AI Influencer Lady */}
          <motion.div
            initial={{ opacity: 0, x: 100 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.8, delay: 0.3 }}
            className="relative flex items-center justify-center lg:justify-end"
          >
            <div className="relative w-full max-w-md lg:max-w-none">
              {/* Premium AI Lady Illustration */}
              <div className="relative">
                {/* Floating glass cards */}
                {floatingCards.map((card, index) => (
                  <motion.div
                    key={index}
                    className="absolute glass-card px-3 py-2 rounded-lg backdrop-blur-md bg-white/5 border border-white/10 shadow-xl z-10 whitespace-nowrap"
                    style={{
                      top: `${15 + index * 18}%`,
                      right: index % 2 === 0 ? '-10%' : 'auto',
                      left: index % 2 === 1 ? '-10%' : 'auto',
                    }}
                    animate={{
                      y: [0, -8, 0],
                      opacity: [0.8, 1, 0.8],
                    }}
                    transition={{
                      duration: 3,
                      delay: card.delay,
                      repeat: Infinity,
                      ease: 'easeInOut',
                    }}
                  >
                    <span className="text-xs lg:text-sm font-medium text-white">
                      {card.text}
                    </span>
                  </motion.div>
                ))}

                {/* AI Lady SVG Illustration */}
                <div className="relative w-full aspect-[3/4] max-w-[400px] mx-auto">
                  <svg
                    viewBox="0 0 400 500"
                    className="w-full h-full"
                    xmlns="http://www.w3.org/2000/svg"
                    aria-label="Professional businesswoman promoting internet service"
                  >
                    {/* Background glow */}
                    <defs>
                      <radialGradient id="heroGlow" cx="50%" cy="50%" r="50%">
                        <stop offset="0%" stopColor="#06b6d4" stopOpacity="0.15" />
                        <stop offset="100%" stopColor="#06b6d4" stopOpacity="0" />
                      </radialGradient>
                      <linearGradient id="hairGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                        <stop offset="0%" stopColor="#1a1a2e" />
                        <stop offset="100%" stopColor="#16213e" />
                      </linearGradient>
                      <linearGradient id="suitGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                        <stop offset="0%" stopColor="#0a66c2" />
                        <stop offset="100%" stopColor="#0891b2" />
                      </linearGradient>
                      <linearGradient id="fiberLine" x1="0%" y1="0%" x2="100%" y2="0%">
                        <stop offset="0%" stopColor="#06b6d4" stopOpacity="0">
                          <animate attributeName="offset" values="0;1" dur="3s" repeatCount="indefinite" />
                        </stop>
                        <stop offset="50%" stopColor="#06b6d4" stopOpacity="0.8">
                          <animate attributeName="offset" values="0;1" dur="3s" repeatCount="indefinite" />
                        </stop>
                        <stop offset="100%" stopColor="#06b6d4" stopOpacity="0">
                          <animate attributeName="offset" values="0;1" dur="3s" repeatCount="indefinite" />
                        </stop>
                      </linearGradient>
                      <filter id="softShadow">
                        <feGaussianBlur in="SourceAlpha" stdDeviation="4" />
                        <feOffset dx="0" dy="4" />
                        <feMerge>
                          <feMergeNode />
                          <feMergeNode in="SourceGraphic" />
                        </feMerge>
                      </filter>
                    </defs>

                    {/* Background circle */}
                    <circle cx="200" cy="250" r="200" fill="url(#heroGlow)" />

                    {/* Fiber optic decorative lines */}
                    <g opacity="0.3">
                      <motion.path
                        d="M 50 400 Q 150 350 200 250 Q 250 150 350 100"
                        stroke="#06b6d4"
                        strokeWidth="1.5"
                        fill="none"
                        initial={{ pathLength: 0 }}
                        animate={{ pathLength: 1 }}
                        transition={{ duration: 3, repeat: Infinity }}
                      />
                      <motion.path
                        d="M 30 300 Q 100 280 200 200 Q 300 120 380 180"
                        stroke="#0a66c2"
                        strokeWidth="1"
                        fill="none"
                        initial={{ pathLength: 0 }}
                        animate={{ pathLength: 1 }}
                        transition={{ duration: 4, repeat: Infinity, delay: 0.5 }}
                      />
                      <motion.path
                        d="M 60 200 Q 120 180 200 300 Q 280 420 370 350"
                        stroke="#22d3ee"
                        strokeWidth="1"
                        fill="none"
                        initial={{ pathLength: 0 }}
                        animate={{ pathLength: 1 }}
                        transition={{ duration: 3.5, repeat: Infinity, delay: 1 }}
                      />
                    </g>

                    {/* Body - Professional Suit */}
                    <g filter="url(#softShadow)">
                      {/* Body */}
                      <path d="M 160 180 Q 150 280 155 400 L 245 400 Q 250 280 240 180 Z" fill="url(#suitGrad)" />
                      
                      {/* Jacket lapels */}
                      <path d="M 200 180 L 185 250 L 200 260 Z" fill="#0a4688" opacity="0.5" />
                      <path d="M 200 180 L 215 250 L 200 260 Z" fill="#0a4688" opacity="0.5" />
                      
                      {/* White blouse */}
                      <path d="M 180 180 Q 200 170 220 180 L 215 210 Q 200 220 185 210 Z" fill="#ffffff" />
                      
                      {/* Arms */}
                      <path d="M 155 190 Q 130 250 120 320 Q 115 340 125 345 L 135 340 Q 140 260 160 200" fill="url(#suitGrad)" />
                      <path d="M 245 190 Q 280 240 300 280 Q 310 300 300 310 L 290 305 Q 270 260 240 200" fill="url(#suitGrad)" />
                      
                      {/* Hands */}
                      <ellipse cx="127" cy="342" rx="8" ry="10" fill="#f5e6d3" />
                      <ellipse cx="297" cy="307" rx="8" ry="10" fill="#f5e6d3" />

                      {/* Smartphone in right hand */}
                      <g transform="translate(285, 275) rotate(-20)">
                        <rect x="0" y="0" width="30" height="50" rx="4" fill="#1a1a2e" stroke="#333" strokeWidth="1" />
                        <rect x="2" y="2" width="26" height="40" rx="2" fill="#0a66c2" opacity="0.3" />
                        <circle cx="15" cy="46" r="2" fill="#333" />
                        {/* Screen glow */}
                        <rect x="4" y="4" width="22" height="36" rx="1" fill="#06b6d4" opacity="0.1" />
                      </g>
                    </g>

                    {/* Neck */}
                    <rect x="190" y="165" width="20" height="20" rx="5" fill="#f5e6d3" />

                    {/* Head */}
                    <ellipse cx="200" cy="120" rx="40" ry="48" fill="#f5e6d3" />

                    {/* Hair - Professional bun */}
                    <path d="M 160 115 Q 160 60 200 55 Q 240 60 240 115 Q 235 90 200 85 Q 165 90 160 115 Z" fill="url(#hairGrad)" />
                    <path d="M 160 115 Q 155 95 170 78 Q 185 65 200 70 Q 215 65 230 78 Q 245 95 240 115" fill="url(#hairGrad)" />
                    
                    {/* Hair bun */}
                    <circle cx="200" cy="65" r="18" fill="#1a1a2e" />
                    <circle cx="195" cy="60" r="8" fill="#16213e" opacity="0.5" />
                    <circle cx="205" cy="62" r="6" fill="#16213e" opacity="0.3" />

                    {/* Face features */}
                    {/* Eyebrows */}
                    <path d="M 180 105 Q 185 100 192 102" stroke="#333" strokeWidth="1.5" fill="none" />
                    <path d="M 208 102 Q 215 100 220 105" stroke="#333" strokeWidth="1.5" fill="none" />

                    {/* Eyes */}
                    <ellipse cx="186" cy="112" rx="5" ry="3" fill="#333" />
                    <ellipse cx="214" cy="112" rx="5" ry="3" fill="#333" />
                    {/* Eye highlights */}
                    <circle cx="184" cy="111" r="1.5" fill="white" opacity="0.8" />
                    <circle cx="212" cy="111" r="1.5" fill="white" opacity="0.8" />

                    {/* Nose */}
                    <path d="M 200 115 Q 198 122 200 125 Q 202 122 200 115" fill="#e8c9b5" opacity="0.5" />

                    {/* Friendly smile */}
                    <path d="M 188 132 Q 200 142 212 132" stroke="#c4956a" strokeWidth="1.5" fill="none" strokeLinecap="round" />
                    
                    {/* Cheek blush */}
                    <ellipse cx="178" cy="128" rx="6" ry="3" fill="#f5a0b0" opacity="0.15" />
                    <ellipse cx="222" cy="128" rx="6" ry="3" fill="#f5a0b0" opacity="0.15" />

                    {/* Earrings */}
                    <circle cx="160" cy="125" r="2.5" fill="#06b6d4" opacity="0.6" />
                    <circle cx="240" cy="125" r="2.5" fill="#06b6d4" opacity="0.6" />

                    {/* Pointing hand (left hand pointing toward logo) */}
                    <g transform="translate(100, 290)">
                      <path d="M 15 10 Q 20 5 30 0 Q 35 -2 32 5 Q 30 10 25 15" fill="#f5e6d3" />
                      <path d="M 30 0 Q 40 -10 50 -15" stroke="#f5e6d3" strokeWidth="3" fill="none" strokeLinecap="round" />
                      {/* Pointing finger */}
                      <path d="M 50 -15 L 58 -20 L 60 -18 L 52 -13" fill="#f5e6d3" />
                    </g>

                    {/* Animated glow particles around the figure */}
                    <circle cx="130" cy="150" r="3" fill="#06b6d4" opacity="0.6">
                      <animate attributeName="opacity" values="0.6;0;0.6" dur="2s" repeatCount="indefinite" />
                      <animate attributeName="cy" values="150;140;150" dur="3s" repeatCount="indefinite" />
                    </circle>
                    <circle cx="270" cy="200" r="2" fill="#22d3ee" opacity="0.5">
                      <animate attributeName="opacity" values="0.5;0;0.5" dur="2.5s" repeatCount="indefinite" />
                      <animate attributeName="cy" values="200;185;200" dur="4s" repeatCount="indefinite" />
                    </circle>
                    <circle cx="150" cy="300" r="2.5" fill="#0a66c2" opacity="0.4">
                      <animate attributeName="opacity" values="0.4;0;0.4" dur="3s" repeatCount="indefinite" />
                      <animate attributeName="cx" values="150;155;150" dur="3s" repeatCount="indefinite" />
                    </circle>
                    <circle cx="260" cy="130" r="2" fill="#06b6d4" opacity="0.5">
                      <animate attributeName="opacity" values="0.5;0.1;0.5" dur="1.8s" repeatCount="indefinite" />
                      <animate attributeName="cy" values="130;120;130" dur="2.5s" repeatCount="indefinite" />
                    </circle>
                  </svg>
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      </div>

      {/* Bottom gradient fade */}
      <div className="absolute bottom-0 left-0 right-0 h-32 bg-gradient-to-t from-dark-950 to-transparent" />
    </section>
  );
}
