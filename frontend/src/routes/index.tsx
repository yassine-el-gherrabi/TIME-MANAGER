import { createBrowserRouter, Navigate } from 'react-router-dom';
import { ProtectedRoute } from '../components/auth';
import { MainLayout } from '../components/layout';
import { DashboardPage } from '../pages/DashboardPage';
import { LoginPage } from '../pages/LoginPage';
import { PasswordResetRequestPage } from '../pages/PasswordResetRequestPage';
import { PasswordResetPage } from '../pages/PasswordResetPage';
import { AcceptInvitePage } from '../pages/AcceptInvitePage';
import { UnauthorizedPage } from '../pages/UnauthorizedPage';
import { UsersPage, CreateUserPage, EditUserPage } from '../pages/admin';
import { ChangePasswordPage, SessionsPage } from '../pages/settings';
import { UserRole } from '../types/auth';

export const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <DashboardPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/login',
    element: <LoginPage />,
  },
  {
    path: '/password-reset-request',
    element: <PasswordResetRequestPage />,
  },
  {
    path: '/password-reset',
    element: <PasswordResetPage />,
  },
  {
    path: '/accept-invite',
    element: <AcceptInvitePage />,
  },
  {
    path: '/admin/users',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <UsersPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/users/new',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <CreateUserPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/users/:id/edit',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <EditUserPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/settings/password',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <ChangePasswordPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/settings/sessions',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <SessionsPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/unauthorized',
    element: <UnauthorizedPage />,
  },
  {
    path: '*',
    element: <Navigate to="/" replace />,
  },
]);
