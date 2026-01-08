/**
 * Tests for authentication store
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { useAuthStore, initializeAuth } from '../authStore';
import { authApi } from '../../api/auth';
import { tokenManager } from '../../api/client';
import { UserRole } from '../../types/auth';
import type { User } from '../../types/auth';

// Mock the API
vi.mock('../../api/auth', () => ({
  authApi: {
    login: vi.fn(),
    logout: vi.fn(),
    logoutAll: vi.fn(),
    me: vi.fn(),
    refresh: vi.fn(),
  },
}));

describe('AuthStore', () => {
  const mockUser: User = {
    id: '123e4567-e89b-12d3-a456-426614174000',
    email: 'test@example.com',
    first_name: 'John',
    last_name: 'Doe',
    role: UserRole.Employee,
    organization_id: '123e4567-e89b-12d3-a456-426614174001',
    created_at: '2024-01-01T00:00:00Z',
  };

  beforeEach(() => {
    // Reset store state
    useAuthStore.getState().clearAuth();
    localStorage.clear();
    vi.clearAllMocks();
  });

  describe('Initial State', () => {
    it('should have correct initial state', () => {
      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);
      expect(state.isLoading).toBe(false);
    });
  });

  describe('Login', () => {
    it('should login user successfully', async () => {
      const loginData = {
        email: 'test@example.com',
        password: 'password123',
      };

      // Login returns only access_token (refresh token is HttpOnly cookie)
      vi.mocked(authApi.login).mockResolvedValue({
        access_token: 'access_token',
      });

      // User is fetched via /me after login
      vi.mocked(authApi.me).mockResolvedValue(mockUser);

      await useAuthStore.getState().login(loginData);

      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
      expect(state.isAuthenticated).toBe(true);
      expect(state.isLoading).toBe(false);
      expect(authApi.me).toHaveBeenCalled();
    });

    it('should handle login error', async () => {
      const loginData = {
        email: 'test@example.com',
        password: 'wrong_password',
      };

      vi.mocked(authApi.login).mockRejectedValue(new Error('Invalid credentials'));

      await expect(useAuthStore.getState().login(loginData)).rejects.toThrow('Invalid credentials');

      const state = useAuthStore.getState();
      expect(state.isLoading).toBe(false);
    });
  });

  describe('Logout', () => {
    it('should logout user successfully', async () => {
      // Set up authenticated state
      useAuthStore.getState().setUser(mockUser);

      vi.mocked(authApi.logout).mockResolvedValue({ message: 'Logged out' });

      await useAuthStore.getState().logout();

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);
      expect(state.isLoading).toBe(false);
    });
  });

  describe('Refresh User', () => {
    it('should refresh user data successfully', async () => {
      vi.mocked(authApi.me).mockResolvedValue(mockUser);

      await useAuthStore.getState().refreshUser();

      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
      expect(state.isAuthenticated).toBe(true);
    });

    it('should clear auth on refresh failure', async () => {
      // Set up authenticated state
      useAuthStore.getState().setUser(mockUser);

      vi.mocked(authApi.me).mockRejectedValue(new Error('Token expired'));

      await expect(useAuthStore.getState().refreshUser()).rejects.toThrow('Token expired');

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);
    });
  });

  describe('Set User', () => {
    it('should set user and authentication state', () => {
      useAuthStore.getState().setUser(mockUser);

      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
      expect(state.isAuthenticated).toBe(true);
    });

    it('should clear authentication when setting null user', () => {
      useAuthStore.getState().setUser(mockUser);
      useAuthStore.getState().setUser(null);

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);
    });
  });

  describe('Clear Auth', () => {
    it('should clear all authentication state', () => {
      // Set up authenticated state
      useAuthStore.getState().setUser(mockUser);
      tokenManager.setAccessToken('test_token');

      useAuthStore.getState().clearAuth();

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);
      expect(state.isLoading).toBe(false);
      expect(tokenManager.getAccessToken()).toBeNull();
    });
  });

  describe('Initialize Auth', () => {
    it('should not refresh user if no CSRF token (no refresh token)', async () => {
      // No CSRF cookie = no refresh token = not authenticated
      document.cookie = 'csrf_token=; Max-Age=0; path=/';

      const result = await initializeAuth();

      expect(result).toBe(false);
      expect(authApi.refresh).not.toHaveBeenCalled();
    });

    it('should refresh user if CSRF token exists (proxy for refresh token)', async () => {
      // Set CSRF cookie to indicate refresh token presence
      document.cookie = 'csrf_token=test_csrf_token; path=/';

      vi.mocked(authApi.refresh).mockResolvedValue({
        access_token: 'new_access_token',
      });
      vi.mocked(authApi.me).mockResolvedValue(mockUser);

      const result = await initializeAuth();

      expect(result).toBe(true);
      expect(authApi.refresh).toHaveBeenCalled();
      expect(authApi.me).toHaveBeenCalled();
      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);

      // Clean up
      document.cookie = 'csrf_token=; Max-Age=0; path=/';
    });

    it('should clear auth if refresh fails during initialization', async () => {
      // Set CSRF cookie to indicate refresh token presence
      document.cookie = 'csrf_token=test_csrf_token; path=/';

      vi.mocked(authApi.refresh).mockRejectedValue(new Error('Invalid token'));

      const result = await initializeAuth();

      expect(result).toBe(false);
      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);

      // Clean up
      document.cookie = 'csrf_token=; Max-Age=0; path=/';
    });
  });
});
