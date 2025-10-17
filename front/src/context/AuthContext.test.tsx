import { describe, it, expect, vi, beforeEach } from 'vitest';
import { waitFor } from '@testing-library/react';
import { renderHook, act } from '@testing-library/react';
import { AuthProvider, useAuth } from './AuthContext';
import { authApi } from '@/api/auth';
import type { User } from '@/types';

// Mock dependencies
vi.mock('@/api/auth');
vi.mock('sonner', () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));

vi.mock('@/utils/jwt', () => ({
  isTokenExpired: vi.fn(),
}));

const mockUser: User = {
  id: 1,
  email: 'test@example.com',
  firstName: 'John',
  lastName: 'Doe',
  role: 'employee',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
};

describe('AuthContext', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  describe('AuthProvider', () => {
    it('provides auth context to children', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      expect(result.current).toBeDefined();
      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(result.current.loading).toBe(false);
    });

    it('throws error when useAuth used outside provider', () => {
      // Suppress console.error for this test
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      expect(() => {
        renderHook(() => useAuth());
      }).toThrow('useAuth must be used within AuthProvider');

      consoleSpy.mockRestore();
    });
  });

  describe('initialization', () => {
    it('loads user from localStorage on mount if token valid', async () => {
      const token = 'valid-token';
      localStorage.setItem('token', token);

      const { isTokenExpired } = await import('@/utils/jwt');
      vi.mocked(isTokenExpired).mockReturnValue(false);
      vi.mocked(authApi.me).mockResolvedValue(mockUser);

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      // Initially loading
      expect(result.current.loading).toBe(true);

      // Wait for initialization
      await waitFor(() => {
        expect(result.current.loading).toBe(false);
      });

      expect(result.current.user).toEqual(mockUser);
      expect(result.current.isAuthenticated).toBe(true);
      expect(authApi.me).toHaveBeenCalled();
    });

    it('clears auth if token is expired', async () => {
      const token = 'expired-token';
      localStorage.setItem('token', token);

      const { isTokenExpired } = await import('@/utils/jwt');
      vi.mocked(isTokenExpired).mockReturnValue(true);

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      await waitFor(() => {
        expect(result.current.loading).toBe(false);
      });

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(localStorage.getItem('token')).toBeNull();
    });

    it('handles API error during initialization', async () => {
      const token = 'valid-token';
      localStorage.setItem('token', token);

      const { isTokenExpired } = await import('@/utils/jwt');
      vi.mocked(isTokenExpired).mockReturnValue(false);
      vi.mocked(authApi.me).mockRejectedValue(new Error('API Error'));

      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      await waitFor(() => {
        expect(result.current.loading).toBe(false);
      });

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(localStorage.getItem('token')).toBeNull();

      consoleSpy.mockRestore();
    });

    it('does nothing if no token in localStorage', async () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      await waitFor(() => {
        expect(result.current.loading).toBe(false);
      });

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(authApi.me).not.toHaveBeenCalled();
    });
  });

  describe('login', () => {
    it('successfully logs in user', async () => {
      vi.mocked(authApi.login).mockResolvedValue({
        token: 'new-token',
        user: mockUser,
      });
      vi.mocked(authApi.me).mockResolvedValue(mockUser);

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      await act(async () => {
        await result.current.login({ email: 'test@example.com', password: 'password' });
      });

      expect(result.current.user).toEqual(mockUser);
      expect(result.current.isAuthenticated).toBe(true);
      expect(localStorage.getItem('token')).toBe('new-token');
      expect(authApi.login).toHaveBeenCalledWith({
        email: 'test@example.com',
        password: 'password',
      });
      expect(authApi.me).toHaveBeenCalled();
    });

    it('throws error on login failure', async () => {
      const error = new Error('Invalid credentials');
      vi.mocked(authApi.login).mockRejectedValue(error);

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      await expect(
        act(async () => {
          await result.current.login({ email: 'test@example.com', password: 'wrong' });
        })
      ).rejects.toThrow('Invalid credentials');

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(localStorage.getItem('token')).toBeNull();
    });

    it('handles API error when fetching user after login', async () => {
      vi.mocked(authApi.login).mockResolvedValue({
        token: 'new-token',
        user: mockUser,
      });
      vi.mocked(authApi.me).mockRejectedValue(new Error('API Error'));

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      await expect(
        act(async () => {
          await result.current.login({ email: 'test@example.com', password: 'password' });
        })
      ).rejects.toThrow('API Error');

      // Token should be stored but user fetch failed
      expect(localStorage.getItem('token')).toBe('new-token');
    });
  });

  describe('logout', () => {
    it('successfully logs out user', async () => {
      // Setup: logged in user
      localStorage.setItem('token', 'test-token');

      const { isTokenExpired } = await import('@/utils/jwt');
      vi.mocked(isTokenExpired).mockReturnValue(false);
      vi.mocked(authApi.me).mockResolvedValue(mockUser);
      vi.mocked(authApi.logout).mockResolvedValue();

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      // Wait for initialization
      await waitFor(() => {
        expect(result.current.user).toEqual(mockUser);
      });

      // Logout
      await act(async () => {
        await result.current.logout();
      });

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(localStorage.getItem('token')).toBeNull();
      expect(authApi.logout).toHaveBeenCalled();
    });

    it('clears state even if API logout fails', async () => {
      // Setup: logged in user
      localStorage.setItem('token', 'test-token');

      const { isTokenExpired } = await import('@/utils/jwt');
      vi.mocked(isTokenExpired).mockReturnValue(false);
      vi.mocked(authApi.me).mockResolvedValue(mockUser);
      vi.mocked(authApi.logout).mockRejectedValue(new Error('API Error'));

      const consoleSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      // Wait for initialization
      await waitFor(() => {
        expect(result.current.user).toEqual(mockUser);
      });

      // Logout should still work
      await act(async () => {
        await result.current.logout();
      });

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(localStorage.getItem('token')).toBeNull();
      expect(consoleSpy).toHaveBeenCalled();

      consoleSpy.mockRestore();
    });
  });

  describe('isAuthenticated', () => {
    it('returns false when no user', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      expect(result.current.isAuthenticated).toBe(false);
    });

    it('returns true when user exists', async () => {
      localStorage.setItem('token', 'valid-token');

      const { isTokenExpired } = await import('@/utils/jwt');
      vi.mocked(isTokenExpired).mockReturnValue(false);
      vi.mocked(authApi.me).mockResolvedValue(mockUser);

      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      });

      await waitFor(() => {
        expect(result.current.isAuthenticated).toBe(true);
      });
    });
  });
});
