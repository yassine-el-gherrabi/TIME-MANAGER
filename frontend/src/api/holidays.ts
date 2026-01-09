/**
 * Holidays API Client
 *
 * API methods for holiday management operations.
 */

import { apiRequest } from './client';
import { HOLIDAY_ENDPOINTS } from '../config/constants';
import type {
  Holiday,
  CreateHolidayRequest,
  UpdateHolidayRequest,
  HolidayFilter,
} from '../types/absence';

/**
 * Build query string from filter object
 */
function buildQueryString(filter?: HolidayFilter): string {
  if (!filter) return '';

  const params = new URLSearchParams();

  if (filter.start_date) params.append('start_date', filter.start_date);
  if (filter.end_date) params.append('end_date', filter.end_date);
  if (filter.is_recurring !== undefined) params.append('is_recurring', filter.is_recurring.toString());

  const queryString = params.toString();
  return queryString ? `?${queryString}` : '';
}

/**
 * Holidays API methods
 */
export const holidaysApi = {
  /**
   * List all holidays for the organization
   *
   * @param filter - Optional filter parameters
   * @returns List of holidays
   */
  list: async (filter?: HolidayFilter): Promise<Holiday[]> => {
    return apiRequest<Holiday[]>({
      method: 'GET',
      url: `${HOLIDAY_ENDPOINTS.LIST}${buildQueryString(filter)}`,
    });
  },

  /**
   * Get a single holiday by ID
   *
   * @param id - Holiday ID
   * @returns Holiday details
   */
  get: async (id: string): Promise<Holiday> => {
    return apiRequest<Holiday>({
      method: 'GET',
      url: HOLIDAY_ENDPOINTS.GET(id),
    });
  },

  /**
   * Create a new holiday (Admin+)
   *
   * @param data - Holiday creation data
   * @returns Created holiday
   */
  create: async (data: CreateHolidayRequest): Promise<Holiday> => {
    return apiRequest<Holiday>({
      method: 'POST',
      url: HOLIDAY_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update an existing holiday (Admin+)
   *
   * @param id - Holiday ID
   * @param data - Fields to update
   * @returns Updated holiday
   */
  update: async (id: string, data: UpdateHolidayRequest): Promise<Holiday> => {
    return apiRequest<Holiday>({
      method: 'PUT',
      url: HOLIDAY_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete a holiday (Admin+)
   *
   * @param id - Holiday ID
   */
  delete: async (id: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: HOLIDAY_ENDPOINTS.DELETE(id),
    });
  },

  /**
   * Seed default French holidays (Admin+)
   *
   * @returns Success message
   */
  seed: async (): Promise<{ message: string }> => {
    return apiRequest<{ message: string }>({
      method: 'POST',
      url: HOLIDAY_ENDPOINTS.SEED,
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listHolidays,
  get: getHoliday,
  create: createHoliday,
  update: updateHoliday,
  delete: deleteHoliday,
  seed: seedHolidays,
} = holidaysApi;
