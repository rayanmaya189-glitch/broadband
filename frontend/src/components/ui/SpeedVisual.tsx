import { motion } from 'framer-motion';

interface SpeedVisualProps {
  speed?: number;
}

export default function SpeedVisual({ speed = 100 }: SpeedVisualProps) {
  const bars = Array.from({ length: 12 }, (_, i) => ({
    height: 20 + Math.random() * 60,
    delay: i * 0.08,
  }));

  return (
    <div className="flex items-end gap-1.5 h-24">
      {bars.map((bar, i) => (
        <motion.div
          key={i}
          className="w-3 rounded-full bg-gradient-to-t from-primary-500 to-accent-400"
          initial={{ height: 0 }}
          animate={{ height: bar.height }}
          transition={{
            duration: 0.6,
            delay: bar.delay,
            repeat: Infinity,
            repeatType: 'reverse',
            repeatDelay: 2,
          }}
        />
      ))}
      <div className="ml-3 flex flex-col justify-center">
        <motion.span
          className="text-3xl font-bold text-white"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.5 }}
        >
          {speed}
        </motion.span>
        <span className="text-xs text-dark-400 font-medium">Mbps</span>
      </div>
    </div>
  );
}
