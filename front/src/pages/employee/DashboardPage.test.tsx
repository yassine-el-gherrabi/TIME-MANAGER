import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import { EmployeeDashboardPage } from './DashboardPage';
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

const mockEmployee: User = {
  id: 1,
  email: 'john.doe@example.com',
  firstName: 'John',
  lastName: 'Doe',
  role: 'employee',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
};

function renderDashboard() {
  return render(
    <BrowserRouter>
      <EmployeeDashboardPage />
    </BrowserRouter>
  );
}

describe('EmployeeDashboardPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useAuth).mockReturnValue({
      user: mockEmployee,
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

    it('renders Header with employee title', () => {
      renderDashboard();

      expect(screen.getByText('Time Manager - Employee')).toBeInTheDocument();
    });

    it('renders welcome message with user name', () => {
      renderDashboard();

      expect(screen.getByText(/Welcome, John Doe!/i)).toBeInTheDocument();
    });

    it('renders user email', () => {
      renderDashboard();

      expect(screen.getByText('Email:')).toBeInTheDocument();
      expect(screen.getByText('john.doe@example.com')).toBeInTheDocument();
    });

    it('renders user role', () => {
      renderDashboard();

      expect(screen.getByText('Role:')).toBeInTheDocument();
      expect(screen.getByText('employee')).toBeInTheDocument();
    });

    it('renders placeholder content message', () => {
      renderDashboard();

      expect(
        screen.getByText(/This is your employee dashboard. Features will be added in the next iterations./i)
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
          ...mockEmployee,
          firstName: 'Jane',
          lastName: 'Smith',
        },
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderDashboard();

      expect(screen.getByText(/Welcome, Jane Smith!/i)).toBeInTheDocument();
    });

    it('handles user with different email', () => {
      vi.mocked(useAuth).mockReturnValue({
        user: {
          ...mockEmployee,
          email: 'different@example.com',
        },
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderDashboard();

      expect(screen.getByText('different@example.com')).toBeInTheDocument();
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
