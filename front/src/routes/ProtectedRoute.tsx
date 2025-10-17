import { Navigate } from 'react-router-dom';
import { useAuth } from '@/hooks/useAuth';
import { ROUTE_PATHS, getDashboardPath } from './config';
import type { Role } from '@/types';

interface ProtectedRouteProps {
  children: React.ReactNode;
  allowedRoles?: Role[];
}

export const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ children, allowedRoles }) => {
  const { user, loading } = useAuth();

  // Show loading state while checking authentication
  if (loading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="h-12 w-12 animate-spin rounded-full border-b-2 border-t-2 border-primary"></div>
      </div>
    );
  }

  // Redirect to login if not authenticated
  if (!user) {
    return <Navigate to={ROUTE_PATHS.LOGIN} replace />;
  }

  // Check role-based access if roles are specified
  if (allowedRoles && !allowedRoles.includes(user.role)) {
    // Redirect to appropriate dashboard based on role
    return <Navigate to={getDashboardPath(user.role)} replace />;
  }

  return <>{children}</>;
};
