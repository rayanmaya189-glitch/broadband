import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Button } from '../Button';

describe('Button', () => {
  it('renders children text', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByRole('button', { name: 'Click me' })).toBeInTheDocument();
  });

  it('calls onClick when clicked', async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Click</Button>);
    await user.click(screen.getByRole('button'));
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('is disabled when disabled prop is true', () => {
    render(<Button disabled>Disabled</Button>);
    expect(screen.getByRole('button')).toBeDisabled();
  });

  it('does not call onClick when disabled', async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();
    render(<Button disabled onClick={onClick}>Disabled</Button>);
    await user.click(screen.getByRole('button'));
    expect(onClick).not.toHaveBeenCalled();
  });

  it('shows spinner when loading', () => {
    const { container } = render(<Button loading>Loading</Button>);
    const spinner = container.querySelector('.animate-spin');
    expect(spinner).toBeInTheDocument();
  });

  it('is disabled when loading', () => {
    render(<Button loading>Loading</Button>);
    expect(screen.getByRole('button')).toBeDisabled();
  });

  it('does not call onClick when loading', async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();
    render(<Button loading onClick={onClick}>Loading</Button>);
    await user.click(screen.getByRole('button'));
    expect(onClick).not.toHaveBeenCalled();
  });

  it('applies primary variant classes by default', () => {
    const { container } = render(<Button>Primary</Button>);
    const btn = container.querySelector('button');
    expect(btn?.className).toContain('bg-primary-600');
  });

  it('applies danger variant classes', () => {
    const { container } = render(<Button variant="danger">Danger</Button>);
    const btn = container.querySelector('button');
    expect(btn?.className).toContain('red');
  });

  it('applies ghost variant classes', () => {
    const { container } = render(<Button variant="ghost">Ghost</Button>);
    const btn = container.querySelector('button');
    expect(btn?.className).toContain('bg-transparent');
  });

  it('applies sm size classes', () => {
    const { container } = render(<Button size="sm">Small</Button>);
    const btn = container.querySelector('button');
    expect(btn?.className).toContain('text-xs');
  });

  it('applies md size classes by default', () => {
    const { container } = render(<Button>Medium</Button>);
    const btn = container.querySelector('button');
    expect(btn?.className).toContain('text-sm');
  });

  it('forwards additional HTML attributes', () => {
    render(<Button type="submit" data-testid="submit-btn">Submit</Button>);
    const btn = screen.getByTestId('submit-btn');
    expect(btn).toHaveAttribute('type', 'submit');
  });

  it('merges custom className', () => {
    const { container } = render(<Button className="my-custom-class">Custom</Button>);
    const btn = container.querySelector('button');
    expect(btn?.className).toContain('my-custom-class');
  });
});
