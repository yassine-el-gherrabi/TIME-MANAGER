import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { RouterProvider, createMemoryRouter } from 'react-router-dom';
import { router } from '../routes';
import * as authApi from '../api/auth';
import * as clocksApi from '../api/clocks';
import * as kpisApi from '../api/kpis';
import { UserRole } from '../types/auth';

vi.mock('../api/auth');
vi.mock('../api/clocks');
vi.mock('../api/kpis');

describe('Auth Integration Tests', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Clear any stored auth state
    localStorage.clear();

    // Mock dashboard API calls to prevent unhandled rejections
    vi.mocked(clocksApi.getStatus).mockResolvedValue({
      is_clocked_in: false,
      current_entry: null,
      elapsed_minutes: null,
    });
    vi.mocked(kpisApi.getMyKpis).mockResolvedValue({
      user_id: 'test-user',
      user_name: 'Test User',
      total_hours_worked: 0,
      theoretical_hours: 40,
      hours_variance: 0,
      punctuality_rate: 100,
      days_worked: 0,
      days_late: 0,
      average_daily_hours: 0,
    });
    vi.mocked(kpisApi.getCharts).mockResolvedValue({
      data: [],
      granularity: 'day',
    });
  });

  it('should complete login flow', async () => {
    const user = userEvent.setup();

    // Mock successful login (access_token only - refresh token is HttpOnly cookie)
    vi.mocked(authApi.login).mockResolvedValueOnce({
      access_token: 'test-access-token',
    });

    // Mock /me endpoint call (user fetched after login)
    vi.mocked(authApi.me).mockResolvedValueOnce({
      id: '123',
      email: 'test@example.com',
      first_name: 'Test',
      last_name: 'User',
      role: UserRole.Employee,
      organization_id: 'org-1', organization_name: 'Test Org',
      created_at: '2024-01-01',
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
      expect(screen.getByText(/welcome, test/i)).toBeInTheDocument();
    });

    expect(authApi.login).toHaveBeenCalledWith({
      email: 'test@example.com',
      password: 'password123',
    });
    expect(authApi.me).toHaveBeenCalled();
  });


  it('should handle password reset request', async () => {
    const user = userEvent.setup();

    vi.mocked(authApi.requestPasswordReset).mockResolvedValueOnce({ message: 'Reset email sent' });

    // Start at password reset request page
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/password-reset-request'],
    });

    render(<RouterProvider router={testRouter} />);

    // Wait for lazy-loaded page and fill form
    const emailInput = await screen.findByLabelText(/email/i);
    await user.type(emailInput, 'test@example.com');
    const submitButton = await screen.findByRole('button', { name: /send reset link/i });
    await user.click(submitButton);

    // Should show success message
    await waitFor(() => {
      expect(screen.getByText(/check your email/i)).toBeInTheDocument();
    });

    expect(authApi.requestPasswordReset).toHaveBeenCalledWith({
      email: 'test@example.com',
    });
  });
});
