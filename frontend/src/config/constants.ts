/**
 * Authentication Constants
 *
 * Configuration constants for authentication and API communication.
 */

/**
 * API base URL - defaults to localhost in development
 */
export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

/**
 * API version prefix
 */
export const API_VERSION = '/v1';

/**
 * Full API URL with version
 */
export const API_URL = `${API_BASE_URL}${API_VERSION}`;

/**
 * Authentication endpoints
 */
export const AUTH_ENDPOINTS = {
  REGISTER: '/auth/register',
  LOGIN: '/auth/login',
  LOGOUT: '/auth/logout',
  LOGOUT_ALL: '/auth/logout-all',
  REFRESH: '/auth/refresh',
  ME: '/auth/me',
  REQUEST_RESET: '/auth/password/request-reset',
  RESET_PASSWORD: '/auth/password/reset',
  SESSIONS: '/auth/sessions',
} as const;

/**
 * Local storage keys for token management
 * Note: Access tokens should be stored in memory only, not localStorage
 */
export const STORAGE_KEYS = {
  REFRESH_TOKEN: 'timemanager_refresh_token',
  USER: 'timemanager_user',
} as const;

/**
 * Token expiration settings
 */
export const TOKEN_CONFIG = {
  ACCESS_TOKEN_LIFETIME: 15 * 60 * 1000, // 15 minutes in milliseconds
  REFRESH_TOKEN_LIFETIME: 7 * 24 * 60 * 60 * 1000, // 7 days in milliseconds
  REFRESH_THRESHOLD: 2 * 60 * 1000, // Refresh 2 minutes before expiry
} as const;

/**
 * HTTP request timeout settings
 */
export const REQUEST_TIMEOUT = {
  DEFAULT: 30000, // 30 seconds
  AUTH: 10000, // 10 seconds for auth requests
} as const;

/**
 * Password validation rules
 */
export const PASSWORD_RULES = {
  MIN_LENGTH: 8,
  REQUIRE_UPPERCASE: true,
  REQUIRE_LOWERCASE: true,
  REQUIRE_NUMBER: true,
  REQUIRE_SPECIAL: false,
} as const;

/**
 * Rate limiting configuration (matches backend)
 */
export const RATE_LIMITS = {
  LOGIN_MAX_ATTEMPTS: 5,
  LOGIN_WINDOW_MINUTES: 15,
  PASSWORD_RESET_MAX_ATTEMPTS: 3,
  PASSWORD_RESET_WINDOW_MINUTES: 60,
} as const;

/**
 * Session configuration
 */
export const SESSION_CONFIG = {
  IDLE_TIMEOUT: 30 * 60 * 1000, // 30 minutes in milliseconds
  WARNING_BEFORE_TIMEOUT: 5 * 60 * 1000, // Show warning 5 minutes before timeout
} as const;

/**
 * Error messages
 */
export const ERROR_MESSAGES = {
  NETWORK_ERROR: 'Network error. Please check your connection.',
  UNAUTHORIZED: 'Session expired. Please log in again.',
  FORBIDDEN: 'You do not have permission to perform this action.',
  SERVER_ERROR: 'Server error. Please try again later.',
  VALIDATION_ERROR: 'Please check your input and try again.',
  RATE_LIMIT: 'Too many attempts. Please try again later.',
} as const;

/**
 * Success messages
 */
export const SUCCESS_MESSAGES = {
  LOGIN: 'Successfully logged in',
  LOGOUT: 'Successfully logged out',
  REGISTER: 'Account created successfully',
  PASSWORD_RESET_REQUEST: 'Password reset email sent',
  PASSWORD_RESET: 'Password reset successfully',
} as const;
