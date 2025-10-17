import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import { ManagerDashboardPage } from './DashboardPage';
import { useAuth } from '@/hooks/useAuth';
import type { User } from '@/types';

// Mock dependencies
vi.mock('@/hooks/useAuth');
vi.mock('@/components/shared/Header', () => ({
  Header: ({ title }: { title?: string }) => (
    <header data-testid="header">
      <h1>{title}</h1>
    </header>
  ),
}));

const mockManager: User = {
  id: 2,
  email: 'jane.smith@example.com',
  firstName: 'Jane',
  lastName: 'Smith',
  role: 'manager',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
};

function renderDashboard() {
  return render(
    <BrowserRouter>
      <ManagerDashboardPage />
    </BrowserRouter>
  );
}

describe('ManagerDashboardPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useAuth).mockReturnValue({
      user: mockManager,
      token: 'test-token',
      login: vi.fn(),
      logout: vi.fn(),
      loading: false,
      isAuthenticated: true,
    });
  });

  describe('rendering', () => {
    it('renders without crashing', () => {
      renderDashboard();

      expect(screen.getByTestId('header')).toBeInTheDocument();
    });

    it('renders Header with manager title', () => {
      renderDashboard();

      expect(screen.getByText('Time Manager - Manager')).toBeInTheDocument();
    });

    it('renders welcome message with user name', () => {
      renderDashboard();

      expect(screen.getByText(/Welcome, Jane Smith!/i)).toBeInTheDocument();
    });

    it('renders user email', () => {
      renderDashboard();

      expect(screen.getByText('Email:')).toBeInTheDocument();
      expect(screen.getByText('jane.smith@example.com')).toBeInTheDocument();
    });

    it('renders user role', () => {
      renderDashboard();

      expect(screen.getByText('Role:')).toBeInTheDocument();
      expect(screen.getByText('manager')).toBeInTheDocument();
    });

    it('renders placeholder content message', () => {
      renderDashboard();

      expect(
        screen.getByText(/This is your manager dashboard. Features will be added in the next iterations./i)
      ).toBeInTheDocument();
    });

    it('has correct layout structure', () => {
      const { container } = renderDashboard();

      // Check for main layout elements
      const mainElement = container.querySelector('main');
      expect(mainElement).toBeInTheDocument();
      expect(mainElement).toHaveClass('container', 'mx-auto', 'px-4', 'py-8');
    });
  });

  describe('user data handling', () => {
    it('handles user with different name', () => {
      vi.mocked(useAuth).mockReturnValue({
        user: {
          ...mockManager,
          firstName: 'Bob',
          lastName: 'Johnson',
        },
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderDashboard();

      expect(screen.getByText(/Welcome, Bob Johnson!/i)).toBeInTheDocument();
    });

    it('handles user with different email', () => {
      vi.mocked(useAuth).mockReturnValue({
        user: {
          ...mockManager,
          email: 'manager@company.com',
        },
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderDashboard();

      expect(screen.getByText('manager@company.com')).toBeInTheDocument();
    });

    it('handles missing user gracefully (optional chaining)', () => {
      vi.mocked(useAuth).mockReturnValue({
        user: null,
        token: null,
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: false,
      });

      renderDashboard();

      // Should not crash, optional chaining handles null user
      expect(screen.getByTestId('header')).toBeInTheDocument();
    });
  });
});
