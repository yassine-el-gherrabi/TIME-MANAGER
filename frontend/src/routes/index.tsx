import { lazy, Suspense } from 'react';
import { createBrowserRouter, Navigate } from 'react-router-dom';
import { ProtectedRoute } from '../components/auth';
import { MainLayout } from '../components/layout';
import { UserRole } from '../types/auth';

// Eagerly loaded pages (critical path)
import { DashboardPage } from '../pages/DashboardPage';
import { ClockPage } from '../pages/ClockPage';
import { LoginPage } from '../pages/LoginPage';
import { AbsencesPage } from '../pages/AbsencesPage';
import { SetupWizardPage } from '../pages/SetupWizardPage';

// Lazy loaded pages (admin)
const UsersPage = lazy(() => import('../pages/admin/UsersPage').then(m => ({ default: m.UsersPage })));
const TeamsPage = lazy(() => import('../pages/admin/TeamsPage').then(m => ({ default: m.TeamsPage })));
const SchedulesPage = lazy(() => import('../pages/admin/SchedulesPage').then(m => ({ default: m.SchedulesPage })));
const AbsenceTypesPage = lazy(() => import('../pages/admin/AbsenceTypesPage').then(m => ({ default: m.AbsenceTypesPage })));
const ClosedDaysPage = lazy(() => import('../pages/admin/ClosedDaysPage').then(m => ({ default: m.ClosedDaysPage })));
const ClockRestrictionsPage = lazy(() => import('../pages/admin/clock-restrictions').then(m => ({ default: m.ClockRestrictionsPage })));
const BreakPoliciesPage = lazy(() => import('../pages/admin/break-policies').then(m => ({ default: m.BreakPoliciesPage })));
const AuditLogsPage = lazy(() => import('../pages/admin/AuditLogsPage').then(m => ({ default: m.AuditLogsPage })));
const OrganizationsPage = lazy(() => import('../pages/admin/OrganizationsPage').then(m => ({ default: m.OrganizationsPage })));

// Lazy loaded pages (settings)
const ChangePasswordPage = lazy(() => import('../pages/settings/ChangePasswordPage').then(m => ({ default: m.ChangePasswordPage })));
const SessionsPage = lazy(() => import('../pages/settings/SessionsPage').then(m => ({ default: m.SessionsPage })));
const ProfilePage = lazy(() => import('../pages/settings/ProfilePage').then(m => ({ default: m.ProfilePage })));

// Lazy loaded pages (secondary)
const PendingApprovalsPage = lazy(() => import('../pages/PendingApprovalsPage').then(m => ({ default: m.PendingApprovalsPage })));
const PendingAbsencesPage = lazy(() => import('../pages/PendingAbsencesPage').then(m => ({ default: m.PendingAbsencesPage })));
const TeamCalendarPage = lazy(() => import('../pages/TeamCalendarPage').then(m => ({ default: m.TeamCalendarPage })));
const PasswordResetRequestPage = lazy(() => import('../pages/PasswordResetRequestPage').then(m => ({ default: m.PasswordResetRequestPage })));
const PasswordResetPage = lazy(() => import('../pages/PasswordResetPage').then(m => ({ default: m.PasswordResetPage })));
const AcceptInvitePage = lazy(() => import('../pages/AcceptInvitePage').then(m => ({ default: m.AcceptInvitePage })));
const UnauthorizedPage = lazy(() => import('../pages/UnauthorizedPage').then(m => ({ default: m.UnauthorizedPage })));

// Loading fallback component
const LoadingFallback = () => (
  <div className="min-h-screen flex items-center justify-center bg-background">
    <div className="flex flex-col items-center gap-4">
      <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary" />
      <p className="text-muted-foreground">Loading...</p>
    </div>
  </div>
);

// Wrapper for lazy loaded components
const LazyPage = ({ children }: { children: React.ReactNode }) => (
  <Suspense fallback={<LoadingFallback />}>
    {children}
  </Suspense>
);

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
    path: '/profile',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <LazyPage>
            <ProfilePage />
          </LazyPage>
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
          <LazyPage>
            <PendingApprovalsPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/absences',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <AbsencesPage />
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/absences/pending',
    element: (
      <ProtectedRoute requiredRole={UserRole.Manager}>
        <MainLayout>
          <LazyPage>
            <PendingAbsencesPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/absences/calendar',
    element: (
      <ProtectedRoute requiredRole={UserRole.Manager}>
        <MainLayout>
          <LazyPage>
            <TeamCalendarPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/login',
    element: <LoginPage />,
  },
  {
    path: '/setup',
    element: <SetupWizardPage />,
  },
  {
    path: '/password-reset-request',
    element: (
      <LazyPage>
        <PasswordResetRequestPage />
      </LazyPage>
    ),
  },
  {
    path: '/password-reset',
    element: (
      <LazyPage>
        <PasswordResetPage />
      </LazyPage>
    ),
  },
  {
    path: '/accept-invite',
    element: (
      <LazyPage>
        <AcceptInvitePage />
      </LazyPage>
    ),
  },
  {
    path: '/admin/users',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <LazyPage>
            <UsersPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/teams',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <LazyPage>
            <TeamsPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/schedules',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <LazyPage>
            <SchedulesPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/absence-types',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <LazyPage>
            <AbsenceTypesPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/closed-days',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <LazyPage>
            <ClosedDaysPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/clock-restrictions',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <LazyPage>
            <ClockRestrictionsPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/break-policies',
    element: (
      <ProtectedRoute requiredRole={UserRole.Admin}>
        <MainLayout>
          <LazyPage>
            <BreakPoliciesPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/audit-logs',
    element: (
      <ProtectedRoute requiredRole={UserRole.SuperAdmin}>
        <MainLayout>
          <LazyPage>
            <AuditLogsPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/admin/organizations',
    element: (
      <ProtectedRoute requiredRole={UserRole.SuperAdmin}>
        <MainLayout>
          <LazyPage>
            <OrganizationsPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/settings/password',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <LazyPage>
            <ChangePasswordPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/settings/sessions',
    element: (
      <ProtectedRoute>
        <MainLayout>
          <LazyPage>
            <SessionsPage />
          </LazyPage>
        </MainLayout>
      </ProtectedRoute>
    ),
  },
  {
    path: '/unauthorized',
    element: (
      <LazyPage>
        <UnauthorizedPage />
      </LazyPage>
    ),
  },
  {
    path: '*',
    element: <Navigate to="/" replace />,
  },
]);
