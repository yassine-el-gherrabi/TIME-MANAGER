/**
 * Balances API Client
 *
 * API methods for leave balance management operations.
 */

import { apiRequest } from './client';
import { BALANCE_ENDPOINTS } from '../config/constants';
import type {
  LeaveBalance,
  SetBalanceRequest,
  AdjustBalanceRequest,
  BalanceFilter,
} from '../types/absence';

/**
 * Build query string from filter object
 */
function buildQueryString(filter?: BalanceFilter): string {
  if (!filter) return '';

  const params = new URLSearchParams();

  if (filter.user_id) params.append('user_id', filter.user_id);
  if (filter.absence_type_id) params.append('absence_type_id', filter.absence_type_id);
  if (filter.year) params.append('year', filter.year.toString());

  const queryString = params.toString();
  return queryString ? `?${queryString}` : '';
}

/**
 * Balances API methods
 */
export const balancesApi = {
  /**
   * Get current user's leave balances
   *
   * @returns User's leave balances for current year
   */
  getMyBalances: async (): Promise<LeaveBalance[]> => {
    return apiRequest<LeaveBalance[]>({
      method: 'GET',
      url: BALANCE_ENDPOINTS.MY_BALANCES,
    });
  },

  /**
   * List all balances with filters (Admin+)
   *
   * @param filter - Optional filter parameters
   * @returns List of leave balances
   */
  list: async (filter?: BalanceFilter): Promise<LeaveBalance[]> => {
    return apiRequest<LeaveBalance[]>({
      method: 'GET',
      url: `${BALANCE_ENDPOINTS.LIST}${buildQueryString(filter)}`,
    });
  },

  /**
   * Set initial balance for a user (Admin+)
   *
   * @param userId - User ID
   * @param data - Balance data
   * @returns Created/updated balance
   */
  setBalance: async (userId: string, data: SetBalanceRequest): Promise<LeaveBalance> => {
    return apiRequest<LeaveBalance>({
      method: 'POST',
      url: BALANCE_ENDPOINTS.SET(userId),
      data,
    });
  },

  /**
   * Adjust a leave balance (Admin+)
   *
   * @param balanceId - Balance ID
   * @param data - Adjustment data
   * @returns Updated balance
   */
  adjustBalance: async (balanceId: string, data: AdjustBalanceRequest): Promise<LeaveBalance> => {
    return apiRequest<LeaveBalance>({
      method: 'PUT',
      url: BALANCE_ENDPOINTS.ADJUST(balanceId),
      data,
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  getMyBalances,
  list: listBalances,
  setBalance,
  adjustBalance,
} = balancesApi;
