/**
 * Clock Restrictions API Client
 *
 * API methods for clock restriction configuration and override requests.
 */

import { apiRequest } from './client';
import { CLOCK_RESTRICTION_ENDPOINTS } from '../config/constants';
import type {
  ClockRestrictionResponse,
  CreateClockRestrictionRequest,
  UpdateClockRestrictionRequest,
  ClockRestrictionFilter,
  ClockValidationResult,
  ClockOverrideRequestResponse,
  CreateOverrideRequest,
  ReviewOverrideRequest,
  ClockOverrideFilter,
  PaginatedOverrideRequests,
} from '../types/clockRestriction';

/**
 * Clock Restrictions API methods
 */
export const clockRestrictionsApi = {
  /**
   * List clock restrictions
   *
   * @param params - Filter and pagination parameters
   * @returns Array of clock restrictions (backend returns array directly)
   */
  list: async (params: ClockRestrictionFilter = {}): Promise<ClockRestrictionResponse[]> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }
    if (params.team_id) {
      queryParams.set('team_id', params.team_id);
    }
    if (params.user_id) {
      queryParams.set('user_id', params.user_id);
    }
    if (params.mode) {
      queryParams.set('mode', params.mode);
    }
    if (params.is_active !== undefined) {
      queryParams.set('is_active', params.is_active.toString());
    }

    const queryString = queryParams.toString();
    const url = queryString
      ? `${CLOCK_RESTRICTION_ENDPOINTS.LIST}?${queryString}`
      : CLOCK_RESTRICTION_ENDPOINTS.LIST;

    return apiRequest<ClockRestrictionResponse[]>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get a single clock restriction
   *
   * @param id - Restriction ID
   * @returns Clock restriction details
   */
  get: async (id: string): Promise<ClockRestrictionResponse> => {
    return apiRequest<ClockRestrictionResponse>({
      method: 'GET',
      url: CLOCK_RESTRICTION_ENDPOINTS.GET(id),
    });
  },

  /**
   * Create a new clock restriction
   *
   * @param data - Restriction configuration
   * @returns Created restriction
   */
  create: async (data: CreateClockRestrictionRequest): Promise<ClockRestrictionResponse> => {
    return apiRequest<ClockRestrictionResponse>({
      method: 'POST',
      url: CLOCK_RESTRICTION_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update a clock restriction
   *
   * @param id - Restriction ID
   * @param data - Updated configuration
   * @returns Updated restriction
   */
  update: async (id: string, data: UpdateClockRestrictionRequest): Promise<ClockRestrictionResponse> => {
    return apiRequest<ClockRestrictionResponse>({
      method: 'PUT',
      url: CLOCK_RESTRICTION_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete a clock restriction
   *
   * @param id - Restriction ID
   */
  delete: async (id: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: CLOCK_RESTRICTION_ENDPOINTS.DELETE(id),
    });
  },

  /**
   * Validate if a clock action is currently allowed
   *
   * @param action - 'clock_in' or 'clock_out'
   * @returns Validation result with details
   */
  validateAction: async (action: 'clock_in' | 'clock_out'): Promise<ClockValidationResult> => {
    const queryParams = new URLSearchParams();
    queryParams.set('action', action);

    return apiRequest<ClockValidationResult>({
      method: 'GET',
      url: `${CLOCK_RESTRICTION_ENDPOINTS.VALIDATE}?${queryParams.toString()}`,
    });
  },

  /**
   * Create an override request (for flexible mode)
   *
   * @param data - Override request with justification
   * @returns Created override request
   */
  createOverrideRequest: async (data: CreateOverrideRequest): Promise<ClockOverrideRequestResponse> => {
    return apiRequest<ClockOverrideRequestResponse>({
      method: 'POST',
      url: CLOCK_RESTRICTION_ENDPOINTS.OVERRIDES,
      data,
    });
  },

  /**
   * List pending override requests (Manager+ only)
   *
   * @param params - Filter and pagination parameters
   * @returns Paginated pending override requests
   */
  listPendingOverrides: async (params: ClockOverrideFilter = {}): Promise<PaginatedOverrideRequests> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }
    if (params.status) {
      queryParams.set('status', params.status);
    }
    if (params.requested_action) {
      queryParams.set('requested_action', params.requested_action);
    }

    const queryString = queryParams.toString();
    const url = queryString
      ? `${CLOCK_RESTRICTION_ENDPOINTS.PENDING_OVERRIDES}?${queryString}`
      : CLOCK_RESTRICTION_ENDPOINTS.PENDING_OVERRIDES;

    return apiRequest<PaginatedOverrideRequests>({
      method: 'GET',
      url,
    });
  },

  /**
   * List user's own override requests
   *
   * @param params - Filter and pagination parameters
   * @returns Paginated user override requests
   */
  listMyOverrides: async (params: ClockOverrideFilter = {}): Promise<PaginatedOverrideRequests> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }
    if (params.status) {
      queryParams.set('status', params.status);
    }
    if (params.requested_action) {
      queryParams.set('requested_action', params.requested_action);
    }

    const queryString = queryParams.toString();
    const url = queryString
      ? `${CLOCK_RESTRICTION_ENDPOINTS.MY_OVERRIDES}?${queryString}`
      : CLOCK_RESTRICTION_ENDPOINTS.MY_OVERRIDES;

    return apiRequest<PaginatedOverrideRequests>({
      method: 'GET',
      url,
    });
  },

  /**
   * Review (approve/reject) an override request (Manager+ only)
   *
   * @param id - Override request ID
   * @param data - Review decision
   * @returns Updated override request
   */
  reviewOverride: async (id: string, data: ReviewOverrideRequest): Promise<ClockOverrideRequestResponse> => {
    return apiRequest<ClockOverrideRequestResponse>({
      method: 'POST',
      url: CLOCK_RESTRICTION_ENDPOINTS.REVIEW_OVERRIDE(id),
      data,
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listClockRestrictions,
  get: getClockRestriction,
  create: createClockRestriction,
  update: updateClockRestriction,
  delete: deleteClockRestriction,
  validateAction: validateClockAction,
  createOverrideRequest,
  listPendingOverrides,
  listMyOverrides,
  reviewOverride,
} = clockRestrictionsApi;
