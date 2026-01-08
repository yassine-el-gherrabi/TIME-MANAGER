/**
 * Clocks API Client
 *
 * API methods for clock in/out operations.
 */

import { apiRequest } from './client';
import { CLOCK_ENDPOINTS } from '../config/constants';
import type {
  ClockEntry,
  ClockStatus,
  ClockInRequest,
  ClockOutRequest,
  RejectEntryRequest,
  ClockHistoryParams,
  PaginatedClockHistoryResponse,
  PendingEntriesParams,
  PaginatedPendingResponse,
} from '../types/clock';

/**
 * Clocks API methods
 */
export const clocksApi = {
  /**
   * Clock in
   *
   * @param data - Optional notes
   * @returns Created clock entry
   */
  clockIn: async (data?: ClockInRequest): Promise<ClockEntry> => {
    return apiRequest<ClockEntry>({
      method: 'POST',
      url: CLOCK_ENDPOINTS.CLOCK_IN,
      data: data || {},
    });
  },

  /**
   * Clock out
   *
   * @param data - Optional notes
   * @returns Updated clock entry
   */
  clockOut: async (data?: ClockOutRequest): Promise<ClockEntry> => {
    return apiRequest<ClockEntry>({
      method: 'POST',
      url: CLOCK_ENDPOINTS.CLOCK_OUT,
      data: data || {},
    });
  },

  /**
   * Get current clock status
   *
   * @returns Current clock status
   */
  getStatus: async (): Promise<ClockStatus> => {
    return apiRequest<ClockStatus>({
      method: 'GET',
      url: CLOCK_ENDPOINTS.STATUS,
    });
  },

  /**
   * Get clock history
   *
   * @param params - Query parameters for filtering and pagination
   * @returns Paginated clock history
   */
  getHistory: async (params: ClockHistoryParams = {}): Promise<PaginatedClockHistoryResponse> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }
    if (params.start_date) {
      queryParams.set('start_date', params.start_date);
    }
    if (params.end_date) {
      queryParams.set('end_date', params.end_date);
    }
    if (params.status) {
      queryParams.set('status', params.status);
    }

    const queryString = queryParams.toString();
    const url = queryString ? `${CLOCK_ENDPOINTS.HISTORY}?${queryString}` : CLOCK_ENDPOINTS.HISTORY;

    return apiRequest<PaginatedClockHistoryResponse>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get pending entries for approval (Manager+)
   *
   * @param params - Query parameters for pagination
   * @returns Paginated pending entries
   */
  getPending: async (params: PendingEntriesParams = {}): Promise<PaginatedPendingResponse> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }

    const queryString = queryParams.toString();
    const url = queryString ? `${CLOCK_ENDPOINTS.PENDING}?${queryString}` : CLOCK_ENDPOINTS.PENDING;

    return apiRequest<PaginatedPendingResponse>({
      method: 'GET',
      url,
    });
  },

  /**
   * Approve a clock entry (Manager+)
   *
   * @param id - Entry ID
   * @returns Approved entry
   */
  approve: async (id: string): Promise<ClockEntry> => {
    return apiRequest<ClockEntry>({
      method: 'POST',
      url: CLOCK_ENDPOINTS.APPROVE(id),
    });
  },

  /**
   * Reject a clock entry (Manager+)
   *
   * @param id - Entry ID
   * @param data - Optional rejection reason
   * @returns Rejected entry
   */
  reject: async (id: string, data?: RejectEntryRequest): Promise<ClockEntry> => {
    return apiRequest<ClockEntry>({
      method: 'POST',
      url: CLOCK_ENDPOINTS.REJECT(id),
      data: data || {},
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  clockIn,
  clockOut,
  getStatus,
  getHistory,
  getPending,
  approve: approveEntry,
  reject: rejectEntry,
} = clocksApi;
