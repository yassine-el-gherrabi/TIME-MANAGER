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

  it('should render password reset request page', async () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/password-reset-request'],
    });

    render(<RouterProvider router={testRouter} />);

    // Wait for lazy-loaded component to render
    const emailInput = await screen.findByLabelText(/email/i);
    expect(emailInput).toBeInTheDocument();
  });

  it('should render unauthorized page', async () => {
    const testRouter = createMemoryRouter(router.routes, {
      initialEntries: ['/unauthorized'],
    });

    render(<RouterProvider router={testRouter} />);

    // Wait for lazy-loaded component to render
    const heading = await screen.findByRole('heading', { name: /access denied/i });
    expect(heading).toBeInTheDocument();
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
