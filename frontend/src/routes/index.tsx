import { createBrowserRouter, Navigate } from 'react-router-dom';
import { ProtectedRoute } from '../components/auth';
import { DashboardPage } from '../pages/DashboardPage';
import { LoginPage } from '../pages/LoginPage';
import { RegisterPage } from '../pages/RegisterPage';
import { PasswordResetRequestPage } from '../pages/PasswordResetRequestPage';
import { PasswordResetPage } from '../pages/PasswordResetPage';
import { UnauthorizedPage } from '../pages/UnauthorizedPage';

export const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <ProtectedRoute>
        <DashboardPage />
      </ProtectedRoute>
    ),
  },
  {
    path: '/login',
    element: <LoginPage />,
  },
  {
    path: '/register',
    element: <RegisterPage />,
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
    path: '/unauthorized',
    element: <UnauthorizedPage />,
  },
  {
    path: '*',
    element: <Navigate to="/" replace />,
  },
]);
