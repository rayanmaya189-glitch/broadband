import { useRef, useState, useCallback } from 'react';
import { motion } from 'framer-motion';
import { Check, ArrowRight, Zap } from 'lucide-react';
import { Link } from 'react-router-dom';
import type { Plan } from '../../types';
import { formatPrice } from '../../utils/helpers';

interface PlanCardProps {
  plan: Plan;
  billingPeriod: number;
  index?: number;
}

export default function PlanCard({ plan, billingPeriod, index = 0 }: PlanCardProps) {
  const cardRef = useRef<HTMLDivElement>(null);
  const [rotate, setRotate] = useState({ x: 0, y: 0 });
  const [glow, setGlow] = useState({ x: 50, y: 50 });

  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    if (!cardRef.current) return;
    const rect = cardRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;
    const rotateX = ((y - centerY) / centerY) * -8;
    const rotateY = ((x - centerX) / centerX) * 8;
    setRotate({ x: rotateX, y: rotateY });
    setGlow({ x: (x / rect.width) * 100, y: (y / rect.height) * 100 });
  }, []);

  const handleMouseLeave = useCallback(() => {
    setRotate({ x: 0, y: 0 });
    setGlow({ x: 50, y: 50 });
  }, []);

  const duration = plan.durations[billingPeriod] || plan.durations[1];
  const monthlyPrice = duration.price;

  return (
    <motion.div
      initial={{ opacity: 0, y: 30 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: '-50px' }}
      transition={{ duration: 0.5, delay: index * 0.1 }}
      className={`relative group ${plan.popular ? 'lg:-mt-4 lg:mb-4' : ''}`}
      style={{ perspective: '1000px' }}
    >
      {plan.popular && (
        <div className="absolute -top-4 left-1/2 -translate-x-1/2 z-10">
          <span className="inline-flex items-center gap-1.5 px-4 py-1.5 bg-gradient-to-r from-accent-500 to-primary-600 text-white text-xs font-bold rounded-full shadow-lg shadow-accent-500/25">
            <Zap className="w-3.5 h-3.5" />
            Most Popular
          </span>
        </div>
      )}

      <div
        ref={cardRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        style={{
          transform: `rotateX(${rotate.x}deg) rotateY(${rotate.y}deg)`,
          transformStyle: 'preserve-3d',
          transition: 'transform 0.1s ease-out',
        }}
        className={`relative h-full rounded-2xl p-6 sm:p-8 flex flex-col transition-shadow duration-300 ${
          plan.popular
            ? 'bg-gradient-to-b from-accent-500/10 via-primary-500/5 to-dark-800 border-2 border-accent-500/30 shadow-xl shadow-accent-500/10 hover:shadow-2xl hover:shadow-accent-500/20'
            : 'glass-card border border-white/[0.06] hover:border-white/[0.15] hover:shadow-2xl'
        }`}
      >
        <div
          className="pointer-events-none absolute inset-0 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"
          style={{
            background: `radial-gradient(circle at ${glow.x}% ${glow.y}%, rgba(6,182,212,0.15), transparent 60%)`,
          }}
        />

        <div className="relative z-10" style={{ transform: 'translateZ(30px)' }}>
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm font-medium text-dark-400 uppercase tracking-wider">{plan.tag}</span>
          </div>
          <span className="text-xs text-dark-500 font-medium">up to</span>
          <h3 className="text-2xl sm:text-3xl font-bold text-white">{plan.speed}</h3>
        </div>

        <div className="relative z-10 mt-4 mb-6" style={{ transform: 'translateZ(40px)' }}>
          <div className="flex items-baseline gap-1.5">
            <span className="text-2xl sm:text-3xl font-bold text-white">
              {formatPrice(monthlyPrice)}
            </span>
            <span className="text-dark-400 text-sm">/mo</span>
          </div>
          {duration.savings && billingPeriod > 1 && (
            <p className="mt-1 text-xs text-accent-400 font-medium">{duration.savings}</p>
          )}
        </div>

        <ul className="relative z-10 space-y-2.5 mb-8 flex-1" style={{ transform: 'translateZ(20px)' }}>
          {plan.features.map((feature) => (
            <li key={feature} className="flex items-start gap-3">
              <Check className="w-4 h-4 text-accent-400 shrink-0 mt-0.5" />
              <span className="text-sm text-dark-300">{feature}</span>
            </li>
          ))}
        </ul>

        <div className="relative z-10" style={{ transform: 'translateZ(50px)' }}>
          <Link
            to={`/plan/${plan.id}`}
            className={`group/btn inline-flex items-center justify-center gap-2 w-full px-4 py-2.5 text-sm rounded-xl font-semibold transition-all duration-300 ${
              plan.popular
                ? 'bg-gradient-to-r from-accent-500 to-primary-600 text-white hover:shadow-lg hover:shadow-accent-500/25'
                : 'border border-white/10 bg-white/5 backdrop-blur-sm text-white hover:bg-white/10 hover:border-white/20'
            }`}
          >
            {plan.popular ? 'Get Connected Now' : 'Buy Now'}
            <ArrowRight className="w-4 h-4 group-hover/btn:translate-x-1 transition-transform" />
          </Link>
        </div>
      </div>
    </motion.div>
  );
}
