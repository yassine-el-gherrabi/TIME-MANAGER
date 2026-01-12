/**
 * Tests for API client and token management
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
  setTokens,
  clearTokens,
  hasRefreshToken,
  tokenManager,
  ApiErrorClass,
} from '../client';

describe('API Client', () => {
  beforeEach(() => {
    // Clear cookies before each test
    document.cookie = 'csrf_token=; Max-Age=0; path=/';
    tokenManager.clearAccessToken();
  });

  describe('Token Management - Memory', () => {
    it('should set access token in memory', () => {
      const token = 'test_access_token';
      tokenManager.setAccessToken(token);
      expect(tokenManager.getAccessToken()).toBe(token);
    });

    it('should clear access token from memory', () => {
      const token = 'test_access_token';
      tokenManager.setAccessToken(token);
      tokenManager.clearAccessToken();
      expect(tokenManager.getAccessToken()).toBeNull();
    });

    it('should return null for expired token', () => {
      vi.useFakeTimers();
      const token = 'test_access_token';
      tokenManager.setAccessToken(token);

      // Fast forward past expiry (15 minutes + 1 second)
      vi.advanceTimersByTime(15 * 60 * 1000 + 1000);

      expect(tokenManager.getAccessToken()).toBeNull();
      vi.useRealTimers();
    });

    it('should indicate when token should be refreshed', () => {
      vi.useFakeTimers();
      const token = 'test_access_token';
      tokenManager.setAccessToken(token);

      // Fast forward to near expiry (13 minutes - within 2 min threshold)
      vi.advanceTimersByTime(13 * 60 * 1000);

      expect(tokenManager.shouldRefresh()).toBe(true);
      vi.useRealTimers();
    });
  });

  describe('Token Operations', () => {
    it('should set access token via setTokens', () => {
      const tokens = {
        access_token: 'test_access',
      };
      setTokens(tokens);

      expect(tokenManager.getAccessToken()).toBe('test_access');
    });

    it('should clear access token via clearTokens', () => {
      setTokens({ access_token: 'test_access' });
      clearTokens();

      expect(tokenManager.getAccessToken()).toBeNull();
    });
  });

  describe('CSRF Token Detection', () => {
    it('should detect presence of CSRF token cookie', () => {
      // Set CSRF token cookie
      document.cookie = 'csrf_token=test_csrf_token; path=/';
      expect(hasRefreshToken()).toBe(true);
    });

    it('should return false when no CSRF token cookie', () => {
      document.cookie = 'csrf_token=; Max-Age=0; path=/';
      expect(hasRefreshToken()).toBe(false);
    });
  });

  describe('ApiErrorClass', () => {
    it('should create error with message', () => {
      const error = new ApiErrorClass('Test error');
      expect(error.message).toBe('Test error');
      expect(error.name).toBe('ApiError');
    });

    it('should create error with status code', () => {
      const error = new ApiErrorClass('Test error', 404);
      expect(error.status).toBe(404);
    });

    it('should create error with details', () => {
      const details = ['Detail 1', 'Detail 2'];
      const error = new ApiErrorClass('Test error', 400, details);
      expect(error.details).toEqual(details);
    });
  });
});
