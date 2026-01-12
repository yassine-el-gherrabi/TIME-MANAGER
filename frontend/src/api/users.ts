/**
 * Users API Client
 *
 * API methods for user management operations (Admin only).
 */

import { apiRequest } from './client';
import { USER_ENDPOINTS } from '../config/constants';
import type {
  UserResponse,
  CreateUserRequest,
  CreateUserResponse,
  UpdateUserRequest,
  ListUsersParams,
  PaginatedUsersResponse,
  ResendInviteResponse,
  DeleteUserResponse,
} from '../types/user';

/**
 * Users API methods
 */
export const usersApi = {
  /**
   * List users with optional filters and pagination
   *
   * @param params - Query parameters for filtering and pagination
   * @returns Paginated list of users
   */
  list: async (params: ListUsersParams = {}): Promise<PaginatedUsersResponse> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }
    if (params.role) {
      queryParams.set('role', params.role);
    }
    if (params.search) {
      queryParams.set('search', params.search);
    }
    if (params.include_deleted) {
      queryParams.set('include_deleted', 'true');
    }
    if (params.organization_id) {
      queryParams.set('organization_id', params.organization_id);
    }
    if (params.team_id) {
      queryParams.set('team_id', params.team_id);
    }

    const queryString = queryParams.toString();
    const url = queryString ? `${USER_ENDPOINTS.LIST}?${queryString}` : USER_ENDPOINTS.LIST;

    return apiRequest<PaginatedUsersResponse>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get a single user by ID
   *
   * @param id - User ID
   * @returns User details
   */
  get: async (id: string): Promise<UserResponse> => {
    return apiRequest<UserResponse>({
      method: 'GET',
      url: USER_ENDPOINTS.GET(id),
    });
  },

  /**
   * Create a new user (sends invitation email)
   *
   * @param data - User creation data
   * @returns Created user and invite token
   */
  create: async (data: CreateUserRequest): Promise<CreateUserResponse> => {
    return apiRequest<CreateUserResponse>({
      method: 'POST',
      url: USER_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update an existing user
   *
   * @param id - User ID
   * @param data - Fields to update
   * @returns Updated user
   */
  update: async (id: string, data: UpdateUserRequest): Promise<UserResponse> => {
    return apiRequest<UserResponse>({
      method: 'PUT',
      url: USER_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete a user (soft delete)
   *
   * @param id - User ID
   * @returns Deletion confirmation
   */
  delete: async (id: string): Promise<DeleteUserResponse> => {
    return apiRequest<DeleteUserResponse>({
      method: 'DELETE',
      url: USER_ENDPOINTS.DELETE(id),
    });
  },

  /**
   * Resend invitation email to a pending user
   *
   * @param id - User ID
   * @returns New invite token
   */
  resendInvite: async (id: string): Promise<ResendInviteResponse> => {
    return apiRequest<ResendInviteResponse>({
      method: 'POST',
      url: USER_ENDPOINTS.RESEND_INVITE(id),
    });
  },

  /**
   * Restore a soft-deleted user
   *
   * @param id - User ID
   * @returns Restored user
   */
  restore: async (id: string): Promise<UserResponse> => {
    return apiRequest<UserResponse>({
      method: 'PUT',
      url: USER_ENDPOINTS.RESTORE(id),
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listUsers,
  get: getUser,
  create: createUser,
  update: updateUser,
  delete: deleteUser,
  resendInvite,
  restore: restoreUser,
} = usersApi;
