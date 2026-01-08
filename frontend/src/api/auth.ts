/**
 * Authentication API Client
 *
 * API methods for all authentication-related operations including
 * registration, login, token refresh, password reset, and session management.
 */

import { apiRequest, setTokens, clearTokens } from './client';
import { AUTH_ENDPOINTS } from '../config/constants';
import type {
  LoginRequest,
  LoginResponse,
  RefreshResponse,
  LogoutResponse,
  LogoutAllResponse,
  MeResponse,
  RequestResetRequest,
  RequestResetResponse,
  ResetPasswordRequest,
  ResetPasswordResponse,
  ActiveSessionsResponse,
  AcceptInviteRequest,
  AcceptInviteResponse,
  VerifyInviteRequest,
  VerifyInviteResponse,
  ChangePasswordRequest,
  ChangePasswordResponse,
  RevokeSessionResponse,
} from '../types/auth';

/**
 * Authentication API methods
 */
export const authApi = {
  /**
   * Login with email and password
   *
   * @param data - Login credentials
   * @returns Access token (refresh token sent as HttpOnly cookie)
   */
  login: async (data: LoginRequest): Promise<LoginResponse> => {
    const response = await apiRequest<LoginResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.LOGIN,
      data,
    });

    // Store access token (refresh token is handled by HttpOnly cookie)
    setTokens({ access_token: response.access_token });

    return response;
  },

  /**
   * Refresh access token using HttpOnly refresh token cookie
   *
   * @returns New access token (refresh token sent/received as HttpOnly cookie)
   */
  refresh: async (): Promise<RefreshResponse> => {
    const response = await apiRequest<RefreshResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.REFRESH,
    });

    // Store new access token (refresh token is handled by HttpOnly cookie)
    setTokens({ access_token: response.access_token });

    return response;
  },

  /**
   * Logout from current device
   * Revokes the refresh token from HttpOnly cookie
   *
   * @returns Logout confirmation message
   */
  logout: async (): Promise<LogoutResponse> => {
    const response = await apiRequest<LogoutResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.LOGOUT,
    });

    // Clear access token from memory (cookies cleared by server response)
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

  /**
   * Accept invitation and set password
   *
   * @param data - Invite token and new password
   * @returns Access token for auto-login
   */
  acceptInvite: async (data: AcceptInviteRequest): Promise<AcceptInviteResponse> => {
    const response = await apiRequest<AcceptInviteResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.ACCEPT_INVITE,
      data,
    });

    // Store access token after successful invite acceptance
    // Refresh token is set as HttpOnly cookie by the server
    if (response.access_token) {
      setTokens({ access_token: response.access_token });
    }

    return response;
  },

  /**
   * Verify if an invite token is valid
   *
   * @param data - Invite token to verify
   * @returns Validity status
   */
  verifyInvite: async (data: VerifyInviteRequest): Promise<VerifyInviteResponse> => {
    return apiRequest<VerifyInviteResponse>({
      method: 'POST',
      url: AUTH_ENDPOINTS.ACCEPT_INVITE.replace('accept-invite', 'verify-invite'),
      data,
    });
  },

  /**
   * Change password for authenticated user
   *
   * @param data - Current and new password
   * @returns Success message
   */
  changePassword: async (data: ChangePasswordRequest): Promise<ChangePasswordResponse> => {
    return apiRequest<ChangePasswordResponse>({
      method: 'PUT',
      url: AUTH_ENDPOINTS.CHANGE_PASSWORD,
      data,
    });
  },

  /**
   * Revoke a specific session
   *
   * @param sessionId - ID of the session to revoke
   * @returns Success message
   */
  revokeSession: async (sessionId: string): Promise<RevokeSessionResponse> => {
    return apiRequest<RevokeSessionResponse>({
      method: 'DELETE',
      url: AUTH_ENDPOINTS.REVOKE_SESSION(sessionId),
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  login,
  refresh,
  logout,
  logoutAll,
  me,
  requestPasswordReset,
  resetPassword,
  getActiveSessions,
  acceptInvite,
  verifyInvite,
  changePassword,
  revokeSession,
} = authApi;
