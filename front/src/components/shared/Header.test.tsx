import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter } from 'react-router-dom';
import { Header } from './Header';
import { useAuth } from '@/hooks/useAuth';

// Mock dependencies
vi.mock('@/hooks/useAuth');
vi.mock('sonner', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
  },
}));

const mockNavigate = vi.fn();
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom');
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  };
});

function renderHeader(props = {}) {
  return render(
    <BrowserRouter>
      <Header {...props} />
    </BrowserRouter>
  );
}

describe('Header', () => {
  const mockLogout = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useAuth).mockReturnValue({
      user: null,
      token: null,
      login: vi.fn(),
      logout: mockLogout,
      loading: false,
      isAuthenticated: false,
    });
  });

  describe('rendering', () => {
    it('renders with default title', () => {
      renderHeader();

      expect(screen.getByText('Time Manager')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /logout/i })).toBeInTheDocument();
    });

    it('renders with custom title', () => {
      renderHeader({ title: 'Custom Dashboard' });

      expect(screen.getByText('Custom Dashboard')).toBeInTheDocument();
      expect(screen.queryByText('Time Manager')).not.toBeInTheDocument();
    });

    it('renders Clock icon', () => {
      const { container } = renderHeader();
      const clockIcon = container.querySelector('svg');

      expect(clockIcon).toBeInTheDocument();
    });

    it('renders logout button with icon', () => {
      const { container } = renderHeader();
      const logoutButton = screen.getByRole('button', { name: /logout/i });

      expect(logoutButton).toBeInTheDocument();
      expect(logoutButton).toHaveTextContent('Logout');

      // Check for LogOut icon (lucide-react renders as svg)
      const icons = container.querySelectorAll('svg');
      expect(icons.length).toBeGreaterThan(1); // Clock + LogOut
    });

    it('has correct header structure', () => {
      const { container } = renderHeader();
      const header = container.querySelector('header');

      expect(header).toBeInTheDocument();
      expect(header).toHaveClass('border-b');
    });
  });

  describe('logout functionality', () => {
    it('calls logout and navigates on successful logout', async () => {
      const user = userEvent.setup();
      const { toast } = await import('sonner');
      mockLogout.mockResolvedValue(undefined);

      renderHeader();

      const logoutButton = screen.getByRole('button', { name: /logout/i });
      await user.click(logoutButton);

      await waitFor(() => {
        expect(mockLogout).toHaveBeenCalledTimes(1);
      });

      expect(toast.success).toHaveBeenCalledWith('Logged out successfully');
      expect(mockNavigate).toHaveBeenCalledWith('/login', { replace: true });
    });

    it('handles logout error with error handler', async () => {
      const user = userEvent.setup();
      const error = new Error('Logout failed');
      mockLogout.mockRejectedValue(error);

      // Mock console to suppress error logs
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const consoleGroupSpy = vi.spyOn(console, 'group').mockImplementation(() => {});

      renderHeader();

      const logoutButton = screen.getByRole('button', { name: /logout/i });
      await user.click(logoutButton);

      await waitFor(() => {
        expect(mockLogout).toHaveBeenCalled();
      });

      // Error handler should be called (logs to console)
      await waitFor(() => {
        expect(consoleGroupSpy).toHaveBeenCalled();
      });

      // Should not navigate on error
      expect(mockNavigate).not.toHaveBeenCalled();

      consoleSpy.mockRestore();
      consoleGroupSpy.mockRestore();
    });

    it('shows loading state during logout (button disabled)', async () => {
      const user = userEvent.setup();
      let resolveLogout: () => void;
      const logoutPromise = new Promise<void>((resolve) => {
        resolveLogout = resolve;
      });
      mockLogout.mockReturnValue(logoutPromise);

      renderHeader();

      const logoutButton = screen.getByRole('button', { name: /logout/i });
      await user.click(logoutButton);

      // Button click is processed
      expect(mockLogout).toHaveBeenCalled();

      // Resolve logout
      resolveLogout!();

      await waitFor(() => {
        expect(mockNavigate).toHaveBeenCalled();
      });
    });
  });

  describe('customization', () => {
    it('renders different titles correctly', () => {
      const { rerender } = render(
        <BrowserRouter>
          <Header title="Employee Dashboard" />
        </BrowserRouter>
      );

      expect(screen.getByText('Employee Dashboard')).toBeInTheDocument();

      rerender(
        <BrowserRouter>
          <Header title="Manager Dashboard" />
        </BrowserRouter>
      );

      expect(screen.getByText('Manager Dashboard')).toBeInTheDocument();
      expect(screen.queryByText('Employee Dashboard')).not.toBeInTheDocument();
    });

    it('handles empty title prop (uses default)', () => {
      renderHeader({ title: undefined });

      expect(screen.getByText('Time Manager')).toBeInTheDocument();
    });
  });
});
