import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StatusBadge, statusTone } from '../StatusBadge';

describe('statusTone', () => {
  it('returns neutral for null/undefined/empty', () => {
    expect(statusTone(null)).toBe('neutral');
    expect(statusTone(undefined)).toBe('neutral');
    expect(statusTone('')).toBe('neutral');
  });

  it('returns success for active statuses', () => {
    expect(statusTone('active')).toBe('success');
    expect(statusTone('online')).toBe('success');
    expect(statusTone('paid')).toBe('success');
    expect(statusTone('completed')).toBe('success');
    expect(statusTone('approved')).toBe('success');
    expect(statusTone('open')).toBe('success');
    expect(statusTone('resolved')).toBe('success');
    expect(statusTone('healthy')).toBe('success');
    expect(statusTone('connected')).toBe('success');
  });

  it('returns danger for negative statuses', () => {
    expect(statusTone('suspended')).toBe('danger');
    expect(statusTone('terminated')).toBe('danger');
    expect(statusTone('expired')).toBe('danger');
    expect(statusTone('cancelled')).toBe('danger');
    expect(statusTone('overdue')).toBe('danger');
    expect(statusTone('failed')).toBe('danger');
    expect(statusTone('closed')).toBe('danger');
    expect(statusTone('rejected')).toBe('danger');
  });

  it('returns warning for pending statuses', () => {
    expect(statusTone('pending')).toBe('warning');
    expect(statusTone('in_progress')).toBe('warning');
    expect(statusTone('draft')).toBe('warning');
    expect(statusTone('processing')).toBe('warning');
    expect(statusTone('unpaid')).toBe('warning');
    expect(statusTone('retrying')).toBe('warning');
  });

  it('returns info for informational statuses', () => {
    expect(statusTone('upgrading')).toBe('info');
    expect(statusTone('downgrading')).toBe('info');
    expect(statusTone('syncing')).toBe('info');
  });

  it('returns neutral for unknown statuses', () => {
    expect(statusTone('something_else')).toBe('neutral');
  });

  it('is case-insensitive', () => {
    expect(statusTone('ACTIVE')).toBe('success');
    expect(statusTone('Active')).toBe('success');
  });
});

describe('StatusBadge', () => {
  it('renders status text', () => {
    render(<StatusBadge status="active" />);
    expect(screen.getByText('Active')).toBeInTheDocument();
  });

  it('renders dash for null status', () => {
    render(<StatusBadge status={null} />);
    expect(screen.getByText('—')).toBeInTheDocument();
  });

  it('renders correct tone class for success', () => {
    const { container } = render(<StatusBadge status="active" />);
    const badge = container.querySelector('span');
    expect(badge?.className).toContain('emerald');
  });

  it('renders correct tone class for danger', () => {
    const { container } = render(<StatusBadge status="suspended" />);
    const badge = container.querySelector('span');
    expect(badge?.className).toContain('red');
  });

  it('renders correct tone class for warning', () => {
    const { container } = render(<StatusBadge status="pending" />);
    const badge = container.querySelector('span');
    expect(badge?.className).toContain('amber');
  });

  it('renders pulse indicator when pulse=true', () => {
    const { container } = render(<StatusBadge status="active" pulse />);
    const pings = container.querySelectorAll('.animate-ping');
    expect(pings.length).toBeGreaterThan(0);
  });

  it('does not render pulse indicator when pulse=false', () => {
    const { container } = render(<StatusBadge status="active" pulse={false} />);
    const pings = container.querySelectorAll('.animate-ping');
    expect(pings.length).toBe(0);
  });
});
