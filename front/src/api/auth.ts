import { apiClient } from './client';
import { transformToApiError } from '@/utils/errorHandler';
import type { User, LoginCredentials, AuthState } from '@/types';

export const authApi = {
  /**
   * Login with email and password
   * Returns JWT token and user data
   */
  login: async (credentials: LoginCredentials): Promise<AuthState> => {
    try {
      const { data } = await apiClient.post<AuthState>('/login', credentials);
      return data;
    } catch (error) {
      // Transform and re-throw as ApiError
      throw transformToApiError(error);
    }
  },

  /**
   * Logout current user
   * Note: Only client-side logout (clearing localStorage)
   * No server-side session to clear with JWT-only auth
   */
  logout: async (): Promise<void> => {
    // Logout is handled entirely client-side (clearing localStorage in AuthContext)
    // No backend endpoint needed for JWT-only auth
    return Promise.resolve();
  },

  /**
   * Get current authenticated user
   * Validates token and returns fresh user data
   */
  me: async (): Promise<User> => {
    try {
      const { data } = await apiClient.get<User>('/me');
      return data;
    } catch (error) {
      throw transformToApiError(error);
    }
  },
};
