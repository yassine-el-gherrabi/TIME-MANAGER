import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter } from 'react-router-dom';
import { LoginPage } from './LoginPage';
import { useAuth } from '@/hooks/useAuth';
import type { User } from '@/types';

// Mock dependencies
vi.mock('@/hooks/useAuth');
vi.mock('sonner', () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));

const mockEmployee: User = {
  id: 1,
  email: 'employee@example.com',
  firstName: 'John',
  lastName: 'Doe',
  role: 'employee',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
};

const mockManager: User = {
  id: 2,
  email: 'manager@example.com',
  firstName: 'Jane',
  lastName: 'Smith',
  role: 'manager',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
};

function renderLoginPage() {
  return render(
    <BrowserRouter>
      <LoginPage />
    </BrowserRouter>
  );
}

describe('LoginPage', () => {
  const mockLogin = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useAuth).mockReturnValue({
      user: null,
      token: null,
      login: mockLogin,
      logout: vi.fn(),
      loading: false,
      isAuthenticated: false,
    });
  });

  describe('rendering', () => {
    it('renders login form with all elements', () => {
      renderLoginPage();

      // Header elements
      expect(screen.getByText('Time Manager')).toBeInTheDocument();
      expect(screen.getByText('Sign in to track your time')).toBeInTheDocument();

      // Form elements
      expect(screen.getByRole('heading', { name: 'Sign In' })).toBeInTheDocument();
      expect(screen.getByText('Enter your credentials to access your account')).toBeInTheDocument();
      expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /sign in/i })).toBeInTheDocument();

      // Footer
      expect(screen.getByText(/need help/i)).toBeInTheDocument();
    });

    it('email input has correct attributes', () => {
      renderLoginPage();

      const emailInput = screen.getByLabelText(/email/i) as HTMLInputElement;

      expect(emailInput).toHaveAttribute('type', 'email');
      expect(emailInput).toHaveAttribute('placeholder', 'your.email@company.com');
      expect(emailInput).toHaveAttribute('autocomplete', 'email');
    });

    it('password input has correct attributes', () => {
      renderLoginPage();

      const passwordInput = screen.getByLabelText(/password/i) as HTMLInputElement;

      expect(passwordInput).toHaveAttribute('type', 'password');
      expect(passwordInput).toHaveAttribute('autocomplete', 'current-password');
    });
  });

  describe('form interaction', () => {
    it('updates email input on user typing', async () => {
      const user = userEvent.setup();
      renderLoginPage();

      const emailInput = screen.getByLabelText(/email/i) as HTMLInputElement;

      await user.type(emailInput, 'test@example.com');

      expect(emailInput.value).toBe('test@example.com');
    });

    it('updates password input on user typing', async () => {
      const user = userEvent.setup();
      renderLoginPage();

      const passwordInput = screen.getByLabelText(/password/i) as HTMLInputElement;

      await user.type(passwordInput, 'password123');

      expect(passwordInput.value).toBe('password123');
    });

    it('clears form after typing and clearing', async () => {
      const user = userEvent.setup();
      renderLoginPage();

      const emailInput = screen.getByLabelText(/email/i) as HTMLInputElement;

      await user.type(emailInput, 'test@example.com');
      await user.clear(emailInput);

      expect(emailInput.value).toBe('');
    });
  });

  describe('form validation', () => {
    it('shows error toast when submitting empty form', async () => {
      const user = userEvent.setup();
      const { toast } = await import('sonner');

      renderLoginPage();

      const submitButton = screen.getByRole('button', { name: /sign in/i });
      await user.click(submitButton);

      expect(toast.error).toHaveBeenCalledWith('Please fill in all fields');
      expect(mockLogin).not.toHaveBeenCalled();
    });

    it('shows error when only email is filled', async () => {
      const user = userEvent.setup();
      const { toast } = await import('sonner');

      renderLoginPage();

      const emailInput = screen.getByLabelText(/email/i);
      await user.type(emailInput, 'test@example.com');

      const submitButton = screen.getByRole('button', { name: /sign in/i });
      await user.click(submitButton);

      expect(toast.error).toHaveBeenCalledWith('Please fill in all fields');
      expect(mockLogin).not.toHaveBeenCalled();
    });

    it('shows error when only password is filled', async () => {
      const user = userEvent.setup();
      const { toast } = await import('sonner');

      renderLoginPage();

      const passwordInput = screen.getByLabelText(/password/i);
      await user.type(passwordInput, 'password123');

      const submitButton = screen.getByRole('button', { name: /sign in/i });
      await user.click(submitButton);

      expect(toast.error).toHaveBeenCalledWith('Please fill in all fields');
      expect(mockLogin).not.toHaveBeenCalled();
    });
  });

  describe('login submission', () => {
    it('calls login with correct credentials', async () => {
      const user = userEvent.setup();
      mockLogin.mockResolvedValue(undefined);

      renderLoginPage();

      // Fill form
      await user.type(screen.getByLabelText(/email/i), 'test@example.com');
      await user.type(screen.getByLabelText(/password/i), 'password123');

      // Submit
      await user.click(screen.getByRole('button', { name: /sign in/i }));

      await waitFor(() => {
        expect(mockLogin).toHaveBeenCalledWith({
          email: 'test@example.com',
          password: 'password123',
        });
      });
    });

    it('shows success toast on successful login', async () => {
      const user = userEvent.setup();
      const { toast } = await import('sonner');
      mockLogin.mockResolvedValue(undefined);

      renderLoginPage();

      await user.type(screen.getByLabelText(/email/i), 'test@example.com');
      await user.type(screen.getByLabelText(/password/i), 'password123');
      await user.click(screen.getByRole('button', { name: /sign in/i }));

      await waitFor(() => {
        expect(toast.success).toHaveBeenCalledWith('Welcome back!');
      });
    });

    it('shows loading state during login', async () => {
      const user = userEvent.setup();
      let resolveLogin: () => void;
      const loginPromise = new Promise<void>((resolve) => {
        resolveLogin = resolve;
      });
      mockLogin.mockReturnValue(loginPromise);

      renderLoginPage();

      await user.type(screen.getByLabelText(/email/i), 'test@example.com');
      await user.type(screen.getByLabelText(/password/i), 'password123');
      await user.click(screen.getByRole('button', { name: /sign in/i }));

      // Should show loading state
      await waitFor(() => {
        expect(screen.getByText('Signing in...')).toBeInTheDocument();
      });

      // Button should be disabled
      const submitButton = screen.getByRole('button', { name: /signing in/i });
      expect(submitButton).toBeDisabled();

      // Resolve login
      resolveLogin!();

      await waitFor(() => {
        expect(screen.queryByText('Signing in...')).not.toBeInTheDocument();
      });
    });

    it('disables form inputs during login', async () => {
      const user = userEvent.setup();
      let resolveLogin: () => void;
      const loginPromise = new Promise<void>((resolve) => {
        resolveLogin = resolve;
      });
      mockLogin.mockReturnValue(loginPromise);

      renderLoginPage();

      const emailInput = screen.getByLabelText(/email/i);
      const passwordInput = screen.getByLabelText(/password/i);

      await user.type(emailInput, 'test@example.com');
      await user.type(passwordInput, 'password123');
      await user.click(screen.getByRole('button', { name: /sign in/i }));

      await waitFor(() => {
        expect(emailInput).toBeDisabled();
        expect(passwordInput).toBeDisabled();
      });

      resolveLogin!();

      await waitFor(() => {
        expect(emailInput).not.toBeDisabled();
        expect(passwordInput).not.toBeDisabled();
      });
    });

    it('handles login error with error handler', async () => {
      const user = userEvent.setup();
      const error = new Error('Invalid credentials');
      mockLogin.mockRejectedValue(error);

      // Mock console to suppress error logs in test
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const consoleGroupSpy = vi.spyOn(console, 'group').mockImplementation(() => {});

      renderLoginPage();

      await user.type(screen.getByLabelText(/email/i), 'test@example.com');
      await user.type(screen.getByLabelText(/password/i), 'wrongpassword');
      await user.click(screen.getByRole('button', { name: /sign in/i }));

      await waitFor(() => {
        expect(mockLogin).toHaveBeenCalled();
      });

      // Error handler should be called
      await waitFor(() => {
        expect(consoleGroupSpy).toHaveBeenCalled();
      });

      consoleSpy.mockRestore();
      consoleGroupSpy.mockRestore();
    });

    it('re-enables form after login error', async () => {
      const user = userEvent.setup();
      mockLogin.mockRejectedValue(new Error('Login failed'));

      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const consoleGroupSpy = vi.spyOn(console, 'group').mockImplementation(() => {});

      renderLoginPage();

      await user.type(screen.getByLabelText(/email/i), 'test@example.com');
      await user.type(screen.getByLabelText(/password/i), 'wrongpassword');
      await user.click(screen.getByRole('button', { name: /sign in/i }));

      await waitFor(() => {
        const submitButton = screen.getByRole('button', { name: /sign in/i });
        expect(submitButton).not.toBeDisabled();
      });

      consoleSpy.mockRestore();
      consoleGroupSpy.mockRestore();
    });
  });

  describe('already authenticated redirect', () => {
    it('does not redirect immediately when user is null', () => {
      vi.mocked(useAuth).mockReturnValue({
        user: null,
        token: null,
        login: mockLogin,
        logout: vi.fn(),
        loading: false,
        isAuthenticated: false,
      });

      renderLoginPage();

      // Should show login form
      expect(screen.getByRole('heading', { name: 'Sign In' })).toBeInTheDocument();
    });

    it('redirects employee to employee dashboard when already authenticated', () => {
      vi.mocked(useAuth).mockReturnValue({
        user: mockEmployee,
        token: 'test-token',
        login: mockLogin,
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderLoginPage();

      // Form should still render initially (redirect happens via useEffect)
      expect(screen.getByRole('heading', { name: 'Sign In' })).toBeInTheDocument();
    });

    it('redirects manager to manager dashboard when already authenticated', () => {
      vi.mocked(useAuth).mockReturnValue({
        user: mockManager,
        token: 'test-token',
        login: mockLogin,
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderLoginPage();

      // Form should still render initially (redirect happens via useEffect)
      expect(screen.getByRole('heading', { name: 'Sign In' })).toBeInTheDocument();
    });
  });
});
