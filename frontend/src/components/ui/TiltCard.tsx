import { useTilt } from '../../hooks/useTilt';
import type { ReactNode } from 'react';

interface TiltCardProps {
  children: ReactNode;
  className?: string;
  tiltDegree?: number;
  glareOpacity?: number;
  perspective?: number;
}

export default function TiltCard({
  children,
  className = '',
  tiltDegree = 6,
  glareOpacity = 0.15,
  perspective = 800,
}: TiltCardProps) {
  const { ref, tiltStyle, glowPos, isHovered, handlers } = useTilt({ tiltDegree });

  return (
    <div
      ref={ref}
      {...handlers}
      className={`relative ${className}`}
      style={{ perspective: `${perspective}px` }}
    >
      <div style={tiltStyle}>
        <div
          className="pointer-events-none absolute inset-0 rounded-[inherit] transition-opacity duration-500 z-10"
          style={{
            opacity: isHovered ? 1 : 0,
            background: `radial-gradient(circle at ${glowPos.x}% ${glowPos.y}%, rgba(6,182,212,${glareOpacity}), transparent 60%)`,
          }}
        />
        {children}
      </div>
    </div>
  );
}
