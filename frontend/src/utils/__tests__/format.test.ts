import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
  formatCurrency,
  formatNumber,
  formatDate,
  formatDateTime,
  timeAgo,
  initials,
  toLabel,
} from '../format';

describe('formatCurrency', () => {
  it('formats positive numbers', () => {
    expect(formatCurrency(1234)).toBe('₹1,234.00');
  });

  it('formats zero', () => {
    expect(formatCurrency(0)).toBe('₹0.00');
  });

  it('formats negative numbers', () => {
    expect(formatCurrency(-100)).toContain('₹');
  });

  it('formats string numbers', () => {
    expect(formatCurrency('999')).toBe('₹999.00');
  });

  it('returns ₹0.00 for null (zero fallback)', () => {
    expect(formatCurrency(null)).toBe('₹0.00');
  });

  it('returns ₹0.00 for undefined (zero fallback)', () => {
    expect(formatCurrency(undefined)).toBe('₹0.00');
  });

  it('returns ₹0 for NaN strings', () => {
    expect(formatCurrency('abc')).toBe('₹0');
  });
});

describe('formatNumber', () => {
  it('formats numbers', () => {
    expect(formatNumber(1234)).toContain('1,234');
  });

  it('returns 0 for null', () => {
    expect(formatNumber(null)).toBe('0');
  });

  it('returns 0 for undefined', () => {
    expect(formatNumber(undefined)).toBe('0');
  });

  it('returns 0 for NaN strings', () => {
    expect(formatNumber('xyz')).toBe('0');
  });
});

describe('formatDate', () => {
  it('returns — for null', () => {
    expect(formatDate(null)).toBe('—');
  });

  it('returns — for undefined', () => {
    expect(formatDate(undefined)).toBe('—');
  });

  it('returns — for invalid date', () => {
    expect(formatDate('not-a-date')).toBe('—');
  });

  it('formats valid date', () => {
    const result = formatDate('2026-01-15');
    expect(result).not.toBe('—');
  });
});

describe('formatDateTime', () => {
  it('returns — for null', () => {
    expect(formatDateTime(null)).toBe('—');
  });

  it('returns — for invalid date', () => {
    expect(formatDateTime('garbage')).toBe('—');
  });

  it('formats valid datetime', () => {
    const result = formatDateTime('2026-06-15T14:30:00Z');
    expect(result).not.toBe('—');
  });
});

describe('timeAgo', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-07-01T12:00:00Z'));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('returns — for null', () => {
    expect(timeAgo(null)).toBe('—');
  });

  it('returns "just now" for recent timestamps', () => {
    expect(timeAgo('2026-07-01T11:59:30Z')).toBe('just now');
  });

  it('returns minutes ago', () => {
    expect(timeAgo('2026-07-01T11:50:00Z')).toBe('10m ago');
  });

  it('returns hours ago', () => {
    expect(timeAgo('2026-07-01T08:00:00Z')).toBe('4h ago');
  });

  it('returns days ago', () => {
    expect(timeAgo('2026-06-28T12:00:00Z')).toBe('3d ago');
  });
});

describe('initials', () => {
  it('returns ? for null', () => {
    expect(initials(null)).toBe('?');
  });

  it('returns ? for undefined', () => {
    expect(initials(undefined)).toBe('?');
  });

  it('returns first letter for single word', () => {
    expect(initials('John')).toBe('J');
  });

  it('returns two initials for full name', () => {
    expect(initials('John Doe')).toBe('JD');
  });

  it('truncates to two for three+ names', () => {
    expect(initials('John Michael Doe')).toBe('JM');
  });
});

describe('toLabel', () => {
  it('returns — for null', () => {
    expect(toLabel(null)).toBe('—');
  });

  it('returns — for undefined', () => {
    expect(toLabel(undefined)).toBe('—');
  });

  it('capitalizes single word', () => {
    expect(toLabel('active')).toBe('Active');
  });

  it('converts snake_case', () => {
    expect(toLabel('in_progress')).toBe('In Progress');
  });

  it('converts complex snake_case', () => {
    expect(toLabel('no_connection')).toBe('No Connection');
  });
});
