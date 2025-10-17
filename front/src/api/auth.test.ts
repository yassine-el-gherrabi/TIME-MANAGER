import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { authApi } from './auth';
import { apiClient } from './client';
import type { LoginCredentials, AuthState, User } from '@/types';

// Mock apiClient
vi.mock('./client', () => ({
  apiClient: {
    post: vi.fn(),
    get: vi.fn(),
  },
}));

describe('authApi', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('login', () => {
    const mockCredentials: LoginCredentials = {
      email: 'test@example.com',
      password: 'password123',
    };

    const mockAuthState: AuthState = {
      token: 'jwt-token-123',
      user: {
        id: 1,
        email: 'test@example.com',
        firstName: 'John',
        lastName: 'Doe',
        role: 'employee',
        createdAt: '2024-01-01T00:00:00Z',
        updatedAt: '2024-01-01T00:00:00Z',
      },
    };

    it('calls apiClient.post with correct credentials', async () => {
      vi.mocked(apiClient.post).mockResolvedValue({ data: mockAuthState });

      await authApi.login(mockCredentials);

      expect(apiClient.post).toHaveBeenCalledWith('/login', mockCredentials);
      expect(apiClient.post).toHaveBeenCalledTimes(1);
    });

    it('returns auth state on successful login', async () => {
      vi.mocked(apiClient.post).mockResolvedValue({ data: mockAuthState });

      const result = await authApi.login(mockCredentials);

      expect(result).toEqual(mockAuthState);
      expect(result.token).toBe('jwt-token-123');
      expect(result.user.email).toBe('test@example.com');
    });

    it('throws transformed ApiError on network failure', async () => {
      const networkError = new Error('Network Error');
      vi.mocked(apiClient.post).mockRejectedValue(networkError);

      await expect(authApi.login(mockCredentials)).rejects.toThrow();
    });

    it('throws transformed ApiError on 401 Unauthorized', async () => {
      const unauthorizedError = {
        response: {
          status: 401,
          data: { message: 'Invalid credentials' },
        },
      };
      vi.mocked(apiClient.post).mockRejectedValue(unauthorizedError);

      await expect(authApi.login(mockCredentials)).rejects.toThrow();
    });

    it('throws transformed ApiError on 400 Bad Request', async () => {
      const badRequestError = {
        response: {
          status: 400,
          data: { message: 'Invalid email format' },
        },
      };
      vi.mocked(apiClient.post).mockRejectedValue(badRequestError);

      await expect(authApi.login(mockCredentials)).rejects.toThrow();
    });
  });

  describe('logout', () => {
    it('resolves successfully without making API call', async () => {
      await expect(authApi.logout()).resolves.toBeUndefined();
    });

    it('does not call any API endpoints', async () => {
      await authApi.logout();

      expect(apiClient.post).not.toHaveBeenCalled();
      expect(apiClient.get).not.toHaveBeenCalled();
    });

    it('returns a resolved promise', async () => {
      const result = await authApi.logout();

      expect(result).toBeUndefined();
    });
  });

  describe('me', () => {
    const mockUser: User = {
      id: 1,
      email: 'test@example.com',
      firstName: 'John',
      lastName: 'Doe',
      role: 'employee',
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-01T00:00:00Z',
    };

    it('calls apiClient.get with correct endpoint', async () => {
      vi.mocked(apiClient.get).mockResolvedValue({ data: mockUser });

      await authApi.me();

      expect(apiClient.get).toHaveBeenCalledWith('/me');
      expect(apiClient.get).toHaveBeenCalledTimes(1);
    });

    it('returns user data on successful request', async () => {
      vi.mocked(apiClient.get).mockResolvedValue({ data: mockUser });

      const result = await authApi.me();

      expect(result).toEqual(mockUser);
      expect(result.email).toBe('test@example.com');
      expect(result.role).toBe('employee');
    });

    it('throws transformed ApiError on network failure', async () => {
      const networkError = new Error('Network Error');
      vi.mocked(apiClient.get).mockRejectedValue(networkError);

      await expect(authApi.me()).rejects.toThrow();
    });

    it('throws transformed ApiError on 401 Unauthorized', async () => {
      const unauthorizedError = {
        response: {
          status: 401,
          data: { message: 'Invalid or expired token' },
        },
      };
      vi.mocked(apiClient.get).mockRejectedValue(unauthorizedError);

      await expect(authApi.me()).rejects.toThrow();
    });

    it('throws transformed ApiError on 403 Forbidden', async () => {
      const forbiddenError = {
        response: {
          status: 403,
          data: { message: 'Access denied' },
        },
      };
      vi.mocked(apiClient.get).mockRejectedValue(forbiddenError);

      await expect(authApi.me()).rejects.toThrow();
    });

    it('returns manager user correctly', async () => {
      const mockManagerUser: User = {
        ...mockUser,
        id: 2,
        email: 'manager@example.com',
        role: 'manager',
      };

      vi.mocked(apiClient.get).mockResolvedValue({ data: mockManagerUser });

      const result = await authApi.me();

      expect(result.role).toBe('manager');
      expect(result.email).toBe('manager@example.com');
    });
  });
});
