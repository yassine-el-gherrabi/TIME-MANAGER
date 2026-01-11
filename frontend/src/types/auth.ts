/**
 * Authentication Types
 *
 * TypeScript type definitions for authentication-related data structures
 * matching the backend Rust API responses.
 */

/**
 * User role enumeration
 */
export enum UserRole {
  SuperAdmin = 'SuperAdmin',
  Admin = 'Admin',
  Manager = 'Manager',
  Employee = 'Employee'
}

/**
 * Token pair containing access and refresh tokens (legacy, for backwards compat)
 */
export interface TokenPair {
  access_token: string;
  refresh_token: string;
}

/**
 * User login request payload
 */
export interface LoginRequest {
  email: string;
  password: string;
}

/**
 * User login response (access token only, refresh token sent as HttpOnly cookie)
 */
export interface LoginResponse {
  access_token: string;
}

/**
 * Token refresh request payload (deprecated - refresh token now in HttpOnly cookie)
 * @deprecated Refresh token is now sent as HttpOnly cookie
 */
export interface RefreshRequest {
  refresh_token?: string;
}

/**
 * Token refresh response (access token only, refresh token sent as HttpOnly cookie)
 */
export interface RefreshResponse {
  access_token: string;
}

/**
 * Logout response
 */
export interface LogoutResponse {
  message: string;
}

/**
 * Logout all devices response
 */
export interface LogoutAllResponse {
  message: string;
}

/**
 * Current user information
 */
export interface User {
  id: string;
  email: string;
  first_name: string;
  last_name: string;
  role: UserRole;
  phone?: string | null;
  organization_id: string;
  created_at: string;
}

/**
 * Current user response (GET /api/v1/auth/me)
 */
export interface MeResponse {
  id: string;
  email: string;
  first_name: string;
  last_name: string;
  role: UserRole;
  phone?: string | null;
  organization_id: string;
  created_at: string;
}

/**
 * Password reset request payload
 */
export interface RequestResetRequest {
  email: string;
}

/**
 * Password reset request response
 */
export interface RequestResetResponse {
  message: string;
  reset_token?: string; // Only in development
}

/**
 * Password reset payload
 */
export interface ResetPasswordRequest {
  reset_token: string;
  new_password: string;
}

/**
 * Password reset response
 */
export interface ResetPasswordResponse {
  message: string;
}

/**
 * User session information
 */
export interface SessionInfo {
  id: string;
  user_agent: string | null;
  created_at: string;
  last_activity: string;
  expires_at: string;
}

/**
 * Active sessions response
 */
export interface ActiveSessionsResponse {
  sessions: SessionInfo[];
  total: number;
}

/**
 * JWT token claims structure (decoded from access token)
 */
export interface JwtClaims {
  sub: string; // user_id
  org_id: string; // organization_id
  role: UserRole;
  exp: number; // expiration timestamp
  iat: number; // issued at timestamp
}

/**
 * Authentication state
 * Note: refreshToken is no longer tracked in JS (HttpOnly cookie)
 */
export interface AuthState {
  user: User | null;
  accessToken: string | null;
  refreshToken: string | null; // Deprecated - kept for backwards compat but always null
  isAuthenticated: boolean;
  isLoading: boolean;
}

/**
 * Accept invite request payload
 */
export interface AcceptInviteRequest {
  token: string;
  password: string;
}

/**
 * Accept invite response
 */
export interface AcceptInviteResponse {
  message: string;
  access_token: string;
  token_type: string;
  expires_in: number;
}

/**
 * Verify invite request payload
 */
export interface VerifyInviteRequest {
  token: string;
}

/**
 * Verify invite response
 */
export interface VerifyInviteResponse {
  valid: boolean;
  message: string;
}

/**
 * Change password request payload
 */
export interface ChangePasswordRequest {
  current_password: string;
  new_password: string;
}

/**
 * Change password response
 */
export interface ChangePasswordResponse {
  message: string;
}

/**
 * Revoke session response
 */
export interface RevokeSessionResponse {
  message: string;
}

/**
 * API error response
 */
export interface ApiError {
  error: string;
  message?: string;
  details?: string[];
}

/**
 * Bootstrap request payload (first superadmin setup)
 */
export interface BootstrapRequest {
  organization_name: string;
  organization_slug: string;
  email: string;
  first_name: string;
  last_name: string;
  password: string;
  timezone?: string;
}

/**
 * Organization response
 */
export interface OrganizationResponse {
  id: string;
  name: string;
  slug: string;
  timezone: string;
  created_at: string;
  updated_at: string;
}

/**
 * User response (from bootstrap)
 */
export interface UserResponse {
  id: string;
  organization_id: string;
  email: string;
  first_name: string;
  last_name: string;
  role: UserRole;
  phone: string | null;
  created_at: string;
  updated_at: string;
  has_password: boolean;
}

/**
 * Bootstrap response
 */
export interface BootstrapResponse {
  message: string;
  organization: OrganizationResponse;
  user: UserResponse;
  access_token: string;
  token_type: string;
  expires_in: number;
}
