/**
 * useRequireAuth Hook
 *
 * Custom hook for protecting routes that require authentication.
 * Redirects to login page if user is not authenticated.
 */

import { useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { useAuth } from './useAuth';
import type { UserRole } from '../types/auth';

/**
 * Options for useRequireAuth hook
 */
export interface UseRequireAuthOptions {
  /**
   * Redirect path when user is not authenticated
   * @default '/login'
   */
  redirectTo?: string;

  /**
   * Required user role(s) for accessing the route
   * If user doesn't have required role, redirects to unauthorized page
   */
  requiredRole?: UserRole | UserRole[];

  /**
   * Path to redirect when user doesn't have required role
   * @default '/unauthorized'
   */
  unauthorizedRedirect?: string;
}

/**
 * Hook for requiring authentication on a route
 *
 * Checks if user is authenticated and optionally validates user role.
 * Redirects to login if not authenticated or to unauthorized if lacking required role.
 *
 * @param options - Configuration options
 * @returns Authentication state
 *
 * @example
 * ```tsx
 * function ProtectedPage() {
 *   const { user, isLoading } = useRequireAuth();
 *
 *   if (isLoading) return <LoadingSpinner />;
 *
 *   return <div>Welcome, {user?.first_name}</div>;
 * }
 * ```
 *
 * @example
 * ```tsx
 * function AdminPage() {
 *   const { user } = useRequireAuth({
 *     requiredRole: UserRole.Admin
 *   });
 *
 *   return <AdminDashboard user={user} />;
 * }
 * ```
 */
export const useRequireAuth = (options: UseRequireAuthOptions = {}) => {
  const {
    redirectTo = '/login',
    requiredRole,
    unauthorizedRedirect = '/unauthorized',
  } = options;

  const { user, isAuthenticated, isLoading } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    // Skip checks while loading
    if (isLoading) return;

    // Redirect to login if not authenticated
    if (!isAuthenticated || !user) {
      navigate(redirectTo, {
        replace: true,
        state: { from: location.pathname },
      });
      return;
    }

    // Check role requirements if specified
    if (requiredRole) {
      const roles = Array.isArray(requiredRole) ? requiredRole : [requiredRole];
      const hasRequiredRole = roles.includes(user.role);

      if (!hasRequiredRole) {
        navigate(unauthorizedRedirect, {
          replace: true,
          state: { from: location.pathname, requiredRole: roles },
        });
      }
    }
  }, [
    isAuthenticated,
    isLoading,
    user,
    navigate,
    redirectTo,
    requiredRole,
    unauthorizedRedirect,
    location.pathname,
  ]);

  return {
    user,
    isAuthenticated,
    isLoading,
  };
};

/**
 * Hook for requiring specific role(s) on a route
 *
 * Simplified version of useRequireAuth that only checks roles.
 * Assumes user is already authenticated.
 *
 * @param requiredRole - Required user role(s)
 * @param unauthorizedRedirect - Path to redirect when user lacks required role
 * @returns Whether user has required role
 *
 * @example
 * ```tsx
 * function ManagerPage() {
 *   const hasAccess = useRequireRole([UserRole.Admin, UserRole.Manager]);
 *
 *   if (!hasAccess) return null;
 *
 *   return <ManagerDashboard />;
 * }
 * ```
 */
export const useRequireRole = (
  requiredRole: UserRole | UserRole[],
  unauthorizedRedirect = '/unauthorized'
): boolean => {
  const { user } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    if (!user) return;

    const roles = Array.isArray(requiredRole) ? requiredRole : [requiredRole];
    const hasRequiredRole = roles.includes(user.role);

    if (!hasRequiredRole) {
      navigate(unauthorizedRedirect, {
        replace: true,
        state: { from: location.pathname, requiredRole: roles },
      });
    }
  }, [user, requiredRole, navigate, unauthorizedRedirect, location.pathname]);

  if (!user) return false;

  const roles = Array.isArray(requiredRole) ? requiredRole : [requiredRole];
  return roles.includes(user.role);
};

/**
 * Hook for checking if current user has specific role(s)
 * Does not redirect, only returns boolean
 *
 * @param role - Role(s) to check
 * @returns Whether user has the specified role(s)
 *
 * @example
 * ```tsx
 * function Dashboard() {
 *   const isAdmin = useHasRole(UserRole.Admin);
 *   const isManager = useHasRole([UserRole.Admin, UserRole.Manager]);
 *
 *   return (
 *     <div>
 *       {isAdmin && <AdminPanel />}
 *       {isManager && <ManagerTools />}
 *       <UserContent />
 *     </div>
 *   );
 * }
 * ```
 */
export const useHasRole = (role: UserRole | UserRole[]): boolean => {
  const { user } = useAuth();

  if (!user) return false;

  const roles = Array.isArray(role) ? role : [role];
  return roles.includes(user.role);
};
