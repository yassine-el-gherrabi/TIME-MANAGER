/**
 * User Management Types
 *
 * TypeScript type definitions for user CRUD operations
 * used by admin interfaces.
 */

import { UserRole } from './auth';

/**
 * Invite status for users
 */
export type InviteStatus = 'pending' | 'accepted';

/**
 * User response from API with invite status
 */
export interface UserResponse {
  id: string;
  email: string;
  first_name: string;
  last_name: string;
  role: UserRole;
  organization_id: string;
  has_password: boolean;
  created_at: string;
  updated_at: string;
}

/**
 * Create user request payload
 */
export interface CreateUserRequest {
  email: string;
  first_name: string;
  last_name: string;
  role: UserRole;
}

/**
 * Create user response
 */
export interface CreateUserResponse {
  user: UserResponse;
  invite_token: string;
  message: string;
}

/**
 * Update user request payload
 */
export interface UpdateUserRequest {
  email?: string;
  first_name?: string;
  last_name?: string;
  role?: UserRole;
}

/**
 * List users query parameters
 */
export interface ListUsersParams {
  page?: number;
  per_page?: number;
  role?: UserRole;
  search?: string;
}

/**
 * Paginated users response
 */
export interface PaginatedUsersResponse {
  data: UserResponse[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

/**
 * Resend invite response
 */
export interface ResendInviteResponse {
  message: string;
  invite_token: string;
}

/**
 * Delete user response
 */
export interface DeleteUserResponse {
  message: string;
}
