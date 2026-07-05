import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { FiStar, FiChevronLeft, FiChevronRight, FiMessageCircle } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';
import { fadeUp } from '../utils/animations';

/**
 * Testimonials Component
 * Professional testimonial cards with smooth carousel animation.
 */
export default function Testimonials() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.1 });
  const [currentIndex, setCurrentIndex] = useState(0);
  const [direction, setDirection] = useState(0);
  const { testimonials } = SITE_CONFIG;

  const slideVariants = {
    enter: (direction) => ({
      x: direction > 0 ? 300 : -300,
      opacity: 0,
    }),
    center: {
      x: 0,
      opacity: 1,
    },
    exit: (direction) => ({
      x: direction < 0 ? 300 : -300,
      opacity: 0,
    }),
  };

  const handleNext = () => {
    setDirection(1);
    setCurrentIndex((prev) => (prev + 1) % testimonials.length);
  };

  const handlePrev = () => {
    setDirection(-1);
    setCurrentIndex((prev) => (prev - 1 + testimonials.length) % testimonials.length);
  };

  // Generate initials for avatar
  const getInitials = (name) => {
    return name
      .split(' ')
      .map((n) => n[0])
      .join('')
      .toUpperCase();
  };

  const current = testimonials[currentIndex];

  return (
    <section className="relative py-20 lg:py-32 overflow-hidden" aria-label="Testimonials">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" ref={ref}>
        {/* Section header */}
        <motion.div
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="text-center max-w-3xl mx-auto mb-16 lg:mb-20"
        >
          <span className="text-xs lg:text-sm font-semibold text-accent-400 tracking-[0.2em] uppercase mb-4 block">
            Testimonials
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6">
            What Our{' '}
            <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
              Customers Say
            </span>
          </h2>
          <p className="text-base lg:text-lg text-dark-300 leading-relaxed">
            Real feedback from real customers who trust AeroXe for their internet needs.
          </p>
        </motion.div>

        {/* Carousel */}
        <div className="relative max-w-2xl mx-auto">
          {/* Quote icon */}
          <div className="text-center mb-8">
            <FiMessageCircle className="w-12 h-12 text-accent-500/20 mx-auto" />
          </div>

          {/* Testimonial card */}
          <div className="relative overflow-hidden min-h-[280px] lg:min-h-[240px]">
            <AnimatePresence initial={false} custom={direction} mode="wait">
              <motion.div
                key={currentIndex}
                custom={direction}
                variants={slideVariants}
                initial="enter"
                animate="center"
                exit="exit"
                transition={{ duration: 0.4, ease: 'easeInOut' }}
                className="absolute inset-0 flex items-center justify-center"
              >
                <div className="text-center px-4">
                  {/* Stars */}
                  <div className="flex items-center justify-center gap-1 mb-6">
                    {[...Array(current.rating)].map((_, i) => (
                      <motion.div
                        key={i}
                        initial={{ opacity: 0, scale: 0 }}
                        animate={{ opacity: 1, scale: 1 }}
                        transition={{ delay: 0.1 * i }}
                      >
                        <FiStar className="w-5 h-5 text-yellow-400 fill-yellow-400" />
                      </motion.div>
                    ))}
                  </div>

                  {/* Feedback */}
                  <blockquote className="text-base lg:text-lg text-dark-200 leading-relaxed mb-8 italic max-w-xl mx-auto">
                    "{current.feedback}"
                  </blockquote>

                  {/* Author */}
                  <div className="flex items-center justify-center gap-4">
                    <div className="w-12 h-12 rounded-full bg-gradient-to-br from-accent-500/30 to-primary-500/30 flex items-center justify-center border border-accent-500/20">
                      <span className="text-sm font-semibold text-accent-300">
                        {getInitials(current.name)}
                      </span>
                    </div>
                    <div className="text-left">
                      <p className="text-sm font-semibold text-white">{current.name}</p>
                      <p className="text-xs text-dark-400">{current.role}</p>
                    </div>
                  </div>
                </div>
              </motion.div>
            </AnimatePresence>
          </div>

          {/* Navigation */}
          <div className="flex items-center justify-center gap-4 mt-8">
            <motion.button
              onClick={handlePrev}
              className="p-2.5 rounded-xl bg-white/[0.04] border border-white/[0.06] text-dark-300 hover:text-white hover:bg-white/[0.08] hover:border-accent-500/30 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              aria-label="Previous testimonial"
            >
              <FiChevronLeft className="w-5 h-5" />
            </motion.button>

            {/* Dots */}
            <div className="flex items-center gap-2">
              {testimonials.map((_, index) => (
                <motion.button
                  key={index}
                  onClick={() => {
                    setDirection(index > currentIndex ? 1 : -1);
                    setCurrentIndex(index);
                  }}
                  className={`w-2 h-2 rounded-full transition-all duration-300 focus:outline-none ${
                    index === currentIndex
                      ? 'w-6 bg-gradient-to-r from-accent-400 to-primary-500'
                      : 'bg-white/10 hover:bg-white/20'
                  }`}
                  aria-label={`Go to testimonial ${index + 1}`}
                />
              ))}
            </div>

            <motion.button
              onClick={handleNext}
              className="p-2.5 rounded-xl bg-white/[0.04] border border-white/[0.06] text-dark-300 hover:text-white hover:bg-white/[0.08] hover:border-accent-500/30 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-accent-400"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              aria-label="Next testimonial"
            >
              <FiChevronRight className="w-5 h-5" />
            </motion.button>
          </div>
        </div>
      </div>
    </section>
  );
}
