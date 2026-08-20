import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import Skeleton, { PlanCardSkeleton } from '../Skeleton';

describe('Skeleton', () => {
  it('renders with default classes', () => {
    const { container } = render(<Skeleton />);
    const el = container.firstChild as HTMLElement;
    expect(el).toBeInTheDocument();
    expect(el.tagName).toBe('DIV');
    expect(el.className).toContain('animate-shimmer');
  });

  it('accepts custom className', () => {
    const { container } = render(<Skeleton className="h-6 w-24" />);
    const el = container.firstChild as HTMLElement;
    expect(el.className).toContain('h-6');
    expect(el.className).toContain('w-24');
  });
});

describe('PlanCardSkeleton', () => {
  it('renders multiple skeleton elements', () => {
    const { container } = render(<PlanCardSkeleton />);
    const skeletons = container.querySelectorAll('.animate-shimmer');
    expect(skeletons.length).toBeGreaterThanOrEqual(4);
  });
});
