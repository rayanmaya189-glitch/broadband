import { motion } from 'framer-motion';
import { Star, Quote } from 'lucide-react';
import { useIntersectionObserver } from '../../hooks/useIntersectionObserver';
import { SITE_CONFIG } from '../../config/site';
import TiltCard from '../ui/TiltCard';

export default function Testimonials() {
  const { ref, isVisible } = useIntersectionObserver();

  return (
    <section className="relative py-20 lg:py-28 overflow-hidden">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      <div ref={ref} className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 40 }}
          animate={isVisible ? { opacity: 1, y: 0 } : {}}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">
            What Our <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">Customers</span> Say
          </h2>
          <p className="mt-4 text-lg text-dark-400 max-w-2xl mx-auto">
            Real feedback from real people in {SITE_CONFIG.location.city}
          </p>
        </motion.div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {SITE_CONFIG.testimonials.map((testimonial, i) => (
            <motion.div
              key={testimonial.name}
              initial={{ opacity: 0, y: 30, scale: 0.95 }}
              animate={isVisible ? { opacity: 1, y: 0, scale: 1 } : {}}
              transition={{ duration: 0.5, delay: i * 0.1, type: 'spring', stiffness: 70 }}
            >
              <TiltCard className="h-full" tiltDegree={5} glareOpacity={0.1}>
                <div className="glass-card rounded-2xl p-6 h-full border border-white/[0.06] relative transition-shadow duration-300 hover:shadow-2xl hover:shadow-accent-500/5">
                  <Quote className="absolute top-4 right-4 w-8 h-8 text-accent-400/10" />
                  <div className="flex gap-1 mb-4">
                    {Array.from({ length: testimonial.rating }).map((_, j) => (
                      <Star key={j} className="w-4 h-4 fill-accent-400 text-accent-400" />
                    ))}
                  </div>
                  <p className="text-dark-300 text-sm leading-relaxed mb-6 italic">
                    &ldquo;{testimonial.feedback}&rdquo;
                  </p>
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-full bg-gradient-to-br from-accent-400 to-primary-600 flex items-center justify-center text-white text-sm font-bold">
                      {testimonial.name.charAt(0)}
                    </div>
                    <div>
                      <p className="text-sm font-semibold text-white">{testimonial.name}</p>
                      <p className="text-xs text-dark-500">{testimonial.role}</p>
                    </div>
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
