import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { RouterProvider, createMemoryRouter } from 'react-router-dom';
import { router } from '../routes';

describe('Routing Integration Tests', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it('should redirect unauthenticated users from dashboard to login', async () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/'],
    });

    render(<RouterProvider router={testRouter} />);

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: /login/i })).toBeInTheDocument();
    });
  });

  it('should render login page', () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/login'],
    });

    render(<RouterProvider router={testRouter} />);

    expect(screen.getByRole('heading', { name: /login/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
  });

  it('should render register page', () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/register'],
    });

    render(<RouterProvider router={testRouter} />);

    expect(screen.getByRole('heading', { name: /register/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/first name/i)).toBeInTheDocument();
  });

  it('should render password reset request page', () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/password-reset-request'],
    });

    render(<RouterProvider router={testRouter} />);

    expect(screen.getByRole('heading', { name: /reset password/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
  });

  it('should render unauthorized page', () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/unauthorized'],
    });

    render(<RouterProvider router={testRouter} />);

    expect(screen.getByRole('heading', { name: /access denied/i })).toBeInTheDocument();
  });

  it('should redirect unknown routes to dashboard', async () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/unknown'],
    });

    render(<RouterProvider router={testRouter} />);

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: /login/i })).toBeInTheDocument();
    });
  });
});
