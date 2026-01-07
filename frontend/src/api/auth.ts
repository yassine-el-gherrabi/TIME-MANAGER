/**
 * Authentication API Client
 *
 * API methods for all authentication-related operations including
 * registration, login, token refresh, password reset, and session management.
 */

import { apiRequest, setTokens, clearTokens } from './client';
import { AUTH_ENDPOINTS } from '../config/constants';
import type {
  RegisterRequest,
  RegisterResponse,
  LoginRequest,
  LoginResponse,
  RefreshRequest,
  RefreshResponse,
  LogoutResponse,
  LogoutAllResponse,
  MeResponse,
  RequestResetRequest,
  RequestResetResponse,
  ResetPasswordRequest,
  ResetPasswordResponse,
  ActiveSessionsResponse,
} from '../types/auth';

/**
 * Authentication API methods
 */
export const authApi = {
  /**
   * Register a new user account
   *
   * @param data - Registration request payload
   * @returns User information and token pair
   */
  register: async (data: RegisterRequest): Promise<RegisterResponse> => {
    const response = await apiRequest<RegisterResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.REGISTER,
      data,
    });

    // Store tokens after successful registration
    setTokens(response.tokens);

    return response;
  },

  /**
   * Login with email and password
   *
   * @param data - Login credentials
   * @returns User information and token pair
   */
  login: async (data: LoginRequest): Promise<LoginResponse> => {
    const response = await apiRequest<LoginResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.LOGIN,
      data,
    });

    // Store tokens after successful login
    setTokens(response.tokens);

    return response;
  },

  /**
   * Refresh access token using refresh token
   *
   * @param data - Refresh token request payload
   * @returns New token pair
   */
  refresh: async (data: RefreshRequest): Promise<RefreshResponse> => {
    const response = await apiRequest<RefreshResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.REFRESH,
      data,
    });

    // Store new tokens
    setTokens(response.tokens);

    return response;
  },

  /**
   * Logout from current device
   * Revokes the current refresh token
   *
   * @returns Logout confirmation message
   */
  logout: async (): Promise<LogoutResponse> => {
    const response = await apiRequest<LogoutResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.LOGOUT,
    });

    // Clear tokens from storage
    clearTokens();

    return response;
  },

  /**
   * Logout from all devices
   * Revokes all refresh tokens for the user
   *
   * @returns Logout confirmation message
   */
  logoutAll: async (): Promise<LogoutAllResponse> => {
    const response = await apiRequest<LogoutAllResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.LOGOUT_ALL,
    });

    // Clear tokens from storage
    clearTokens();

    return response;
  },

  /**
   * Get current authenticated user information
   *
   * @returns Current user data
   */
  me: async (): Promise<MeResponse> => {
    return apiRequest<MeResponse>({
      method: 'GET',
      url: AUTH_ENDPOINTS.ME,
    });
  },

  /**
   * Request password reset email
   *
   * @param data - Email address for password reset
   * @returns Reset request confirmation message
   */
  requestPasswordReset: async (
    data: RequestResetRequest
  ): Promise<RequestResetResponse> => {
    return apiRequest<RequestResetResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.REQUEST_RESET,
      data,
    });
  },

  /**
   * Reset password using reset token
   *
   * @param data - Reset token and new password
   * @returns Password reset confirmation message
   */
  resetPassword: async (
    data: ResetPasswordRequest
  ): Promise<ResetPasswordResponse> => {
    return apiRequest<ResetPasswordResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.RESET_PASSWORD,
      data,
    });
  },

  /**
   * Get all active sessions for current user
   *
   * @returns List of active sessions
   */
  getActiveSessions: async (): Promise<ActiveSessionsResponse> => {
    return apiRequest<ActiveSessionsResponse>({
      method: 'GET',
      url: AUTH_ENDPOINTS.SESSIONS,
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  register,
  login,
  refresh,
  logout,
  logoutAll,
  me,
  requestPasswordReset,
  resetPassword,
  getActiveSessions,
} = authApi;
