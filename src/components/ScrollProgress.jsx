import { motion, useScroll, useSpring } from 'framer-motion';

/**
 * Scroll Progress Bar
 * Shows a thin gradient progress bar at the top of the page
 * indicating how far the user has scrolled.
 */
export default function ScrollProgress() {
  const { scrollYProgress } = useScroll();
  const scaleX = useSpring(scrollYProgress, {
    stiffness: 100,
    damping: 30,
    restDelta: 0.001,
  });

  return (
    <motion.div
      className="fixed top-0 left-0 right-0 z-[9998] h-[3px] origin-left bg-gradient-to-r from-accent-500 via-primary-500 to-accent-400 shadow-lg shadow-accent-500/25"
      style={{ scaleX }}
      role="progressbar"
      aria-label="Page scroll progress"
    />
  );
}
