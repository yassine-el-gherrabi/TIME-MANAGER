import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { RouterProvider, createMemoryRouter } from 'react-router-dom';
import { router } from '../routes';
import * as authApi from '../api/auth';
import { UserRole } from '../types/auth';

vi.mock('../api/auth');

describe('Auth Integration Tests', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Clear any stored auth state
    localStorage.clear();
  });

  it('should complete registration and login flow', async () => {
    const user = userEvent.setup();

    // Mock successful registration
    vi.mocked(authApi.register).mockResolvedValueOnce({
      user: {
        id: '123',
        email: 'test@example.com',
        first_name: 'Test',
        last_name: 'User',
        role: UserRole.Employee,
        organization_id: '456',
        created_at: '2024-01-01',
      },
      tokens: {
        access_token: 'test-access-token',
        refresh_token: 'test-refresh-token',
      },
    });

    // Start at register page
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/register'],
    });

    render(<RouterProvider router={testRouter} />);

    // Fill registration form
    await user.type(screen.getByLabelText(/first name/i), 'Test');
    await user.type(screen.getByLabelText(/last name/i), 'User');
    await user.type(screen.getByLabelText(/email/i), 'test@example.com');
    await user.type(screen.getByLabelText(/^password$/i), 'password123');

    // Submit form
    await user.click(screen.getByRole('button', { name: /register/i }));

    // Should navigate to dashboard after successful registration
    await waitFor(() => {
      expect(screen.getByText(/welcome to time manager/i)).toBeInTheDocument();
    });

    expect(authApi.register).toHaveBeenCalledWith(
      expect.objectContaining({
        first_name: 'Test',
        last_name: 'User',
        email: 'test@example.com',
        password: 'password123',
      })
    );
  });


  it('should complete login flow', async () => {
    const user = userEvent.setup();

    // Mock successful login
    vi.mocked(authApi.login).mockResolvedValueOnce({
      user: {
        id: '123',
        email: 'test@example.com',
        first_name: 'Test',
        last_name: 'User',
        role: UserRole.Employee,
        organization_id: '456',
        created_at: '2024-01-01',
      },
      tokens: {
        access_token: 'test-access-token',
        refresh_token: 'test-refresh-token',
      },
    });

    // Start at login page
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/login'],
    });

    render(<RouterProvider router={testRouter} />);

    // Fill login form
    await user.type(screen.getByLabelText(/email/i), 'test@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');

    // Submit form
    await user.click(screen.getByRole('button', { name: /^login$/i }));

    // Should navigate to dashboard after successful login
    await waitFor(() => {
      expect(screen.getByText(/welcome to time manager/i)).toBeInTheDocument();
    });

    expect(authApi.login).toHaveBeenCalledWith({
      email: 'test@example.com',
      password: 'password123',
    });
  });


  it('should handle password reset request', async () => {
    const user = userEvent.setup();

    vi.mocked(authApi.requestPasswordReset).mockResolvedValueOnce({ message: 'Reset email sent' });

    // Start at password reset request page
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/password-reset-request'],
    });

    render(<RouterProvider router={testRouter} />);

    // Fill and submit email
    await user.type(screen.getByLabelText(/email/i), 'test@example.com');
    await user.click(screen.getByRole('button', { name: /send reset link/i }));

    // Should show success message
    await waitFor(() => {
      expect(screen.getByText(/check your email/i)).toBeInTheDocument();
    });

    expect(authApi.requestPasswordReset).toHaveBeenCalledWith({
      email: 'test@example.com',
    });
  });
});
