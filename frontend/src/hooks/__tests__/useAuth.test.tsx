/**
 * Tests for useAuth hooks
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { renderHook } from '@testing-library/react';
import { useAuth, useCurrentUser, useIsAuthenticated, useAuthLoading } from '../useAuth';
import { useAuthStore } from '../../stores/authStore';
import { UserRole } from '../../types/auth';
import type { User } from '../../types/auth';

describe('useAuth Hooks', () => {
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
    useAuthStore.getState().clearAuth();
  });

  describe('useAuth', () => {
    it('should return authentication state and actions', () => {
      const { result } = renderHook(() => useAuth());

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
      expect(result.current.isLoading).toBe(false);
      expect(typeof result.current.register).toBe('function');
      expect(typeof result.current.login).toBe('function');
      expect(typeof result.current.logout).toBe('function');
      expect(typeof result.current.logoutAll).toBe('function');
      expect(typeof result.current.refreshUser).toBe('function');
    });

    it('should reflect authenticated state', () => {
      useAuthStore.getState().setUser(mockUser);

      const { result } = renderHook(() => useAuth());

      expect(result.current.user).toEqual(mockUser);
      expect(result.current.isAuthenticated).toBe(true);
    });

    it('should reflect loading state', () => {
      useAuthStore.getState().setLoading(true);

      const { result } = renderHook(() => useAuth());

      expect(result.current.isLoading).toBe(true);
    });
  });

  describe('useCurrentUser', () => {
    it('should return null when not authenticated', () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current).toBeNull();
    });

    it('should return user when authenticated', () => {
      useAuthStore.getState().setUser(mockUser);

      const { result } = renderHook(() => useCurrentUser());

      expect(result.current).toEqual(mockUser);
    });
  });

  describe('useIsAuthenticated', () => {
    it('should return false when not authenticated', () => {
      const { result } = renderHook(() => useIsAuthenticated());

      expect(result.current).toBe(false);
    });

    it('should return true when authenticated', () => {
      useAuthStore.getState().setUser(mockUser);

      const { result } = renderHook(() => useIsAuthenticated());

      expect(result.current).toBe(true);
    });
  });

  describe('useAuthLoading', () => {
    it('should return false when not loading', () => {
      const { result } = renderHook(() => useAuthLoading());

      expect(result.current).toBe(false);
    });

    it('should return true when loading', () => {
      useAuthStore.getState().setLoading(true);

      const { result } = renderHook(() => useAuthLoading());

      expect(result.current).toBe(true);
    });
  });
});
