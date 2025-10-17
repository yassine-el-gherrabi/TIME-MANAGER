import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import App from './App';

// Mock dependencies
vi.mock('@/context/AuthContext', () => ({
  AuthProvider: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="auth-provider">{children}</div>
  ),
}));

vi.mock('@/routes', () => ({
  AppRoutes: () => <div data-testid="app-routes">Routes</div>,
}));

vi.mock('@/components/ui/toaster', () => ({
  Toaster: () => <div data-testid="toaster">Toaster</div>,
}));

describe('App', () => {
  it('renders without crashing', () => {
    render(<App />);
    expect(screen.getByTestId('auth-provider')).toBeInTheDocument();
  });

  it('renders AuthProvider wrapper', () => {
    render(<App />);
    expect(screen.getByTestId('auth-provider')).toBeInTheDocument();
  });

  it('renders AppRoutes component', () => {
    render(<App />);
    expect(screen.getByTestId('app-routes')).toBeInTheDocument();
    expect(screen.getByText('Routes')).toBeInTheDocument();
  });

  it('renders Toaster component', () => {
    render(<App />);
    expect(screen.getByTestId('toaster')).toBeInTheDocument();
    expect(screen.getByText('Toaster')).toBeInTheDocument();
  });

  it('renders all components in correct structure', () => {
    const { container } = render(<App />);
    const authProvider = screen.getByTestId('auth-provider');
    const appRoutes = screen.getByTestId('app-routes');
    const toaster = screen.getByTestId('toaster');

    // AuthProvider should contain both AppRoutes and Toaster
    expect(authProvider).toContainElement(appRoutes);
    expect(authProvider).toContainElement(toaster);
  });
});
