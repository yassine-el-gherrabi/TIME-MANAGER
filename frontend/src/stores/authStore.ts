/**
 * Authentication Store (Zustand)
 *
 * Global state management for authentication using Zustand.
 * Manages user session, tokens, and authentication state.
 */

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { authApi } from '../api/auth';
import { clearTokens, getRefreshToken } from '../api/client';
import { STORAGE_KEYS } from '../config/constants';
import type {
  User,
  LoginRequest,
  AcceptInviteRequest,
  AuthState,
} from '../types/auth';

/**
 * Authentication store state and actions
 */
interface AuthStore extends AuthState {
  // Actions
  login: (data: LoginRequest) => Promise<void>;
  acceptInvite: (data: AcceptInviteRequest) => Promise<void>;
  logout: () => Promise<void>;
  logoutAll: () => Promise<void>;
  refreshUser: () => Promise<void>;
  setUser: (user: User | null) => void;
  setLoading: (isLoading: boolean) => void;
  clearAuth: () => void;
}

/**
 * Zustand auth store
 *
 * User data kept in memory only (RGPD compliant - no PII in localStorage)
 * Tokens managed separately by TokenManager
 */
export const useAuthStore = create<AuthStore>()(
  persist(
    (set, get) => ({
      // Initial state
      user: null,
      accessToken: null,
      refreshToken: null,
      isAuthenticated: false,
      isLoading: false,

      /**
       * Login with email and password
       * Tokens are stored by authApi.login(), then user is fetched via /me
       */
      login: async (data: LoginRequest) => {
        set({ isLoading: true });
        try {
          // Get tokens (stored automatically by authApi.login)
          await authApi.login(data);

          // Fetch user data via /me endpoint (RGPD compliant - no PII in login response)
          const user = await authApi.me();

          set({
            user,
            isAuthenticated: true,
            isLoading: false,
          });
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      /**
       * Accept invite and set password (auto-login)
       */
      acceptInvite: async (data: AcceptInviteRequest) => {
        set({ isLoading: true });
        try {
          await authApi.acceptInvite(data);

          // After accepting invite, fetch user data
          const user = await authApi.me();
          set({
            user,
            isAuthenticated: true,
            isLoading: false,
          });
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      /**
       * Logout from current device
       * Always clears local auth state, even if API call fails
       */
      logout: async () => {
        set({ isLoading: true });
        try {
          await authApi.logout();
        } catch (error) {
          // Log but don't throw - user should still be logged out locally
          console.error('Logout API call failed:', error);
        } finally {
          // Always clear local auth state
          get().clearAuth();
        }
      },

      /**
       * Logout from all devices
       * Always clears local auth state, even if API call fails
       */
      logoutAll: async () => {
        set({ isLoading: true });
        try {
          await authApi.logoutAll();
        } catch (error) {
          // Log but don't throw - user should still be logged out locally
          console.error('Logout all API call failed:', error);
        } finally {
          // Always clear local auth state
          get().clearAuth();
        }
      },

      /**
       * Refresh current user information
       */
      refreshUser: async () => {
        try {
          const user = await authApi.me();
          set({
            user,
            isAuthenticated: true,
          });
        } catch (error) {
          // If refresh fails, clear authentication
          get().clearAuth();
          throw error;
        }
      },

      /**
       * Manually set user (useful for testing or edge cases)
       */
      setUser: (user: User | null) => {
        set({
          user,
          isAuthenticated: !!user,
        });
      },

      /**
       * Set loading state
       */
      setLoading: (isLoading: boolean) => {
        set({ isLoading });
      },

      /**
       * Clear authentication state
       */
      clearAuth: () => {
        clearTokens();
        set({
          user: null,
          accessToken: null,
          refreshToken: null,
          isAuthenticated: false,
          isLoading: false,
        });
      },
    }),
    {
      name: STORAGE_KEYS.USER,
      // RGPD: Do not persist user PII to localStorage
      // User data is fetched via /me on each session
      partialize: () => ({}),
    }
  )
);

/**
 * Initialize auth store on app startup
 * Checks for existing refresh token in localStorage and restores session
 * Access tokens are stored in memory only (lost on reload), so we use
 * the refresh token to obtain a new access token on page load.
 */
export const initializeAuth = async (): Promise<boolean> => {
  const store = useAuthStore.getState();
  const refreshToken = getRefreshToken();

  // No refresh token = not authenticated
  if (!refreshToken) {
    return false;
  }

  store.setLoading(true);

  try {
    // Use refresh token to get new access token
    // authApi.refresh() internally calls setTokens() after successful refresh
    await authApi.refresh({ refresh_token: refreshToken });

    // Fetch user data via /me endpoint
    const user = await authApi.me();
    store.setUser(user);
    store.setLoading(false);
    return true;
  } catch (error) {
    // Refresh token invalid or expired, clear auth
    store.clearAuth();
    return false;
  }
};
