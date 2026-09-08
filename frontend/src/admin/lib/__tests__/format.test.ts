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
  it('formats number with ₹ symbol and 2 decimal places', () => {
    expect(formatCurrency(1234)).toBe('₹1,234.00');
  });

  it('formats zero', () => {
    expect(formatCurrency(0)).toBe('₹0.00');
  });

  it('formats negative numbers', () => {
    expect(formatCurrency(-500)).toBe('₹-500.00');
  });

  it('formats string numbers', () => {
    expect(formatCurrency('999.5')).toBe('₹999.50');
  });

  it('returns ₹0.00 for null/undefined (zero fallback)', () => {
    expect(formatCurrency(null)).toBe('₹0.00');
    expect(formatCurrency(undefined)).toBe('₹0.00');
  });

  it('returns ₹0 for NaN strings', () => {
    expect(formatCurrency('abc')).toBe('₹0');
  });

  it('formats large numbers with Indian grouping', () => {
    const result = formatCurrency(100000);
    expect(result).toContain('1,00,000');
  });
});

describe('formatNumber', () => {
  it('formats number with Indian grouping', () => {
    expect(formatNumber(1234)).toContain('1,234');
  });

  it('returns 0 for null/undefined', () => {
    expect(formatNumber(null)).toBe('0');
    expect(formatNumber(undefined)).toBe('0');
  });

  it('returns 0 for NaN strings', () => {
    expect(formatNumber('abc')).toBe('0');
  });

  it('formats string numbers', () => {
    expect(formatNumber('5678')).toContain('5,678');
  });
});

describe('formatDate', () => {
  it('returns — for null/undefined', () => {
    expect(formatDate(null)).toBe('—');
    expect(formatDate(undefined)).toBe('—');
  });

  it('returns — for invalid date strings', () => {
    expect(formatDate('not-a-date')).toBe('—');
  });

  it('formats valid ISO date', () => {
    const result = formatDate('2026-01-15');
    // Should contain day, month, year
    expect(result).toMatch(/\d{2}/);
    expect(result).not.toBe('—');
  });
});

describe('formatDateTime', () => {
  it('returns — for null/undefined', () => {
    expect(formatDateTime(null)).toBe('—');
    expect(formatDateTime(undefined)).toBe('—');
  });

  it('returns — for invalid dates', () => {
    expect(formatDateTime('invalid')).toBe('—');
  });

  it('formats valid datetime with time components', () => {
    const result = formatDateTime('2026-06-15T14:30:00Z');
    expect(result).not.toBe('—');
    // Should have some time-like content
    expect(result.length).toBeGreaterThan(5);
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

  it('returns — for null/undefined', () => {
    expect(timeAgo(null)).toBe('—');
    expect(timeAgo(undefined)).toBe('—');
  });

  it('returns — for invalid dates', () => {
    expect(timeAgo('not-a-date')).toBe('—');
  });

  it('returns "just now" for < 60 seconds ago', () => {
    expect(timeAgo('2026-07-01T11:59:30Z')).toBe('just now');
  });

  it('returns minutes ago', () => {
    expect(timeAgo('2026-07-01T11:55:00Z')).toBe('5m ago');
  });

  it('returns hours ago', () => {
    expect(timeAgo('2026-07-01T09:00:00Z')).toBe('3h ago');
  });

  it('returns days ago', () => {
    expect(timeAgo('2026-06-28T12:00:00Z')).toBe('3d ago');
  });

  it('falls back to formatDate for > 30 days', () => {
    const result = timeAgo('2026-04-01T12:00:00Z');
    // Should be formatted date, not "Xm ago"
    expect(result).not.toMatch(/\d+[mhd] ago/);
  });
});

describe('initials', () => {
  it('returns ? for null/undefined', () => {
    expect(initials(null)).toBe('?');
    expect(initials(undefined)).toBe('?');
  });

  it('returns first letter for single word', () => {
    expect(initials('John')).toBe('J');
  });

  it('returns first two initials for full name', () => {
    expect(initials('John Doe')).toBe('JD');
  });

  it('truncates to two initials for three+ names', () => {
    expect(initials('John Michael Doe')).toBe('JM');
  });

  it('handles extra whitespace', () => {
    expect(initials('  John   Doe  ')).toBe('JD');
  });

  it('uppercases single-letter names', () => {
    expect(initials('a')).toBe('A');
  });
});

describe('toLabel', () => {
  it('returns — for null/undefined', () => {
    expect(toLabel(null)).toBe('—');
    expect(toLabel(undefined)).toBe('—');
  });

  it('capitalizes single word', () => {
    expect(toLabel('active')).toBe('Active');
  });

  it('converts snake_case to Title Case', () => {
    expect(toLabel('in_progress')).toBe('In Progress');
  });

  it('converts complex snake_case', () => {
    expect(toLabel('no_connection')).toBe('No Connection');
  });

  it('handles empty string', () => {
    expect(toLabel('')).toBe('—');
  });
});
