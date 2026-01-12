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
  LOGIN: '/auth/login',
  LOGOUT: '/auth/logout',
  LOGOUT_ALL: '/auth/logout-all',
  REFRESH: '/auth/refresh',
  ME: '/auth/me',
  REQUEST_RESET: '/auth/password/request-reset',
  RESET_PASSWORD: '/auth/password/reset',
  SESSIONS: '/auth/sessions',
  REVOKE_SESSION: (id: string) => `/auth/sessions/${id}`,
  ACCEPT_INVITE: '/auth/accept-invite',
  CHANGE_PASSWORD: '/auth/change-password',
  BOOTSTRAP: '/auth/bootstrap',
} as const;

/**
 * User management endpoints (Admin)
 */
export const USER_ENDPOINTS = {
  LIST: '/users',
  CREATE: '/users',
  GET: (id: string) => `/users/${id}`,
  UPDATE: (id: string) => `/users/${id}`,
  DELETE: (id: string) => `/users/${id}`,
  RESTORE: (id: string) => `/users/${id}/restore`,
  RESEND_INVITE: (id: string) => `/users/${id}/resend-invite`,
  ASSIGN_SCHEDULE: (id: string) => `/users/${id}/schedule`,
} as const;

/**
 * Clock endpoints
 */
export const CLOCK_ENDPOINTS = {
  CLOCK_IN: '/clocks/in',
  CLOCK_OUT: '/clocks/out',
  STATUS: '/clocks/status',
  HISTORY: '/clocks/history',
  PENDING: '/clocks/pending',
  APPROVE: (id: string) => `/clocks/${id}/approve`,
  REJECT: (id: string) => `/clocks/${id}/reject`,
} as const;

/**
 * Team endpoints
 */
export const TEAM_ENDPOINTS = {
  LIST: '/teams',
  CREATE: '/teams',
  MY_TEAMS: '/teams/my',
  GET: (id: string) => `/teams/${id}`,
  UPDATE: (id: string) => `/teams/${id}`,
  DELETE: (id: string) => `/teams/${id}`,
  ADD_MEMBER: (id: string) => `/teams/${id}/members`,
  REMOVE_MEMBER: (teamId: string, userId: string) => `/teams/${teamId}/members/${userId}`,
} as const;

/**
 * Schedule endpoints
 */
export const SCHEDULE_ENDPOINTS = {
  LIST: '/schedules',
  CREATE: '/schedules',
  MY_SCHEDULE: '/schedules/me',
  GET: (id: string) => `/schedules/${id}`,
  UPDATE: (id: string) => `/schedules/${id}`,
  DELETE: (id: string) => `/schedules/${id}`,
  ADD_DAY: (id: string) => `/schedules/${id}/days`,
  UPDATE_DAY: (dayId: string) => `/schedules/days/${dayId}`,
  REMOVE_DAY: (dayId: string) => `/schedules/days/${dayId}`,
} as const;

/**
 * KPI endpoints
 */
export const KPI_ENDPOINTS = {
  MY_KPIS: '/kpis/me',
  USER_KPIS: (id: string) => `/kpis/users/${id}`,
  TEAM_KPIS: (id: string) => `/kpis/teams/${id}`,
  ORG_KPIS: '/kpis/organization',
  PRESENCE: '/kpis/presence',
  CHARTS: '/kpis/charts',
} as const;

/**
 * Absence type endpoints
 */
export const ABSENCE_TYPE_ENDPOINTS = {
  LIST: '/absence-types',
  CREATE: '/absence-types',
  GET: (id: string) => `/absence-types/${id}`,
  UPDATE: (id: string) => `/absence-types/${id}`,
  DELETE: (id: string) => `/absence-types/${id}`,
} as const;

/**
 * Absence endpoints
 */
export const ABSENCE_ENDPOINTS = {
  LIST: '/absences',
  CREATE: '/absences',
  PENDING: '/absences/pending',
  GET: (id: string) => `/absences/${id}`,
  APPROVE: (id: string) => `/absences/${id}/approve`,
  REJECT: (id: string) => `/absences/${id}/reject`,
  CANCEL: (id: string) => `/absences/${id}/cancel`,
} as const;

/**
 * Balance endpoints
 */
export const BALANCE_ENDPOINTS = {
  LIST: '/balances',
  MY_BALANCES: '/balances/me',
  ADJUST: (id: string) => `/balances/${id}/adjust`,
  SET: (userId: string) => `/users/${userId}/balances`,
} as const;

