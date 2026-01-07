/**
 * Authentication Store (Zustand)
 *
 * Global state management for authentication using Zustand.
 * Manages user session, tokens, and authentication state.
 */

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { authApi } from '../api/auth';
import { clearTokens, tokenManager } from '../api/client';
import { STORAGE_KEYS } from '../config/constants';
import type {
  User,
  RegisterRequest,
  LoginRequest,
  AuthState,
} from '../types/auth';

/**
 * Authentication store state and actions
 */
interface AuthStore extends AuthState {
  // Actions
  register: (data: RegisterRequest) => Promise<void>;
  login: (data: LoginRequest) => Promise<void>;
  logout: () => Promise<void>;
  logoutAll: () => Promise<void>;
  refreshUser: () => Promise<void>;
  setUser: (user: User | null) => void;
  setLoading: (isLoading: boolean) => void;
  clearAuth: () => void;
}

/**
 * Zustand auth store with persistence
 *
 * Persists user data to localStorage, but keeps access tokens in memory only
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
       * Register a new user account
       */
      register: async (data: RegisterRequest) => {
        set({ isLoading: true });
        try {
          const response = await authApi.register(data);

          set({
            user: response.user,
            isAuthenticated: true,
            isLoading: false,
          });
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      /**
       * Login with email and password
       */
      login: async (data: LoginRequest) => {
        set({ isLoading: true });
        try {
          const response = await authApi.login(data);

          set({
            user: response.user,
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
       */
      logout: async () => {
        set({ isLoading: true });
        try {
          await authApi.logout();
          get().clearAuth();
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      /**
       * Logout from all devices
       */
      logoutAll: async () => {
        set({ isLoading: true });
        try {
          await authApi.logoutAll();
          get().clearAuth();
        } catch (error) {
          set({ isLoading: false });
          throw error;
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
      // Only persist user data, not tokens
      partialize: (state) => ({
        user: state.user,
      }),
    }
  )
);

/**
 * Initialize auth store on app startup
 * Checks for existing tokens and refreshes user if available
 */
export const initializeAuth = async () => {
  const store = useAuthStore.getState();
  const accessToken = tokenManager.getAccessToken();

  if (accessToken) {
    try {
      await store.refreshUser();
    } catch (error) {
      // Token invalid or expired, clear auth
      store.clearAuth();
    }
  }
};
