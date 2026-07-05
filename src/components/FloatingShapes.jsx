import { motion } from 'framer-motion';

/**
 * Floating Shapes Component
 * Renders subtle animated geometric shapes in the background
 * for a premium, dynamic visual effect.
 */
const shapes = [
  {
    type: 'circle',
    size: 300,
    x: '10%',
    y: '20%',
    delay: 0,
    duration: 8,
    color: 'from-primary-500/5 to-accent-500/5',
  },
  {
    type: 'circle',
    size: 200,
    x: '80%',
    y: '40%',
    delay: 2,
    duration: 10,
    color: 'from-accent-500/5 to-cyan-500/5',
  },
  {
    type: 'blob',
    size: 400,
    x: '50%',
    y: '70%',
    delay: 1,
    duration: 12,
    color: 'from-primary-500/3 to-accent-500/3',
  },
  {
    type: 'circle',
    size: 150,
    x: '20%',
    y: '80%',
    delay: 3,
    duration: 7,
    color: 'from-accent-500/5 to-primary-500/5',
  },
  {
    type: 'blob',
    size: 250,
    x: '70%',
    y: '15%',
    delay: 4,
    duration: 9,
    color: 'from-primary-500/4 to-accent-400/4',
  },
];

export default function FloatingShapes() {
  return (
    <div className="fixed inset-0 pointer-events-none overflow-hidden -z-10" aria-hidden="true">
      {shapes.map((shape, index) => (
        <motion.div
          key={index}
          className={`absolute rounded-full bg-gradient-to-br ${shape.color} blur-3xl`}
          style={{
            width: shape.size,
            height: shape.size,
            left: shape.x,
            top: shape.y,
            transform: 'translate(-50%, -50%)',
          }}
          animate={{
            y: [0, -30, 0, 30, 0],
            x: [0, 20, -20, 10, 0],
            scale: [1, 1.1, 0.9, 1.05, 1],
          }}
          transition={{
            duration: shape.duration,
            delay: shape.delay,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      ))}
    </div>
  );
}
