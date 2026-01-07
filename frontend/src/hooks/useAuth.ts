/**
 * useAuth Hook
 *
 * Custom React hook providing easy access to authentication state and actions.
 * Wraps the Zustand auth store for convenient component usage.
 */

import { useAuthStore } from '../stores/authStore';
import type {
  User,
  RegisterRequest,
  LoginRequest,
} from '../types/auth';

/**
 * Authentication hook return type
 */
export interface UseAuthReturn {
  // State
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;

  // Actions
  register: (data: RegisterRequest) => Promise<void>;
  login: (data: LoginRequest) => Promise<void>;
  logout: () => Promise<void>;
  logoutAll: () => Promise<void>;
  refreshUser: () => Promise<void>;
}

/**
 * useAuth hook for accessing authentication state and actions
 *
 * @example
 * ```tsx
 * function MyComponent() {
 *   const { user, isAuthenticated, login, logout } = useAuth();
 *
 *   if (!isAuthenticated) {
 *     return <LoginForm onSubmit={login} />;
 *   }
 *
 *   return (
 *     <div>
 *       <h1>Welcome, {user?.first_name}</h1>
 *       <button onClick={logout}>Logout</button>
 *     </div>
 *   );
 * }
 * ```
 */
export const useAuth = (): UseAuthReturn => {
  const {
    user,
    isAuthenticated,
    isLoading,
    register,
    login,
    logout,
    logoutAll,
    refreshUser,
  } = useAuthStore();

  return {
    // State
    user,
    isAuthenticated,
    isLoading,

    // Actions
    register,
    login,
    logout,
    logoutAll,
    refreshUser,
  };
};

/**
 * Hook for getting only the current user (without actions)
 * Useful for components that only need to display user information
 *
 * @example
 * ```tsx
 * function UserProfile() {
 *   const user = useCurrentUser();
 *
 *   if (!user) return null;
 *
 *   return <div>{user.email}</div>;
 * }
 * ```
 */
export const useCurrentUser = (): User | null => {
  return useAuthStore((state) => state.user);
};

/**
 * Hook for getting only the authentication status (without user data or actions)
 * Useful for conditional rendering based on auth state
 *
 * @example
 * ```tsx
 * function ConditionalContent() {
 *   const isAuthenticated = useIsAuthenticated();
 *
 *   return isAuthenticated ? <Dashboard /> : <LandingPage />;
 * }
 * ```
 */
export const useIsAuthenticated = (): boolean => {
  return useAuthStore((state) => state.isAuthenticated);
};

/**
 * Hook for getting only the loading state
 * Useful for showing loading indicators during auth operations
 *
 * @example
 * ```tsx
 * function LoginButton() {
 *   const isLoading = useAuthLoading();
 *
 *   return (
 *     <button disabled={isLoading}>
 *       {isLoading ? 'Loading...' : 'Login'}
 *     </button>
 *   );
 * }
 * ```
 */
export const useAuthLoading = (): boolean => {
  return useAuthStore((state) => state.isLoading);
};
