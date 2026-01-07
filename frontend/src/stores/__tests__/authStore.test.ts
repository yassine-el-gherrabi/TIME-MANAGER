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
    register: vi.fn(),
    login: vi.fn(),
    logout: vi.fn(),
    logoutAll: vi.fn(),
    me: vi.fn(),
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

  describe('Register', () => {
    it('should register user successfully', async () => {
      const registerData = {
        email: 'new@example.com',
        password: 'password123',
        first_name: 'Jane',
        last_name: 'Smith',
        organization_id: '123e4567-e89b-12d3-a456-426614174001',
      };

      vi.mocked(authApi.register).mockResolvedValue({
        user: mockUser,
        tokens: {
          access_token: 'access_token',
          refresh_token: 'refresh_token',
        },
      });

      await useAuthStore.getState().register(registerData);

      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
      expect(state.isAuthenticated).toBe(true);
      expect(state.isLoading).toBe(false);
    });

    it('should handle registration error', async () => {
      const registerData = {
        email: 'new@example.com',
        password: 'password123',
        first_name: 'Jane',
        last_name: 'Smith',
        organization_id: '123e4567-e89b-12d3-a456-426614174001',
      };

      vi.mocked(authApi.register).mockRejectedValue(new Error('Registration failed'));

      await expect(useAuthStore.getState().register(registerData)).rejects.toThrow('Registration failed');

      const state = useAuthStore.getState();
      expect(state.isLoading).toBe(false);
    });
  });

  describe('Login', () => {
    it('should login user successfully', async () => {
      const loginData = {
        email: 'test@example.com',
        password: 'password123',
      };

      vi.mocked(authApi.login).mockResolvedValue({
        user: mockUser,
        tokens: {
          access_token: 'access_token',
          refresh_token: 'refresh_token',
        },
      });

      await useAuthStore.getState().login(loginData);

      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
      expect(state.isAuthenticated).toBe(true);
      expect(state.isLoading).toBe(false);
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
    it('should not refresh user if no access token', async () => {
      await initializeAuth();

      expect(authApi.me).not.toHaveBeenCalled();
    });

    it('should refresh user if access token exists', async () => {
      tokenManager.setAccessToken('test_token');
      vi.mocked(authApi.me).mockResolvedValue(mockUser);

      await initializeAuth();

      expect(authApi.me).toHaveBeenCalled();
      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
    });

    it('should clear auth if refresh fails during initialization', async () => {
      tokenManager.setAccessToken('invalid_token');
      vi.mocked(authApi.me).mockRejectedValue(new Error('Invalid token'));

      await initializeAuth();

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);
    });
  });
});
