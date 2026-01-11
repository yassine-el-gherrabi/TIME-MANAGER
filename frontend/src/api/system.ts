/**
 * System API Client
 *
 * API methods for system-level operations.
 */

import { apiRequest } from './client';

/**
 * System status response
 */
export interface SystemStatusResponse {
  needs_setup: boolean;
  version: string;
}

/**
 * System endpoints
 */
export const SYSTEM_ENDPOINTS = {
  STATUS: '/system/status',
} as const;

/**
 * System API methods
 */
export const systemApi = {
  /**
   * Get system status
   * Returns whether the system needs initial setup
   */
  getStatus: async (): Promise<SystemStatusResponse> => {
    return apiRequest<SystemStatusResponse>({
      method: 'GET',
      url: SYSTEM_ENDPOINTS.STATUS,
    });
  },
};
