import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Pagination } from '../Pagination';

describe('Pagination', () => {
  it('renders nothing when total fits in one page', () => {
    const { container } = render(
      <Pagination page={1} pageSize={20} total={10} onPageChange={vi.fn()} />
    );
    expect(container.innerHTML).toBe('');
  });

  it('renders nothing when total equals pageSize', () => {
    const { container } = render(
      <Pagination page={1} pageSize={20} total={20} onPageChange={vi.fn()} />
    );
    expect(container.innerHTML).toBe('');
  });

  it('shows page info for multi-page results', () => {
    render(<Pagination page={1} pageSize={20} total={50} onPageChange={vi.fn()} />);
    expect(screen.getByText(/Page 1 of 3/)).toBeInTheDocument();
    expect(screen.getByText(/50 records/)).toBeInTheDocument();
  });

  it('enables previous button only when not on first page', () => {
    const { rerender } = render(
      <Pagination page={1} pageSize={20} total={50} onPageChange={vi.fn()} />
    );
    const prevBtn = screen.getByLabelText('Previous page');
    expect(prevBtn).toBeDisabled();

    rerender(<Pagination page={2} pageSize={20} total={50} onPageChange={vi.fn()} />);
    const prevBtn2 = screen.getByLabelText('Previous page');
    expect(prevBtn2).not.toBeDisabled();
  });

  it('enables next button only when not on last page', () => {
    const { rerender } = render(
      <Pagination page={1} pageSize={20} total={50} onPageChange={vi.fn()} />
    );
    const nextBtn = screen.getByLabelText('Next page');
    expect(nextBtn).not.toBeDisabled();

    rerender(<Pagination page={3} pageSize={20} total={50} onPageChange={vi.fn()} />);
    const nextBtn2 = screen.getByLabelText('Next page');
    expect(nextBtn2).toBeDisabled();
  });

  it('calls onPageChange when clicking next', async () => {
    const user = userEvent.setup();
    const onPageChange = vi.fn();
    render(<Pagination page={1} pageSize={20} total={50} onPageChange={onPageChange} />);
    await user.click(screen.getByLabelText('Next page'));
    expect(onPageChange).toHaveBeenCalledWith(2);
  });

  it('calls onPageChange when clicking previous', async () => {
    const user = userEvent.setup();
    const onPageChange = vi.fn();
    render(<Pagination page={2} pageSize={20} total={50} onPageChange={onPageChange} />);
    await user.click(screen.getByLabelText('Previous page'));
    expect(onPageChange).toHaveBeenCalledWith(1);
  });

  it('handles edge case of pageSize=0 (treated as pageSize=1, no crash)', () => {
    const { container } = render(
      <Pagination page={1} pageSize={0} total={10} onPageChange={vi.fn()} />
    );
    // With pageSize=0 → Math.max(1,0)=1, so 10 pages exist and renders pagination
    expect(screen.getByText(/Page 1 of/)).toBeInTheDocument();
  });
});
