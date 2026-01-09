/**
 * Organization Types
 *
 * TypeScript type definitions for organization management (Super Admin only).
 */

/**
 * Organization response from API
 */
export interface OrganizationResponse {
  id: string;
  name: string;
  slug: string;
  timezone: string;
  created_at: string;
  updated_at: string;
  user_count?: number;
}

/**
 * Create organization request payload
 */
export interface CreateOrganizationRequest {
  name: string;
  slug: string;
  timezone?: string;
}

/**
 * Update organization request payload
 */
export interface UpdateOrganizationRequest {
  name?: string;
  timezone?: string;
}

/**
 * List organizations query parameters
 */
export interface ListOrganizationsParams {
  page?: number;
  per_page?: number;
  search?: string;
}

/**
 * Paginated organizations response
 */
export interface PaginatedOrganizationsResponse {
  data: OrganizationResponse[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

/**
 * Delete organization response
 */
export interface DeleteOrganizationResponse {
  message: string;
}
