import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { ProtectedRoute } from './ProtectedRoute';
import { useAuth } from '@/hooks/useAuth';
import type { User } from '@/types';

// Mock useAuth hook
vi.mock('@/hooks/useAuth');

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

function renderProtectedRoute(
  allowedRoles?: ('employee' | 'manager')[],
  children: React.ReactNode = <div>Protected Content</div>
) {
  return render(
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<ProtectedRoute allowedRoles={allowedRoles}>{children}</ProtectedRoute>} />
        <Route path="/login" element={<div>Login Page</div>} />
        <Route path="/employee/dashboard" element={<div>Employee Dashboard</div>} />
        <Route path="/manager/dashboard" element={<div>Manager Dashboard</div>} />
      </Routes>
    </BrowserRouter>
  );
}

describe('ProtectedRoute', () => {
  beforeEach(() => {
    // Reset mock to default state
    (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
      user: null,
      token: null,
      login: vi.fn(),
      logout: vi.fn(),
      loading: false,
      isAuthenticated: false,
    });
  });

  describe('loading state', () => {
    it('shows loading spinner while authenticating', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: null,
        token: null,
        login: vi.fn(),
        logout: vi.fn(),
        loading: true,
        isAuthenticated: false,
      });

      renderProtectedRoute();

      // Should show loading spinner
      const spinner = document.querySelector('.animate-spin');
      expect(spinner).toBeInTheDocument();
      expect(screen.queryByText('Protected Content')).not.toBeInTheDocument();
    });
  });

  describe('authentication', () => {
    it('redirects to login if not authenticated', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: null,
        token: null,
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: false,
      });

      renderProtectedRoute();

      // Should redirect to login
      expect(screen.getByText('Login Page')).toBeInTheDocument();
      expect(screen.queryByText('Protected Content')).not.toBeInTheDocument();
    });

    it('renders children if authenticated and no role restriction', () => {
      const mockReturn = {
        user: mockEmployee,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      };

      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue(mockReturn);

      renderProtectedRoute(undefined);

      // Should render protected content
      expect(screen.getByText('Protected Content')).toBeInTheDocument();
    });
  });

  describe('role-based access', () => {
    it('allows employee to access employee-only route', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockEmployee,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderProtectedRoute(['employee']);

      expect(screen.getByText('Protected Content')).toBeInTheDocument();
    });

    it('allows manager to access manager-only route', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockManager,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderProtectedRoute(['manager']);

      expect(screen.getByText('Protected Content')).toBeInTheDocument();
    });

    it('redirects employee trying to access manager-only route to employee dashboard', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockEmployee,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderProtectedRoute(['manager']);

      // Should redirect to employee dashboard
      expect(screen.getByText('Employee Dashboard')).toBeInTheDocument();
      expect(screen.queryByText('Protected Content')).not.toBeInTheDocument();
    });

    it('redirects manager trying to access employee-only route to manager dashboard', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockManager,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderProtectedRoute(['employee']);

      // Should redirect to manager dashboard
      expect(screen.getByText('Manager Dashboard')).toBeInTheDocument();
      expect(screen.queryByText('Protected Content')).not.toBeInTheDocument();
    });

    it('allows access when user role is in allowed roles array', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockEmployee,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderProtectedRoute(['employee', 'manager']);

      expect(screen.getByText('Protected Content')).toBeInTheDocument();
    });

    it('allows both roles when both specified', () => {
      // Test employee access
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockEmployee,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      const { unmount } = renderProtectedRoute(['employee', 'manager']);
      expect(screen.getByText('Protected Content')).toBeInTheDocument();

      unmount();

      // Test manager access
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockManager,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderProtectedRoute(['employee', 'manager']);
      expect(screen.getByText('Protected Content')).toBeInTheDocument();
    });
  });

  describe('complex scenarios', () => {
    it('handles transition from loading to authenticated', () => {
      const { rerender } = render(
        <BrowserRouter>
          <Routes>
            <Route
              path="/"
              element={
                <ProtectedRoute>
                  <div>Protected Content</div>
                </ProtectedRoute>
              }
            />
          </Routes>
        </BrowserRouter>
      );

      // Initially loading
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: null,
        token: null,
        login: vi.fn(),
        logout: vi.fn(),
        loading: true,
        isAuthenticated: false,
      });

      rerender(
        <BrowserRouter>
          <Routes>
            <Route
              path="/"
              element={
                <ProtectedRoute>
                  <div>Protected Content</div>
                </ProtectedRoute>
              }
            />
          </Routes>
        </BrowserRouter>
      );

      // Then authenticated
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockEmployee,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      rerender(
        <BrowserRouter>
          <Routes>
            <Route
              path="/"
              element={
                <ProtectedRoute>
                  <div>Protected Content</div>
                </ProtectedRoute>
              }
            />
          </Routes>
        </BrowserRouter>
      );

      expect(screen.getByText('Protected Content')).toBeInTheDocument();
    });

    it('renders different children content', () => {
      (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
        user: mockEmployee,
        token: 'test-token',
        login: vi.fn(),
        logout: vi.fn(),
        loading: false,
        isAuthenticated: true,
      });

      renderProtectedRoute(
        undefined,
        <div>
          <h1>Dashboard</h1>
          <p>Welcome User</p>
        </div>
      );

      expect(screen.getByText('Dashboard')).toBeInTheDocument();
      expect(screen.getByText('Welcome User')).toBeInTheDocument();
    });
  });
});
