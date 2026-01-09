/**
 * Absences API Client
 *
 * API methods for absence/leave management operations.
 */

import { apiRequest } from './client';
import { ABSENCE_ENDPOINTS } from '../config/constants';
import type {
  Absence,
  PaginatedAbsences,
  CreateAbsenceRequest,
  RejectAbsenceRequest,
  AbsenceFilter,
} from '../types/absence';

/**
 * Build query string from filter object
 */
function buildQueryString(filter?: AbsenceFilter): string {
  if (!filter) return '';

  const params = new URLSearchParams();

  if (filter.user_id) params.append('user_id', filter.user_id);
  if (filter.type_id) params.append('type_id', filter.type_id);
  if (filter.status) params.append('status', filter.status);
  if (filter.start_date) params.append('start_date', filter.start_date);
  if (filter.end_date) params.append('end_date', filter.end_date);
  if (filter.page) params.append('page', filter.page.toString());
  if (filter.per_page) params.append('per_page', filter.per_page.toString());

  const queryString = params.toString();
  return queryString ? `?${queryString}` : '';
}

/**
 * Absences API methods
 */
export const absencesApi = {
  /**
   * List absences with optional filters
   *
   * @param filter - Optional filter parameters
   * @returns Paginated absences
   */
  list: async (filter?: AbsenceFilter): Promise<PaginatedAbsences> => {
    return apiRequest<PaginatedAbsences>({
      method: 'GET',
      url: `${ABSENCE_ENDPOINTS.LIST}${buildQueryString(filter)}`,
    });
  },

  /**
   * Get a single absence by ID
   *
   * @param id - Absence ID
   * @returns Absence details
   */
  get: async (id: string): Promise<Absence> => {
    return apiRequest<Absence>({
      method: 'GET',
      url: ABSENCE_ENDPOINTS.GET(id),
    });
  },

  /**
   * Create a new absence request
   *
   * @param data - Absence request data
   * @returns Created absence
   */
  create: async (data: CreateAbsenceRequest): Promise<Absence> => {
    return apiRequest<Absence>({
      method: 'POST',
      url: ABSENCE_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * List pending absences for approval (Manager+)
   *
   * @param page - Page number
   * @param perPage - Items per page
   * @returns Paginated pending absences
   */
  listPending: async (page = 1, perPage = 20): Promise<PaginatedAbsences> => {
    const params = new URLSearchParams({
      page: page.toString(),
      per_page: perPage.toString(),
    });
    return apiRequest<PaginatedAbsences>({
      method: 'GET',
      url: `${ABSENCE_ENDPOINTS.PENDING}?${params.toString()}`,
    });
  },

  /**
   * Approve an absence request (Manager+)
   *
   * @param id - Absence ID
   * @returns Approved absence
   */
  approve: async (id: string): Promise<Absence> => {
    return apiRequest<Absence>({
      method: 'POST',
      url: ABSENCE_ENDPOINTS.APPROVE(id),
    });
  },

  /**
   * Reject an absence request (Manager+)
   *
   * @param id - Absence ID
   * @param data - Rejection reason
   * @returns Rejected absence
   */
  reject: async (id: string, data?: RejectAbsenceRequest): Promise<Absence> => {
    return apiRequest<Absence>({
      method: 'POST',
      url: ABSENCE_ENDPOINTS.REJECT(id),
      data: data || {},
    });
  },

  /**
   * Cancel an absence request (owner only)
   *
   * @param id - Absence ID
   * @returns Cancelled absence
   */
  cancel: async (id: string): Promise<Absence> => {
    return apiRequest<Absence>({
      method: 'POST',
      url: ABSENCE_ENDPOINTS.CANCEL(id),
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listAbsences,
  get: getAbsence,
  create: createAbsence,
  listPending: listPendingAbsences,
  approve: approveAbsence,
  reject: rejectAbsence,
  cancel: cancelAbsence,
} = absencesApi;
