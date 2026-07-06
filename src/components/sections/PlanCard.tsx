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
  isFocused?: boolean;
  onHover?: () => void;
  onLeave?: () => void;
}

export default function PlanCard({ plan, billingPeriod, index = 0, isFocused, onHover, onLeave }: PlanCardProps) {
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
    const rotateX = ((y - centerY) / centerY) * -6;
    const rotateY = ((x - centerX) / centerX) * 6;
    setRotate({ x: rotateX, y: rotateY });
    setGlow({ x: (x / rect.width) * 100, y: (y / rect.height) * 100 });
  }, []);

  const handleMouseLeave = useCallback(() => {
    setRotate({ x: 0, y: 0 });
    setGlow({ x: 50, y: 50 });
    onLeave?.();
  }, [onLeave]);

  const duration = plan.durations[billingPeriod] || plan.durations[1];
  const monthlyPrice = duration.price;

  return (
    <motion.div
      initial={{ opacity: 0, y: 30 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: '-50px' }}
      transition={{ duration: 0.5, delay: index * 0.08 }}
      onMouseEnter={onHover}
      className="relative"
      style={{ perspective: '800px' }}
    >
      {plan.popular && isFocused && (
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="absolute -top-3 left-1/2 -translate-x-1/2 z-20"
        >
          <span className="inline-flex items-center gap-1.5 px-3 py-1 bg-gradient-to-r from-accent-500 to-primary-600 text-white text-[10px] font-bold rounded-full shadow-lg shadow-accent-500/25">
            <Zap className="w-3 h-3" />
            Most Popular
          </span>
        </motion.div>
      )}

      <div
        ref={cardRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        className={`relative rounded-2xl flex flex-col ${
          plan.popular
            ? 'bg-gradient-to-b from-accent-500/10 via-primary-500/5 to-dark-800 border-2 border-accent-500/30 shadow-xl shadow-accent-500/10'
            : 'glass-card border border-white/[0.06]'
        }`}
        style={{
          transform: `rotateX(${rotate.x}deg) rotateY(${rotate.y}deg) scale(${isFocused ? 1 : 0.88})`,
          transformStyle: 'preserve-3d',
          opacity: isFocused ? 1 : 0.4,
          filter: isFocused ? 'none' : 'blur(1px)',
          zIndex: isFocused ? 10 : 0,
          transition: 'all 0.45s cubic-bezier(0.34, 1.56, 0.64, 1)',
        }}
      >
        <div
          className="pointer-events-none absolute inset-0 rounded-2xl opacity-0 transition-opacity duration-500"
          style={{
            opacity: isFocused ? 1 : 0,
            background: `radial-gradient(circle at ${glow.x}% ${glow.y}%, rgba(6,182,212,0.15), transparent 60%)`,
          }}
        />

        <div className="relative z-10 p-4 sm:p-5">
          <div className="relative z-10" style={{ transform: isFocused ? 'translateZ(25px)' : 'none' }}>
            <div className="flex items-center justify-between mb-1.5">
              <span className="text-[10px] font-medium text-dark-400 uppercase tracking-widest">{plan.tag}</span>
            </div>
            <span className="text-[10px] text-dark-500 font-medium">up to</span>
            <h3 className="text-lg sm:text-xl font-bold text-white leading-tight">{plan.speed}</h3>
          </div>

          <div className="relative z-10 mt-2 mb-3" style={{ transform: isFocused ? 'translateZ(35px)' : 'none' }}>
            <div className="flex items-baseline gap-1">
              <span className="text-xl sm:text-2xl font-bold text-white">{formatPrice(monthlyPrice)}</span>
              <span className="text-dark-400 text-[11px]">/mo</span>
            </div>
            {duration.savings && billingPeriod > 1 && (
              <p className="mt-0.5 text-[10px] text-accent-400 font-medium">{duration.savings}</p>
            )}
          </div>

          <ul className="relative z-10 space-y-1.5 mb-4" style={{ transform: isFocused ? 'translateZ(15px)' : 'none' }}>
            {plan.features.map((feature) => (
              <li key={feature} className="flex items-start gap-2">
                <Check className="w-3 h-3 text-accent-400 shrink-0 mt-0.5" />
                <span className="text-[11px] text-dark-300">{feature}</span>
              </li>
            ))}
          </ul>

          <div className="relative z-10" style={{ transform: isFocused ? 'translateZ(45px)' : 'none' }}>
            <Link
              to={`/plan/${plan.id}`}
              className={`group/btn inline-flex items-center justify-center gap-1.5 w-full px-3 py-2 text-xs rounded-xl font-semibold transition-all duration-300 ${
                plan.popular
                  ? 'bg-gradient-to-r from-accent-500 to-primary-600 text-white hover:shadow-lg hover:shadow-accent-500/25'
                  : 'border border-white/10 bg-white/5 backdrop-blur-sm text-white hover:bg-white/10 hover:border-white/20'
              }`}
            >
              {plan.popular ? 'Get Connected' : 'Buy Now'}
              <ArrowRight className="w-3 h-3 group-hover/btn:translate-x-1 transition-transform" />
            </Link>
          </div>
        </div>
      </div>
    </motion.div>
  );
}
