import { createBrowserRouter, Navigate } from 'react-router-dom';
import { ProtectedRoute } from '../components/auth';
import { MainLayout } from '../components/layout';
import { DashboardPage } from '../pages/DashboardPage';
import { ClockPage } from '../pages/ClockPage';
import { PendingApprovalsPage } from '../pages/PendingApprovalsPage';
import { LoginPage } from '../pages/LoginPage';
import { PasswordResetRequestPage } from '../pages/PasswordResetRequestPage';
import { PasswordResetPage } from '../pages/PasswordResetPage';
import { AcceptInvitePage } from '../pages/AcceptInvitePage';
import { UnauthorizedPage } from '../pages/UnauthorizedPage';
import { UsersPage, TeamsPage, SchedulesPage } from '../pages/admin';
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
    path: '/clock',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <ClockPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/clock/pending',
    element: (
      <ProtectedRoute requiredRole={UserRole.Manager}>
        <MainLayout>
          <PendingApprovalsPage />
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
    path: '/admin/teams',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <TeamsPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/schedules',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <SchedulesPage />
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
