/**
 * Closed Days API Client
 *
 * API methods for closed day (company holidays) management operations.
 */

import { apiRequest } from './client';
import { CLOSED_DAY_ENDPOINTS } from '../config/constants';
import type {
  ClosedDay,
  CreateClosedDayRequest,
  UpdateClosedDayRequest,
  ClosedDayFilter,
} from '../types/absence';

/**
 * Build query string from filter object
 */
function buildQueryString(filter?: ClosedDayFilter): string {
  if (!filter) return '';

  const params = new URLSearchParams();

  if (filter.start_date) params.append('start_date', filter.start_date);
  if (filter.end_date) params.append('end_date', filter.end_date);
  if (filter.is_recurring !== undefined) params.append('is_recurring', filter.is_recurring.toString());
  if (filter.organization_id) params.append('organization_id', filter.organization_id);

  const queryString = params.toString();
  return queryString ? `?${queryString}` : '';
}

/**
 * Closed Days API methods
 */
export const closedDaysApi = {
  /**
   * List all closed days for the organization
   *
   * @param filter - Optional filter parameters
   * @returns List of closed days
   */
  list: async (filter?: ClosedDayFilter): Promise<ClosedDay[]> => {
    return apiRequest<ClosedDay[]>({
      method: 'GET',
      url: `${CLOSED_DAY_ENDPOINTS.LIST}${buildQueryString(filter)}`,
    });
  },

  /**
   * Get a single closed day by ID
   *
   * @param id - Closed day ID
   * @returns Closed day details
   */
  get: async (id: string): Promise<ClosedDay> => {
    return apiRequest<ClosedDay>({
      method: 'GET',
      url: CLOSED_DAY_ENDPOINTS.GET(id),
    });
  },

  /**
   * Create a new closed day (Admin+)
   *
   * @param data - Closed day creation data
   * @returns Created closed day
   */
  create: async (data: CreateClosedDayRequest): Promise<ClosedDay> => {
    return apiRequest<ClosedDay>({
      method: 'POST',
      url: CLOSED_DAY_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update an existing closed day (Admin+)
   *
   * @param id - Closed day ID
   * @param data - Fields to update
   * @returns Updated closed day
   */
  update: async (id: string, data: UpdateClosedDayRequest): Promise<ClosedDay> => {
    return apiRequest<ClosedDay>({
      method: 'PUT',
      url: CLOSED_DAY_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete a closed day (Admin+)
   *
   * @param id - Closed day ID
   */
  delete: async (id: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: CLOSED_DAY_ENDPOINTS.DELETE(id),
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listClosedDays,
  get: getClosedDay,
  create: createClosedDay,
  update: updateClosedDay,
  delete: deleteClosedDay,
} = closedDaysApi;
