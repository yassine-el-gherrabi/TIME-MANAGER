import React from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';
import { UserRole } from '../../types/auth';

/**
 * Role hierarchy for permission checking.
 * Higher number = more permissions.
 * SuperAdmin > Admin > Manager > Employee
 */
const ROLE_HIERARCHY: Record<UserRole, number> = {
  [UserRole.Employee]: 0,
  [UserRole.Manager]: 1,
  [UserRole.Admin]: 2,
  [UserRole.SuperAdmin]: 3,
};

export interface ProtectedRouteProps {
  children: React.ReactNode;
  requiredRole?: UserRole | UserRole[];
  redirectTo?: string;
}

export const ProtectedRoute: React.FC<ProtectedRouteProps> = ({
  children,
  requiredRole,
  redirectTo = '/login',
}) => {
  const { isAuthenticated, user, isLoading, needsSetup } = useAuth();
  const location = useLocation();

  // Show loading state while checking authentication
  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
          <p className="mt-4 text-sm text-muted-foreground">Loading...</p>
        </div>
      </div>
    );
  }

  // Redirect to setup wizard if system needs initial setup
  if (needsSetup) {
    return <Navigate to="/setup" replace />;
  }

  // Redirect to login if not authenticated
  if (!isAuthenticated || !user) {
    return <Navigate to={redirectTo} state={{ from: location.pathname }} replace />;
  }

  // Check role requirements if specified (using hierarchical comparison)
  if (requiredRole) {
    const roles = Array.isArray(requiredRole) ? requiredRole : [requiredRole];
    // User has access if their role level >= any of the required role levels
    const userRoleLevel = ROLE_HIERARCHY[user.role] ?? 0;
    const hasRequiredRole = roles.some(
      (role) => userRoleLevel >= (ROLE_HIERARCHY[role] ?? 0)
    );

    if (!hasRequiredRole) {
      return <Navigate to="/unauthorized" state={{ from: location.pathname }} replace />;
    }
  }

  return <>{children}</>;
};
