import { useRef, useState, useCallback } from 'react';

interface TiltOptions {
  tiltDegree?: number;
}

export function useTilt(options: TiltOptions = {}) {
  const { tiltDegree = 6 } = options;
  const ref = useRef<HTMLDivElement>(null);
  const [rotate, setRotate] = useState({ x: 0, y: 0 });
  const [glow, setGlow] = useState({ x: 50, y: 50 });
  const [isHovered, setIsHovered] = useState(false);

  const handleMouseMove = useCallback(
    (e: React.MouseEvent<HTMLDivElement>) => {
      if (!ref.current) return;
      const rect = ref.current.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const cx = rect.width / 2;
      const cy = rect.height / 2;
      setRotate({
        x: ((y - cy) / cy) * -tiltDegree,
        y: ((x - cx) / cx) * tiltDegree,
      });
      setGlow({
        x: (x / rect.width) * 100,
        y: (y / rect.height) * 100,
      });
    },
    [tiltDegree]
  );

  const handleMouseEnter = useCallback(() => setIsHovered(true), []);
  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    setRotate({ x: 0, y: 0 });
    setGlow({ x: 50, y: 50 });
  }, []);

  const tiltStyle: React.CSSProperties = {
    transform: `rotateX(${rotate.x}deg) rotateY(${rotate.y}deg)`,
    transformStyle: 'preserve-3d',
    transition: isHovered ? 'transform 0.08s ease-out' : 'transform 0.4s ease-out',
  };

  return {
    ref,
    tiltStyle,
    glowPos: glow,
    isHovered,
    handlers: {
      onMouseMove: handleMouseMove,
      onMouseEnter: handleMouseEnter,
      onMouseLeave: handleMouseLeave,
    },
  };
}
