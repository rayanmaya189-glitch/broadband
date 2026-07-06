import { motion } from 'framer-motion';
import { useEffect, useState } from 'react';

interface LoaderProps {
  onLoaded?: () => void;
}

export default function Loader({ onLoaded }: LoaderProps) {
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const timer = setInterval(() => {
      setProgress((p) => {
        if (p >= 100) {
          clearInterval(timer);
          return 100;
        }
        return p + Math.random() * 15 + 5;
      });
    }, 200);

    return () => clearInterval(timer);
  }, []);

  useEffect(() => {
    if (progress >= 100) {
      const timeout = setTimeout(() => onLoaded?.(), 500);
      return () => clearTimeout(timeout);
    }
  }, [progress, onLoaded]);

  return (
    <motion.div
      exit={{ opacity: 0, scale: 1.1 }}
      transition={{ duration: 0.5, ease: 'easeInOut' }}
      className="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-dark-950"
    >
      <div className="relative mb-8">
        <motion.div
          animate={{ rotate: 360 }}
          transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}
          className="w-20 h-20 rounded-full border-2 border-transparent border-t-accent-400 border-r-primary-500"
        />
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="w-3 h-3 rounded-full bg-accent-400 shadow-lg shadow-accent-400/50" />
        </div>
      </div>

      <div className="w-48 h-1 bg-dark-700 rounded-full overflow-hidden">
        <motion.div
          className="h-full bg-gradient-to-r from-accent-400 to-primary-500 rounded-full"
          initial={{ width: '0%' }}
          animate={{ width: `${Math.min(progress, 100)}%` }}
          transition={{ duration: 0.3 }}
        />
      </div>

      <p className="mt-4 text-sm text-dark-400 font-mono">
        {Math.min(Math.floor(progress), 100)}%
      </p>
    </motion.div>
  );
}
