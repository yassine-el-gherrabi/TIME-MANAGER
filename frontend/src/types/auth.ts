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
  Admin = 'Admin',
  Manager = 'Manager',
  Employee = 'Employee'
}

/**
 * Token pair containing access and refresh tokens
 */
export interface TokenPair {
  access_token: string;
  refresh_token: string;
}

/**
 * User registration request payload
 */
export interface RegisterRequest {
  email: string;
  password: string;
  first_name: string;
  last_name: string;
  organization_id: string;
  role?: UserRole;
}

/**
 * User registration response
 */
export interface RegisterResponse {
  user: User;
  tokens: TokenPair;
}

/**
 * User login request payload
 */
export interface LoginRequest {
  email: string;
  password: string;
}

/**
 * User login response
 */
export interface LoginResponse {
  user: User;
  tokens: TokenPair;
}

/**
 * Token refresh request payload
 */
export interface RefreshRequest {
  refresh_token: string;
}

/**
 * Token refresh response
 */
export interface RefreshResponse {
  tokens: TokenPair;
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
  ip_address: string | null;
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
 */
export interface AuthState {
  user: User | null;
  accessToken: string | null;
  refreshToken: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

/**
 * API error response
 */
export interface ApiError {
  error: string;
  message?: string;
  details?: string[];
}
