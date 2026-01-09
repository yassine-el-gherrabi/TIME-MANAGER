/**
 * Organizations API Client
 *
 * API methods for organization management (Super Admin only).
 */

import { apiRequest } from './client';
import { ORGANIZATION_ENDPOINTS } from '../config/constants';
import type {
  OrganizationResponse,
  CreateOrganizationRequest,
  UpdateOrganizationRequest,
  ListOrganizationsParams,
  PaginatedOrganizationsResponse,
  DeleteOrganizationResponse,
} from '../types/organization';

/**
 * Organizations API methods
 */
export const organizationsApi = {
  /**
   * List organizations with optional filters and pagination
   *
   * @param params - Query parameters for filtering and pagination
   * @returns Paginated list of organizations
   */
  list: async (params: ListOrganizationsParams = {}): Promise<PaginatedOrganizationsResponse> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }
    if (params.search) {
      queryParams.set('search', params.search);
    }

    const queryString = queryParams.toString();
    const url = queryString
      ? `${ORGANIZATION_ENDPOINTS.LIST}?${queryString}`
      : ORGANIZATION_ENDPOINTS.LIST;

    return apiRequest<PaginatedOrganizationsResponse>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get a single organization by ID
   *
   * @param id - Organization ID
   * @returns Organization details
   */
  get: async (id: string): Promise<OrganizationResponse> => {
    return apiRequest<OrganizationResponse>({
      method: 'GET',
      url: ORGANIZATION_ENDPOINTS.GET(id),
    });
  },

  /**
   * Create a new organization
   *
   * @param data - Organization creation data
   * @returns Created organization
   */
  create: async (data: CreateOrganizationRequest): Promise<OrganizationResponse> => {
    return apiRequest<OrganizationResponse>({
      method: 'POST',
      url: ORGANIZATION_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update an existing organization
   *
   * @param id - Organization ID
   * @param data - Fields to update
   * @returns Updated organization
   */
  update: async (id: string, data: UpdateOrganizationRequest): Promise<OrganizationResponse> => {
    return apiRequest<OrganizationResponse>({
      method: 'PUT',
      url: ORGANIZATION_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete an organization
   *
   * @param id - Organization ID
   * @returns Deletion confirmation
   */
  delete: async (id: string): Promise<DeleteOrganizationResponse> => {
    return apiRequest<DeleteOrganizationResponse>({
      method: 'DELETE',
      url: ORGANIZATION_ENDPOINTS.DELETE(id),
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listOrganizations,
  get: getOrganization,
  create: createOrganization,
  update: updateOrganization,
  delete: deleteOrganization,
} = organizationsApi;