/**
 * Closed day endpoints
 */
export const CLOSED_DAY_ENDPOINTS = {
  LIST: '/closed-days',
  CREATE: '/closed-days',
  GET: (id: string) => `/closed-days/${id}`,
  UPDATE: (id: string) => `/closed-days/${id}`,
  DELETE: (id: string) => `/closed-days/${id}`,
} as const;

/**
 * Notification endpoints
 */
export const NOTIFICATION_ENDPOINTS = {
  LIST: '/notifications',
  UNREAD_COUNT: '/notifications/unread-count',
  MARK_READ: (id: string) => `/notifications/${id}/read`,
  MARK_ALL_READ: '/notifications/read-all',
} as const;

/**
 * Audit log endpoints (Super Admin only)
 */
export const AUDIT_ENDPOINTS = {
  LIST: '/audit-logs',
  EXPORT: '/audit-logs/export',
} as const;

/**
 * Organization endpoints (Super Admin only)
 */
export const ORGANIZATION_ENDPOINTS = {
  LIST: '/organizations',
  CREATE: '/organizations',
  GET: (id: string) => `/organizations/${id}`,
  UPDATE: (id: string) => `/organizations/${id}`,
  DELETE: (id: string) => `/organizations/${id}`,
} as const;

/**
 * Reports endpoints (Admin+)
 */
export const REPORTS_ENDPOINTS = {
  EXPORT: '/reports/export',
} as const;

/**
 * Clock restriction endpoints (Admin+)
 */
export const CLOCK_RESTRICTION_ENDPOINTS = {
  LIST: '/clock-restrictions',
  CREATE: '/clock-restrictions',
  GET: (id: string) => `/clock-restrictions/${id}`,
  UPDATE: (id: string) => `/clock-restrictions/${id}`,
  DELETE: (id: string) => `/clock-restrictions/${id}`,
  VALIDATE: '/clock-restrictions/validate',
  OVERRIDES: '/clock-restrictions/overrides',
  PENDING_OVERRIDES: '/clock-restrictions/overrides/pending',
  MY_OVERRIDES: '/clock-restrictions/overrides/me',
  REVIEW_OVERRIDE: (id: string) => `/clock-restrictions/overrides/${id}/review`,
} as const;

/**
 * Break system endpoints (Admin+ for policies, all users for entries)
 */
export const BREAK_ENDPOINTS = {
  // Break policies CRUD
  POLICIES: '/breaks/policies',
  CREATE_POLICY: '/breaks/policies',
  GET_POLICY: (id: string) => `/breaks/policies/${id}`,
  UPDATE_POLICY: (id: string) => `/breaks/policies/${id}`,
  DELETE_POLICY: (id: string) => `/breaks/policies/${id}`,
  // Break windows management
  GET_WINDOWS: (policyId: string) => `/breaks/policies/${policyId}/windows`,
  ADD_WINDOW: (policyId: string) => `/breaks/policies/${policyId}/windows`,
  DELETE_WINDOW: (policyId: string, windowId: string) =>
    `/breaks/policies/${policyId}/windows/${windowId}`,
  // Break entries (explicit tracking)
  ENTRIES: '/breaks/entries',
  START_BREAK: (clockEntryId: string) => `/breaks/entries/${clockEntryId}/start`,
  END_BREAK: '/breaks/entries/end',
  // Status and effective policy
  STATUS: '/breaks/status',
  EFFECTIVE: '/breaks/effective',
} as const;

/**
 * Local storage keys
 * Note: Access tokens stored in memory only, refresh tokens in HttpOnly cookies
 */
export const STORAGE_KEYS = {
  USER: 'timemanager_user',
  ONBOARDING_SEEN: 'timemanager_onboarding_seen',
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
 * Password validation rules (must match backend validation)
 */
export const PASSWORD_RULES = {
  MIN_LENGTH: 8,
  REQUIRE_UPPERCASE: true,
  REQUIRE_LOWERCASE: true,
  REQUIRE_NUMBER: true,
  REQUIRE_SPECIAL: true,
  SPECIAL_CHARS: '!@#$%^&*()_+-=[]{}|;:,.<>?',
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
  PASSWORD_RESET_REQUEST: 'Password reset email sent',
  PASSWORD_RESET: 'Password reset successfully',
  INVITE_ACCEPTED: 'Account activated successfully',
  PASSWORD_CHANGED: 'Password changed successfully',
} as const;
