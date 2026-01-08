/**
 * Tests for configuration constants
 */

import { describe, it, expect } from 'vitest';
import {
  API_BASE_URL,
  API_VERSION,
  API_URL,
  AUTH_ENDPOINTS,
  STORAGE_KEYS,
  TOKEN_CONFIG,
  PASSWORD_RULES,
  ERROR_MESSAGES,
} from '../constants';

describe('Configuration Constants', () => {
  describe('API Configuration', () => {
    it('should have correct API base URL', () => {
      expect(API_BASE_URL).toBe('http://localhost:8080');
    });

    it('should have correct API version', () => {
      expect(API_VERSION).toBe('/v1');
    });

    it('should construct full API URL correctly', () => {
      expect(API_URL).toBe('http://localhost:8080/v1');
    });
  });

  describe('Auth Endpoints', () => {
    it('should have all required auth endpoints', () => {
      expect(AUTH_ENDPOINTS.LOGIN).toBe('/auth/login');
      expect(AUTH_ENDPOINTS.LOGOUT).toBe('/auth/logout');
      expect(AUTH_ENDPOINTS.LOGOUT_ALL).toBe('/auth/logout-all');
      expect(AUTH_ENDPOINTS.REFRESH).toBe('/auth/refresh');
      expect(AUTH_ENDPOINTS.ME).toBe('/auth/me');
      expect(AUTH_ENDPOINTS.REQUEST_RESET).toBe('/auth/password/request-reset');
      expect(AUTH_ENDPOINTS.RESET_PASSWORD).toBe('/auth/password/reset');
      expect(AUTH_ENDPOINTS.SESSIONS).toBe('/auth/sessions');
      expect(AUTH_ENDPOINTS.ACCEPT_INVITE).toBe('/auth/accept-invite');
      expect(AUTH_ENDPOINTS.CHANGE_PASSWORD).toBe('/auth/change-password');
    });
  });

  describe('Storage Keys', () => {
    it('should have correct storage key names', () => {
      expect(STORAGE_KEYS.REFRESH_TOKEN).toBe('timemanager_refresh_token');
      expect(STORAGE_KEYS.USER).toBe('timemanager_user');
    });
  });

  describe('Token Configuration', () => {
    it('should have correct token lifetimes', () => {
      expect(TOKEN_CONFIG.ACCESS_TOKEN_LIFETIME).toBe(15 * 60 * 1000); // 15 minutes
      expect(TOKEN_CONFIG.REFRESH_TOKEN_LIFETIME).toBe(7 * 24 * 60 * 60 * 1000); // 7 days
      expect(TOKEN_CONFIG.REFRESH_THRESHOLD).toBe(2 * 60 * 1000); // 2 minutes
    });
  });

  describe('Password Rules', () => {
    it('should have correct password validation rules', () => {
      expect(PASSWORD_RULES.MIN_LENGTH).toBe(8);
      expect(PASSWORD_RULES.REQUIRE_UPPERCASE).toBe(true);
      expect(PASSWORD_RULES.REQUIRE_LOWERCASE).toBe(true);
      expect(PASSWORD_RULES.REQUIRE_NUMBER).toBe(true);
      expect(PASSWORD_RULES.REQUIRE_SPECIAL).toBe(true);
    });
  });

  describe('Error Messages', () => {
    it('should have all required error messages', () => {
      expect(ERROR_MESSAGES.NETWORK_ERROR).toBeTruthy();
      expect(ERROR_MESSAGES.UNAUTHORIZED).toBeTruthy();
      expect(ERROR_MESSAGES.FORBIDDEN).toBeTruthy();
      expect(ERROR_MESSAGES.SERVER_ERROR).toBeTruthy();
      expect(ERROR_MESSAGES.VALIDATION_ERROR).toBeTruthy();
      expect(ERROR_MESSAGES.RATE_LIMIT).toBeTruthy();
    });
  });
});
