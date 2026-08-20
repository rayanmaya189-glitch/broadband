import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ErrorBoundary from '../ErrorBoundary';

function ThrowingComponent(): React.ReactNode {
  throw new Error('Test error');
}

function GoodComponent() {
  return <div>All good</div>;
}

describe('ErrorBoundary', () => {
  it('renders children when no error', () => {
    render(
      <ErrorBoundary>
        <GoodComponent />
      </ErrorBoundary>
    );
    expect(screen.getByText('All good')).toBeInTheDocument();
  });

  it('renders default fallback UI when error occurs', () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    render(
      <ErrorBoundary>
        <ThrowingComponent />
      </ErrorBoundary>
    );
    expect(screen.getByText('Something went wrong')).toBeInTheDocument();
    expect(screen.getByText('Try Again')).toBeInTheDocument();
    consoleSpy.mockRestore();
  });

  it('renders custom fallback when provided', () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    render(
      <ErrorBoundary fallback={<div>Custom error UI</div>}>
        <ThrowingComponent />
      </ErrorBoundary>
    );
    expect(screen.getByText('Custom error UI')).toBeInTheDocument();
    consoleSpy.mockRestore();
  });

  it('resets error state on retry click', async () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const user = userEvent.setup();

    let shouldThrow = true;
    const ConditionalComponent = () => {
      if (shouldThrow) throw new Error('Test error');
      return <div>Recovered</div>;
    };

    const { rerender } = render(
      <ErrorBoundary>
        <ConditionalComponent />
      </ErrorBoundary>
    );

    expect(screen.getByText('Something went wrong')).toBeInTheDocument();

    // Stop throwing before retry
    shouldThrow = false;
    await user.click(screen.getByText('Try Again'));

    rerender(
      <ErrorBoundary>
        <ConditionalComponent />
      </ErrorBoundary>
    );

    expect(screen.getByText('Recovered')).toBeInTheDocument();
    consoleSpy.mockRestore();
  });
});
