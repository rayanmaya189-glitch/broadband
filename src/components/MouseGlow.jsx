import { useEffect, useRef } from 'react';

/**
 * Mouse Glow Effect Component
 * Creates a subtle gradient glow that follows the mouse cursor.
 * Only visible on devices with a mouse (not touch devices).
 */
export default function MouseGlow() {
  const glowRef = useRef(null);

  useEffect(() => {
    const glow = glowRef.current;
    if (!glow) return;

    const handleMouseMove = (e) => {
      const { clientX, clientY } = e;
      glow.style.transform = `translate(${clientX - 150}px, ${clientY - 150}px)`;
    };

    const handleMouseLeave = () => {
      glow.style.opacity = '0';
    };

    const handleMouseEnter = () => {
      glow.style.opacity = '1';
    };

    // Only enable on devices that support hover (mouse)
    const isTouchDevice = window.matchMedia('(hover: none)').matches;
    if (isTouchDevice) return;

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseleave', handleMouseLeave);
    document.addEventListener('mouseenter', handleMouseEnter);

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseleave', handleMouseLeave);
      document.removeEventListener('mouseenter', handleMouseEnter);
    };
  }, []);

  return (
    <div
      ref={glowRef}
      className="fixed top-0 left-0 w-[300px] h-[300px] rounded-full bg-gradient-to-br from-primary-500/8 to-accent-500/8 blur-3xl pointer-events-none transition-opacity duration-500 z-[-5]"
      aria-hidden="true"
      style={{ willChange: 'transform' }}
    />
  );
}
