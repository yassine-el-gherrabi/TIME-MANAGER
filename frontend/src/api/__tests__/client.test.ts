/**
 * Tests for API client and token management
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
  getRefreshToken,
  setRefreshToken,
  removeRefreshToken,
  setTokens,
  clearTokens,
  tokenManager,
  ApiErrorClass,
} from '../client';
import { STORAGE_KEYS } from '../../config/constants';

describe('API Client', () => {
  beforeEach(() => {
    // Clear localStorage before each test
    localStorage.clear();
    tokenManager.clearAccessToken();
  });

  describe('Token Management - LocalStorage', () => {
    it('should set refresh token in localStorage', () => {
      const token = 'test_refresh_token';
      setRefreshToken(token);
      expect(localStorage.getItem(STORAGE_KEYS.REFRESH_TOKEN)).toBe(token);
    });

    it('should get refresh token from localStorage', () => {
      const token = 'test_refresh_token';
      localStorage.setItem(STORAGE_KEYS.REFRESH_TOKEN, token);
      expect(getRefreshToken()).toBe(token);
    });

    it('should remove refresh token from localStorage', () => {
      const token = 'test_refresh_token';
      localStorage.setItem(STORAGE_KEYS.REFRESH_TOKEN, token);
      removeRefreshToken();
      expect(localStorage.getItem(STORAGE_KEYS.REFRESH_TOKEN)).toBeNull();
    });
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

  describe('Token Pair Operations', () => {
    it('should set both access and refresh tokens', () => {
      const tokens = {
        access_token: 'test_access',
        refresh_token: 'test_refresh',
      };
      setTokens(tokens);

      expect(tokenManager.getAccessToken()).toBe('test_access');
      expect(getRefreshToken()).toBe('test_refresh');
    });

    it('should clear all tokens', () => {
      const tokens = {
        access_token: 'test_access',
        refresh_token: 'test_refresh',
      };
      setTokens(tokens);
      clearTokens();

      expect(tokenManager.getAccessToken()).toBeNull();
      expect(getRefreshToken()).toBeNull();
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
