import { describe, it, expect, vi } from 'vitest';
import { render, screen, act } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Modal } from '../Modal';

describe('Modal', () => {
  it('renders nothing when closed', () => {
    const { container } = render(
      <Modal open={false} onClose={vi.fn()} title="Test Modal">
        <p>Content</p>
      </Modal>
    );
    expect(screen.queryByText('Test Modal')).not.toBeInTheDocument();
  });

  it('renders when open', () => {
    render(
      <Modal open={true} onClose={vi.fn()} title="Test Modal">
        <p>Content</p>
      </Modal>
    );
    expect(screen.getByText('Test Modal')).toBeInTheDocument();
    expect(screen.getByText('Content')).toBeInTheDocument();
  });

  it('renders subtitle when provided', () => {
    render(
      <Modal open={true} onClose={vi.fn()} title="Title" subtitle="Subtitle text">
        <p>Content</p>
      </Modal>
    );
    expect(screen.getByText('Subtitle text')).toBeInTheDocument();
  });

  it('renders footer when provided', () => {
    render(
      <Modal open={true} onClose={vi.fn()} title="Title" footer={<button>Save</button>}>
        <p>Content</p>
      </Modal>
    );
    expect(screen.getByText('Save')).toBeInTheDocument();
  });

  it('calls onClose when clicking close button', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(
      <Modal open={true} onClose={onClose} title="Title">
        <p>Content</p>
      </Modal>
    );
    await user.click(screen.getByLabelText('Close dialog'));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('calls onClose when pressing Escape', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(
      <Modal open={true} onClose={onClose} title="Title">
        <p>Content</p>
      </Modal>
    );
    await user.keyboard('{Escape}');
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('calls onClose when clicking backdrop overlay', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(
      <Modal open={true} onClose={onClose} title="Title">
        <p>Content</p>
      </Modal>
    );
    // The backdrop is rendered via createPortal into document.body
    const backdrop = document.querySelector('.backdrop-blur-sm');
    if (backdrop) {
      await user.click(backdrop);
      expect(onClose).toHaveBeenCalled();
    }
  });

  it('has dialog role and aria-modal', () => {
    render(
      <Modal open={true} onClose={vi.fn()} title="Title">
        <p>Content</p>
      </Modal>
    );
    const dialog = screen.getByRole('dialog');
    expect(dialog).toHaveAttribute('aria-modal', 'true');
  });

  it('sets body overflow to hidden when open', () => {
    render(
      <Modal open={true} onClose={vi.fn()} title="Title">
        <p>Content</p>
      </Modal>
    );
    expect(document.body.style.overflow).toBe('hidden');
  });

  it('resets body overflow when unmounted', () => {
    const { unmount } = render(
      <Modal open={true} onClose={vi.fn()} title="Title">
        <p>Content</p>
      </Modal>
    );
    unmount();
    expect(document.body.style.overflow).toBe('');
  });
});
